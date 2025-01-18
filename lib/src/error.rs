use thiserror::Error;

#[derive(Debug, Error)]
pub enum BtcError {
    #[error("This transaction is invalid")]
    InvalidTransaction,
    #[error("This block is invalid")]
    InvalidBlock,
    #[error("This block header is invalid")]
    InvalidBlockHeader,
    #[error("This transaction input is invalid")]
    InvalidTransactionInput,
    #[error("This transaction output is invalid")]
    InvalidTransactionOutput,
    #[error("This merkle root is invalid")]
    InvalidMerkleRoot,
    #[error("This hash is invalid")]
    InvalidHash,
    #[error("This signature is invalid")]
    InvalidSignature,
    #[error("This public key is invalid")]
    InvalidPublicKey,
    #[error("This private key is invalid")]
    InvalidPrivateKey,
}

pub type Result<T> = std::result::Result<T, BtcError>;
