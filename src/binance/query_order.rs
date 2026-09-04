use crate::{
    binance::{recv_window::BinanceRecvWindow, supporting_types::BinanceOrderResponse},
    response::ResponseFor,
};
use query_params::QueryParams;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceQueryOrderRequest {
    pub apiKey: Option<String>,
    pub orderId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

impl ResponseFor for BinanceQueryOrderRequest {
    type Response = BinanceOrderResponse;
}
