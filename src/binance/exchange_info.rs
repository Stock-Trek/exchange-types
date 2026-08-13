use crate::{binance::rate_limits::BinanceRateLimit, ticker::Ticker};
use rust_decimal::Decimal;
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
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub rateLimits: Vec<BinanceRateLimit>,
    pub serverTime: i64,
    pub symbols: Vec<BinanceExchangeInfoSymbol>,
    pub timezone: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType")]
pub enum BinanceExchangeFilter {
    EXCHANGE_MAX_NUM_ORDERS {
        max_num_orders: i64,
    },
    EXCHANGE_MAX_NUM_ALGO_ORDERS {
        max_num_algo_orders: i64,
    },
    #[serde(other)]
    Unknown,
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Clone, Deserialize)]
pub struct BinanceExchangeInfoSymbol {
    pub baseAsset: Ticker,
    pub baseAssetPrecision: u8,
    pub baseCommissionPrecision: u8,
    pub filters: Vec<BinanceFilter>,
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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType")]
pub enum BinanceFilter {
    PRICE_FILTER {
        min_price: Decimal,
        max_price: Decimal,
        tick_size: Decimal,
    },
    /// for limit orders
    LOT_SIZE {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },
    /// for market orders, if absent LOT_SIZE is used
    MARKET_LOT_SIZE {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },
    MIN_NOTIONAL {
        min_notional: Decimal,
        #[serde(default)]
        apply_min_to_market: Option<bool>,
        #[serde(default)]
        avg_price_mins: Option<i64>,
    },
    MAX_NUM_ORDERS {
        max_num_orders: i64,
    },
    MAX_NUM_ALGO_ORDERS {
        max_num_algo_orders: i64,
    },
    MAX_NUM_ICEBERG_ORDERS {
        max_num_iceberg_orders: i64,
    },
    ICEBERG_PARTS {
        limit: i64,
    },
    PERCENT_PRICE {
        multiplier_up: Decimal,
        multiplier_down: Decimal,
        avg_price_mins: i64,
    },
    TRAILING_DELTA {
        min_trailing_delta: Decimal,
        max_trailing_delta: Decimal,
        avg_price_mins: i64,
    },
    #[serde(other)]
    Unknown,
}
