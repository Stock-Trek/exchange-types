use crate::urls::Protocol;
use std::collections::HashMap;
use strum::Display;

pub trait RateLimited {
    fn weight(&self, protocol: Protocol) -> u32;
    fn order_count(&self, protocol: Protocol) -> u32;
}

pub trait RateLimits {
    fn default(&self) -> HashMap<RateLimitType, Vec<RateLimit>>;
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub interval_nanos: u128,
    pub capacity_per_interval: u32,
    pub restriction: RateLimitRestriction,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitRestriction {
    IP,
    Account,
}

#[derive(Debug, Clone, Copy, Display, Hash, PartialEq, Eq)]
pub enum RateLimitType {
    Connection,
    OrderCount,
    RawRequests,
    Weight,
}
