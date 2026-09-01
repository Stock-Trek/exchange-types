#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceSignature {
    pub apiKey: String,
    pub signature: String,
}
