pub type EncryptResult<T> = Result<T, EncryptError>;

#[derive(Debug, thiserror::Error)]
pub enum EncryptError {
    #[error("Crypto key error: {0}")]
    CryptoKey(String),
}
