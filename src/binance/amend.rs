use crate::binance::{
    exchange_info::BinanceOrderType,
    recv_window::BinanceRecvWindow,
    spot::{BinanceOrderStatus, BinanceSelfTradeProtection, BinanceSide, BinanceTimeInForce},
};
use query_params::QueryParams;
use rust_decimal::Decimal;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceAmendOrderParams {
    pub newClientOrderId: Option<String>,
    pub newQty: Decimal,
    pub orderId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceAmendOrderResult {
    pub amendedOrder: BinanceAmendedOrder,
    pub executionId: i64,
    pub transactTime: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceAmendedOrder {
    pub clientOrderId: String,
    pub cumulativeQuoteQty: Decimal,
    pub executedQty: Decimal,
    pub icebergQty: Option<Decimal>,
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
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub symbol: String,
    pub timeInForce: BinanceTimeInForce,
    pub trailingDelta: Option<i64>,
    pub trailingTime: Option<i64>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub r#type: BinanceOrderType,
    pub workingTime: Option<i64>,
}
