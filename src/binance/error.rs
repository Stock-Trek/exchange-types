use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceError {
    pub code: i64,
    pub msg: String,
    pub data: Option<BinanceErrorData>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceErrorData {
    pub serverTime: Option<i64>,
    pub retryAfter: Option<i64>,
}

impl std::fmt::Display for BinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.msg)
    }
}

impl std::error::Error for BinanceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_without_data_deserializes() {
        let error: BinanceError =
            serde_json::from_str(r#"{"code":-2010,"msg":"Order would immediately trigger."}"#)
                .unwrap();
        assert_eq!(error.code, -2010);
        assert_eq!(error.msg, "Order would immediately trigger.");
        assert!(error.data.is_none());
    }

    #[test]
    fn error_data_deserializes_as_typed_payload() {
        let error: BinanceError = serde_json::from_str(
            r#"{"code":-1003,"msg":"Too many requests","data":{"serverTime":1720812284068,"retryAfter":45}}"#,
        )
        .unwrap();
        assert_eq!(error.code, -1003);
        let data = error.data.expect("error data should be present");
        assert_eq!(data.serverTime, Some(1720812284068));
        assert_eq!(data.retryAfter, Some(45));
    }
}
