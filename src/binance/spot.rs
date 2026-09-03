use crate::{binance::exchange_info::BinanceOrderType, ticker::Ticker};
use query_params::QueryParams;
use rust_decimal::Decimal;
use strum::Display;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceSpotOrderParams {
    pub icebergQty: Option<Decimal>,
    pub newClientOrderId: String,
    pub newOrderRespType: BinanceNewOrderResponseType,
    pub pegPriceType: Option<BinancePegPriceType>,
    pub pegOffsetValue: Option<i32>,
    pub pegOffsetType: Option<BinancePegOffsetType>,
    pub price: Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub quoteOrderQty: Option<Decimal>,
    pub recvWindow: Option<Decimal>,
    pub selfTradePreventionMode: BinanceSelfTradeProtection,
    pub side: BinanceSide,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub symbol: String,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub timestamp: i64,
    pub trailingDelta: Option<u32>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub r#type: BinanceOrderType,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinanceNewOrderResponseType {
    ACK,
    RESULT,
    FULL,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinancePegPriceType {
    PRIMARY_PEG,
    MARKET_PEG,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinancePegOffsetType {
    PRICE_LEVEL,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinanceSide {
    BUY,
    SELL,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinanceSelfTradeProtection {
    EXPIRE_BOTH,
    EXPIRE_MAKER,
    EXPIRE_TAKER,
    DECREMENT,
    NONE,
    TRANSFER,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinanceTimeInForce {
    FOK,
    GTC,
    IOC,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum BinanceOrderStatus {
    CANCELED,
    EXPIRED,
    EXPIRED_IN_MATCH,
    FILLED,
    NEW,
    PARTIALLY_FILLED,
    PENDING_CANCEL,
    REJECTED,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceFill {
    pub commission: Decimal,
    pub commissionAsset: Ticker,
    pub price: Decimal,
    pub qty: Decimal,
    pub tradeId: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceSpotOrderResult {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Option<Decimal>,
    pub executedQty: Option<Decimal>,
    pub fills: Option<Vec<BinanceFill>>,
    pub icebergQty: Option<Decimal>,
    pub orderId: i64,
    pub orderListId: i32,
    pub origQty: Option<Decimal>,
    pub origQuoteOrderQty: Option<Decimal>,
    pub preventedMatchId: Option<i64>,
    pub preventedQuantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub selfTradePreventionMode: Option<BinanceSelfTradeProtection>,
    pub side: Option<BinanceSide>,
    pub status: Option<BinanceOrderStatus>,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub symbol: String,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub trailingDelta: Option<i64>,
    pub trailingTime: Option<i64>,
    pub transactTime: i64,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub r#type: Option<BinanceOrderType>,
    pub workingTime: Option<i64>,
}
