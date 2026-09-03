pub trait IntoHttpRequest {
    fn into_http_request(self) -> HttpRequest;
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub query: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    DELETE,
    PATCH,
    POST,
    PUT,
}
