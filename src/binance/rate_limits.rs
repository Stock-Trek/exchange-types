use crate::{
    error::{ETError, ETResult},
    rate_limited::{RateLimit, RateLimitRestriction, RateLimitType, RateLimits},
};
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};
use strum::Display;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceRateLimit {
    pub count: Option<i64>,
    pub interval: BinanceRateLimitInterval,
    pub intervalNum: i32,
    pub limit: i64,
    pub rateLimitType: BinanceRateLimitType,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Clone, Copy, Display, PartialEq, Eq)]
pub enum BinanceRateLimitInterval {
    DAY,
    HOUR,
    MINUTE,
    SECONDS_TEN,
    SECOND,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Clone, Copy, Display, Hash, PartialEq, Eq)]
pub enum BinanceRateLimitType {
    CONNECTIONS,
    ORDERS,
    RAW_REQUESTS,
    REQUEST_WEIGHT,
    #[serde(other)]
    Unknown,
}

pub struct BinanceRateLimits;

impl BinanceRateLimitInterval {
    pub fn try_into_nanos(self) -> ETResult<i64> {
        match self {
            BinanceRateLimitInterval::DAY => Ok(24 * 60 * 60 * 1_000_000_000),
            BinanceRateLimitInterval::HOUR => Ok(60 * 60 * 1_000_000_000),
            BinanceRateLimitInterval::MINUTE => Ok(60 * 1_000_000_000),
            BinanceRateLimitInterval::SECONDS_TEN => Ok(10 * 1_000_000_000),
            BinanceRateLimitInterval::SECOND => Ok(1_000_000_000),
            BinanceRateLimitInterval::Unknown => Err(ETError::UnknownValue),
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
    fn default(&self) -> HashMap<RateLimitType, Vec<RateLimit>> {
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

#[cfg(test)]
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
