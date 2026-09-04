use crate::binance::{
    recv_window::BinanceRecvWindow,
    supporting_types::{
        BinanceExpiryReason, BinanceOrderStatus, BinanceOrderType, BinancePegOffsetType,
        BinancePegPriceType, BinanceSelfTradeProtection, BinanceSide, BinanceTimeInForce,
        BinanceWorkingFloor,
    },
};
use query_params::QueryParams;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Check an order's status (`GET /api/v3/order`, WebSocket `order.status`).
///
/// Either `orderId` or `origClientOrderId` must be provided. When both are
/// provided the `orderId` is searched first, then the `origClientOrderId`
/// from that result is checked against that order.
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

/// The order status report returned by Query Order (`order.status`) and by
/// Current Open Orders (`openOrders.status`, as a flat list).
///
/// The payload above both endpoints is the same order object; fields that
/// Binance only emits under certain conditions (iceberg, STP, trailing stop,
/// pegged, SOR and strategy orders) are optional.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> BinanceQueryOrderParams {
        BinanceQueryOrderParams {
            apiKey: None,
            orderId: Some(1),
            origClientOrderId: None,
            recvWindow: None,
            symbol: "BTCUSDT".into(),
            timestamp: 1_660_801_720_951,
        }
    }

    #[test]
    fn query_params_are_alphabetical_and_omit_unset_optionals() {
        let params = params();
        assert_eq!(
            params.query_params(true, true),
            "orderId=1&symbol=BTCUSDT&timestamp=1660801720951"
        );
        let params = BinanceQueryOrderParams {
            apiKey: Some("api-key".into()),
            origClientOrderId: Some("client-order-id".into()),
            recvWindow: BinanceRecvWindow::try_new(60_000),
            ..params
        };
        assert_eq!(
            params.query_params(true, false),
            "apiKey=api-key&orderId=1&origClientOrderId=client-order-id&recvWindow=60000&symbol=BTCUSDT&timestamp=1660801720951"
        );
    }

    #[test]
    fn serialization_skips_unset_optional_fields() {
        let json = serde_json::to_value(params()).unwrap();
        assert_eq!(json["orderId"], 1);
        assert_eq!(json["symbol"], "BTCUSDT");
        assert!(json.get("apiKey").is_none());
        assert!(json.get("origClientOrderId").is_none());
        assert!(json.get("recvWindow").is_none());
    }
}
