use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Baseline {
    pub status: u16,
    pub content_length: usize,
    pub body_hash: String,
}

#[derive(Debug)]
pub struct ResponseInfo {
    pub path: String,
    pub status: u16,
    pub content_length: usize,
    pub body_hash: String,
    pub latency_ms: u128,
}

#[derive(Debug, Serialize)]
pub struct Finding {
    pub path: String,
    pub status: u16,
    pub content_length: usize,
}
