use crate::{
    error::ETError,
    rate_limited::{RateLimit, RateLimitRestriction, RateLimits},
    time::Nanoseconds,
};
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};
use strum::Display;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceRateLimit {
    pub count: Option<i64>,
    pub interval: BinanceRateLimitInterval,
    pub intervalNum: u32,
    pub limit: i64,
    pub rateLimitType: BinanceRateLimitType,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Clone, Copy, Display, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BinanceRateLimitInterval {
    DAY,
    HOUR,
    MINUTE,
    SECOND,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Clone, Copy, Display, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum BinanceRateLimitType {
    CONNECTIONS,
    ORDERS,
    RAW_REQUESTS,
    REQUEST_WEIGHT,
    #[serde(other)]
    Unknown,
}

pub struct BinanceRateLimits;

impl TryFrom<char> for BinanceRateLimitInterval {
    type Error = ETError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            's' => Ok(Self::SECOND),
            'm' => Ok(Self::MINUTE),
            'h' => Ok(Self::HOUR),
            'd' => Ok(Self::DAY),
            other => Err(ETError::ParseError(other.into())),
        }
    }
}

impl TryFrom<BinanceRateLimitInterval> for Nanoseconds {
    type Error = ETError;

    fn try_from(value: BinanceRateLimitInterval) -> Result<Self, Self::Error> {
        match value {
            BinanceRateLimitInterval::DAY => Ok(Nanoseconds(24 * 60 * 60 * 1_000_000_000)),
            BinanceRateLimitInterval::HOUR => Ok(Nanoseconds(60 * 60 * 1_000_000_000)),
            BinanceRateLimitInterval::MINUTE => Ok(Nanoseconds(60 * 1_000_000_000)),
            BinanceRateLimitInterval::SECOND => Ok(Nanoseconds(1_000_000_000)),
            BinanceRateLimitInterval::Unknown => {
                Err(ETError::UnknownValue("BinanceRateLimitInterval".into()))
            }
        }
    }
}

impl TryFrom<BinanceRateLimitType> for RateLimitRestriction {
    type Error = ETError;

    fn try_from(value: BinanceRateLimitType) -> Result<Self, Self::Error> {
        match value {
            BinanceRateLimitType::CONNECTIONS => Ok(RateLimitRestriction::Connection),
            BinanceRateLimitType::ORDERS => Ok(RateLimitRestriction::OrderCount),
            BinanceRateLimitType::RAW_REQUESTS => Ok(RateLimitRestriction::RawRequests),
            BinanceRateLimitType::REQUEST_WEIGHT => Ok(RateLimitRestriction::Weight),
            BinanceRateLimitType::Unknown => {
                Err(ETError::UnknownValue("BinanceRateLimitType".into()))
            }
        }
    }
}

impl RateLimits for BinanceRateLimits {
    fn default_capacity(&self) -> HashMap<RateLimit, u32> {
        let mut map = HashMap::new();
        map.insert(
            RateLimit {
                restriction: RateLimitRestriction::Weight,
                interval_nanos: Duration::from_mins(1).as_nanos() as u64,
            },
            6_000,
        );
        map.insert(
            RateLimit {
                restriction: RateLimitRestriction::OrderCount,
                interval_nanos: Duration::from_secs(10).as_nanos() as u64,
            },
            50,
        );
        map.insert(
            RateLimit {
                restriction: RateLimitRestriction::OrderCount,
                interval_nanos: Duration::from_hours(24).as_nanos() as u64,
            },
            160_000,
        );
        map
    }
}
