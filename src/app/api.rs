
use crate::models::ResponseInfo;

pub fn detect_api(resp: &ResponseInfo) -> Option<String> {

    if resp.path.to_lowercase().contains("graphql") {
        return Some("GraphQL".into());
    }

    if let Some(ct) = resp.headers.get("content-type") {
        if let Ok(v) = ct.to_str() {
            if v.contains("application/json") {
                return Some("REST API".into());
            }
        }
    }

    None
}
