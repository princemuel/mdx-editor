use uuid::Uuid;

use crate::U256;

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

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block { header, transactions }
    }
    pub fn hash(&self) -> ! {
        todo!()
    }
}

pub struct BlockHeader {
    /// The timestamp of the block
    pub timestamp: u64,
    /// The nonce used to mine the block
    pub nonce: u64,
    /// The hash of the previous block
    pub prev_block_hash: [u8; 32],
    /// The merkle root of the block's transactions
    pub merkle_root: [u8; 32],
    /// The target number, which has to be higher than the hash of the block
    pub target: U256,
}
impl BlockHeader {
    pub fn new(
        timestamp: u64,
        nonce: u64,
        prev_block_hash: [u8; 32],
        merkle_root: [u8; 32],
        target: U256,
    ) -> Self {
        BlockHeader { timestamp, nonce, prev_block_hash, merkle_root, target }
    }
    pub fn hash(&self) -> ! {
        todo!()
    }
}

pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

pub struct TransactionInput {
    pub prev_transaction_output_hash: [u8; 32],
    pub signature: [u8; 64],
}
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: [u8; 33],
}
