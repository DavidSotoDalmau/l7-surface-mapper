use hyper::{Client, Request, Body, Uri};
use std::time::Instant;
use crate::models::ResponseInfo;

pub async fn fetch(
    client: &Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    base_url: &str,
    path: &str,
) -> Result<Option<ResponseInfo>, hyper::Error> {

    let clean_path = path.trim();

    if !clean_path.is_ascii() {
        return Ok(None);
    }

    let url = format!("{}/{}", base_url.trim_end_matches('/'), clean_path);

    let uri: Uri = match url.parse() {
        Ok(u) => u,
        Err(_) => return Ok(None),
    };

    let req = match Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
    {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };

    let start = Instant::now();
    let resp = client.request(req).await?;

    let latency = start.elapsed().as_millis();
    let status = resp.status().as_u16();

    // 🔥 SOLO leer Content-Length header
    let content_length = resp
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    // ⚠️ No consumimos body
    // Simplemente lo dejamos caer

    Ok(Some(ResponseInfo {
        path: path.to_string(),
        status,
        content_length,
        body_hash: String::new(), // ya no lo usamos
        latency_ms: latency,
    }))
}
