use crate::binance::{
    exchange_info::BinanceOrderType,
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceAmendOrderParams {
    pub newClientOrderId: Option<String>,
    pub newQty: Decimal,
    pub orderId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub recvWindow: Option<Decimal>,
    pub symbol: String,
    pub timestamp: i64,
}

/// The response envelope of an order amend (`order.amend.keepPriority`).
///
/// When an order that is a member of an order list is amended, the payload
/// additionally carries an order-list status object that is not modeled
/// here; unknown fields are ignored.
#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceAmendOrderResult {
    pub transactTime: i64,
    pub executionId: i64,
    pub amendedOrder: BinanceAmendedOrder,
}

/// The amended order reported in an amend response.
///
/// `workingTime` is absent from amend responses for orders that are members
/// of an order list, and the conditional fields only appear when the amended
/// order uses them, so all of those are optional.
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
    pub icebergQty: Option<Decimal>,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub trailingDelta: Option<i64>,
    pub trailingTime: Option<i64>,
    pub workingTime: Option<i64>,
}
