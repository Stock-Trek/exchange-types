use crate::{
    error::ETError,
    rate_limited::{RateLimit, RateLimitRestriction, RateLimitType, RateLimits},
    time::Nanoseconds,
};
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};
use strum::Display;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceRateLimit {
    pub count: Option<i64>,
    pub interval: Option<BinanceRateLimitInterval>,
    pub intervalNum: Option<u32>,
    pub limit: Option<i64>,
    pub rateLimitType: Option<BinanceRateLimitType>,
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

impl TryFrom<BinanceRateLimitType> for RateLimitType {
    type Error = ETError;

    fn try_from(value: BinanceRateLimitType) -> Result<Self, Self::Error> {
        match value {
            BinanceRateLimitType::CONNECTIONS => Ok(RateLimitType::Connection),
            BinanceRateLimitType::ORDERS => Ok(RateLimitType::OrderCount),
            BinanceRateLimitType::RAW_REQUESTS => Ok(RateLimitType::RawRequests),
            BinanceRateLimitType::REQUEST_WEIGHT => Ok(RateLimitType::Weight),
            BinanceRateLimitType::Unknown => {
                Err(ETError::UnknownValue("BinanceRateLimitType".into()))
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
