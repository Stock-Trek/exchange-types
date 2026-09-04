use crate::{
    binance::{
        filters::{BinanceExchangeFilter, BinanceSymbolFilter},
        rate_limits::BinanceRateLimit,
        supporting_types::{BinanceOrderType, BinanceSelfTradeProtection},
    },
    encode::ByteEncoder,
    response::ResponseFor,
    ticker::Ticker,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone, Hash)]
#[serde(untagged)]
pub enum BinanceExchangeInfoRequest {
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

impl Default for BinanceExchangeInfoRequest {
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
#[non_exhaustive]
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
pub struct BinanceExchangeInfoResponse {
    pub exchangeFilters: Option<Vec<BinanceExchangeFilter>>,
    pub rateLimits: Option<Vec<BinanceRateLimit>>,
    pub serverTime: Option<i64>,
    pub sors: Option<Vec<BinanceExchangeInfoSors>>,
    pub symbols: Option<Vec<BinanceExchangeInfoSymbol>>,
    pub timezone: Option<String>,
}

impl ResponseFor for BinanceExchangeInfoRequest {
    type Response = BinanceExchangeInfoResponse;
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceExchangeInfoSors {
    pub baseAsset: Option<Ticker>,
    pub symbols: Option<Vec<Ticker>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceExchangeInfoSymbol {
    pub allowTrailingStop: Option<bool>,
    pub allowedSelfTradePreventionModes: Option<Vec<BinanceSelfTradeProtection>>,
    pub amendAllowed: Option<bool>,
    pub baseAsset: Option<Ticker>,
    pub baseAssetPrecision: Option<u8>,
    pub baseCommissionPrecision: Option<u8>,
    pub cancelReplaceAllowed: Option<bool>,
    pub defaultSelfTradePreventionMode: Option<BinanceSelfTradeProtection>,
    pub filters: Option<Vec<BinanceSymbolFilter>>,
    pub icebergAllowed: Option<bool>,
    pub isMarginTradingAllowed: Option<bool>,
    pub isSpotTradingAllowed: Option<bool>,
    pub ocoAllowed: Option<bool>,
    pub opoAllowed: Option<bool>,
    pub orderTypes: Option<Vec<BinanceOrderType>>,
    pub otoAllowed: Option<bool>,
    pub pegInstructionsAllowed: Option<bool>,
    pub permissionSets: Option<Vec<Vec<BinanceExchangeInfoPermission>>>,
    pub permissions: Option<Vec<BinanceExchangeInfoPermission>>,
    pub quoteAsset: Option<Ticker>,
    pub quoteAssetPrecision: Option<u8>,
    pub quoteCommissionPrecision: Option<u8>,
    pub quoteOrderQtyMarketAllowed: Option<bool>,
    pub quotePrecision: Option<u8>,
    pub status: Option<String>,
    pub symbol: Option<String>,
}

impl BinanceExchangeInfoRequest {
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
                ByteEncoder::Percent.encode(Self::json_array(symbols).as_bytes())
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
                        ByteEncoder::Percent.encode(Self::json_array(permissions).as_bytes())
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
}
