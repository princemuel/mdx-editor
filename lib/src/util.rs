use serde::{Deserialize, Serialize};

use crate::{interfaces::Transaction, sha256::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleRoot(Hash);
impl MerkleRoot {
    /// calculate the merkle root of a block's transactions
    pub fn new(transactions: &[Transaction]) -> Self {
        let mut layer = vec![];
        for tx in transactions {
            layer.push(Hash::new(tx));
        }

        while layer.len() > 1 {
            let mut new_layer = vec![];
            for pair in layer.chunks(2) {
                let left = pair[0];
                // if there is no right, use the left hash again
                let right = pair.get(1).unwrap_or(&pair[0]);
                new_layer.push(Hash::new(&[left, *right]));
            }
            layer = new_layer;
        }
        MerkleRoot(layer[0])
    }
}
