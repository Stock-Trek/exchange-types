use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceError {
    pub code: i64,
    pub data: Option<serde_json::Value>,
    pub msg: String,
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
    fn deserializes_error_without_data() {
        let error: BinanceError =
            serde_json::from_str(r#"{"code": -1121, "msg": "Invalid symbol."}"#).unwrap();
        assert_eq!(error.code, -1121);
        assert_eq!(error.msg, "Invalid symbol.");
        assert!(error.data.is_none());
    }

    #[test]
    fn deserializes_optional_error_data() {
        let error: BinanceError = serde_json::from_str(
            r#"{
                "code": -1003,
                "msg": "Way too much request weight used; IP banned until 1659146400000.",
                "data": {
                    "serverTime": 1659142907531,
                    "retryAfter": 1659146400000
                }
            }"#,
        )
        .unwrap();
        assert_eq!(error.code, -1003);
        let data = error.data.unwrap();
        assert_eq!(data["retryAfter"], serde_json::json!(1_659_146_400_000i64));
    }
}
