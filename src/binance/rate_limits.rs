use serde::Deserialize;
use strum::Display;

#[allow(non_snake_case, unused)]
#[derive(Debug, Deserialize)]
pub struct BinanceRateLimit {
    pub count: i64,
    pub interval: BinanceRateLimitInterval,
    pub intervalNum: i32,
    pub limit: i64,
    pub rateLimitType: BinanceRateLimitType,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Deserialize)]
pub enum BinanceRateLimitInterval {
    DAY,
    HOUR,
    MINUTE,
    SECOND,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Deserialize)]
pub enum BinanceRateLimitType {
    CONNECTIONS,
    ORDERS,
    REQUEST_WEIGHT,
}
