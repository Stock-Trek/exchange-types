use crate::{
    binance::{
        filters::{BinanceExchangeFilter, BinanceSymbolFilter},
        rate_limits::BinanceRateLimit,
        supporting_types::{BinanceOrderType, BinanceSelfTradeProtection},
    },
    ticker::Ticker,
};
use serde::{Deserialize, Serialize};
use strum::Display;

/// Parameters for the Binance `exchangeInfo` REST endpoint and WebSocket API
/// method.
///
/// The shape mirrors the documented constraints:
/// * only one of `symbol`, `symbols`, `permissions` can be specified;
/// * `permissions` accepts either a single permission name or a list of
///   permission names;
/// * `symbolStatus` cannot be combined with `symbol` or `symbols`.
///
/// Every parameter is optional: `All` with no `symbolStatus` returns the
/// default set of symbols.
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone, Hash)]
#[serde(untagged)]
pub enum BinanceExchangeInfoParams {
    /// No `symbol`/`symbols`/`permissions` filter, optionally restricted to a
    /// `symbolStatus`.
    All {
        #[serde(skip_serializing_if = "Option::is_none")]
        symbolStatus: Option<BinanceExchangeInfoSymbolStatus>,
    },
    /// Information for a single `symbol`.
    Symbol { symbol: String },
    /// Information for the given `symbols`.
    Symbols { symbols: Vec<String> },
    /// Symbols with the given `permissions`, optionally restricted to a
    /// `symbolStatus`.
    Permissions {
        permissions: BinanceExchangeInfoPermissions,
        #[serde(skip_serializing_if = "Option::is_none")]
        symbolStatus: Option<BinanceExchangeInfoSymbolStatus>,
    },
}

impl Default for BinanceExchangeInfoParams {
    fn default() -> Self {
        Self::All { symbolStatus: None }
    }
}

/// The `permissions` filter accepts either a single permission name or a list
/// of permission names.
#[derive(Serialize, Debug, Clone, Hash)]
#[serde(untagged)]
pub enum BinanceExchangeInfoPermissions {
    Single(BinanceExchangeInfoPermission),
    List(Vec<BinanceExchangeInfoPermission>),
}

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum BinanceExchangeInfoPermission {
    LEVERAGED,
    MARGIN,
    SPOT,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceExchangeInfoSymbolStatus {
    TRADING,
    HALT,
    BREAK,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceExchangeInfoResult {
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub rateLimits: Vec<BinanceRateLimit>,
    pub serverTime: i64,
    pub sors: Option<Vec<BinanceExchangeInfoSors>>,
    pub symbols: Vec<BinanceExchangeInfoSymbol>,
    pub timezone: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceExchangeInfoSors {
    pub baseAsset: Ticker,
    pub symbols: Vec<Ticker>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceExchangeInfoSymbol {
    pub allowTrailingStop: bool,
    pub allowedSelfTradePreventionModes: Vec<BinanceSelfTradeProtection>,
    pub amendAllowed: bool,
    pub baseAsset: Ticker,
    pub baseAssetPrecision: u8,
    pub baseCommissionPrecision: u8,
    pub cancelReplaceAllowed: bool,
    pub defaultSelfTradePreventionMode: BinanceSelfTradeProtection,
    pub filters: Vec<BinanceSymbolFilter>,
    pub icebergAllowed: bool,
    pub isMarginTradingAllowed: bool,
    pub isSpotTradingAllowed: bool,
    pub ocoAllowed: bool,
    pub opoAllowed: bool,
    pub orderTypes: Vec<BinanceOrderType>,
    pub otoAllowed: bool,
    pub pegInstructionsAllowed: bool,
    pub permissionSets: Vec<Vec<BinanceExchangeInfoPermission>>,
    pub permissions: Vec<BinanceExchangeInfoPermission>,
    pub quoteAsset: Ticker,
    pub quoteAssetPrecision: u8,
    pub quoteCommissionPrecision: u8,
    pub quoteOrderQtyMarketAllowed: bool,
    pub quotePrecision: u8,
    pub status: String,
    pub symbol: String,
}

impl BinanceExchangeInfoParams {
    pub fn query_params(&self) -> String {
        let mut pairs = Vec::new();
        match self {
            Self::All { symbolStatus } => {
                if let Some(symbol_status) = symbolStatus {
                    pairs.push(format!("symbolStatus={symbol_status}"));
                }
            }
            Self::Symbol { symbol } => pairs.push(format!("symbol={symbol}")),
            Self::Symbols { symbols } => pairs.push(format!(
                "symbols={}",
                Self::percent_encode(&Self::json_array(symbols))
            )),
            Self::Permissions {
                permissions,
                symbolStatus,
            } => {
                match permissions {
                    BinanceExchangeInfoPermissions::Single(permission) => {
                        pairs.push(format!("permissions={permission}"));
                    }
                    BinanceExchangeInfoPermissions::List(permissions) => pairs.push(format!(
                        "permissions={}",
                        Self::percent_encode(&Self::json_array(permissions))
                    )),
                }
                if let Some(symbol_status) = symbolStatus {
                    pairs.push(format!("symbolStatus={symbol_status}"));
                }
            }
        }
        pairs.join("&")
    }
    fn json_array(values: &[impl std::fmt::Display]) -> String {
        let values = values
            .iter()
            .map(|value| format!("\"{value}\""))
            .collect::<Vec<_>>()
            .join(",");
        format!("[{values}]")
    }
    fn percent_encode(value: &str) -> String {
        let mut encoded = String::with_capacity(value.len());
        for byte in value.bytes() {
            match byte {
                b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'.' | b'_' | b'~' => {
                    encoded.push(char::from(byte))
                }
                byte => encoded.push_str(&format!("%{byte:02X}")),
            }
        }
        encoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn symbol(symbol: &str) -> BinanceExchangeInfoParams {
        BinanceExchangeInfoParams::Symbol {
            symbol: symbol.into(),
        }
    }

    #[test]
    fn all_params_query_is_empty() {
        assert_eq!(
            BinanceExchangeInfoParams::default().query_params(),
            "".to_string()
        );
        assert_eq!(
            BinanceExchangeInfoParams::All {
                symbolStatus: Some(BinanceExchangeInfoSymbolStatus::HALT)
            }
            .query_params(),
            "symbolStatus=HALT".to_string()
        );
    }

    #[test]
    fn symbol_query_uses_single_symbol() {
        assert_eq!(
            symbol("BTCUSDT").query_params(),
            "symbol=BTCUSDT".to_string()
        );
    }

    #[test]
    fn symbols_query_encodes_json_array() {
        let params = BinanceExchangeInfoParams::Symbols {
            symbols: vec!["BTCUSDT".into(), "BNBBTC".into()],
        };
        assert_eq!(
            params.query_params(),
            "symbols=%5B%22BTCUSDT%22%2C%22BNBBTC%22%5D".to_string()
        );
    }

    #[test]
    fn single_permission_query_is_a_bare_name() {
        let params = BinanceExchangeInfoParams::Permissions {
            permissions: BinanceExchangeInfoPermissions::Single(
                BinanceExchangeInfoPermission::SPOT,
            ),
            symbolStatus: Some(BinanceExchangeInfoSymbolStatus::TRADING),
        };
        assert_eq!(
            params.query_params(),
            "permissions=SPOT&symbolStatus=TRADING".to_string()
        );
    }

    #[test]
    fn permissions_list_query_encodes_json_array() {
        let params = BinanceExchangeInfoParams::Permissions {
            permissions: BinanceExchangeInfoPermissions::List(vec![
                BinanceExchangeInfoPermission::SPOT,
                BinanceExchangeInfoPermission::MARGIN,
            ]),
            symbolStatus: None,
        };
        assert_eq!(
            params.query_params(),
            "permissions=%5B%22SPOT%22%2C%22MARGIN%22%5D".to_string()
        );
    }

    #[test]
    fn json_matches_the_documented_parameter_shape() {
        let all = serde_json::to_value(BinanceExchangeInfoParams::default()).unwrap();
        assert_eq!(all, serde_json::json!({}));
        let symbol = serde_json::to_value(symbol("BTCUSDT")).unwrap();
        assert_eq!(symbol, serde_json::json!({ "symbol": "BTCUSDT" }));
        let symbols = serde_json::to_value(BinanceExchangeInfoParams::Symbols {
            symbols: vec!["BTCUSDT".into(), "BNBBTC".into()],
        })
        .unwrap();
        assert_eq!(
            symbols,
            serde_json::json!({ "symbols": ["BTCUSDT", "BNBBTC"] })
        );
        let single_permission = serde_json::to_value(BinanceExchangeInfoParams::Permissions {
            permissions: BinanceExchangeInfoPermissions::Single(
                BinanceExchangeInfoPermission::SPOT,
            ),
            symbolStatus: None,
        })
        .unwrap();
        assert_eq!(
            single_permission,
            serde_json::json!({ "permissions": "SPOT" })
        );
        let permissions = serde_json::to_value(BinanceExchangeInfoParams::Permissions {
            permissions: BinanceExchangeInfoPermissions::List(vec![
                BinanceExchangeInfoPermission::SPOT,
                BinanceExchangeInfoPermission::MARGIN,
            ]),
            symbolStatus: Some(BinanceExchangeInfoSymbolStatus::HALT),
        })
        .unwrap();
        assert_eq!(
            permissions,
            serde_json::json!({
                "permissions": ["SPOT", "MARGIN"],
                "symbolStatus": "HALT"
            })
        );
    }
}
