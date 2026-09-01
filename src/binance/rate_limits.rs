use strum::Display;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceRateLimit {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub count: Option<i64>,
    pub interval: BinanceRateLimitInterval,
    pub intervalNum: i32,
    pub limit: i64,
    pub rateLimitType: BinanceRateLimitType,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Display)]
pub enum BinanceRateLimitInterval {
    DAY,
    HOUR,
    MINUTE,
    SECOND,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Display)]
pub enum BinanceRateLimitType {
    CONNECTIONS,
    ORDERS,
    RAW_REQUESTS,
    REQUEST_WEIGHT,
}
