pub type ETResult<T> = Result<T, ETError>;

#[derive(Debug, thiserror::Error)]
pub enum ETError {
    #[error("Crypto key error: {0}")]
    CryptoKey(String),
    #[error(
        "apiKey must not be set on signed HTTP request params; it is sent via the API key header and excluded from the signed query string"
    )]
    HttpParamsApiKey,
    #[cfg(feature = "serde")]
    #[error("Deserialize response error: {0}")]
    DeserializeResponse(serde_json::Error),
    #[cfg(feature = "serde")]
    #[error("Serialize request error: {0}")]
    SerializeRequest(serde_json::Error),
}
