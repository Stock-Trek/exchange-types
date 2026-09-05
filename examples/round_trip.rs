//! A round trip from request to response.
//!
//! exchange-types only defines the request/response types and their
//! conversions, so this example stands in for the network with an in-memory
//! transport. Point the transport at your HTTP/WebSocket client to send real
//! requests: `Rq::try_into_http` produces the `HttpRequest` to send and
//! `<Rq as ResponseFor>::Response::try_from_http` parses the reply. Because
//! every request type implements `ResponseFor`, the response type is inferred
//! from the request and never needs to be named.
//!
//! ```sh
//! cargo run --example round_trip
//! ```

use exchange_types::{
    api_key_credential::ApiKeyCredentials,
    binance::{
        account::BinanceAccountRequest,
        response::BinanceResponsePayload,
        signer::SignerFactory,
        spot::BinanceSpotOrderRequest,
        supporting_types::{BinanceOrderType, BinanceSide},
        time::BinanceTimeRequest,
    },
    error::ETError,
    http::{HttpRequest, HttpResponse},
    request::{ETHttpRequest, ETWebsocketRequest},
    response::{ETHttpResponse, ETWebsocketResponse, ResponseFor},
    signer::Signer,
    websocket_id::ETWebsocketId,
};
use secrecy::SecretString;

fn http_round_trip<Rq, T>(
    request: Rq,
    signer: &Signer,
    transport: T,
) -> Result<<Rq as ResponseFor>::Response, ETError>
where
    Rq: ETHttpRequest + ResponseFor,
    <Rq as ResponseFor>::Response: ETHttpResponse,
    T: FnOnce(HttpRequest) -> Result<HttpResponse, ETError>,
{
    let http_request = request.try_into_http(signer)?;
    let http_response = transport(http_request)?;
    <Rq as ResponseFor>::Response::try_from_http(http_response)
}

fn websocket_round_trip<Rq, T>(
    request: Rq,
    signer: &Signer,
    id: ETWebsocketId,
    transport: T,
) -> Result<<Rq as ResponseFor>::Response, ETError>
where
    Rq: ETWebsocketRequest + ResponseFor,
    <Rq as ResponseFor>::Response: ETWebsocketResponse,
    T: FnOnce(String) -> Result<String, ETError>,
{
    let frame = request.try_into_websocket(signer, id)?;
    let reply = transport(frame)?;
    <Rq as ResponseFor>::Response::try_from_websocket(reply)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = SignerFactory::hmac_sha256(ApiKeyCredentials {
        api_key: "your-api-key".to_string(),
        secret: SecretString::from("your-api-secret"),
    })?;

    // An unsigned request: GET /api/v3/time
    let response = http_round_trip(BinanceTimeRequest::default(), &signer, |http_request| {
        println!("-> {}", http_request.query.unwrap_or_default());
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: br#"{"serverTime":1700000000000}"#.to_vec(),
        })
    })?;
    match response.payload {
        BinanceResponsePayload::Success(time) => println!("<- serverTime: {}", time.serverTime),
        BinanceResponsePayload::Failure(error) => println!("<- error: {error}"),
    }

    // A signed request: POST /api/v3/order
    let order = BinanceSpotOrderRequest {
        side: BinanceSide::BUY,
        symbol: "BTCUSDT".to_string(),
        r#type: BinanceOrderType::MARKET,
        quantity: Some("0.001".parse()?),
        timestamp: 1_700_000_000_000,
        ..Default::default()
    };
    let response = http_round_trip(order, &signer, |http_request| {
        println!(
            "-> {:?} {}",
            http_request.method,
            http_request.query.unwrap_or_default()
        );
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: br#"{"symbol":"BTCUSDT","orderId":12569099453,"orderListId":-1,"clientOrderId":"my-order-1","transactTime":1700000000000,"price":"0.00000000","origQty":"0.00100000","executedQty":"0.00100000","cummulativeQuoteQty":"43.80000000","status":"FILLED","timeInForce":"GTC","type":"MARKET","side":"BUY"}"#
                .to_vec(),
        })
    })?;
    match response.payload {
        BinanceResponsePayload::Success(order) => {
            println!(
                "<- filled order {} (orderId {})",
                order.symbol, order.orderId
            )
        }
        BinanceResponsePayload::Failure(error) => println!("<- error: {error}"),
    }

    // Exchange errors and rate limiting surface as a typed Failure payload
    // rather than a transport error, so metadata survives.
    let response = http_round_trip(
        BinanceAccountRequest {
            timestamp: 1_700_000_000_000,
            ..Default::default()
        },
        &signer,
        |http_request| {
            println!(
                "-> {:?} {}",
                http_request.method,
                http_request.query.unwrap_or_default()
            );
            Ok(HttpResponse {
                status: 400,
                headers: vec![("X-MBX-USED-WEIGHT-1M".to_string(), "42".to_string())],
                body: br#"{"code":-2014,"msg":"API-key format invalid."}"#.to_vec(),
            })
        },
    )?;
    match response.payload {
        BinanceResponsePayload::Success(account) => println!("<- uid: {}", account.uid),
        BinanceResponsePayload::Failure(error) => {
            println!(
                "<- status {} error [{:?}] {}",
                response.metadata.status, error.code, error.msg
            )
        }
    }

    // The same round trip over a WebSocket: the request becomes a frame and
    // the reply frame is parsed back into the same response type.
    let response =
        websocket_round_trip(BinanceTimeRequest::default(), &signer, 1.into(), |frame| {
            println!("-> {frame}");
            Ok(r#"{"id":1,"status":200,"result":{"serverTime":1700000000000}}"#.to_string())
        })?;
    match response.payload {
        BinanceResponsePayload::Success(time) => {
            println!("<- serverTime: {}", time.serverTime)
        }
        BinanceResponsePayload::Failure(error) => println!("<- error: {error}"),
    }
    Ok(())
}
