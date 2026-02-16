
use crate::waf::types::{WafAnalysis, WafKind};

pub fn behavioral_analysis(
    ratio_403: f64,
    ratio_429: f64,
) -> WafAnalysis {

    let mut result = WafAnalysis::new();

    if ratio_403 > 0.3 {
        result.add_signal(
            "Behavioral Block",
            WafKind::Waf,
            40,
            "High 403 ratio detected",
        );
    }

    if ratio_429 > 0.2 {
        result.add_signal(
            "Rate Limiting Layer",
            WafKind::Waf,
            30,
            "High 429 ratio detected",
        );
    }

    result
}
