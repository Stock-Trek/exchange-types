#[cfg(feature = "serde")]
use serde::Serialize;

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct BinanceSignature {
    #[cfg_attr(feature = "serde", serde(skip))]
    pub apiKey: String,
    pub signature: String,
}
