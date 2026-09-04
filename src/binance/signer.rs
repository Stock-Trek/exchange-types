use crate::{
    api_key_credential::ApiKeyCredentials, encode::ByteEncoder, encrypt::EncryptionAlgorithm,
    error::ETResult, signer::Signer,
};

pub struct SignerFactory;

impl SignerFactory {
    pub fn hmac_sha256(credentials: ApiKeyCredentials) -> ETResult<Signer> {
        Self::signer(
            credentials,
            EncryptionAlgorithm::HmacSha256,
            ByteEncoder::HexLower,
        )
    }
    pub fn rsa_sha256(credentials: ApiKeyCredentials) -> ETResult<Signer> {
        Self::signer(
            credentials,
            EncryptionAlgorithm::RsaSha256,
            ByteEncoder::Base64,
        )
    }
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
