use crate::response::ResponseFor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Default, Hash)]
pub struct BinanceTimeRequest {}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceTimeResponse {
    pub serverTime: i64,
}

impl ResponseFor for BinanceTimeRequest {
    type Response = BinanceTimeResponse;
}

impl BinanceTimeRequest {
    pub fn query_params(&self) -> String {
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        binance::{
            rate_limits::BinanceRateLimitInterval,
            response::{BinanceResponse, BinanceResponsePayload, BinanceUsageInterval},
        },
        http::HttpResponse,
        response::{ETHttpResponse, ETWebsocketResponse},
        websocket_id::ETWebsocketId,
    };

    #[test]
    fn deserializes_real_binance_us_http_time_response() {
        // Captured live from `GET https://api.binance.us/api/v3/time` on 2026-09-04.
        // Binance.US returns the standard x-mbx-* usage headers parsed by the crate.
        let response = BinanceResponse::<BinanceTimeResponse>::try_from_http(HttpResponse {
            status: 200,
            headers: vec![
                (
                    "content-type".into(),
                    "application/json;charset=UTF-8".into(),
                ),
                (
                    "x-mbx-uuid".into(),
                    "4c513f42-cc01-4c6f-8c61-af153f37d652".into(),
                ),
                ("x-mbx-used-weight".into(), "21".into()),
                ("x-mbx-used-weight-1m".into(), "21".into()),
            ],
            body: br#"{"serverTime":1788540215540}"#.to_vec(),
        })
        .unwrap();

        assert_eq!(response.metadata.status, 200);
        assert_eq!(
            response
                .metadata
                .usage
                .used_weight
                .get(&BinanceUsageInterval {
                    interval: BinanceRateLimitInterval::MINUTE,
                    interval_num: 1,
                }),
            Some(&21)
        );
        assert!(response.metadata.usage.order_count.is_empty());
        match response.payload {
            BinanceResponsePayload::Success(result) => {
                assert_eq!(result.serverTime, 1788540215540);
            }
            BinanceResponsePayload::Failure(error) => panic!("unexpected failure: {error:?}"),
        }
    }

    #[test]
    fn deserializes_real_binance_us_websocket_time_response() {
        // Captured live from `wss://ws-api.binance.us:443/ws-api/v3` (method `time`)
        // on 2026-09-04. Binance.US serves the same WebSocket API schema as Binance.
        let frame = r#"{"id":1,"status":200,"result":{"serverTime":1788540234915},"rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":6000,"count":25}]}"#;
        let response =
            BinanceResponse::<BinanceTimeResponse>::try_from_websocket(frame.into()).unwrap();

        assert_eq!(response.metadata.websocket_id, Some(ETWebsocketId::Int(1)));
        assert_eq!(response.metadata.status, 200);
        assert_eq!(
            response
                .metadata
                .usage
                .used_weight
                .get(&BinanceUsageInterval {
                    interval: BinanceRateLimitInterval::MINUTE,
                    interval_num: 1,
                }),
            Some(&25)
        );
        match response.payload {
            BinanceResponsePayload::Success(result) => {
                assert_eq!(result.serverTime, 1788540234915);
            }
            BinanceResponsePayload::Failure(error) => panic!("unexpected failure: {error:?}"),
        }
    }
}
