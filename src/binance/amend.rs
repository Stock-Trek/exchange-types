use crate::binance::{
    cancel::BinanceOrderListStatus,
    exchange_info::BinanceOrderType,
    recv_window::BinanceRecvWindow,
    spot::{
        BinanceExpiryReason, BinanceOrderStatus, BinancePegOffsetType, BinancePegPriceType,
        BinanceSelfTradeProtection, BinanceSide, BinanceTimeInForce, BinanceWorkingFloor,
    },
};
use query_params::QueryParams;
use rust_decimal::Decimal;

use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceAmendOrderParams {
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
pub struct BinanceAmendOrderResult {
    pub amendedOrder: BinanceAmendedOrder,
    pub executionId: i64,
    pub listStatus: Option<BinanceOrderListStatus>,
    pub transactTime: i64,
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
    pub orderListId: i32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_amend_result_with_list_status() {
        let result: BinanceAmendOrderResult = serde_json::from_str(
            r#"{
                "transactTime": 1741669661670,
                "executionId": 22,
                "amendedOrder": {
                    "symbol": "BTCUSDT",
                    "orderId": 9,
                    "orderListId": 1,
                    "origClientOrderId": "W0fJ9fiLKHOJutovPK3oJp",
                    "clientOrderId": "UQ1Np3bmQ71jJzsSDW9Vpi",
                    "price": "0.00000000",
                    "qty": "4.00000000",
                    "executedQty": "0.00000000",
                    "preventedQty": "0.00000000",
                    "quoteOrderQty": "0.00000000",
                    "cumulativeQuoteQty": "0.00000000",
                    "status": "PENDING_NEW",
                    "timeInForce": "GTC",
                    "type": "MARKET",
                    "side": "BUY",
                    "usedSor": true,
                    "workingFloor": "SOR",
                    "selfTradePreventionMode": "NONE"
                },
                "listStatus": {
                    "orderListId": 1,
                    "contingencyType": "OTO",
                    "listOrderStatus": "EXECUTING",
                    "listClientOrderId": "AT7FTxZXylVSwRoZs52mt3",
                    "symbol": "BTCUSDT",
                    "orders": [
                        {
                            "symbol": "BTCUSDT",
                            "orderId": 8,
                            "clientOrderId": "GkwwHZUUbFtZOoH1YsZk9Q"
                        },
                        {
                            "symbol": "BTCUSDT",
                            "orderId": 9,
                            "clientOrderId": "UQ1Np3bmQ71jJzsSDW9Vpi"
                        }
                    ]
                }
            }"#,
        )
        .unwrap();
        assert_eq!(result.transactTime, 1_741_669_661_670);
        assert_eq!(result.executionId, 22);
        assert_eq!(result.amendedOrder.usedSor, Some(true));
        assert_eq!(
            result.amendedOrder.workingFloor,
            Some(BinanceWorkingFloor::SOR)
        );
        let list_status = result.listStatus.unwrap();
        assert_eq!(list_status.orderListId, 1);
        assert_eq!(list_status.contingencyType, "OTO");
        assert_eq!(list_status.listOrderStatus, "EXECUTING");
        assert_eq!(list_status.orders.len(), 2);
    }
}
