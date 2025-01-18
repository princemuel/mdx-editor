use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crypto::{PublicKey, Signature};
use crate::error::{BtcError, Result};
use crate::sha256::Hash;
use crate::utils::MerkleRoot;
use crate::U256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub utxos: HashMap<Hash, TxOut>,
}
impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![], utxos: HashMap::new() }
    }
    /// try to add a new block to the blockchain, return an error if it's not valid
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        if self.blocks.is_empty() {
            if block.header.prev_block_hash != Hash::zero() {
                println!("zero hash");
                return Err(BtcError::InvalidBlock);
            }
        } else {
            let last_block = self.blocks.last().unwrap();
            if block.header.prev_block_hash != last_block.hash() {
                println!("the previous block hash is invalid");
                return Err(BtcError::InvalidBlock);
            }

            if !block.header.hash().matches_target(block.header.target) {
                println!("the block header does not match target");
                return Err(BtcError::InvalidBlock);
            }

            let merkel_root = MerkleRoot::new(&block.transactions);
            if merkel_root != block.header.merkle_root {
                println!("invalid merkle root");
                return Err(BtcError::InvalidMerkleRoot);
            }

            if block.header.timestamp <= last_block.header.timestamp {
                return Err(BtcError::InvalidBlock);
            }

            // block.verify_transactions(self.block_height(), &self.utxos)?;
        }
        self.blocks.push(block);
        Ok(())
    }

    /// Rebuild the UTXO set from the blockchain
    pub fn rebuild_utxos(&mut self) {
        for block in &self.blocks {
            for tx in &block.transactions {
                for input in &tx.inputs {
                    self.utxos.remove(&input.prev_transaction_output_hash);
                }

                for output in tx.outputs.iter() {
                    self.utxos.insert(tx.hash(), output.clone());
                }
            }
        }
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Blockchain::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block { header, transactions }
    }
    pub fn hash(&self) -> Hash {
        Hash::new(self)
    }
    pub fn verify_transactions(
        &self,
        utxos: &HashMap<Hash, TxOut>,
    ) -> Result<()> {
        let mut inputs = HashMap::new();
        if self.transactions.is_empty() {
            return Err(BtcError::InvalidTransaction);
        }
        for tx in &self.transactions {
            let mut input_value = 0;
            let mut output_value = 0;

            for input in &tx.inputs {
                let prev_output =
                    utxos.get(&input.prev_transaction_output_hash);

                if prev_output.is_none() {
                    return Err(BtcError::InvalidTransaction);
                }
                let prev_output = prev_output.unwrap();

                // prevents same-block double-spending
                if inputs.contains_key(&input.prev_transaction_output_hash) {
                    return Err(BtcError::InvalidTransaction);
                }

                // check if the signature is valid
                if !input.signature.verify(
                    &input.prev_transaction_output_hash,
                    &prev_output.pubkey,
                ) {
                    return Err(BtcError::InvalidSignature);
                }

                input_value += prev_output.value;
                inputs.insert(
                    input.prev_transaction_output_hash,
                    prev_output.clone(),
                );
            }

            for output in &tx.outputs {
                output_value += output.value;
            }

            // It is fine for output value to be less than input value
            // as the difference is the fee for the miner
            if input_value < output_value {
                return Err(BtcError::InvalidTransaction);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// The timestamp of the block
    pub timestamp: DateTime<Utc>,
    /// The nonce used to mine the block
    pub nonce: u64,
    /// The hash of the previous block
    pub prev_block_hash: Hash,
    /// The merkle root of the block's transactions
    pub merkle_root: MerkleRoot,
    /// The target number, which has to be higher than the hash of the block
    pub target: U256,
}
impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
    ) -> Self {
        BlockHeader { timestamp, nonce, prev_block_hash, merkle_root, target }
    }
    pub fn hash(&self) -> Hash {
        Hash::new(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
}
impl Transaction {
    pub fn new(inputs: Vec<TxIn>, outputs: Vec<TxOut>) -> Self {
        Transaction { inputs, outputs }
    }
    pub fn hash(&self) -> Hash {
        Hash::new(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxIn {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOut {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: PublicKey,
}
impl TxOut {
    pub fn hash(&self) -> Hash {
        Hash::new(self)
    }
}
