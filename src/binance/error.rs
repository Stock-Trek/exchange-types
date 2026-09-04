use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceError {
    pub code: i64,
    pub msg: String,
}

impl std::fmt::Display for BinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.msg)
    }
}

impl std::error::Error for BinanceError {}
