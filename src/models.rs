use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Baseline {
    pub status: u16,
    pub content_length: usize,
}

#[derive(Debug)]
pub struct ResponseInfo {
    pub path: String,
    pub status: u16,
    pub content_length: usize,
    pub latency_ms: u128,
	pub headers: hyper::HeaderMap,
	pub body_sample: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Finding {
    pub path: String,
    pub status: u16,
    pub content_length: usize,
}
