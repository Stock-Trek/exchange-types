use crate::{
    binance::{
        filters::{BinanceExchangeFilter, BinanceSymbolFilter},
        rate_limits::BinanceRateLimit,
        spot::BinanceSelfTradeProtection,
    },
    ticker::Ticker,
};
use strum::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Debug, Clone, Hash)]
pub struct BinanceExchangeInfoParams {
    pub permissions: Vec<BinanceExchangeInfoPermission>,
    pub symbolStatus: BinanceExchangeInfoSymbolStatus,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceExchangeInfoPermission {
    SPOT,
    MARGIN,
    LEVERAGED,
    /// Any other permission value. Binance returns an open-ended set of
    /// permissions (e.g. `TRD_GRP_*` trading-group permissions), so unknown
    /// values must not fail deserialization.
    #[cfg_attr(feature = "serde", serde(other))]
    UNKNOWN,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceExchangeInfoSymbolStatus {
    TRADING,
}

/// The `sors` member of an exchange-info response, present only when smart
/// order routing is available.
#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceExchangeInfoSors {
    pub baseAsset: Ticker,
    pub symbols: Vec<Ticker>,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceExchangeInfoResult {
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub rateLimits: Vec<BinanceRateLimit>,
    pub serverTime: i64,
    pub symbols: Vec<BinanceExchangeInfoSymbol>,
    pub timezone: String,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub sors: Option<Vec<BinanceExchangeInfoSors>>,
}

/// One symbol of an exchange-info response.
///
/// Real exchange-info payloads include the trading-permission and order-type
/// capability fields below (all verified to be present on every currently
/// listed symbol), so they are mandatory here. Unknown permission values are
/// tolerated by [`BinanceExchangeInfoPermission::UNKNOWN`].
#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
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

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinanceOrderType {
    LIMIT,
    LIMIT_MAKER,
    MARKET,
    STOP_LOSS,
    STOP_LOSS_LIMIT,
    TAKE_PROFIT,
    TAKE_PROFIT_LIMIT,
}

impl BinanceExchangeInfoParams {
    pub fn query_params(&self) -> String {
        let mut pairs = Vec::new();
        if !self.permissions.is_empty() {
            let permissions_string = self
                .permissions
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",");
            pairs.push(format!("permissions={}", permissions_string));
        }
        pairs.push(format!("symbolStatus={}", self.symbolStatus));
        pairs.join("&")
    }
}
