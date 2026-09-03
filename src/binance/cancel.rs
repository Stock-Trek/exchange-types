use crate::binance::{
    exchange_info::BinanceOrderType,
    recv_window::BinanceRecvWindow,
    spot::{BinanceOrderStatus, BinanceSelfTradeProtection, BinanceSide, BinanceTimeInForce},
};
use query_params::QueryParams;
use rust_decimal::Decimal;
use strum::Display;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceCancelAllOrdersParams {
    /// The API key. `into_signed` sets it from the signer when signing a
    /// WebSocket API request; it must be `None` for HTTP requests.
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceCancelOrderParams {
    /// The API key. `into_signed` sets it from the signer when signing a
    /// WebSocket API request; it must be `None` for HTTP requests.
    pub apiKey: Option<String>,
    pub cancelRestrictions: Option<BinanceCancelRestrictions>,
    pub newClientOrderId: Option<String>,
    pub orderId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone, Copy, Hash, Display)]
pub enum BinanceCancelRestrictions {
    ONLY_NEW,
    ONLY_PARTIALLY_FILLED,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceCancelOrderResult {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Option<Decimal>,
    pub executedQty: Option<Decimal>,
    pub icebergQty: Option<Decimal>,
    pub orderId: i64,
    pub orderListId: i32,
    pub origClientOrderId: String,
    pub origQty: Option<Decimal>,
    pub origQuoteOrderQty: Option<Decimal>,
    pub preventedMatchId: Option<i64>,
    pub preventedQuantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub selfTradePreventionMode: Option<BinanceSelfTradeProtection>,
    pub side: BinanceSide,
    pub status: BinanceOrderStatus,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub symbol: String,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub trailingDelta: Option<i64>,
    pub trailingTime: Option<i64>,
    pub transactTime: Option<i64>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub r#type: BinanceOrderType,
    pub workingTime: Option<i64>,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceCancelOrderListResult {
    pub contingencyType: String,
    pub listClientOrderId: String,
    pub listOrderStatus: String,
    pub listStatusType: String,
    pub orderListId: i32,
    #[cfg_attr(feature = "serde", serde(default))]
    pub orderReports: Vec<BinanceCancelOrderResult>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub orders: Vec<BinanceOrderListOrder>,
    pub symbol: String,
    pub transactionTime: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceOrderListOrder {
    pub clientOrderId: String,
    pub orderId: i64,
    pub symbol: String,
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceCancelReport {
    Order(BinanceCancelOrderResult),
    OrderList(BinanceCancelOrderListResult),
}
