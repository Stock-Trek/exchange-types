use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Hash, Serialize)]
pub struct BinanceLogonParams {
    pub timestamp: i64,
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Deserialize)]
pub struct BinanceSessionAuthenticationResult {
    pub apiKey: String,
    pub authorizedSince: i64,
    pub connectedSince: i64,
    pub returnRateLimits: bool,
    pub serverTime: i64,
    pub userDataStream: bool,
}
