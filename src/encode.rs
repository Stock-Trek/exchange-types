use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByteEncoder {
    Base16,
    Base32,
    Base58,
    Base64,
    HexLower,
    HexUpper,
}

impl ByteEncoder {
    pub fn encode(&self, bytes: &[u8]) -> String {
        match self {
            ByteEncoder::Base16 => data_encoding::HEXUPPER.encode(bytes),
            ByteEncoder::Base32 => data_encoding::BASE32.encode(bytes),
            ByteEncoder::Base58 => bs58::encode(bytes).into_string(),
            ByteEncoder::Base64 => data_encoding::BASE64.encode(bytes),
            ByteEncoder::HexLower => data_encoding::HEXLOWER.encode(bytes),
            ByteEncoder::HexUpper => data_encoding::HEXUPPER.encode(bytes),
        }
    }
}
