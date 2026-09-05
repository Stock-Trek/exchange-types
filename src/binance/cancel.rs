use crate::{
    binance::{
        recv_window::BinanceRecvWindow,
        request::BinanceRequestFactory,
        response::BinanceResponse,
        supporting_types::{
            BinanceExpiryReason, BinanceOrderListOrder, BinanceOrderStatus, BinanceOrderType,
            BinancePegOffsetType, BinancePegPriceType, BinanceSelfTradeProtection, BinanceSide,
            BinanceTimeInForce, BinanceWorkingFloor,
        },
    },
    error::ETResult,
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimitRestriction,
    request::{ETHttpRequest, ETRequest, ETWebsocketRequest},
    signer::Signer,
    websocket_id::ETWebsocketId,
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

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceCancelOrderResponse {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Option<Decimal>,
    pub executedQty: Option<Decimal>,
    pub expiryReason: Option<BinanceExpiryReason>,
    pub icebergQty: Option<Decimal>,
    pub orderId: i64,
    pub orderListId: i64,
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
pub struct BinanceCancelOrderListResponse {
    pub contingencyType: String,
    pub listClientOrderId: String,
    pub listOrderStatus: String,
    pub listStatusType: String,
    pub orderListId: i64,
    #[serde(default)]
    pub orderReports: Vec<BinanceCancelOrderResponse>,
    #[serde(default)]
    pub orders: Vec<BinanceOrderListOrder>,
    pub symbol: String,
    pub transactionTime: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum BinanceCancelResponse {
    Order(BinanceCancelOrderResponse),
    OrderList(BinanceCancelOrderListResponse),
}

impl ETRequest for BinanceCancelOrderRequest {
    type Response = BinanceResponse<BinanceCancelOrderResponse>;

    fn is_signed(&self) -> bool {
        true
    }
    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32 {
        match restriction {
            RateLimitRestriction::Weight => 1,
            _ => 0,
        }
    }
    fn set_api_key(&mut self, api_key: Option<String>) {
        self.apiKey = api_key;
    }
    fn query_params(&self, percent_encode: bool) -> String {
        self.query_params(true, percent_encode)
    }
}

impl ETHttpRequest for BinanceCancelOrderRequest {
    fn endpoint(&self) -> &'static str {
        "order"
    }
    fn method(&self) -> HttpMethod {
        HttpMethod::DELETE
    }
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest> {
        BinanceRequestFactory::try_into_http(self, signer)
    }
}

impl ETWebsocketRequest for BinanceCancelOrderRequest {
    fn method(&self) -> &'static str {
        "order.cancel"
    }
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        BinanceRequestFactory::try_into_websocket(self, signer, id)
    }
}

impl ETRequest for BinanceCancelAllOrdersRequest {
    type Response = BinanceResponse<Vec<BinanceCancelOrderResponse>>;

    fn is_signed(&self) -> bool {
        true
    }
    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32 {
        match restriction {
            RateLimitRestriction::Weight => 1,
            _ => 0,
        }
    }
    fn set_api_key(&mut self, api_key: Option<String>) {
        self.apiKey = api_key;
    }
    fn query_params(&self, percent_encode: bool) -> String {
        self.query_params(true, percent_encode)
    }
}

impl ETHttpRequest for BinanceCancelAllOrdersRequest {
    fn endpoint(&self) -> &'static str {
        "openOrders"
    }
    fn method(&self) -> HttpMethod {
        HttpMethod::DELETE
    }
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest> {
        BinanceRequestFactory::try_into_http(self, signer)
    }
}

impl ETWebsocketRequest for BinanceCancelAllOrdersRequest {
    fn method(&self) -> &'static str {
        "openOrders.cancelAll"
    }
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        BinanceRequestFactory::try_into_websocket(self, signer, id)
    }
}
