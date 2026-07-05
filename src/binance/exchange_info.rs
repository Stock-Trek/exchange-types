use crate::{binance::rate_limits::BinanceRateLimit, ticker::Ticker};
use serde::{Deserialize, Serialize};
use strum::Display;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Hash, Serialize)]
pub struct BinanceExchangeInfoParams {
    pub permissions: Vec<BinanceExchangeInfoPermission>,
    pub symbolStatus: BinanceExchangeInfoSymbolStatus,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum BinanceExchangeInfoPermission {
    SPOT,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum BinanceExchangeInfoSymbolStatus {
    TRADING,
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Clone, Deserialize)]
pub struct BinanceExchangeInfoResult {
    pub timezone: String,
    pub serverTime: i64,
    pub rateLimits: Vec<BinanceRateLimit>,
    pub symbols: Vec<BinanceExchangeInfoSymbol>,
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Clone, Deserialize)]
pub struct BinanceExchangeInfoSymbol {
    pub baseAsset: Ticker,
    pub baseAssetPrecision: u8,
    pub baseCommissionPrecision: u8,
    pub isSpotTradingAllowed: bool,
    pub orderTypes: Vec<BinanceOrderType>,
    pub quoteAsset: Ticker,
    pub quoteAssetPrecision: u8,
    pub quoteCommissionPrecision: u8,
    pub quoteOrderQtyMarketAllowed: bool,
    pub quotePrecision: u8,
    pub status: String,
    pub symbol: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum BinanceOrderType {
    LIMIT,
    LIMIT_MAKER,
    MARKET,
    STOP_LOSS,
    STOP_LOSS_LIMIT,
    TAKE_PROFIT,
    TAKE_PROFIT_LIMIT,
}
