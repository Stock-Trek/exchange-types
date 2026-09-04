use crate::{
    api_key_credential::ApiKeyCredentials, encode::ByteEncoder, encrypt::EncryptionAlgorithm,
    error::ETResult, signer::Signer,
};

/// Creates Binance `Signer`s wired with the only (encrypt, encode) method
/// combinations accepted by the Binance REST and WebSocket APIs.
pub struct SignerFactory;

impl SignerFactory {
    /// HMAC-SHA256 encrypted, lowercase hex encoded.
    pub fn hmac_sha256(credentials: ApiKeyCredentials) -> ETResult<Signer> {
        Self::signer(
            credentials,
            EncryptionAlgorithm::HmacSha256,
            ByteEncoder::HexLower,
        )
    }

    /// RSA PKCS#1 v1.5 SHA-256 encrypted, base64 encoded.
    pub fn rsa_sha256(credentials: ApiKeyCredentials) -> ETResult<Signer> {
        Self::signer(
            credentials,
            EncryptionAlgorithm::RsaSha256,
            ByteEncoder::Base64,
        )
    }

    /// Ed25519 encrypted, base64 encoded.
    pub fn ed25519(credentials: ApiKeyCredentials) -> ETResult<Signer> {
        Self::signer(
            credentials,
            EncryptionAlgorithm::Ed25519,
            ByteEncoder::Base64,
        )
    }

    fn signer(
        credentials: ApiKeyCredentials,
        algorithm: EncryptionAlgorithm,
        encoder: ByteEncoder,
    ) -> ETResult<Signer> {
        let api_key = credentials.api_key.clone();
        let encryptor = algorithm.encryptor(credentials)?;
        Ok(Signer::new(api_key, encryptor, encoder))
    }
}
