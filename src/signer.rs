use crate::{encode::ByteEncoder, encrypt::Encryptor, error::ETResult};

#[derive(Debug)]
pub struct Signer {
    api_key: String,
    encryptor: Encryptor,
    encoder: ByteEncoder,
}

impl Signer {
    pub fn new(api_key: String, encryptor: Encryptor, encoder: ByteEncoder) -> Self {
        Self {
            api_key,
            encryptor,
            encoder,
        }
    }
    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }
    pub fn signature(&self, bytes: &[u8]) -> ETResult<String> {
        let encrypted = self.encryptor.encrypt(bytes)?;
        Ok(self.encoder.encode(&encrypted))
    }
}
