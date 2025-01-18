use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use crate::U256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
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
