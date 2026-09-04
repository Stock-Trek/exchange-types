use crate::ticker::Ticker;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType")]
#[allow(non_snake_case, non_camel_case_types)]
#[non_exhaustive]
pub enum BinanceExchangeFilter {
    EXCHANGE_MAX_NUM_ORDERS {
        maxNumOrders: Option<i64>,
    },
    EXCHANGE_MAX_NUM_ALGO_ORDERS {
        maxNumAlgoOrders: Option<i64>,
    },
    EXCHANGE_MAX_NUM_ICEBERG_ORDERS {
        maxNumIcebergOrders: Option<i64>,
    },
    EXCHANGE_MAX_NUM_ORDER_LISTS {
        maxNumOrderLists: Option<i64>,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType")]
#[allow(non_snake_case, non_camel_case_types)]
#[non_exhaustive]
pub enum BinanceSymbolFilter {
    PRICE_FILTER {
        minPrice: Option<Decimal>,
        maxPrice: Option<Decimal>,
        tickSize: Option<Decimal>,
    },
    PERCENT_PRICE {
        multiplierUp: Option<Decimal>,
        multiplierDown: Option<Decimal>,
        avgPriceMins: Option<i64>,
    },
    PERCENT_PRICE_BY_SIDE {
        bidMultiplierUp: Option<Decimal>,
        bidMultiplierDown: Option<Decimal>,
        askMultiplierUp: Option<Decimal>,
        askMultiplierDown: Option<Decimal>,
        avgPriceMins: Option<i64>,
    },
    LOT_SIZE {
        minQty: Option<Decimal>,
        maxQty: Option<Decimal>,
        stepSize: Option<Decimal>,
    },
    MIN_NOTIONAL {
        minNotional: Option<Decimal>,
        applyToMarket: Option<bool>,
        avgPriceMins: Option<i64>,
    },
    NOTIONAL {
        minNotional: Option<Decimal>,
        applyMinToMarket: Option<bool>,
        maxNotional: Option<Decimal>,
        applyMaxToMarket: Option<bool>,
        avgPriceMins: Option<i64>,
    },
    ICEBERG_PARTS {
        limit: Option<i64>,
    },
    MARKET_LOT_SIZE {
        minQty: Option<Decimal>,
        maxQty: Option<Decimal>,
        stepSize: Option<Decimal>,
    },
    MAX_NUM_ORDERS {
        maxNumOrders: Option<i64>,
    },
    MAX_NUM_ALGO_ORDERS {
        maxNumAlgoOrders: Option<i64>,
    },
    MAX_NUM_ICEBERG_ORDERS {
        maxNumIcebergOrders: Option<i64>,
    },
    MAX_POSITION {
        maxPosition: Option<Decimal>,
    },
    TRAILING_DELTA {
        minTrailingAboveDelta: Option<i64>,
        maxTrailingAboveDelta: Option<i64>,
        minTrailingBelowDelta: Option<i64>,
        maxTrailingBelowDelta: Option<i64>,
    },
    MAX_NUM_ORDER_AMENDS {
        maxNumOrderAmends: Option<i64>,
    },
    MAX_NUM_ORDER_LISTS {
        maxNumOrderLists: Option<i64>,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType")]
#[allow(non_snake_case, non_camel_case_types)]
#[non_exhaustive]
pub enum BinanceAssetFilter {
    MAX_ASSET {
        asset: Option<Ticker>,
        limit: Option<Decimal>,
    },
    #[serde(other)]
    Unknown,
}
