use crate::urls::Protocol;
use std::collections::HashMap;
use strum::Display;

pub trait RateLimited {
    fn weight(&self, protocol: Protocol) -> u32;
    fn order_count(&self, protocol: Protocol) -> u32;
}

pub trait RateLimits {
    fn default_capacity(&self) -> HashMap<RateLimit, u32>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RateLimit {
    pub restriction: RateLimitRestriction,
    pub interval_nanos: u64,
}

#[derive(Debug, Clone, Copy, Display, Hash, PartialEq, Eq)]
pub enum RateLimitRestriction {
    Connection,
    OrderCount,
    RawRequests,
    Weight,
}
