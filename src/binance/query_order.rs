use crate::{
    binance::{
        recv_window::BinanceRecvWindow,
        supporting_types::{
            BinanceExpiryReason, BinanceOrderStatus, BinanceOrderType, BinancePegOffsetType,
            BinancePegPriceType, BinanceSelfTradeProtection, BinanceSide, BinanceTimeInForce,
            BinanceWorkingFloor,
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
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceQueryOrderParams {
    pub apiKey: Option<String>,
    pub orderId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceOrderResult {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Option<Decimal>,
    pub executedQty: Option<Decimal>,
    pub expiryReason: Option<BinanceExpiryReason>,
    pub icebergQty: Option<Decimal>,
    pub isWorking: Option<bool>,
    pub orderId: i64,
    pub orderListId: i64,
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
    pub time: Option<i64>,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub trailingDelta: Option<i64>,
    pub trailingTime: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: BinanceOrderType,
    pub updateTime: Option<i64>,
    pub usedSor: Option<bool>,
    pub workingFloor: Option<BinanceWorkingFloor>,
    pub workingTime: Option<i64>,
}

impl ResponseFor for BinanceQueryOrderParams {
    type Result = BinanceOrderResult;
}
