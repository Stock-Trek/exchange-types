use secrecy::SecretString;

#[cfg(feature = "serde")]
use serde::Deserialize;

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct ApiKeyCredentials {
    pub api_key: String,
    pub secret: SecretString,
}
