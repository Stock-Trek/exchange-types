use crate::response::ResponseFor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Hash)]
pub struct BinanceTimeRequest {}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceTimeResponse {
    pub serverTime: i64,
}

impl ResponseFor for BinanceTimeRequest {
    type Response = BinanceTimeResponse;
}

impl BinanceTimeRequest {
    pub fn query_params(&self) -> String {
        "".into()
    }
}
