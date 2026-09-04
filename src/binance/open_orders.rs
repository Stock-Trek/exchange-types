use crate::binance::recv_window::BinanceRecvWindow;
use query_params::QueryParams;
use serde::Serialize;
use serde_with::skip_serializing_none;

/// Get all open orders (`GET /api/v3/openOrders`, WebSocket
/// `openOrders.status`).
///
/// When `symbol` is omitted, open orders for all symbols are returned as a
/// flat list (each report carries its own `symbol`). Omitting the symbol also
/// increases the request weight from 6 to 80.
#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceOpenOrdersParams {
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: Option<String>,
    pub timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(symbol: Option<&str>) -> BinanceOpenOrdersParams {
        BinanceOpenOrdersParams {
            apiKey: None,
            recvWindow: None,
            symbol: symbol.map(String::from),
            timestamp: 1_660_801_720_951,
        }
    }

    #[test]
    fn query_params_omit_unset_symbol() {
        assert_eq!(
            params(None).query_params(true, true),
            "timestamp=1660801720951"
        );
        assert_eq!(
            params(Some("BTCUSDT")).query_params(true, true),
            "symbol=BTCUSDT&timestamp=1660801720951"
        );
    }

    #[test]
    fn serialization_skips_unset_symbol() {
        let json = serde_json::to_value(params(None)).unwrap();
        assert!(json.get("symbol").is_none());
        let json = serde_json::to_value(params(Some("BTCUSDT"))).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
    }
}
