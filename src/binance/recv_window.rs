use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const BINANCE_DEFAULT_RECV_WINDOW_MILLIS: u64 = 5000;
pub const BINANCE_MAX_RECV_WINDOW_MILLIS: u64 = 60_000;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BinanceRecvWindow(u64);

impl BinanceRecvWindow {
    pub fn try_new(millis: u64) -> Option<Self> {
        (millis <= BINANCE_MAX_RECV_WINDOW_MILLIS).then_some(Self(millis))
    }
}

impl Default for BinanceRecvWindow {
    fn default() -> Self {
        Self(BINANCE_DEFAULT_RECV_WINDOW_MILLIS)
    }
}

impl fmt::Display for BinanceRecvWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
