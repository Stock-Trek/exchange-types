use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::binance::rate_limits::BINANCE_DEFAULT_RECV_WINDOW_MILLIS;

/// The largest `recvWindow` Binance accepts, in milliseconds.
pub const BINANCE_MAX_RECV_WINDOW_MILLIS: u64 = 60_000;

/// A Binance `recvWindow`: how long, in milliseconds, a signed request stays
/// valid after its `timestamp`.
///
/// Binance rejects any `recvWindow` greater than
/// [`BINANCE_MAX_RECV_WINDOW_MILLIS`] (60,000 ms) with error -1021, so this
/// type can only represent values in `0..=60_000`. When a request omits
/// `recvWindow`, Binance applies its default
/// ([`BINANCE_DEFAULT_RECV_WINDOW_MILLIS`]), which is also this type's
/// [`Default`].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BinanceRecvWindow(u64);

impl BinanceRecvWindow {
    /// Creates a `recvWindow` of `millis` milliseconds.
    ///
    /// # Panics
    ///
    /// Panics when `millis` is greater than
    /// [`BINANCE_MAX_RECV_WINDOW_MILLIS`]; use [`Self::try_new`] for a
    /// fallible variant.
    pub fn new(millis: u64) -> Self {
        assert!(
            millis <= BINANCE_MAX_RECV_WINDOW_MILLIS,
            "recvWindow must not exceed {} ms, got {millis}",
            BINANCE_MAX_RECV_WINDOW_MILLIS
        );
        Self(millis)
    }

    /// Creates a `recvWindow` of `millis` milliseconds, or `None` when
    /// `millis` is greater than [`BINANCE_MAX_RECV_WINDOW_MILLIS`].
    pub fn try_new(millis: u64) -> Option<Self> {
        (millis <= BINANCE_MAX_RECV_WINDOW_MILLIS).then_some(Self(millis))
    }

    /// Returns the receive window in milliseconds.
    pub const fn get(self) -> u64 {
        self.0
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

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn accepts_values_up_to_the_binance_maximum() {
        assert_eq!(BinanceRecvWindow::new(0).get(), 0);
        assert_eq!(BinanceRecvWindow::new(60_000).get(), 60_000);
        assert_eq!(
            BinanceRecvWindow::try_new(60_000).map(BinanceRecvWindow::get),
            Some(60_000)
        );
    }

    #[test]
    fn rejects_values_above_the_binance_maximum() {
        assert_eq!(BinanceRecvWindow::try_new(60_001), None);
    }

    #[test]
    fn defaults_to_the_binance_default_receive_window() {
        assert_eq!(BinanceRecvWindow::default().get(), 5_000);
    }

    #[test]
    fn displays_as_plain_milliseconds() {
        assert_eq!(BinanceRecvWindow::new(5_000).to_string(), "5000");
    }

    #[test]
    fn round_trips_through_json_as_an_integer() {
        let window = BinanceRecvWindow::new(10_000);
        let json = serde_json::to_string(&window).unwrap();
        assert_eq!(json, "10000");
        assert_eq!(
            serde_json::from_str::<BinanceRecvWindow>(&json).unwrap(),
            window
        );
    }
}
