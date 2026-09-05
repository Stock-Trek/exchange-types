use crate::{
    binance::{
        recv_window::BinanceRecvWindow, response::BinanceResponse,
        supporting_types::BinanceOrderResponse,
    },
    response::ResponseFor,
};
use query_params::QueryParams;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceOpenOrdersRequest {
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: Option<String>,
    pub timestamp: i64,
}

impl ResponseFor for BinanceOpenOrdersRequest {
    type Response = BinanceResponse<Vec<BinanceOrderResponse>>;
}
