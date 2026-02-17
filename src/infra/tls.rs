use hyper::HeaderMap;

pub fn detect_tls(headers: &HeaderMap) -> Option<String> {

    // HTTP/3 detection
    if let Some(alt_svc) = headers.get("alt-svc") {
        if let Ok(v) = alt_svc.to_str() {
            if v.contains("h3") {
                return Some("HTTP/3 (QUIC) enabled".into());
            }
        }
    }

    // Cloudflare
    if headers.contains_key("cf-ray") {
        return Some("Cloudflare Managed TLS".into());
    }

    // AWS CloudFront
    if headers.contains_key("x-amz-cf-id") {
        return Some("AWS CloudFront TLS".into());
    }

    // Google Frontend
    if let Some(server) = headers.get("server") {
        if let Ok(s) = server.to_str() {
            let s = s.to_lowercase();
            if s.contains("google frontend") {
                return Some("Google Cloud TLS".into());
            }
        }
    }

    // Vercel
    if headers.contains_key("x-vercel-id") {
        return Some("Vercel Managed TLS (Let's Encrypt)".into());
    }

    // Fastly
    if headers.contains_key("x-served-by") {
        return Some("Fastly TLS".into());
    }

    None
}
