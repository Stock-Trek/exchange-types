use crate::binance::{
    recv_window::BinanceRecvWindow,
    supporting_types::{
        BinanceExpiryReason, BinanceOrderListOrder, BinanceOrderStatus, BinanceOrderType,
        BinancePegOffsetType, BinancePegPriceType, BinanceSelfTradeProtection, BinanceSide,
        BinanceTimeInForce, BinanceWorkingFloor,
    },
};
use query_params::QueryParams;
use rust_decimal::Decimal;
use strum::Display;

use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceCancelAllOrdersParams {
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceCancelOrderParams {
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
#[derive(Serialize, Debug, Clone, Copy, Hash, Display)]
pub enum BinanceCancelRestrictions {
    ONLY_NEW,
    ONLY_PARTIALLY_FILLED,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceCancelOrderResult {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Option<Decimal>,
    pub executedQty: Option<Decimal>,
    pub expiryReason: Option<BinanceExpiryReason>,
    pub icebergQty: Option<Decimal>,
    pub orderId: i64,
    pub orderListId: i32,
    pub origClientOrderId: String,
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
    #[serde(rename = "type")]
    pub r#type: BinanceOrderType,
    pub usedSor: Option<bool>,
    pub workingFloor: Option<BinanceWorkingFloor>,
    pub workingTime: Option<i64>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceCancelOrderListResult {
    pub contingencyType: String,
    pub listClientOrderId: String,
    pub listOrderStatus: String,
    pub listStatusType: String,
    pub orderListId: i32,
    #[serde(default)]
    pub orderReports: Vec<BinanceCancelOrderResult>,
    #[serde(default)]
    pub orders: Vec<BinanceOrderListOrder>,
    pub symbol: String,
    pub transactionTime: i64,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum BinanceCancelReport {
    Order(BinanceCancelOrderResult),
    OrderList(BinanceCancelOrderListResult),
}
