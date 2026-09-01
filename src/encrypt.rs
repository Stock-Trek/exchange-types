use crate::{
    api_key_credential::ApiKeyCredentials,
    error::{EncryptError, EncryptResult},
};
use hmac::{Hmac, Mac};
use p256::ecdsa::signature::Signer as SignerTrait;
use secrecy::{ExposeSecret, SecretSlice};
use sha2::{Sha256, Sha512};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionAlgorithm {
    EcdsaP256,
    EcdsaP384,
    Ed25519,
    HmacSha256,
    HmacSha512,
}

#[derive(Debug, Display)]
pub enum Encryptor {
    EcdsaP256(p256::ecdsa::SigningKey),
    EcdsaP384(p384::ecdsa::SigningKey),
    Ed25519(ed25519_compact::SecretKey),
    HmacSha256(secrecy::SecretSlice<u8>),
    HmacSha512(secrecy::SecretSlice<u8>),
}

impl EncryptionAlgorithm {
    pub fn encryptor(&self, api_key_credentials: ApiKeyCredentials) -> EncryptResult<Encryptor> {
        let secret_key_bytes = api_key_credentials.secret.expose_secret().as_bytes();
        match self {
            Self::EcdsaP256 => {
                let signing_key = p256::ecdsa::SigningKey::from_slice(secret_key_bytes)
                    .map_err(|e| EncryptError::CryptoKey(format!("ECDSA P-256 key error: {e}")))?;
                Ok(Encryptor::EcdsaP256(signing_key))
            }
            Self::EcdsaP384 => {
                let signing_key = p384::ecdsa::SigningKey::from_slice(secret_key_bytes)
                    .map_err(|e| EncryptError::CryptoKey(format!("ECDSA P-384 key error: {e}")))?;
                Ok(Encryptor::EcdsaP384(signing_key))
            }
            Self::Ed25519 => {
                let signing_key = ed25519_compact::SecretKey::from_slice(secret_key_bytes)
                    .map_err(|_| {
                        EncryptError::CryptoKey("Ed25519 key must be exactly 32 bytes".to_string())
                    })?;
                Ok(Encryptor::Ed25519(signing_key))
            }
            Self::HmacSha256 => {
                let signing_slice = SecretSlice::from(secret_key_bytes.to_vec());
                Ok(Encryptor::HmacSha256(signing_slice))
            }
            Self::HmacSha512 => {
                let signing_slice = SecretSlice::from(secret_key_bytes.to_vec());
                Ok(Encryptor::HmacSha512(signing_slice))
            }
        }
    }
}

impl Encryptor {
    pub fn encrypt(&self, bytes: &[u8]) -> EncryptResult<Vec<u8>> {
        match self {
            Self::EcdsaP256(signing_key) => {
                let signature: p256::ecdsa::Signature = signing_key.sign(bytes);
                Ok(signature.to_der().to_bytes().to_vec())
            }
            Self::EcdsaP384(signing_key) => {
                let signature: p384::ecdsa::Signature = signing_key.sign(bytes);
                Ok(signature.to_der().to_bytes().to_vec())
            }
            Self::Ed25519(signing_key) => {
                let signature = signing_key.sign(bytes, None);
                Ok(signature.to_vec())
            }
            Self::HmacSha256(signing_slice) => {
                let mut mac = Hmac::<Sha256>::new_from_slice(signing_slice.expose_secret())
                    .map_err(|e| EncryptError::CryptoKey(format!("HMAC-SHA256 key error: {e}")))?;
                mac.update(bytes);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            Self::HmacSha512(signing_slice) => {
                let mut mac = Hmac::<Sha512>::new_from_slice(signing_slice.expose_secret())
                    .map_err(|e| EncryptError::CryptoKey(format!("HMAC-SHA512 key error: {e}")))?;
                mac.update(bytes);
                Ok(mac.finalize().into_bytes().to_vec())
            }
        }
    }
}
