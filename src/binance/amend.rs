use crate::{
    binance::{
        recv_window::BinanceRecvWindow,
        supporting_types::{
            BinanceExpiryReason, BinanceOrderListStatus, BinanceOrderStatus, BinanceOrderType,
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceAmendOrderRequest {
    pub apiKey: Option<String>,
    pub newClientOrderId: Option<String>,
    pub newQty: Decimal,
    pub orderId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAmendOrderResponse {
    pub amendedOrder: BinanceAmendedOrder,
    pub executionId: i64,
    pub listStatus: Option<BinanceOrderListStatus>,
    pub transactTime: i64,
}

impl ResponseFor for BinanceAmendOrderRequest {
    type Response = BinanceAmendOrderResponse;
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAmendedOrder {
    pub clientOrderId: String,
    pub cumulativeQuoteQty: Decimal,
    pub executedQty: Decimal,
    pub expiryReason: Option<BinanceExpiryReason>,
    pub icebergQty: Option<Decimal>,
    pub orderId: i64,
    pub orderListId: i64,
    pub origClientOrderId: String,
    pub pegOffsetType: Option<BinancePegOffsetType>,
    pub pegOffsetValue: Option<i32>,
    pub pegPriceType: Option<BinancePegPriceType>,
    pub peggedPrice: Option<Decimal>,
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
    #[serde(rename = "type")]
    pub r#type: BinanceOrderType,
    pub usedSor: Option<bool>,
    pub workingFloor: Option<BinanceWorkingFloor>,
    pub workingTime: Option<i64>,
}
