use hyper::HeaderMap;

pub fn detect_hosting(headers: &HeaderMap) -> Option<String> {

    if headers.contains_key("x-azure-ref") {
        return Some("Microsoft Azure Front Door".into());
    }

    if headers.contains_key("x-heroku-request-id") {
        return Some("Heroku PaaS".into());
    }

    if headers.contains_key("fly-request-id") {
        return Some("Fly.io".into());
    }

    if headers.contains_key("x-render-origin-server") {
        return Some("Render PaaS".into());
    }

    if headers.contains_key("server") {
        if let Ok(s) = headers["server"].to_str() {
            let s = s.to_lowercase();
            if s.contains("netlify") {
                return Some("Netlify".into());
            }
            if s.contains("azure") {
                return Some("Azure App Service".into());
            }
        }
    }

    None
}
