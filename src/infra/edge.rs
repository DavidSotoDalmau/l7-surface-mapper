use hyper::HeaderMap;

pub fn detect_edge(headers: &HeaderMap) -> Option<String> {

    if headers.contains_key("cf-ray") {
        return Some("Cloudflare CDN / WAF".into());
    }

    if headers.contains_key("x-vercel-id") {
        return Some("Vercel Edge Network".into());
    }

    if headers.contains_key("x-vercel-mitigated") {
        return Some("Vercel Edge WAF".into());
    }

    if headers.contains_key("x-iinfo") {
        return Some("Imperva Incapsula WAF".into());
    }

    if headers.contains_key("akamai-origin-hop") {
        return Some("Akamai CDN".into());
    }

    if headers.contains_key("x-amz-cf-id") {
        return Some("AWS CloudFront".into());
    }

    if headers.contains_key("via") {
        if let Ok(v) = headers["via"].to_str() {
            if v.to_lowercase().contains("fastly") {
                return Some("Fastly CDN".into());
            }
            if v.to_lowercase().contains("varnish") {
                return Some("Varnish Cache".into());
            }
        }
    }

    None
}
