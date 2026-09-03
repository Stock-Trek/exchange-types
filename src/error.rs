pub type ETResult<T> = Result<T, ETError>;

#[derive(Debug, thiserror::Error)]
pub enum ETError {
    #[error("Crypto key error: {0}")]
    CryptoKey(String),
    #[error("Serialize request error: {0}")]
    SerializeRequest(serde_json::Error),
    #[error("Deserialize response error: {0}")]
    DeserializeResponse(serde_json::Error),
}
