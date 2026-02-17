
use crate::models::ResponseInfo;

pub fn detect_language(resp: &ResponseInfo) -> Option<String> {

    if let Some(h) = resp.headers.get("x-powered-by") {
        if let Ok(v) = h.to_str() {
            let v = v.to_lowercase();
            if v.contains("php") { return Some("PHP".into()); }
            if v.contains("express") { return Some("Node.js".into()); }
            if v.contains("asp.net") { return Some(".NET".into()); }
        }
    }
	
	if let Some(body) = &resp.body_sample {
		let lower = body.to_lowercase();
if lower.contains("/wp-content/") {
    return Some("PHP".into());
}
		if lower.contains("<script") &&
		   lower.contains(".js") {
			return Some("JavaScript / TypeScript".into());
		}
	}

    if let Some(cookie) = resp.headers.get("set-cookie") {
        if let Ok(v) = cookie.to_str() {
            let v = v.to_lowercase();
            if v.contains("phpsessid") { return Some("PHP".into()); }
            if v.contains("jsessionid") { return Some("Java".into()); }
            if v.contains("csrftoken") { return Some("Python".into()); }
        }
    }
	if let Some(body) = &resp.body_sample {
    let lower = body.to_lowercase();

    if lower.contains("<?php") {
        return Some("PHP".into());
    }

    if lower.contains("wp-content") {
        return Some("PHP".into());
    }

    if lower.contains("django") || lower.contains("csrfmiddlewaretoken") {
        return Some("Python".into());
    }

    if lower.contains("reactroot") || lower.contains("__next_data__") {
        return Some("Node.js".into());
    }
}

    None
}
