use crate::rate_limited::{RateLimit, RateLimitRestriction, RateLimitType, RateLimits};
use std::{collections::HashMap, time::Duration};
use strum::Display;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceRateLimit {
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
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Display, Hash, PartialEq, Eq)]
pub enum BinanceRateLimitType {
    CONNECTIONS,
    ORDERS,
    RAW_REQUESTS,
    REQUEST_WEIGHT,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

pub struct BinanceRateLimits;

impl BinanceRateLimitInterval {
    pub fn into_nanos(self) -> i64 {
        match self {
            BinanceRateLimitInterval::DAY => 24 * 60 * 60 * 1_000_000_000,
            BinanceRateLimitInterval::HOUR => 60 * 60 * 1_000_000_000,
            BinanceRateLimitInterval::MINUTE => 60 * 1_000_000_000,
            BinanceRateLimitInterval::SECOND => 1_000_000_000,
            BinanceRateLimitInterval::Unknown => {
                panic!("unsupported Binance rate limit interval: {self}")
            }
        }
    }
}

impl From<BinanceRateLimitType> for RateLimitType {
    fn from(value: BinanceRateLimitType) -> Self {
        match value {
            BinanceRateLimitType::CONNECTIONS => RateLimitType::Connection,
            BinanceRateLimitType::ORDERS => RateLimitType::OrderCount,
            BinanceRateLimitType::RAW_REQUESTS => RateLimitType::RawRequests,
            BinanceRateLimitType::REQUEST_WEIGHT => RateLimitType::Weight,
            BinanceRateLimitType::Unknown => {
                panic!("unsupported Binance rate limit type: {value}")
            }
        }
    }
}

impl RateLimits for BinanceRateLimits {
    fn rate_limits(&self) -> HashMap<RateLimitType, Vec<RateLimit>> {
        let mut map = HashMap::new();
        map.insert(
            RateLimitType::Weight,
            vec![RateLimit {
                capacity_per_interval: 6000,
                interval_nanos: Duration::from_mins(1).as_nanos(),
                restriction: RateLimitRestriction::IP,
            }],
        );
        map.insert(
            RateLimitType::OrderCount,
            vec![
                RateLimit {
                    capacity_per_interval: 50,
                    interval_nanos: Duration::from_secs(10).as_nanos(),
                    restriction: RateLimitRestriction::Account,
                },
                RateLimit {
                    capacity_per_interval: 160_000,
                    interval_nanos: Duration::from_hours(24).as_nanos(),
                    restriction: RateLimitRestriction::Account,
                },
            ],
        );
        map
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn unknown_rate_limit_enums_deserialize_as_unknown() {
        let interval: BinanceRateLimitInterval = serde_json::from_str(r#""FORTNIGHT""#).unwrap();
        assert!(matches!(interval, BinanceRateLimitInterval::Unknown));
        let rate_limit_type: BinanceRateLimitType =
            serde_json::from_str(r#""FUTURE_TYPE""#).unwrap();
        assert!(matches!(rate_limit_type, BinanceRateLimitType::Unknown));
    }
}
