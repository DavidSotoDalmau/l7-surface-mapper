
pub fn analyze_behavior(avg_latency: f64, baseline_latency: f64) -> bool {

    if baseline_latency == 0.0 {
        return false;
    }
	if avg_latency > baseline_latency * 2.5 && avg_latency > 300.0 {
        return true;
    }

    false
}
