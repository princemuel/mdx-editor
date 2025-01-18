use ecdsa::{Signature as ECDSASignature, SigningKey, VerifyingKey};
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(ECDSASignature<Secp256k1>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey(VerifyingKey<Secp256k1>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateKey(
    #[serde(with = "signkey_serde")] pub SigningKey<Secp256k1>,
);
impl PrivateKey {
    pub fn new() -> Self {
        PrivateKey(SigningKey::random(&mut rand::thread_rng()))
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

mod signkey_serde {
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
