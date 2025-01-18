use ecdsa::signature::Verifier;
use ecdsa::{
    signature::Signer, Signature as ECDSASignature, SigningKey, VerifyingKey,
};
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::sha256::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(ECDSASignature<Secp256k1>);
impl Signature {
    /// signs a Transaction Output from its' Sha256 hash
    pub fn sign(hash: &Hash, private_key: &PrivateKey) -> Self {
        let signing_key = &private_key.0;
        let signature = signing_key.sign(&hash.as_bytes());
        Self(signature)
    }
    /// verifies a signature
    pub fn verify(&self, hash: &Hash, key: &PublicKey) -> bool {
        key.0.verify(&hash.as_bytes(), &self.0).is_ok()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey(VerifyingKey<Secp256k1>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateKey(#[serde(with = "transformer")] pub SigningKey<Secp256k1>);
impl PrivateKey {
    pub fn new() -> Self {
        Self(SigningKey::random(&mut rand::thread_rng()))
    }
    pub fn pubkey(&self) -> PublicKey {
        PublicKey(*self.0.verifying_key())
    }
}
impl Default for PrivateKey {
    fn default() -> Self {
        Self::new()
    }
}

mod transformer {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(
        key: &SigningKey<Secp256k1>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }

    pub fn deserialize<'d, D>(
        deserializer: D,
    ) -> Result<SigningKey<Secp256k1>, D::Error>
    where
        D: Deserializer<'d>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Ok(SigningKey::from_slice(&bytes).unwrap())
    }
}
