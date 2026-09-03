use crate::{
    binance::{
        amend::{BinanceAmendOrderParams, BinanceAmendOrderResult},
        asset_limits::{BinanceAssetLimitsParams, BinanceAssetLimitsResult},
        cancel::{
            BinanceCancelAllOrdersParams, BinanceCancelOrderListResult, BinanceCancelOrderParams,
            BinanceCancelOrderResult, BinanceCancelReport,
        },
        error::BinanceError,
        exchange_info::{BinanceExchangeInfoParams, BinanceExchangeInfoResult},
        logon::{BinanceLogonParams, BinanceSessionAuthenticationResult},
        rate_limits::BinanceRateLimit,
        signature::BinanceSignature,
        spot::{BinanceSpotOrderParams, BinanceSpotOrderResult},
        time::{BinanceTimeParams, BinanceTimeResult},
    },
    error::ETResult,
    rate_limited::RateLimited,
    signer::{IntoSigned, Signer},
};

#[cfg(feature = "serde")]
use {
    crate::error::ETError,
    serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct},
    serde_with::skip_serializing_none,
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinanceWebsocketMethodName {
    #[cfg_attr(feature = "serde", serde(rename = "order.amend.keepPriority"))]
    AmendOrder,
    #[cfg_attr(feature = "serde", serde(rename = "myFilters"))]
    AssetLimits,
    #[cfg_attr(feature = "serde", serde(rename = "order.cancel"))]
    CancelOrder,
    #[cfg_attr(feature = "serde", serde(rename = "openOrders.cancelAll"))]
    CancelAllOrders,
    #[cfg_attr(feature = "serde", serde(rename = "exchangeInfo"))]
    ExchangeInfo,
    #[cfg_attr(feature = "serde", serde(rename = "session.logon"))]
    Logon,
    #[cfg_attr(feature = "serde", serde(rename = "session.logout"))]
    Logout,
    #[cfg_attr(feature = "serde", serde(rename = "order.place"))]
    PlaceOrder,
    #[cfg_attr(feature = "serde", serde(rename = "time"))]
    Time,
}

#[derive(Debug, Clone)]
pub struct BinanceWebsocketRequest {
    pub id: String,
    pub params: BinanceWebsocketSignedParams,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceWebsocketSignedParams {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub unsigned: BinanceWebsocketUnsignedParams,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub signature: Option<BinanceSignature>,
}

#[derive(Debug, Clone)]
pub struct BinanceWebsocketUnsignedRequest {
    pub id: String,
    pub params: BinanceWebsocketUnsignedParams,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, Hash)]
pub enum BinanceWebsocketUnsignedParams {
    AmendOrderRequest(BinanceAmendOrderParams),
    AssetLimits(BinanceAssetLimitsParams),
    CancelAllOrdersRequest(BinanceCancelAllOrdersParams),
    CancelOrderRequest(BinanceCancelOrderParams),
    ExchangeInfo(BinanceExchangeInfoParams),
    Logon(BinanceLogonParams),
    SpotOrderRequest(Box<BinanceSpotOrderParams>),
    Time(BinanceTimeParams),
}

impl BinanceWebsocketUnsignedParams {
    pub fn method_name(&self) -> BinanceWebsocketMethodName {
        match self {
            BinanceWebsocketUnsignedParams::AmendOrderRequest(..) => {
                BinanceWebsocketMethodName::AmendOrder
            }
            BinanceWebsocketUnsignedParams::AssetLimits(..) => {
                BinanceWebsocketMethodName::AssetLimits
            }
            BinanceWebsocketUnsignedParams::CancelAllOrdersRequest(..) => {
                BinanceWebsocketMethodName::CancelAllOrders
            }
            BinanceWebsocketUnsignedParams::CancelOrderRequest(..) => {
                BinanceWebsocketMethodName::CancelOrder
            }
            BinanceWebsocketUnsignedParams::ExchangeInfo(..) => {
                BinanceWebsocketMethodName::ExchangeInfo
            }
            BinanceWebsocketUnsignedParams::Logon(..) => BinanceWebsocketMethodName::Logon,
            BinanceWebsocketUnsignedParams::SpotOrderRequest(..) => {
                BinanceWebsocketMethodName::PlaceOrder
            }
            BinanceWebsocketUnsignedParams::Time(..) => BinanceWebsocketMethodName::Time,
        }
    }
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceWebsocketResponse {
    pub error: Option<BinanceError>,
    pub id: String,
    pub rateLimits: Vec<BinanceRateLimit>,
    pub result: Option<BinanceWebsocketResponseResult>,
    pub status: i32,
}

/// The deserialized `result` of a Binance WebSocket API response.
///
/// The variants are ordered so that an untagged payload is matched against
/// the most specific result shape first, and so that a payload Binance
/// returns for one request can never be mistaken for another request's
/// result (e.g. an order-list cancel report has no `origClientOrderId` and
/// an order cancel report has no `contingencyType`).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceWebsocketResponseResult {
    AmendOrder(BinanceAmendOrderResult),
    AssetLimits(BinanceAssetLimitsResult),
    CancelAllOrders(Vec<BinanceCancelReport>),
    CancelOrder(BinanceCancelOrderResult),
    CancelOrderList(BinanceCancelOrderListResult),
    ExchangeInfo(BinanceExchangeInfoResult),
    SessionAuthentication(BinanceSessionAuthenticationResult),
    SpotOrder(BinanceSpotOrderResult),
    Time(BinanceTimeResult),
}

impl RateLimited for BinanceWebsocketUnsignedParams {
    fn order_count(&self) -> u32 {
        match self {
            BinanceWebsocketUnsignedParams::SpotOrderRequest(..) => 1,
            _ => 0,
        }
    }
    fn weight(&self) -> u32 {
        match self {
            BinanceWebsocketUnsignedParams::AmendOrderRequest(..) => 4,
            BinanceWebsocketUnsignedParams::AssetLimits(..) => 40,
            BinanceWebsocketUnsignedParams::CancelAllOrdersRequest(..) => 1,
            BinanceWebsocketUnsignedParams::CancelOrderRequest(..) => 1,
            BinanceWebsocketUnsignedParams::ExchangeInfo(..) => 20,
            BinanceWebsocketUnsignedParams::Logon(..) => 2,
            BinanceWebsocketUnsignedParams::SpotOrderRequest(..) => 1,
            BinanceWebsocketUnsignedParams::Time(..) => 1,
        }
    }
}

impl IntoSigned for BinanceWebsocketUnsignedRequest {
    type Signed = BinanceWebsocketRequest;

    fn into_signed(self, signer: &Signer) -> ETResult<BinanceWebsocketRequest> {
        let BinanceWebsocketUnsignedRequest { id, params } = self;
        let query_string = match &params {
            BinanceWebsocketUnsignedParams::AmendOrderRequest(params) => {
                Some(params.query_params(true))
            }
            BinanceWebsocketUnsignedParams::AssetLimits(params) => Some(params.query_params(true)),
            BinanceWebsocketUnsignedParams::CancelAllOrdersRequest(params) => {
                Some(params.query_params(true))
            }
            BinanceWebsocketUnsignedParams::CancelOrderRequest(params) => {
                Some(params.query_params(true))
            }
            BinanceWebsocketUnsignedParams::Logon(params) => Some(params.query_params(true)),
            BinanceWebsocketUnsignedParams::SpotOrderRequest(params) => {
                Some(params.query_params(true))
            }
            _ => None,
        };
        let signature = match query_string {
            Some(query_string) => Some(BinanceSignature {
                apiKey: signer.api_key(),
                signature: signer.signature(&query_string.into_bytes())?,
            }),
            None => None,
        };
        Ok(BinanceWebsocketRequest {
            id,
            params: BinanceWebsocketSignedParams {
                unsigned: params,
                signature,
            },
        })
    }
}

impl BinanceWebsocketRequest {
    #[cfg(feature = "serde")]
    pub fn serialize(&self) -> ETResult<String> {
        serde_json::to_string(self).map_err(ETError::SerializeRequest)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BinanceWebsocketUnsignedRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("BinanceWebsocketUnsignedRequest", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("method", &self.params.method_name())?;
        state.serialize_field("params", &self.params)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl Serialize for BinanceWebsocketRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("BinanceWebsocketRequest", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("method", &self.params.unsigned.method_name())?;
        state.serialize_field("params", &self.params)?;
        state.end()
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_method_derived_from_params() {
        let request = BinanceWebsocketUnsignedRequest {
            id: "1".into(),
            params: BinanceWebsocketUnsignedParams::Time(BinanceTimeParams {}),
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({ "id": "1", "method": "time", "params": {} })
        );
    }

    #[test]
    fn signed_request_serializes_method_derived_from_params() {
        let request = BinanceWebsocketRequest {
            id: "1".into(),
            params: BinanceWebsocketSignedParams {
                unsigned: BinanceWebsocketUnsignedParams::Logon(BinanceLogonParams {
                    timestamp: 123,
                }),
                signature: Some(BinanceSignature {
                    apiKey: "api-key".into(),
                    signature: "signature".into(),
                }),
            },
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "id": "1",
                "method": "session.logon",
                "params": { "apiKey": "api-key", "timestamp": 123, "signature": "signature" },
            })
        );
    }

    #[test]
    fn parses_logon_result_with_api_key() {
        let response = serde_json::from_value::<BinanceWebsocketResponse>(json!({
            "id": "c174a2b1-3f51-4580-b200-8528bd237cb7",
            "status": 200,
            "result": {
                "apiKey": "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
                "authorizedSince": 1649729878532_i64,
                "connectedSince": 1649729873021_i64,
                "returnRateLimits": false,
                "serverTime": 1649729878630_i64,
                "userDataStream": false,
            },
            "rateLimits": [],
        }))
        .unwrap();
        match response.result {
            Some(BinanceWebsocketResponseResult::SessionAuthentication(result)) => {
                assert_eq!(
                    result.apiKey.as_deref(),
                    Some("vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A")
                );
                assert_eq!(result.authorizedSince, Some(1649729878532_i64));
                assert_eq!(result.connectedSince, 1649729873021_i64);
            }
            other => panic!("expected SessionAuthentication, got: {other:?}"),
        }
    }

    #[test]
    fn parses_unauthenticated_status_with_null_api_key() {
        // session.status/session.logout on an unauthenticated connection
        // report apiKey and authorizedSince as JSON null.
        let response = serde_json::from_value::<BinanceWebsocketResponse>(json!({
            "id": "c174a2b1-3f51-4580-b200-8528bd237cb7",
            "status": 200,
            "result": {
                "apiKey": null,
                "authorizedSince": null,
                "connectedSince": 1649729873021_i64,
                "returnRateLimits": false,
                "serverTime": 1649730611671_i64,
                "userDataStream": false,
            },
            "rateLimits": [],
        }))
        .unwrap();
        match response.result {
            Some(BinanceWebsocketResponseResult::SessionAuthentication(result)) => {
                assert_eq!(result.apiKey, None);
                assert_eq!(result.authorizedSince, None);
            }
            other => panic!("expected SessionAuthentication, got: {other:?}"),
        }
    }

    #[test]
    fn parses_cancel_all_with_order_list_reports() {
        // openOrders.cancelAll returns an array of cancellation reports;
        // cancelled order lists appear as order-list-shaped reports.
        let response = serde_json::from_value::<BinanceWebsocketResponse>(json!({
            "id": "778f938f-9041-4b88-9914-efbf64eeacc8",
            "status": 200,
            "result": [
                {
                    "symbol": "BTCUSDT",
                    "origClientOrderId": "4d96324ff9d44481926157",
                    "orderId": 12569099453_i64,
                    "orderListId": -1,
                    "clientOrderId": "91fe37ce9e69c90d6358c0",
                    "transactTime": 1684804350068_i64,
                    "price": "23416.10000000",
                    "origQty": "0.00847000",
                    "executedQty": "0.00001000",
                    "origQuoteOrderQty": "0.000000",
                    "cummulativeQuoteQty": "0.23416100",
                    "status": "CANCELED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "SELL",
                    "selfTradePreventionMode": "NONE",
                },
                {
                    "orderListId": 19431,
                    "contingencyType": "OCO",
                    "listStatusType": "ALL_DONE",
                    "listOrderStatus": "ALL_DONE",
                    "listClientOrderId": "iuVNVJYYrByz6C4yGOPPK0",
                    "transactionTime": 1660803702431_i64,
                    "symbol": "BTCUSDT",
                    "orders": [
                        {
                            "symbol": "BTCUSDT",
                            "orderId": 12569099453_i64,
                            "clientOrderId": "bX5wROblo6YeDwa9iTLeyY",
                        }
                    ],
                },
            ],
            "rateLimits": [],
        }))
        .unwrap();
        match response.result {
            Some(BinanceWebsocketResponseResult::CancelAllOrders(reports)) => {
                assert_eq!(reports.len(), 2);
                assert!(matches!(reports[0], BinanceCancelReport::Order(..)));
                match &reports[1] {
                    BinanceCancelReport::OrderList(list) => {
                        assert_eq!(list.orderListId, 19431);
                        // The list element above carries no orderReports.
                        assert!(list.orderReports.is_empty());
                    }
                    other => panic!("expected OrderList report, got: {other:?}"),
                }
            }
            other => panic!("expected CancelAllOrders, got: {other:?}"),
        }
    }

    #[test]
    fn parses_cancel_of_order_list_member_as_list_report() {
        // Cancelling an order that is part of an order list cancels the whole
        // order list; the order.cancel result is then order-list-shaped.
        let response = serde_json::from_value::<BinanceWebsocketResponse>(json!({
            "id": "16eaf097-bbec-44b9-96ff-e97e6e875870",
            "status": 200,
            "result": {
                "orderListId": 19431,
                "contingencyType": "OCO",
                "listStatusType": "ALL_DONE",
                "listOrderStatus": "ALL_DONE",
                "listClientOrderId": "iuVNVJYYrByz6C4yGOPPK0",
                "transactionTime": 1660803702431_i64,
                "symbol": "BTCUSDT",
                "orders": [
                    {
                        "symbol": "BTCUSDT",
                        "orderId": 12569099453_i64,
                        "clientOrderId": "bX5wROblo6YeDwa9iTLeyY",
                    }
                ],
                "orderReports": [
                    {
                        "symbol": "BTCUSDT",
                        "origClientOrderId": "bX5wROblo6YeDwa9iTLeyY",
                        "orderId": 12569099453_i64,
                        "orderListId": 19431,
                        "clientOrderId": "OFFXQtxVFZ6Nbcg4PgE2DA",
                        "transactTime": 1684804350068_i64,
                        "price": "23450.50000000",
                        "origQty": "0.00850000",
                        "executedQty": "0.00000000",
                        "origQuoteOrderQty": "0.000000",
                        "cummulativeQuoteQty": "0.00000000",
                        "status": "CANCELED",
                        "timeInForce": "GTC",
                        "type": "STOP_LOSS_LIMIT",
                        "side": "BUY",
                        "stopPrice": "23430.00000000",
                        "selfTradePreventionMode": "NONE",
                    }
                ],
            },
            "rateLimits": [],
        }))
        .unwrap();
        match response.result {
            Some(BinanceWebsocketResponseResult::CancelOrderList(result)) => {
                assert_eq!(result.orderListId, 19431);
                assert_eq!(result.orders.len(), 1);
                assert_eq!(result.orderReports.len(), 1);
            }
            other => panic!("expected CancelOrderList, got: {other:?}"),
        }
    }

    #[test]
    fn parses_my_filters_result() {
        let response = serde_json::from_value::<BinanceWebsocketResponse>(json!({
            "id": "c174a2b1-3f51-4580-b200-8528bd237cb7",
            "status": 200,
            "result": {
                "exchangeFilters": [],
                "symbolFilters": [],
                "assetFilters": [
                    {
                        "filterType": "MAX_ASSET",
                        "asset": "USDC",
                        "limit": "42.00000000",
                    }
                ],
            },
            "rateLimits": [],
        }))
        .unwrap();
        match response.result {
            Some(BinanceWebsocketResponseResult::AssetLimits(result)) => {
                assert_eq!(result.assetFilters.len(), 1);
                assert!(result.exchangeFilters.is_empty());
            }
            other => panic!("expected AssetLimits, got: {other:?}"),
        }
    }
}
