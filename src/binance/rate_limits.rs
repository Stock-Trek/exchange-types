use crate::rate_limited::{RateLimit, RateLimitRestriction, RateLimits};
use std::{collections::HashMap, time::Duration};
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
#[derive(Debug, Clone, Copy, Display, Hash, PartialEq, Eq)]
pub enum BinanceRateLimitType {
    CONNECTIONS,
    ORDERS,
    RAW_REQUESTS,
    REQUEST_WEIGHT,
}

pub struct BinanceRateLimits;

impl RateLimits for BinanceRateLimits {
    type LimitType = BinanceRateLimitType;

    fn rate_limits(&self) -> HashMap<Self::LimitType, Vec<RateLimit>> {
        let mut map = HashMap::new();
        map.insert(
            BinanceRateLimitType::REQUEST_WEIGHT,
            vec![RateLimit {
                capacity_per_interval: 6000,
                interval_nanos: Duration::from_mins(1).as_nanos(),
                restriction: RateLimitRestriction::IP,
            }],
        );
        map.insert(
            BinanceRateLimitType::ORDERS,
            vec![
                RateLimit {
                    capacity_per_interval: 50,
                    interval_nanos: Duration::from_secs(10).as_nanos(),
                    restriction: RateLimitRestriction::ACCOUNT,
                },
                RateLimit {
                    capacity_per_interval: 160_000,
                    interval_nanos: Duration::from_hours(24).as_nanos(),
                    restriction: RateLimitRestriction::ACCOUNT,
                },
            ],
        );
        map
    }
}
