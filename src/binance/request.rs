use crate::{
    binance::{
        account::BinanceAccountRequest,
        amend::BinanceAmendOrderRequest,
        asset_limits::BinanceAssetLimitsRequest,
        cancel::{BinanceCancelAllOrdersRequest, BinanceCancelOrderRequest},
        exchange_info::BinanceExchangeInfoRequest,
        open_orders::BinanceOpenOrdersRequest,
        query_order::BinanceQueryOrderRequest,
        session::{BinanceSessionLogonRequest, BinanceSessionLogoutRequest},
        spot::BinanceSpotOrderRequest,
        time::BinanceTimeRequest,
    },
    error::{ETError, ETResult},
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimited,
    request::{ETHttpRequest, ETWebsocketRequest},
    signer::Signer,
    urls::Protocol,
    websocket_id::ETWebsocketId,
};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Serialize)]
#[serde(untagged)]
#[derive(Debug, Clone, Hash)]
pub enum BinanceRequest {
    Account(BinanceAccountRequest),
    AmendOrderRequest(BinanceAmendOrderRequest),
    AssetLimits(BinanceAssetLimitsRequest),
    CancelAllOrdersRequest(BinanceCancelAllOrdersRequest),
    CancelOrderRequest(BinanceCancelOrderRequest),
    ExchangeInfo(BinanceExchangeInfoRequest),
    OpenOrders(BinanceOpenOrdersRequest),
    QueryOrder(BinanceQueryOrderRequest),
    SpotOrderRequest(BinanceSpotOrderRequest),
    Time(BinanceTimeRequest),
    WebsocketSessionLogon(BinanceSessionLogonRequest),
    WebsocketSessionLogout(BinanceSessionLogoutRequest),
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
enum WebsocketMethod {
    #[serde(rename = "order.amend.keepPriority")]
    AmendOrder,
    #[serde(rename = "myFilters")]
    AssetLimits,
    #[serde(rename = "account.status")]
    Account,
    #[serde(rename = "order.cancel")]
    CancelOrder,
    #[serde(rename = "openOrders.cancelAll")]
    CancelAllOrders,
    #[serde(rename = "exchangeInfo")]
    ExchangeInfo,
    #[serde(rename = "openOrders.status")]
    OpenOrders,
    #[serde(rename = "order.status")]
    QueryOrder,
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
struct WebsocketRequest {
    id: ETWebsocketId,
    method: WebsocketMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<WebsocketParams>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
struct WebsocketParams {
    #[serde(flatten)]
    request: BinanceRequest,
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
            BinanceRequest::Account(..) => 20,
            BinanceRequest::AmendOrderRequest(..) => 4,
            BinanceRequest::AssetLimits(..) => 40,
            BinanceRequest::CancelAllOrdersRequest(..) => 1,
            BinanceRequest::CancelOrderRequest(..) => 1,
            BinanceRequest::ExchangeInfo(..) => 20,
            BinanceRequest::OpenOrders(params) => {
                if params.symbol.is_some() {
                    6
                } else {
                    80
                }
            }
            BinanceRequest::QueryOrder(..) => 4,
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
            BinanceRequest::Account(params) => params.apiKey = api_key,
            BinanceRequest::AmendOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::AssetLimits(params) => params.apiKey = api_key,
            BinanceRequest::CancelAllOrdersRequest(params) => params.apiKey = api_key,
            BinanceRequest::CancelOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::OpenOrders(params) => params.apiKey = api_key,
            BinanceRequest::QueryOrder(params) => params.apiKey = api_key,
            BinanceRequest::SpotOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::WebsocketSessionLogon(params) => params.apiKey = api_key,
            BinanceRequest::ExchangeInfo(..)
            | BinanceRequest::Time(..)
            | BinanceRequest::WebsocketSessionLogout(..) => {}
        }
    }
    fn query_params(&self, percent_encode: bool) -> String {
        match self {
            BinanceRequest::Account(params) => params.query_params(true, percent_encode),
            BinanceRequest::AmendOrderRequest(params) => params.query_params(true, percent_encode),
            BinanceRequest::AssetLimits(params) => params.query_params(true, percent_encode),
            BinanceRequest::CancelAllOrdersRequest(params) => {
                params.query_params(true, percent_encode)
            }
            BinanceRequest::CancelOrderRequest(params) => params.query_params(true, percent_encode),
            BinanceRequest::ExchangeInfo(params) => params.query_params(),
            BinanceRequest::OpenOrders(params) => params.query_params(true, percent_encode),
            BinanceRequest::QueryOrder(params) => params.query_params(true, percent_encode),
            BinanceRequest::SpotOrderRequest(params) => params.query_params(true, percent_encode),
            BinanceRequest::Time(params) => params.query_params(),
            BinanceRequest::WebsocketSessionLogon(params) => {
                params.query_params(true, percent_encode)
            }
            BinanceRequest::WebsocketSessionLogout(params) => {
                params.query_params(true, percent_encode)
            }
        }
    }
    fn websocket_method(&self) -> WebsocketMethod {
        match self {
            BinanceRequest::Account(..) => WebsocketMethod::Account,
            BinanceRequest::AmendOrderRequest(..) => WebsocketMethod::AmendOrder,
            BinanceRequest::AssetLimits(..) => WebsocketMethod::AssetLimits,
            BinanceRequest::CancelAllOrdersRequest(..) => WebsocketMethod::CancelAllOrders,
            BinanceRequest::CancelOrderRequest(..) => WebsocketMethod::CancelOrder,
            BinanceRequest::ExchangeInfo(..) => WebsocketMethod::ExchangeInfo,
            BinanceRequest::OpenOrders(..) => WebsocketMethod::OpenOrders,
            BinanceRequest::QueryOrder(..) => WebsocketMethod::QueryOrder,
            BinanceRequest::SpotOrderRequest(..) => WebsocketMethod::PlaceOrder,
            BinanceRequest::Time(..) => WebsocketMethod::Time,
            BinanceRequest::WebsocketSessionLogon(..) => WebsocketMethod::Logon,
            BinanceRequest::WebsocketSessionLogout(..) => WebsocketMethod::Logout,
        }
    }
}

fn percent_encode(value: &str) -> String {
    const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(char::from(byte))
            }
            byte => {
                encoded.push('%');
                encoded.push(char::from(HEX_DIGITS[(byte >> 4) as usize]));
                encoded.push(char::from(HEX_DIGITS[(byte & 0x0f) as usize]));
            }
        }
    }
    encoded
}

impl ETHttpRequest for BinanceRequest {
    fn try_into_http(mut self, signer: &Signer) -> ETResult<HttpRequest> {
        self.set_api_key(None);
        let (method, endpoint, is_signed) = match self {
            BinanceRequest::Account(..) => (HttpMethod::GET, "account", true),
            BinanceRequest::AmendOrderRequest(..) => {
                (HttpMethod::PUT, "order/amend/keepPriority", true)
            }
            BinanceRequest::AssetLimits(..) => (HttpMethod::GET, "myFilters", true),
            BinanceRequest::CancelAllOrdersRequest(..) => (HttpMethod::DELETE, "openOrders", true),
            BinanceRequest::CancelOrderRequest(..) => (HttpMethod::DELETE, "order", true),
            BinanceRequest::ExchangeInfo(..) => (HttpMethod::GET, "exchangeInfo", false),
            BinanceRequest::OpenOrders(..) => (HttpMethod::GET, "openOrders", true),
            BinanceRequest::QueryOrder(..) => (HttpMethod::GET, "order", true),
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
        let query_params = self.query_params(true);
        let (query_params, headers) = if is_signed {
            let signature = signer.signature(query_params.as_bytes())?;
            (
                format!("{}&signature={}", query_params, percent_encode(&signature)),
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
        let params = match self {
            BinanceRequest::Time(..) | BinanceRequest::WebsocketSessionLogout(..) => None,
            BinanceRequest::ExchangeInfo(..) => Some(WebsocketParams {
                request: self,
                signature: None,
            }),
            request => {
                let signature = Some(signer.signature(request.query_params(false).as_bytes())?);
                Some(WebsocketParams { request, signature })
            }
        };
        let websocket_request = WebsocketRequest { id, method, params };
        let message =
            serde_json::to_string(&websocket_request).map_err(ETError::SerializeRequest)?;
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api_key_credential::ApiKeyCredentials, binance::account::BinanceAccountRequest,
        encode::ByteEncoder, encrypt::EncryptionAlgorithm, request::ETHttpRequest,
    };
    use secrecy::SecretString;

    fn hmac_signer(api_key: &str, secret: &str, encoder: ByteEncoder) -> Signer {
        let encryptor = EncryptionAlgorithm::HmacSha256
            .encryptor(ApiKeyCredentials {
                api_key: api_key.to_string(),
                secret: SecretString::from(secret.to_string()),
            })
            .expect("HMAC-SHA256 key should be accepted");
        Signer::new(api_key.to_string(), encryptor, encoder)
    }

    fn account_request() -> BinanceRequest {
        BinanceRequest::Account(BinanceAccountRequest {
            apiKey: None,
            omitZeroBalances: None,
            recvWindow: None,
            timestamp: 1_499_827_319_559,
        })
    }

    fn signature_value(http_query: &str) -> &str {
        http_query
            .split("&signature=")
            .nth(1)
            .expect("query should contain a signature parameter")
    }

    fn percent_decode(encoded: &str) -> String {
        let mut decoded = Vec::with_capacity(encoded.len());
        let bytes = encoded.as_bytes();
        let mut index = 0;
        while index < bytes.len() {
            match bytes[index] {
                b'%' => {
                    let hex = std::str::from_utf8(&bytes[index + 1..index + 3])
                        .expect("valid percent escape");
                    decoded.push(u8::from_str_radix(hex, 16).expect("valid hex digit"));
                    index += 3;
                }
                byte => {
                    decoded.push(byte);
                    index += 1;
                }
            }
        }
        String::from_utf8(decoded).expect("valid UTF-8")
    }

    #[test]
    fn http_signature_is_percent_encoded_when_base64() {
        let signer = hmac_signer("api-key", "super-secret", ByteEncoder::Base64);
        let request = account_request();
        let query_params = request.query_params(true);
        let raw_signature = signer
            .signature(query_params.as_bytes())
            .expect("signing should succeed");
        // A 32-byte HMAC-SHA256 always base64-pads with '='.
        assert!(raw_signature.contains('='), "raw base64: {raw_signature}");

        let http = request.try_into_http(&signer).expect("http request");
        let query = http.query.expect("query string");
        let value = signature_value(&query);
        assert_eq!(value, percent_encode(&raw_signature));
        // No reserved characters may survive raw in the query string.
        assert!(!value.contains('+') && !value.contains('/') && !value.contains('='));
        // Percent-decoding must round-trip to the raw signature.
        assert_eq!(percent_decode(value), raw_signature);
    }

    #[test]
    fn http_signature_is_unchanged_when_hex() {
        let signer = hmac_signer("api-key", "super-secret", ByteEncoder::HexLower);
        let request = account_request();
        let query_params = request.query_params(true);
        let raw_signature = signer
            .signature(query_params.as_bytes())
            .expect("signing should succeed");
        assert!(!raw_signature.contains('%'));

        let http = request.try_into_http(&signer).expect("http request");
        let query = http.query.expect("query string");
        let value = signature_value(&query);
        assert_eq!(value, raw_signature);
        assert_eq!(value, percent_encode(&raw_signature));
    }
}
