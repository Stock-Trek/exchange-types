use crate::binance::recv_window::BinanceRecvWindow;
use query_params::QueryParams;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceOpenOrdersParams {
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: Option<String>,
    pub timestamp: i64,
}
