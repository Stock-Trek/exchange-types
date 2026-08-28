use crate::binance::{
    exchange_info::BinanceOrderType,
    spot::{BinanceOrderStatus, BinanceSelfTradeProtection, BinanceSide, BinanceTimeInForce},
};
use query_params::QueryParams;
use rust_decimal::Decimal;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceAmendOrderParams {
    pub apiKey: Option<String>,
    pub newClientOrderId: Option<String>,
    pub newQty: Decimal,
    pub orderId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub recvWindow: Option<Decimal>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceAmendOrderResult {
    pub transactTime: i64,
    pub executionId: i64,
    pub amendedOrder: BinanceAmendedOrder,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceAmendedOrder {
    pub clientOrderId: String,
    pub cumulativeQuoteQty: Decimal,
    pub executedQty: Decimal,
    pub orderId: i64,
    pub orderListId: i32,
    pub origClientOrderId: String,
    pub preventedQty: Decimal,
    pub price: Decimal,
    pub qty: Decimal,
    pub quoteOrderQty: Decimal,
    pub selfTradePreventionMode: BinanceSelfTradeProtection,
    pub side: BinanceSide,
    pub status: BinanceOrderStatus,
    pub symbol: String,
    pub timeInForce: BinanceTimeInForce,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub r#type: BinanceOrderType,
    pub workingTime: i64,
}
