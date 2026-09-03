#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Binance API error payload (`{"code":…,"msg":…}`).
///
/// Some errors carry additional context in unmodeled fields (e.g. the
/// `data` member of a failed order cancel-replace), so unknown fields are
/// ignored rather than rejected.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceError {
    pub code: i64,
    pub msg: String,
}

impl std::fmt::Display for BinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.msg)
    }
}

impl std::error::Error for BinanceError {}
