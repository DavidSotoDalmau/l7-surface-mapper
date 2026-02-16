
use crate::waf::types::{WafAnalysis, WafKind};

pub fn merge(mut base: WafAnalysis, other: WafAnalysis) -> WafAnalysis {
    if other.detected {
        base.detected = true;

        if base.vendor.is_none() {
            base.vendor = other.vendor;
        }

        base.confidence = (base.confidence + other.confidence).min(100);
		if base.confidence > 80 {
			base.kind = WafKind::Waf;
		} else if base.confidence > 50 {
			base.kind = WafKind::ReverseProxy;
		} else if base.confidence > 30 {
			base.kind = WafKind::Cdn;
		}
        for s in other.signals {
            base.signals.push(s);
        }
    }

    base
}
