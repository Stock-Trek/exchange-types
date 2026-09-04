use crate::{
    binance::{
        account::BinanceAccountResult,
        amend::BinanceAmendOrderResult,
        asset_limits::BinanceAssetLimitsResult,
        cancel::{BinanceCancelOrderListResult, BinanceCancelOrderResult, BinanceCancelReport},
        error::BinanceError,
        exchange_info::BinanceExchangeInfoResult,
        query_order::BinanceOrderResult,
        rate_limits::{BinanceRateLimit, BinanceRateLimitInterval, BinanceRateLimitType},
        session::BinanceSessionAuthenticationResult,
        spot::BinanceSpotOrderResult,
        time::BinanceTimeResult,
    },
    error::{ETError, ETResult},
    http::HttpResponse,
    response::{ETHttpResponse, ETWebsocketResponse},
    websocket_id::ETWebsocketId,
};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Clone)]
pub struct BinanceResponse {
    pub metadata: BinanceMetadata,
    pub payload: BinanceResponsePayload,
}

#[allow(clippy::large_enum_variant)]
#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug, Clone)]
pub enum BinanceResponsePayload {
    Success(BinanceResult),
    Failure(BinanceError),
}

#[derive(Debug, Clone)]
pub struct BinanceMetadata {
    pub usage: BinanceUsage,
    pub retry_after: Option<u64>,
    pub websocket_id: Option<ETWebsocketId>,
    pub status: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BinanceUsage {
    pub used_weight_1m: Option<u32>,
    pub order_count_10s: Option<u32>,
    pub order_count_1m: Option<u32>,
    pub order_count_1h: Option<u32>,
    pub order_count_1d: Option<u32>,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug, Clone)]
pub enum BinanceResult {
    Account(BinanceAccountResult),
    AmendOrder(BinanceAmendOrderResult),
    AssetLimits(BinanceAssetLimitsResult),
    CancelAllOrders(Vec<BinanceCancelReport>),
    CancelOrder(BinanceCancelOrderResult),
    CancelOrderList(BinanceCancelOrderListResult),
    ExchangeInfo(BinanceExchangeInfoResult),
    OpenOrders(Vec<BinanceOrderResult>),
    QueryOrder(BinanceOrderResult),
    SpotOrder(BinanceSpotOrderResult),
    Time(BinanceTimeResult),
    WebsocketSessionAuthentication(BinanceSessionAuthenticationResult),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct BinanceWebsocketResponse {
    pub error: Option<BinanceError>,
    pub id: Option<ETWebsocketId>,
    #[serde(default)]
    pub rateLimits: Vec<BinanceRateLimit>,
    pub result: Option<BinanceWebsocketResponseResult>,
    pub status: u16,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug, Clone)]
pub enum BinanceWebsocketResponseResult {
    Account(BinanceAccountResult),
    AmendOrder(BinanceAmendOrderResult),
    AssetLimits(BinanceAssetLimitsResult),
    CancelAllOrders(Vec<BinanceCancelReport>),
    CancelOrder(BinanceCancelOrderResult),
    CancelOrderList(BinanceCancelOrderListResult),
    ExchangeInfo(BinanceExchangeInfoResult),
    OpenOrders(Vec<BinanceOrderResult>),
    QueryOrder(BinanceOrderResult),
    SessionAuthentication(BinanceSessionAuthenticationResult),
    SpotOrder(BinanceSpotOrderResult),
    Time(BinanceTimeResult),
}

impl ETHttpResponse for BinanceResponse {
    fn try_from_http(response: HttpResponse) -> ETResult<Self> {
        let mut usage = BinanceUsage::default();
        let mut retry_after = None;
        for (name, value) in response.headers {
            let name = name.to_ascii_lowercase();
            let value = value.trim();
            match name.as_str() {
                "x-mbx-used-weight-1m" => usage.used_weight_1m = value.parse().ok(),
                "x-mbx-order-count-10s" => usage.order_count_10s = value.parse().ok(),
                "x-mbx-order-count-1m" => usage.order_count_1m = value.parse().ok(),
                "x-mbx-order-count-1h" => usage.order_count_1h = value.parse().ok(),
                "x-mbx-order-count-1d" => usage.order_count_1d = value.parse().ok(),
                "retry-after" => retry_after = value.parse().ok(),
                _ => {}
            }
        }
        let metadata = BinanceMetadata {
            usage,
            retry_after,
            websocket_id: None,
            status: response.status,
        };
        match serde_json::from_slice::<BinanceResponsePayload>(&response.body) {
            Ok(payload) => Ok(BinanceResponse { metadata, payload }),
            Err(error) => {
                if (200..300).contains(&response.status) {
                    // A 2xx body that is neither a Binance result nor a
                    // Binance error payload is not a Binance response (e.g.
                    // an HTML page served by an intermediary proxy).
                    Err(ETError::DeserializeResponse(error))
                } else {
                    // Non-2xx responses can have an empty or non-JSON body
                    // (e.g. HTTP 429/418 rate limiting, 5xx gateway pages).
                    // Surface them as a failure carrying the HTTP status and
                    // the raw body so nothing is lost.
                    Ok(BinanceResponse {
                        metadata,
                        payload: BinanceResponsePayload::Failure(BinanceError {
                            code: i64::from(response.status),
                            msg: String::from_utf8_lossy(&response.body).into_owned(),
                            data: None,
                        }),
                    })
                }
            }
        }
    }
}

impl ETWebsocketResponse for BinanceResponse {
    fn try_from_websocket(response: String) -> ETResult<Self> {
        let websocket_response: BinanceWebsocketResponse =
            serde_json::from_str(&response).map_err(ETError::DeserializeResponse)?;
        let mut metadata = BinanceMetadata {
            usage: BinanceUsage::default(),
            retry_after: None,
            websocket_id: websocket_response.id,
            status: websocket_response.status,
        };
        for rate_limit in websocket_response.rateLimits {
            let count_u32 = rate_limit.count.map(|c| c as u32);
            match (
                rate_limit.rateLimitType,
                rate_limit.interval,
                rate_limit.intervalNum,
            ) {
                (BinanceRateLimitType::REQUEST_WEIGHT, BinanceRateLimitInterval::MINUTE, 1) => {
                    metadata.usage.used_weight_1m = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::SECOND, 10) => {
                    metadata.usage.order_count_10s = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::MINUTE, 1) => {
                    metadata.usage.order_count_1m = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::HOUR, 1) => {
                    metadata.usage.order_count_1h = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::DAY, 1) => {
                    metadata.usage.order_count_1d = count_u32;
                }
                _ => {}
            }
        }
        let payload = if let Some(error) = websocket_response.error {
            BinanceResponsePayload::Failure(error)
        } else if let Some(result) = websocket_response.result {
            let binance_result = match result {
                BinanceWebsocketResponseResult::Account(r) => BinanceResult::Account(r),
                BinanceWebsocketResponseResult::AmendOrder(r) => BinanceResult::AmendOrder(r),
                BinanceWebsocketResponseResult::AssetLimits(r) => BinanceResult::AssetLimits(r),
                BinanceWebsocketResponseResult::CancelAllOrders(r) => {
                    BinanceResult::CancelAllOrders(r)
                }
                BinanceWebsocketResponseResult::CancelOrder(r) => BinanceResult::CancelOrder(r),
                BinanceWebsocketResponseResult::CancelOrderList(r) => {
                    BinanceResult::CancelOrderList(r)
                }
                BinanceWebsocketResponseResult::ExchangeInfo(r) => BinanceResult::ExchangeInfo(r),
                BinanceWebsocketResponseResult::OpenOrders(r) => BinanceResult::OpenOrders(r),
                BinanceWebsocketResponseResult::QueryOrder(r) => BinanceResult::QueryOrder(r),
                BinanceWebsocketResponseResult::SessionAuthentication(r) => {
                    BinanceResult::WebsocketSessionAuthentication(r)
                }
                BinanceWebsocketResponseResult::SpotOrder(r) => BinanceResult::SpotOrder(r),
                BinanceWebsocketResponseResult::Time(r) => BinanceResult::Time(r),
            };
            BinanceResponsePayload::Success(binance_result)
        } else {
            // No error and no result – treat as a generic failure
            BinanceResponsePayload::Failure(BinanceError {
                code: -1,
                msg: "Websocket response missing both error and result".to_string(),
                data: None,
            })
        };
        Ok(BinanceResponse { metadata, payload })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::{
        exchange_info::BinanceExchangeInfoPermission,
        supporting_types::{BinanceOrderStatus, BinanceOrderType, BinanceSide},
    };

    fn http_response(body: &str) -> HttpResponse {
        HttpResponse {
            status: 200,
            headers: vec![],
            body: body.as_bytes().to_vec(),
        }
    }

    #[test]
    fn parses_query_order_http_response() {
        let body = r#"{
            "symbol": "LTCBTC",
            "orderId": 1,
            "orderListId": -1,
            "clientOrderId": "myOrder1",
            "price": "0.1",
            "origQty": "1.0",
            "executedQty": "0.0",
            "cummulativeQuoteQty": "0.0",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "stopPrice": "0.0",
            "icebergQty": "0.0",
            "time": 1499827319559,
            "updateTime": 1499827319559,
            "isWorking": true,
            "workingTime": 1499827319559,
            "origQuoteOrderQty": "0.000000",
            "selfTradePreventionMode": "NONE"
        }
        "#;
        let response = BinanceResponse::try_from_http(http_response(body)).unwrap();
        match response.payload {
            BinanceResponsePayload::Success(BinanceResult::QueryOrder(order)) => {
                assert_eq!(order.symbol, "LTCBTC");
                assert_eq!(order.orderId, 1);
                assert_eq!(order.orderListId, -1);
                assert_eq!(order.clientOrderId, "myOrder1");
                assert!(matches!(order.status, BinanceOrderStatus::NEW));
                assert!(matches!(order.side, BinanceSide::BUY));
                assert!(matches!(order.r#type, BinanceOrderType::LIMIT));
                assert_eq!(order.time, Some(1_499_827_319_559));
                assert_eq!(order.updateTime, Some(1_499_827_319_559));
                assert_eq!(order.isWorking, Some(true));
                assert_eq!(order.icebergQty.unwrap().to_string(), "0.0");
            }
            other => panic!("expected QueryOrder result, got {other:?}"),
        }
    }

    #[test]
    fn parses_open_orders_http_response() {
        let body = r#"[
            {
                "symbol": "LTCBTC",
                "orderId": 1,
                "orderListId": -1,
                "clientOrderId": "myOrder1",
                "price": "0.1",
                "origQty": "1.0",
                "executedQty": "0.0",
                "cummulativeQuoteQty": "0.0",
                "status": "NEW",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "BUY",
                "stopPrice": "0.0",
                "icebergQty": "0.0",
                "time": 1499827319559,
                "updateTime": 1499827319559,
                "isWorking": true,
                "origQuoteOrderQty": "0.000000",
                "workingTime": 1499827319559,
                "selfTradePreventionMode": "NONE"
            }
        ]
        "#;
        let response = BinanceResponse::try_from_http(http_response(body)).unwrap();
        match response.payload {
            BinanceResponsePayload::Success(BinanceResult::OpenOrders(orders)) => {
                assert_eq!(orders.len(), 1);
                assert_eq!(orders[0].symbol, "LTCBTC");
                assert_eq!(orders[0].orderId, 1);
                assert!(matches!(orders[0].status, BinanceOrderStatus::NEW));
            }
            other => panic!("expected OpenOrders result, got {other:?}"),
        }
    }

    #[test]
    fn parses_account_http_response() {
        let body = r#"{
            "makerCommission": 15,
            "takerCommission": 15,
            "buyerCommission": 0,
            "sellerCommission": 0,
            "commissionRates": {
                "maker": "0.00150000",
                "taker": "0.00150000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "canTrade": true,
            "canWithdraw": true,
            "canDeposit": true,
            "brokered": false,
            "requireSelfTradePrevention": false,
            "preventSor": false,
            "updateTime": 123456789,
            "accountType": "SPOT",
            "balances": [
                {
                    "asset": "BTC",
                    "free": "4723846.89208129",
                    "locked": "0.00000000"
                },
                {
                    "asset": "LTC",
                    "free": "4763368.68006011",
                    "locked": "0.00000000"
                }
            ],
            "permissions": ["SPOT"],
            "uid": 354937868
        }
        "#;
        let response = BinanceResponse::try_from_http(http_response(body)).unwrap();
        match response.payload {
            BinanceResponsePayload::Success(BinanceResult::Account(account)) => {
                assert_eq!(account.accountType, "SPOT");
                assert_eq!(account.makerCommission, 15);
                assert_eq!(account.takerCommission, 15);
                assert_eq!(account.buyerCommission, 0);
                assert_eq!(account.sellerCommission, 0);
                assert!(account.canTrade);
                assert!(account.canWithdraw);
                assert!(account.canDeposit);
                assert!(!account.brokered);
                assert!(!account.requireSelfTradePrevention);
                assert!(!account.preventSor);
                assert_eq!(account.updateTime, 123_456_789);
                assert_eq!(account.uid, 354_937_868);
                assert_eq!(account.balances.len(), 2);
                assert_eq!(account.balances[0].asset, "BTC");
                assert_eq!(account.balances[0].free.to_string(), "4723846.89208129");
                assert_eq!(account.balances[0].locked.to_string(), "0.00000000");
                assert_eq!(account.commissionRates.maker.to_string(), "0.00150000");
                assert_eq!(account.commissionRates.taker.to_string(), "0.00150000");
                assert_eq!(
                    account.permissions,
                    vec![BinanceExchangeInfoPermission::SPOT]
                );
            }
            other => panic!("expected Account result, got {other:?}"),
        }
    }

    #[test]
    fn parses_query_order_websocket_response() {
        let frame = r#"{
            "id": "aa62318a-5a97-4f3b-bdc7-640bbe33b291",
            "status": 200,
            "result": {
                "symbol": "BTCUSDT",
                "orderId": 12569099453,
                "orderListId": -1,
                "clientOrderId": "4d96324ff9d44481926157",
                "price": "23416.10000000",
                "origQty": "0.00847000",
                "executedQty": "0.00847000",
                "cummulativeQuoteQty": "198.33521500",
                "status": "FILLED",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "SELL",
                "stopPrice": "0.00000000",
                "trailingDelta": 10,
                "trailingTime": -1,
                "icebergQty": "0.00000000",
                "time": 1660801715639,
                "updateTime": 1660801717945,
                "isWorking": true,
                "workingTime": 1660801715639,
                "origQuoteOrderQty": "0.00000000",
                "strategyId": 37463720,
                "strategyType": 1000000,
                "selfTradePreventionMode": "NONE",
                "preventedMatchId": 0,
                "preventedQuantity": "1.200000"
            },
            "rateLimits": [
                {
                    "rateLimitType": "REQUEST_WEIGHT",
                    "interval": "MINUTE",
                    "intervalNum": 1,
                    "limit": 6000,
                    "count": 4
                }
            ]
        }
        "#;
        let response = BinanceResponse::try_from_websocket(frame.to_string()).unwrap();
        assert_eq!(
            response.metadata.websocket_id,
            Some(ETWebsocketId::Str(
                "aa62318a-5a97-4f3b-bdc7-640bbe33b291".into()
            ))
        );
        assert_eq!(response.metadata.usage.used_weight_1m, Some(4));
        match response.payload {
            BinanceResponsePayload::Success(BinanceResult::QueryOrder(order)) => {
                assert_eq!(order.symbol, "BTCUSDT");
                assert_eq!(order.orderId, 12_569_099_453);
                assert!(matches!(order.status, BinanceOrderStatus::FILLED));
                assert!(matches!(order.side, BinanceSide::SELL));
                assert_eq!(order.trailingDelta, Some(10));
                assert_eq!(order.trailingTime, Some(-1));
                assert_eq!(order.strategyId, Some(37_463_720));
                assert_eq!(order.strategyType, Some(1_000_000));
                assert_eq!(order.preventedMatchId, Some(0));
                assert_eq!(order.preventedQuantity.unwrap().to_string(), "1.200000");
            }
            other => panic!("expected QueryOrder result, got {other:?}"),
        }
    }

    #[test]
    fn parses_account_websocket_response() {
        let frame = r#"{
            "id": "605a6d20-6588-4cb9-afa0-b0ab087507ba",
            "status": 200,
            "result": {
                "makerCommission": 15,
                "takerCommission": 15,
                "buyerCommission": 0,
                "sellerCommission": 0,
                "canTrade": true,
                "canWithdraw": true,
                "canDeposit": true,
                "commissionRates": {
                    "maker": "0.00150000",
                    "taker": "0.00150000",
                    "buyer": "0.00000000",
                    "seller": "0.00000000"
                },
                "brokered": false,
                "requireSelfTradePrevention": false,
                "preventSor": false,
                "updateTime": 1660801833000,
                "accountType": "SPOT",
                "balances": [
                    {
                        "asset": "BNB",
                        "free": "0.00000000",
                        "locked": "0.00000000"
                    },
                    {
                        "asset": "BTC",
                        "free": "1.3447112",
                        "locked": "0.08600000"
                    },
                    {
                        "asset": "USDT",
                        "free": "1021.21000000",
                        "locked": "0.00000000"
                    }
                ],
                "permissions": ["SPOT"],
                "uid": 354937868
            },
            "rateLimits": [
                {
                    "rateLimitType": "REQUEST_WEIGHT",
                    "interval": "MINUTE",
                    "intervalNum": 1,
                    "limit": 6000,
                    "count": 20
                }
            ]
        }
        "#;
        let response = BinanceResponse::try_from_websocket(frame.to_string()).unwrap();
        assert_eq!(response.metadata.usage.used_weight_1m, Some(20));
        match response.payload {
            BinanceResponsePayload::Success(BinanceResult::Account(account)) => {
                assert_eq!(account.accountType, "SPOT");
                assert_eq!(account.uid, 354_937_868);
                assert_eq!(account.balances.len(), 3);
                assert_eq!(account.balances[1].asset, "BTC");
                assert_eq!(account.balances[1].free.to_string(), "1.3447112");
                assert_eq!(account.balances[1].locked.to_string(), "0.08600000");
                assert_eq!(account.commissionRates.buyer.to_string(), "0.00000000");
            }
            other => panic!("expected Account result, got {other:?}"),
        }
    }

    #[test]
    fn parses_existing_payloads_to_their_original_variants() {
        // A spot order (order.place) response must keep parsing as SpotOrder.
        let spot = r#"{
            "symbol": "BTCUSDT",
            "orderId": 1,
            "orderListId": -1,
            "clientOrderId": "myOrder1",
            "transactTime": 1499827319559
        }
        "#;
        let response = BinanceResponse::try_from_http(http_response(spot)).unwrap();
        assert!(matches!(
            response.payload,
            BinanceResponsePayload::Success(BinanceResult::SpotOrder(_))
        ));
        // A cancel (order.cancel) response must keep parsing as CancelOrder.
        let cancel = r#"{
            "symbol": "LTCBTC",
            "origClientOrderId": "myOrder1",
            "orderId": 1,
            "orderListId": -1,
            "clientOrderId": "cancelMyOrder1",
            "price": "0.1",
            "origQty": "1.0",
            "executedQty": "0.0",
            "cummulativeQuoteQty": "0.0",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "selfTradePreventionMode": "NONE"
        }
        "#;
        let response = BinanceResponse::try_from_http(http_response(cancel)).unwrap();
        assert!(matches!(
            response.payload,
            BinanceResponsePayload::Success(BinanceResult::CancelOrder(_))
        ));
    }
}
