use crate::binance::{
    exchange_info::BinanceOrderType,
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceCancelAllOrdersParams {
    pub recvWindow: Option<Decimal>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceCancelOrderParams {
    pub cancelRestrictions: Option<BinanceCancelRestrictions>,
    pub newClientOrderId: Option<String>,
    pub orderId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub recvWindow: Option<Decimal>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, Display)]
pub enum BinanceCancelRestrictions {
    ONLY_NEW,
    ONLY_PARTIALLY_FILLED,
}

/// The cancellation report for one order.
///
/// Canceling an order that is part of an order list cancels the whole order
/// list; in that case Binance returns an order-list-shaped report instead
/// (see [`BinanceCancelOrderListResult`]). Cancel reports are also missing
/// `workingTime` even though Binance documents it for other order payloads,
/// so only the fields every report contains are mandatory.
#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceCancelOrderResult {
    pub clientOrderId: String,
    pub orderId: i64,
    pub orderListId: i32,
    pub origClientOrderId: String,
    pub side: BinanceSide,
    pub status: BinanceOrderStatus,
    pub symbol: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub r#type: BinanceOrderType,
    pub cummulativeQuoteQty: Option<Decimal>,
    pub executedQty: Option<Decimal>,
    pub icebergQty: Option<Decimal>,
    pub origQty: Option<Decimal>,
    pub origQuoteOrderQty: Option<Decimal>,
    pub preventedMatchId: Option<i64>,
    pub preventedQuantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub selfTradePreventionMode: Option<BinanceSelfTradeProtection>,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub trailingDelta: Option<i64>,
    pub trailingTime: Option<i64>,
    pub transactTime: Option<i64>,
    pub workingTime: Option<i64>,
}

/// One order contained in an order-list-shaped payload.
#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceOrderListOrder {
    pub clientOrderId: String,
    pub orderId: i64,
    pub symbol: String,
}

/// The order-list-shaped report Binance returns when a cancel request
/// cancels an entire order list: cancelling an order that is a member of an
/// order list (`order.cancel` / `DELETE /api/v3/order`), cancelling an order
/// list directly (`DELETE /api/v3/orderList`), and the order-list elements
/// of a cancel-all (`openOrders.cancelAll`) response.
#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceCancelOrderListResult {
    pub contingencyType: String,
    pub listClientOrderId: String,
    pub listOrderStatus: String,
    pub listStatusType: String,
    pub orderListId: i32,
    pub symbol: String,
    pub transactionTime: i64,
    #[cfg_attr(feature = "serde", serde(default))]
    pub orders: Vec<BinanceOrderListOrder>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub orderReports: Vec<BinanceCancelOrderResult>,
}

/// One element of a cancel-all response: an order-list-shaped report is
/// returned for every order list that the cancel-all cancelled.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceCancelReport {
    Order(BinanceCancelOrderResult),
    OrderList(BinanceCancelOrderListResult),
}
