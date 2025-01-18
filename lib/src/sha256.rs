use std::fmt;

use serde::{Deserialize, Serialize};

use crate::U256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash(U256);
impl Hash {
    /// hash anything that can be serde Serialized via ciborium
    pub fn new<T: Serialize>(data: &T) -> Self {
        // ? check performance using a array buffer vs vector
        let mut buffer = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut buffer) {
            panic!("Failed to serialize data: {e:?}.  This should not happen",);
        }

        let hash = sha256::digest(&buffer);
        let hash_bytes = hex::decode(hash).unwrap();
        let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();

        Hash(U256::from_big_endian(&hash_array))
    }

    /// convert hash to bytes
    pub fn as_bytes(&self) -> [u8; 32] {
        self.0.to_little_endian()
    }

    /// check if a hash matches a target
    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }
    /// zero hash
    pub fn zero() -> Self {
        Hash(U256::zero())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
