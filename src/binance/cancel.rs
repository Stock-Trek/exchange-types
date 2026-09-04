use crate::{
    binance::{
        recv_window::BinanceRecvWindow,
        supporting_types::{
            BinanceExpiryReason, BinanceOrderListOrder, BinanceOrderStatus, BinanceOrderType,
            BinancePegOffsetType, BinancePegPriceType, BinanceSelfTradeProtection, BinanceSide,
            BinanceTimeInForce, BinanceWorkingFloor,
        },
    },
    response::ResponseFor,
};
use query_params::QueryParams;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::Display;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceCancelAllOrdersRequest {
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceCancelOrderRequest {
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
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum BinanceCancelRestrictions {
    ONLY_NEW,
    ONLY_PARTIALLY_FILLED,
}

/// A cancelled order report.
///
/// Only [`BinanceCancelOrderResult::clientOrderId`] is required: it is the
/// discriminator that lets the untagged [`BinanceCancelResponse`] reject
/// order-list bodies (which carry `listClientOrderId`, never `clientOrderId`)
/// while staying tolerant of schema drift everywhere else.
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceCancelOrderResult {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Option<Decimal>,
    pub executedQty: Option<Decimal>,
    pub expiryReason: Option<BinanceExpiryReason>,
    pub icebergQty: Option<Decimal>,
    pub orderId: Option<i64>,
    pub orderListId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub origQty: Option<Decimal>,
    pub origQuoteOrderQty: Option<Decimal>,
    pub pegOffsetType: Option<BinancePegOffsetType>,
    pub pegOffsetValue: Option<i32>,
    pub pegPriceType: Option<BinancePegPriceType>,
    pub peggedPrice: Option<Decimal>,
    pub preventedMatchId: Option<i64>,
    pub preventedQuantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub selfTradePreventionMode: Option<BinanceSelfTradeProtection>,
    pub side: Option<BinanceSide>,
    pub status: Option<BinanceOrderStatus>,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub symbol: Option<String>,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub trailingDelta: Option<i64>,
    pub trailingTime: Option<i64>,
    pub transactTime: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: Option<BinanceOrderType>,
    pub usedSor: Option<bool>,
    pub workingFloor: Option<BinanceWorkingFloor>,
    pub workingTime: Option<i64>,
}

/// A cancelled order-list (e.g. OCO) response.
///
/// Only [`BinanceCancelOrderListResponse::contingencyType`] is required: it is
/// the discriminator that lets the untagged [`BinanceCancelResponse`] reject
/// single-order bodies (which never carry `contingencyType`) while staying
/// tolerant of schema drift everywhere else.
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceCancelOrderListResponse {
    pub contingencyType: String,
    pub listClientOrderId: Option<String>,
    pub listOrderStatus: Option<String>,
    pub listStatusType: Option<String>,
    pub orderListId: Option<i64>,
    pub orderReports: Option<Vec<BinanceCancelOrderResult>>,
    pub orders: Option<Vec<BinanceOrderListOrder>>,
    pub symbol: Option<String>,
    pub transactionTime: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum BinanceCancelResponse {
    Order(BinanceCancelOrderResult),
    OrderList(BinanceCancelOrderListResponse),
}

impl ResponseFor for BinanceCancelAllOrdersRequest {
    type Response = Vec<BinanceCancelResponse>;
}

impl ResponseFor for BinanceCancelOrderRequest {
    type Response = BinanceCancelResponse;
}
