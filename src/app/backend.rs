use crate::models::ResponseInfo;

pub fn detect_backend_framework(resp: &ResponseInfo) -> Option<String> {

    if let Some(body) = &resp.body_sample {
        let lower = body.to_lowercase();

        // Django
        if lower.contains("csrftoken") ||
           lower.contains("csrfmiddlewaretoken") {
            return Some("Django".into());
        }

        // Flask
        if lower.contains("werkzeug") {
            return Some("Flask".into());
        }

        // FastAPI
        if lower.contains("\"openapi\"") {
            return Some("FastAPI".into());
        }

        // Spring Boot
        if lower.contains("whitelabel error page") {
            return Some("Spring Boot".into());
        }

        // Express
        if lower.contains("at layer.handle") {
            return Some("Express".into());
        }

        // ASP.NET
        if lower.contains("asp.net_sessionid") {
            return Some("ASP.NET".into());
        }

        // Rails
        if lower.contains("_rails_session") {
            return Some("Ruby on Rails".into());
        }
    }

    None
}
