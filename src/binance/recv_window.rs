use serde::Serialize;
use std::fmt;

pub const BINANCE_DEFAULT_RECV_WINDOW_MICROSECONDS: u64 = 5_000_000;
pub const BINANCE_MAX_RECV_WINDOW_MICROSECONDS: u64 = 60_000_000;

#[derive(Serialize)]
#[serde(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BinanceRecvWindow(u64);

impl BinanceRecvWindow {
    pub fn try_new(microseconds: u64) -> Option<Self> {
        ((microseconds <= BINANCE_MAX_RECV_WINDOW_MICROSECONDS) && (microseconds > 0))
            .then_some(Self(microseconds))
    }
}

impl Default for BinanceRecvWindow {
    fn default() -> Self {
        Self(BINANCE_DEFAULT_RECV_WINDOW_MICROSECONDS)
    }
}

impl fmt::Display for BinanceRecvWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
