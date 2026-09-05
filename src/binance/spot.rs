use crate::{
    binance::{
        recv_window::BinanceRecvWindow,
        response::BinanceResponse,
        supporting_types::{
            BinanceExpiryReason, BinanceOrderStatus, BinanceOrderType, BinancePegOffsetType,
            BinancePegPriceType, BinanceSelfTradeProtection, BinanceSide, BinanceTimeInForce,
            BinanceWorkingFloor,
        },
    },
    response::ResponseFor,
    ticker::Ticker,
};
use query_params::QueryParams;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::Display;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceSpotOrderRequest {
    pub apiKey: Option<String>,
    pub icebergQty: Option<Decimal>,
    pub newClientOrderId: Option<String>,
    pub newOrderRespType: Option<BinanceNewOrderResponseType>,
    pub pegPriceType: Option<BinancePegPriceType>,
    pub pegOffsetValue: Option<i32>,
    pub pegOffsetType: Option<BinancePegOffsetType>,
    pub price: Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub quoteOrderQty: Option<Decimal>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub selfTradePreventionMode: Option<BinanceSelfTradeProtection>,
    pub side: BinanceSide,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub symbol: String,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub timestamp: i64,
    pub trailingDelta: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: BinanceOrderType,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceNewOrderResponseType {
    ACK,
    RESULT,
    FULL,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize)]
#[skip_serializing_none]
pub struct BinanceSpotOrderResponse {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Option<Decimal>,
    pub executedQty: Option<Decimal>,
    pub expiryReason: Option<BinanceExpiryReason>,
    pub fills: Option<Vec<BinanceFill>>,
    pub icebergQty: Option<Decimal>,
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
    pub side: Option<BinanceSide>,
    pub status: Option<BinanceOrderStatus>,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub symbol: String,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub trailingDelta: Option<i64>,
    pub trailingTime: Option<i64>,
    pub transactTime: i64,
    #[serde(rename = "type")]
    pub r#type: Option<BinanceOrderType>,
    pub usedSor: Option<bool>,
    pub workingFloor: Option<BinanceWorkingFloor>,
    pub workingTime: Option<i64>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinanceFill {
    pub allocId: Option<i64>,
    pub commission: Decimal,
    pub commissionAsset: Ticker,
    pub matchType: Option<String>,
    pub price: Decimal,
    pub qty: Decimal,
    pub tradeId: i64,
}

impl ResponseFor for BinanceSpotOrderRequest {
    type Response = BinanceResponse<BinanceSpotOrderResponse>;
}
