
use crate::models::ResponseInfo;

pub fn detect_frontend(resp: &ResponseInfo) -> Option<String> {

    if let Some(body) = &resp.body_sample {
        let lower = body.to_lowercase();
		if lower.contains("<app-root") {
            return Some("SPA (Angular)".into());
        }

        // Angular CLI build pattern
        if lower.contains("runtime.") &&
           lower.contains("polyfills.") &&
           lower.contains("main.") {
            return Some("SPA (Angular CLI build)".into());
        }

        // React
        if lower.contains("id=\"root\"") ||
           lower.contains("data-reactroot") {
            return Some("SPA (React)".into());
        }

        // Next.js SSR
        if lower.contains("_next/static") {
            return Some("SSR (Next.js)".into());
        }

        // Vue
        if lower.contains("data-v-") ||
           lower.contains("vue.js") {
            return Some("SPA (Vue)".into());
        }
        if lower.contains("<div id=\"root\"") { return Some("SPA (React-like)".into()); }
        if lower.contains("ng-app") { return Some("SPA (Angular)".into()); }
        if lower.contains("_next/static") { return Some("SSR (Next.js)".into()); }
        if lower.contains("vue.js") { return Some("SPA (Vue)".into()); }
		if lower.contains("<app-root") {
			return Some("SPA (Angular)".into());
		}
if lower.contains("<main") &&
   lower.contains("<h1") &&
   !lower.contains("<app-root") {
    return Some("SSR (Server-Side Rendering)".into());
}
		if lower.contains("runtime.") &&
		   lower.contains("polyfills.") &&
		   lower.contains("main.") {
			return Some("SPA (Angular CLI build)".into());
		}
    }

    None
}
