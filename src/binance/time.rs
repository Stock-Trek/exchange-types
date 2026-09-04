use crate::response::ResponseFor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Hash)]
pub struct BinanceTimeParams {}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceTimeResult {
    pub serverTime: i64,
}

impl ResponseFor for BinanceTimeParams {
    type Result = BinanceTimeResult;
}

impl BinanceTimeParams {
    pub fn query_params(&self) -> String {
        "".into()
    }
}
