use hyper::{Client, Request, Body, Uri, Method};
use std::time::Instant;
use crate::models::ResponseInfo;

pub async fn fetch(
    client: &Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    base_url: &str,
    path: &str,
	method: &str,
	data: Option<&str>,
) -> Result<Option<ResponseInfo>, hyper::Error> {

    let clean_path = path.trim();

    if !clean_path.is_ascii() {
        return Ok(None);
    }

    let base = base_url.trim_end_matches('/');

let (final_url, final_body) = match (base.contains("FUZZ"), data) {

    // 🔥 FUZZ en URL
    (true, Some(d)) => {
        let replaced_url = base.replace("FUZZ", clean_path);
        let replaced_body = if d.contains("FUZZ") {
            Some(d.replace("FUZZ", clean_path))
        } else {
            Some(d.to_string())
        };
        (replaced_url, replaced_body)
    }

    // 🔥 FUZZ solo en URL sin body
    (true, None) => {
        (base.replace("FUZZ", clean_path), None)
    }

    // 🔥 FUZZ solo en body
    (false, Some(d)) if d.contains("FUZZ") => {
        (base.to_string(), Some(d.replace("FUZZ", clean_path)))
    }

    // 🔥 Sin FUZZ en ningún sitio → concatenación clásica
    (false, Some(d)) => {
        (format!("{}/{}", base, clean_path), Some(d.to_string()))
    }

    (false, None) => {
        (format!("{}/{}", base, clean_path), None)
    }
};


    let uri: Uri = match final_url.parse() {
        Ok(u) => u,
        Err(_) => return Ok(None),
    };
	let http_method = match method.to_uppercase().as_str() {
		"POST" => Method::POST,
		"HEAD" => Method::HEAD,
		"OPTIONS" => Method::OPTIONS,
		"PUT" => Method::PUT,
		"DELETE" => Method::DELETE,
		_ => Method::GET,
	};
	let body = match final_body  {
		Some(ref b) => Body::from(b.clone()),
		None => Body::empty(),
	};
    let req = match Request::builder()
        .method(http_method)
        .uri(uri)
		.header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
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
