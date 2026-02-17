
use crate::models::ResponseInfo;

pub fn detect_framework(resp: &ResponseInfo) -> Option<String> {

    if let Some(body) = &resp.body_sample {
        let lower = body.to_lowercase();

        if lower.contains("_next/static") { return Some("Next.js".into()); }
        if lower.contains("wp-content") { return Some("WordPress".into()); }
        if lower.contains("laravel_session") { return Some("Laravel".into()); }
        if lower.contains("django") { return Some("Django".into()); }
        if lower.contains("spring") { return Some("Spring Boot".into()); }
        if lower.contains("rails") { return Some("Ruby on Rails".into()); }
    }

    None
}
