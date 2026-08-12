use crate::binance::exchange_info::BinanceOrderType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::Display;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Hash, Serialize)]
#[skip_serializing_none]
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
    pub r#type: BinanceOrderType,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Clone, Copy, Hash, Serialize)]
pub enum BinanceNewOrderResponseType {
    ACK,
    RESULT,
    FULL,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Clone, Copy, Hash, Serialize)]
pub enum BinancePegPriceType {
    PRIMARY_PEG,
    MARKET_PEG,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Clone, Copy, Hash, Serialize)]
pub enum BinancePegOffsetType {
    PRICE_LEVEL,
}

#[derive(Debug, Display, Clone, Copy, Hash, Serialize)]
pub enum BinanceSide {
    BUY,
    SELL,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Clone, Copy, Hash, Serialize)]
pub enum BinanceSelfTradeProtection {
    EXPIRE_BOTH,
    EXPIRE_MAKER,
    EXPIRE_TAKER,
    DECREMENT,
    NONE,
    TRANSFER,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Clone, Copy, Hash, Serialize)]
pub enum BinanceTimeInForce {
    FOK,
    GTC,
    IOC,
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Clone, Deserialize)]
pub struct BinanceSpotOrderResult {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Decimal,
    pub executedQty: Decimal,
    pub orderId: i64,
    pub orderListId: i32,
    pub origQty: Decimal,
    pub origQuoteOrderQty: Decimal,
    pub price: Decimal,
    pub selfTradePreventionMode: String,
    pub side: String,
    pub status: String,
    pub symbol: String,
    pub timeInForce: String,
    pub transactTime: i64,
    pub r#type: String,
    pub workingTime: i64,
}
