use crate::{
    binance::{
        amend::BinanceAmendOrderParams,
        asset_limits::BinanceAssetLimitsParams,
        cancel::{BinanceCancelAllOrdersParams, BinanceCancelOrderParams},
        exchange_info::BinanceExchangeInfoParams,
        session::{BinanceSessionLogonParams, BinanceSessionLogoutParams},
        spot::BinanceSpotOrderParams,
        time::BinanceTimeParams,
    },
    error::{ETError, ETResult},
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimited,
    request::{ETHttpRequest, ETWebsocketId, ETWebsocketRequest},
    signer::Signer,
    urls::Protocol,
};

use {serde::Serialize, serde_with::skip_serializing_none};

#[derive(Serialize)]
#[serde(untagged)]
#[derive(Debug, Clone, Hash)]
pub enum BinanceRequest {
    AmendOrderRequest(BinanceAmendOrderParams),
    AssetLimits(BinanceAssetLimitsParams),
    CancelAllOrdersRequest(BinanceCancelAllOrdersParams),
    CancelOrderRequest(BinanceCancelOrderParams),
    ExchangeInfo(BinanceExchangeInfoParams),
    SpotOrderRequest(BinanceSpotOrderParams),
    Time(BinanceTimeParams),
    WebsocketSessionLogon(BinanceSessionLogonParams),
    WebsocketSessionLogout(BinanceSessionLogoutParams),
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
enum WebsocketMethod {
    #[serde(rename = "order.amend.keepPriority")]
    AmendOrder,
    #[serde(rename = "myFilters")]
    AssetLimits,
    #[serde(rename = "order.cancel")]
    CancelOrder,
    #[serde(rename = "openOrders.cancelAll")]
    CancelAllOrders,
    #[serde(rename = "exchangeInfo")]
    ExchangeInfo,
    #[serde(rename = "session.logon")]
    Logon,
    #[serde(rename = "session.logout")]
    Logout,
    #[serde(rename = "order.place")]
    PlaceOrder,
    #[serde(rename = "time")]
    Time,
}

#[derive(Debug, Clone, Serialize)]
struct WebsocketSignedRequest {
    id: ETWebsocketId,
    method: WebsocketMethod,
    params: SignedParams,
}

#[derive(Debug, Clone, Serialize)]
#[skip_serializing_none]
struct SignedParams {
    #[serde(flatten)]
    params: BinanceRequest,
    signature: Option<String>,
}

impl RateLimited for BinanceRequest {
    fn order_count(&self) -> u32 {
        match self {
            BinanceRequest::SpotOrderRequest(..) => 1,
            _ => 0,
        }
    }
    fn weight(&self) -> u32 {
        match self {
            BinanceRequest::AmendOrderRequest(..) => 4,
            BinanceRequest::AssetLimits(..) => 40,
            BinanceRequest::CancelAllOrdersRequest(..) => 1,
            BinanceRequest::CancelOrderRequest(..) => 1,
            BinanceRequest::ExchangeInfo(..) => 20,
            BinanceRequest::SpotOrderRequest(..) => 1,
            BinanceRequest::Time(..) => 1,
            BinanceRequest::WebsocketSessionLogon(..) => 2,
            BinanceRequest::WebsocketSessionLogout(..) => 2,
        }
    }
}

impl BinanceRequest {
    fn set_api_key(&mut self, api_key: Option<String>) {
        match self {
            BinanceRequest::AmendOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::AssetLimits(params) => params.apiKey = api_key,
            BinanceRequest::CancelAllOrdersRequest(params) => params.apiKey = api_key,
            BinanceRequest::CancelOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::SpotOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::WebsocketSessionLogon(params) => params.apiKey = api_key,
            BinanceRequest::ExchangeInfo(..)
            | BinanceRequest::Time(..)
            | BinanceRequest::WebsocketSessionLogout(..) => {}
        }
    }
    fn query_params(&self) -> String {
        match self {
            BinanceRequest::AmendOrderRequest(params) => params.query_params(true),
            BinanceRequest::AssetLimits(params) => params.query_params(true),
            BinanceRequest::CancelAllOrdersRequest(params) => params.query_params(true),
            BinanceRequest::CancelOrderRequest(params) => params.query_params(true),
            BinanceRequest::ExchangeInfo(params) => params.query_params(),
            BinanceRequest::SpotOrderRequest(params) => params.query_params(true),
            BinanceRequest::Time(params) => params.query_params(),
            BinanceRequest::WebsocketSessionLogon(params) => params.query_params(true),
            BinanceRequest::WebsocketSessionLogout(params) => params.query_params(true),
        }
    }
    fn websocket_method(&self) -> WebsocketMethod {
        match self {
            BinanceRequest::AmendOrderRequest(..) => WebsocketMethod::AmendOrder,
            BinanceRequest::AssetLimits(..) => WebsocketMethod::AssetLimits,
            BinanceRequest::CancelAllOrdersRequest(..) => WebsocketMethod::CancelAllOrders,
            BinanceRequest::CancelOrderRequest(..) => WebsocketMethod::CancelOrder,
            BinanceRequest::ExchangeInfo(..) => WebsocketMethod::ExchangeInfo,
            BinanceRequest::SpotOrderRequest(..) => WebsocketMethod::PlaceOrder,
            BinanceRequest::Time(..) => WebsocketMethod::Time,
            BinanceRequest::WebsocketSessionLogon(..) => WebsocketMethod::Logon,
            BinanceRequest::WebsocketSessionLogout(..) => WebsocketMethod::Logout,
        }
    }
}

impl ETHttpRequest for BinanceRequest {
    fn try_into_http(mut self, signer: &Signer) -> ETResult<HttpRequest> {
        self.set_api_key(None);
        let (method, endpoint, is_signed) = match self {
            BinanceRequest::AmendOrderRequest(..) => {
                (HttpMethod::PUT, "order/amend/keepPriority", true)
            }
            BinanceRequest::AssetLimits(..) => (HttpMethod::GET, "myFilters", true),
            BinanceRequest::CancelAllOrdersRequest(..) => (HttpMethod::DELETE, "openOrders", true),
            BinanceRequest::CancelOrderRequest(..) => (HttpMethod::DELETE, "order", true),
            BinanceRequest::ExchangeInfo(..) => (HttpMethod::GET, "exchangeInfo", false),
            BinanceRequest::SpotOrderRequest(..) => (HttpMethod::POST, "order", true),
            BinanceRequest::Time(..) => (HttpMethod::GET, "time", false),
            BinanceRequest::WebsocketSessionLogon(..)
            | BinanceRequest::WebsocketSessionLogout(..) => {
                let websocket_method = self.websocket_method();
                let request_type =
                    serde_json::to_string(&websocket_method).map_err(ETError::SerializeRequest)?;
                return Err(ETError::BadProtocol {
                    request_type,
                    protocol: Protocol::Http,
                });
            }
        };
        let query_params = self.query_params();
        let (query_params, headers) = if is_signed {
            let signature = signer.signature(query_params.as_bytes())?;
            (
                format!("{}&signature={}", query_params, signature),
                vec![("X-MBX-APIKEY".into(), signer.api_key().clone())],
            )
        } else {
            (query_params, vec![])
        };
        let query = Some(format!("{}?{}", endpoint, query_params));
        let body = None;
        Ok(HttpRequest {
            method,
            query,
            headers,
            body,
        })
    }
}

impl ETWebsocketRequest for BinanceRequest {
    fn try_into_websocket(mut self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        self.set_api_key(Some(signer.api_key()));
        let method = self.websocket_method();
        let is_signed = match self {
            BinanceRequest::AmendOrderRequest(..)
            | BinanceRequest::AssetLimits(..)
            | BinanceRequest::CancelAllOrdersRequest(..)
            | BinanceRequest::CancelOrderRequest(..)
            | BinanceRequest::SpotOrderRequest(..)
            | BinanceRequest::WebsocketSessionLogon(..) => true,
            BinanceRequest::ExchangeInfo(..)
            | BinanceRequest::Time(..)
            | BinanceRequest::WebsocketSessionLogout(..) => false,
        };
        let signature = if is_signed {
            Some(signer.signature(self.query_params().as_bytes())?)
        } else {
            None
        };
        let signed_request = WebsocketSignedRequest {
            id,
            method,
            params: SignedParams {
                params: self,
                signature,
            },
        };
        let message = serde_json::to_string(&signed_request).map_err(ETError::SerializeRequest)?;
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api_key_credential::ApiKeyCredentials, encode::ByteEncoder, encrypt::EncryptionAlgorithm,
        signer::Signer,
    };
    use secrecy::SecretString;

    fn signer() -> Signer {
        Signer::new(
            "api-key".into(),
            EncryptionAlgorithm::HmacSha256
                .encryptor(ApiKeyCredentials {
                    api_key: "api-key".into(),
                    secret: SecretString::from("secret".to_string()),
                })
                .unwrap(),
            ByteEncoder::Base64,
        )
    }

    fn envelope(id: ETWebsocketId) -> serde_json::Value {
        let message = BinanceRequest::WebsocketSessionLogon(BinanceSessionLogonParams {
            apiKey: None,
            timestamp: 1_700_000_000_000,
        })
        .try_into_websocket(&signer(), id)
        .unwrap();
        serde_json::from_str(&message).unwrap()
    }

    #[test]
    fn serializes_integer_id() {
        let envelope = envelope(ETWebsocketId::Int(1));
        assert_eq!(envelope["id"].as_i64(), Some(1));
        assert_eq!(envelope["method"], "session.logon");
    }

    #[test]
    fn serializes_string_id() {
        let envelope = envelope(ETWebsocketId::Str("abc".into()));
        assert_eq!(envelope["id"].as_str(), Some("abc"));
        assert_eq!(envelope["method"], "session.logon");
    }
}
