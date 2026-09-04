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

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone, Hash)]
#[serde(untagged)]
pub enum BinanceExchangeInfoParams {
    All {
        #[serde(skip_serializing_if = "Option::is_none")]
        symbolStatus: Option<BinanceExchangeInfoSymbolStatus>,
    },
    Symbol {
        symbol: String,
    },
    Symbols {
        symbols: Vec<String>,
    },
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
