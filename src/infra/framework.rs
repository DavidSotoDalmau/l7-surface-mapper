pub fn detect_framework(body: Option<&str>) -> Option<String> {

    if let Some(body) = body {
        let lower = body.to_lowercase();

        if lower.contains("_next/static") {
            return Some("Next.js".into());
        }

        if lower.contains("wp-content") {
            return Some("WordPress".into());
        }

        if lower.contains("laravel_session") {
            return Some("Laravel".into());
        }

        if lower.contains("angular") {
            return Some("Angular".into());
        }

        if lower.contains("react") {
            return Some("React".into());
        }

        if lower.contains("django") {
            return Some("Django".into());
        }
		if lower.contains("<app-root") {
			return Some("Angular".into());
		}

		if lower.contains("ng-version") {
			return Some("Angular".into());
		}
		if lower.contains("/wp-content/") ||
		   lower.contains("/wp-includes/") ||
		   lower.contains("wp-json") {
			return Some("WordPress".into());
		}
    }

    None
}
