
use hyper::HeaderMap;
use crate::waf::types::{WafAnalysis, WafKind};

pub fn detect_from_headers(headers: &HeaderMap) -> WafAnalysis {
    let mut result = WafAnalysis::new();

    if headers.contains_key("cf-ray") {
        result.add_signal("Cloudflare", WafKind::Cdn, 80, "cf-ray header detected");
    }

    if headers.contains_key("x-iinfo") {
        result.add_signal("Imperva Incapsula", WafKind::Waf, 80, "x-iinfo header detected");
    }

    if headers.contains_key("x-sucuri-id") {
        result.add_signal("Sucuri", WafKind::Waf, 75, "x-sucuri-id header detected");
    }

    if headers.contains_key("akamai-origin-hop") {
        result.add_signal("Akamai", WafKind::Cdn, 70, "akamai-origin-hop header detected");
    }

    if headers.contains_key("x-amzn-requestid") {
        result.add_signal("AWS (ALB/WAF)", WafKind::Waf, 60, "x-amzn-requestid header detected");
    }
	if headers.contains_key("x-vercel-mitigated") {
		result.add_signal(
			"Vercel Edge Protection",
			WafKind::ReverseProxy,
			85,
			"x-vercel-mitigated header detected",
		);
	}

	if headers.contains_key("x-vercel-id") {
		result.add_signal(
			"Vercel Edge",
			WafKind::ReverseProxy,
			40,
			"x-vercel-id header detected",
		);
	}
    result
}
pub fn detect_from_body(body: &str) -> Option<(String, u8, String)> {
    let lower = body.to_lowercase();

    if lower.contains("attention required") {
        return Some(("Cloudflare Challenge".into(), 90, "Cloudflare challenge page".into()));
    }

    if lower.contains("access denied") {
        return Some(("Generic WAF".into(), 60, "Access denied pattern".into()));
    }

    if lower.contains("request blocked") {
        return Some(("Generic WAF".into(), 70, "Blocked request pattern".into()));
    }

    if lower.contains("vercel") && lower.contains("denied") {
        return Some(("Vercel Edge Protection".into(), 85, "Vercel block page".into()));
    }

    None
}
