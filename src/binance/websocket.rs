use crate::binance::{
    error::BinanceError,
    exchange_info::BinanceExchangeInfoResult,
    logon::BinanceSessionAuthenticationResult,
    rate_limits::BinanceRateLimit,
    signed::{BinanceParams, BinanceUnsignedParams},
    spot::BinanceSpotOrderResult,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Serialize)]
pub struct BinanceWebsocketMetadata {
    pub id: String,
    pub method: BinanceWebsocketMethodName,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum BinanceWebsocketMethodName {
    #[serde(rename = "exchangeInfo")]
    ExchangeInfo,
    #[serde(rename = "session.logon")]
    Logon,
    #[serde(rename = "session.logout")]
    Logout,
    #[serde(rename = "order.place")]
    PlaceOrder,
}

#[derive(Debug, Clone, Serialize)]
#[skip_serializing_none]
pub struct BinanceWebsocketRequest {
    #[serde(flatten)]
    pub metadata: BinanceWebsocketMetadata,
    pub params: BinanceParams,
}

#[derive(Debug, Clone, Serialize)]
#[skip_serializing_none]
pub struct BinanceWebsocketUnsignedRequest {
    #[serde(flatten)]
    pub metadata: BinanceWebsocketMetadata,
    pub params: BinanceUnsignedParams,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize)]
pub struct BinanceWebsocketResponse {
    pub error: Option<BinanceError>,
    pub id: String,
    pub rateLimits: Vec<BinanceRateLimit>,
    pub result: BinanceWebsocketResponseResult,
    pub status: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BinanceWebsocketResponseResult {
    ExchangeInfo(BinanceExchangeInfoResult),
    SessionAuthentication(BinanceSessionAuthenticationResult),
    SpotOrder(BinanceSpotOrderResult),
}
