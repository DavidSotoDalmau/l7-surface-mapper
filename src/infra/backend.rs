
use hyper::HeaderMap;

pub fn detect_server(headers: &HeaderMap) -> Option<String> {

    if let Some(server) = headers.get("server") {
        if let Ok(s) = server.to_str() {
            let s = s.to_lowercase();

            if s.contains("nginx") {
                return Some("Nginx".into());
            }

            if s.contains("apache") {
                return Some("Apache".into());
            }
			
			if s.contains("iis") {
                return Some("Microsoft IIS".into());
            }

            if s.contains("gunicorn") {
                return Some("Python (Gunicorn)".into());
            }
			
            if s.contains("vercel") {
                return Some("Serverless (Vercel)".into());
            }
			
			if s.contains("caddy") {
                return Some("Caddy".into());
            }

            if s.contains("envoy") {
                return Some("Envoy Proxy".into());
            }
        }
		
    }
	None
}
pub fn detect_language(headers: &HeaderMap, body: Option<&str>) -> Option<String> {
    if let Some(powered) = headers.get("x-powered-by") {
        if let Ok(p) = powered.to_str() {
            let p = p.to_lowercase();

            if p.contains("express") {
                return Some("Node.js (Express)".into());
            }

            if p.contains("php") {
                return Some("PHP".into());
            }
			
			if p.contains("asp.net") {
                return Some(".NET (ASP.NET)".into());
            }

            if p.contains("next.js") {
                return Some("Next.js".into());
            }
        }
    }
	if let Some(cookie) = headers.get("set-cookie") {
        if let Ok(c) = cookie.to_str() {
            let c = c.to_lowercase();

            if c.contains("laravel_session") {
                return Some("PHP (Laravel)".into());
            }

            if c.contains("phpsessid") {
                return Some("PHP (Generic)".into());
            }

            if c.contains("asp.net") {
                return Some(".NET".into());
            }

            if c.contains("_next") {
                return Some("Next.js".into());
            }
			if c.contains("JSESSIONID") {
                return Some("Java".into());
            }
        }
    }
    if let Some(body) = body {
        let lower = body.to_lowercase();

        if lower.contains("_next/static") {
            return Some("Next.js".into());
        }

        if lower.contains("laravel_session") {
            return Some("Laravel".into());
        }
		 if lower.contains("ng-app") || lower.contains("angular") {
            return Some("Angular".into());
        }

        if lower.contains("reactroot") || lower.contains("data-reactroot") {
            return Some("React".into());
        }

        if lower.contains("vue.js") || lower.contains("data-v-") {
            return Some("Vue.js".into());
        }

        if lower.contains("wp-content") {
            return Some("WordPress (PHP)".into());
        }

        if lower.contains("drupal") {
            return Some("Drupal".into());
        }
    }

    None
}
