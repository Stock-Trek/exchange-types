use crate::{
    binance::{
        amend::{BinanceAmendOrderParams, BinanceAmendOrderResult},
        asset_limits::BinanceAssetLimitsParams,
        cancel::{
            BinanceCancelAllOrdersParams, BinanceCancelOrderParams, BinanceCancelOrderResult,
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
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct BinanceWebsocketRequest {
    pub id: String,
    pub params: BinanceWebsocketSignedParams,
}

#[cfg_attr(feature = "serde", skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[cfg_attr(feature = "serde", serde(default))]
    pub rateLimits: Vec<BinanceRateLimit>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub result: Option<BinanceWebsocketResponseResult>,
    pub status: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceWebsocketResponseResult {
    AmendOrder(BinanceAmendOrderResult),
    CancelAllOrders(Vec<BinanceCancelOrderResult>),
    CancelOrder(BinanceCancelOrderResult),
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
    fn deserializes_response_without_rate_limits() {
        // Some methods (e.g. session.logout, error responses) omit rateLimits.
        let response: BinanceWebsocketResponse = serde_json::from_str(
            r#"{"id":"1","status":200,"result":{"serverTime":1700000000000}}"#,
        )
        .unwrap();
        assert!(response.rateLimits.is_empty());
        assert!(response.error.is_none());
    }

    #[test]
    fn response_tolerates_unknown_fields() {
        // Binance adds fields over time; unknown fields must not fail parsing.
        let response: BinanceWebsocketResponse = serde_json::from_str(
            r#"{"id":"1","status":200,"rateLimits":[],"futureField":true,"result":{"serverTime":1700000000000,"alsoFuture":1}}"#,
        )
        .unwrap();
        assert!(response.rateLimits.is_empty());
        assert!(response.error.is_none());
    }

    #[test]
    fn error_response_tolerates_unknown_fields() {
        let response: BinanceWebsocketResponse = serde_json::from_str(
            r#"{"id":"1","status":400,"error":{"code":-2014,"msg":"API-key format invalid.","extra":true}}"#,
        )
        .unwrap();
        let error = response.error.expect("expected an error");
        assert_eq!(error.code, -2014);
        assert_eq!(error.msg, "API-key format invalid.");
    }

    #[test]
    fn unknown_method_name_deserializes_as_unknown() {
        let method: BinanceWebsocketMethodName =
            serde_json::from_str(r#""future.method""#).unwrap();
        assert!(matches!(method, BinanceWebsocketMethodName::Unknown));
    }
}
