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
        spot::{BinanceSpotOrderParams, BinanceSpotOrderResult},
        time::{BinanceTimeParams, BinanceTimeResult},
    },
    error::ETResult,
    signer::Signer,
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
    pub signature: Option<BinanceWebsocketSignature>,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Debug, Clone)]
pub struct BinanceWebsocketSignature {
    pub apiKey: String,
    pub signature: String,
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
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
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

impl BinanceWebsocketUnsignedRequest {
    pub fn into_signed(self, signer: &Signer) -> ETResult<BinanceWebsocketRequest> {
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
            Some(query_string) => Some(BinanceWebsocketSignature {
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
                signature: Some(BinanceWebsocketSignature {
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
}
