#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash)]
pub struct BinanceTimeParams {}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceTimeResult {
    pub serverTime: i64,
}

impl BinanceTimeParams {
    pub fn query_params(&self) -> String {
        "".into()
    }
}
