#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceSignedParams<Params> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub params: Params,
    pub signature: Option<String>,
}
