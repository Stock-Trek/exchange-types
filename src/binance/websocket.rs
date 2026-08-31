use crate::binance::{
    amend::{BinanceAmendOrderParams, BinanceAmendOrderResult},
    asset_limits::BinanceAssetLimitsParams,
    cancel::{BinanceCancelAllOrdersParams, BinanceCancelOrderParams, BinanceCancelOrderResult},
    error::BinanceError,
    exchange_info::{BinanceExchangeInfoParams, BinanceExchangeInfoResult},
    logon::{BinanceLogonParams, BinanceSessionAuthenticationResult},
    rate_limits::BinanceRateLimit,
    signed::BinanceSignedParams,
    spot::{BinanceSpotOrderParams, BinanceSpotOrderResult},
    time::{BinanceTimeParams, BinanceTimeResult},
};
#[cfg(feature = "serde")]
use serde::de::Error as DeError;
#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceWebsocketBody {
    Request(BinanceWebsocketRequest),
    Response(BinanceWebsocketResponse),
}

/// The websocket method name for a request. Each variant corresponds to exactly
/// one `BinanceWebsocketUnsignedParams` variant, so the method is derived from
/// the params (via `BinanceWebsocketUnsignedParams::method_name`) instead of
/// being stored independently on the request. This ensures the serialized
/// method always matches the serialized parameters.
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

/// A signed websocket request. The `method` field is not stored on the request;
/// it is derived from the params on serialization and validated on
/// deserialization, so the method name always corresponds with the parameters.
#[derive(Debug, Clone)]
pub struct BinanceWebsocketRequest {
    pub id: String,
    pub params: BinanceSignedParams<BinanceWebsocketUnsignedParams>,
}

/// An unsigned websocket request. The `method` field is not stored on the
/// request; it is derived from the params on serialization and validated on
/// deserialization, so the method name always corresponds with the parameters.
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceWebsocketResponse {
    pub error: Option<BinanceError>,
    pub id: String,
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

#[cfg(feature = "serde")]
impl Serialize for BinanceWebsocketRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("BinanceWebsocketRequest", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("method", &self.params.params.method_name())?;
        state.serialize_field("params", &self.params.params)?;
        if let Some(signature) = &self.params.signature {
            state.serialize_field("signature", signature)?;
        }
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BinanceWebsocketRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct BinanceWebsocketRequestFields {
            id: String,
            method: BinanceWebsocketMethodName,
            params: BinanceWebsocketUnsignedParams,
            #[serde(default)]
            signature: Option<String>,
        }
        let fields = BinanceWebsocketRequestFields::deserialize(deserializer)?;
        let method = fields.params.method_name();
        if fields.method != method {
            return Err(DeError::custom(format!(
                "websocket request method {:?} does not correspond with the params method {:?}",
                fields.method, method
            )));
        }
        Ok(BinanceWebsocketRequest {
            id: fields.id,
            params: BinanceSignedParams {
                params: fields.params,
                signature: fields.signature,
            },
        })
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
impl<'de> Deserialize<'de> for BinanceWebsocketUnsignedRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct BinanceWebsocketUnsignedRequestFields {
            id: String,
            method: BinanceWebsocketMethodName,
            params: BinanceWebsocketUnsignedParams,
        }
        let fields = BinanceWebsocketUnsignedRequestFields::deserialize(deserializer)?;
        let method = fields.params.method_name();
        if fields.method != method {
            return Err(DeError::custom(format!(
                "websocket request method {:?} does not correspond with the params method {:?}",
                fields.method, method
            )));
        }
        Ok(BinanceWebsocketUnsignedRequest {
            id: fields.id,
            params: fields.params,
        })
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
            params: BinanceSignedParams {
                params: BinanceWebsocketUnsignedParams::Logon(BinanceLogonParams {
                    apiKey: "api-key".into(),
                    timestamp: 123,
                }),
                signature: Some("signature".into()),
            },
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "id": "1",
                "method": "session.logon",
                "params": { "apiKey": "api-key", "timestamp": 123 },
                "signature": "signature"
            })
        );
    }

    #[test]
    fn rejects_method_that_does_not_correspond_with_params() {
        let result = serde_json::from_value::<BinanceWebsocketUnsignedRequest>(json!({
            "id": "1",
            "method": "time",
            "params": { "symbol": "BTCUSDT", "timestamp": 123 }
        }));
        assert!(result.is_err());
    }

    #[test]
    fn round_trips_request() {
        let request = BinanceWebsocketUnsignedRequest {
            id: "1".into(),
            params: BinanceWebsocketUnsignedParams::Time(BinanceTimeParams {}),
        };
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: BinanceWebsocketUnsignedRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, request.id);
    }
}
