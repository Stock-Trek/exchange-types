use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BinanceError {
    pub code: String,
    pub msg: String,
}
