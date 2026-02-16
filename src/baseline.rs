use crate::models::{Baseline, ResponseInfo};

pub fn build_baseline(resp: &ResponseInfo) -> Baseline {
    Baseline {
        status: resp.status,
        content_length: resp.content_length,
        body_hash: resp.body_hash.clone(),
    }
}

pub fn is_baseline(resp: &ResponseInfo, baseline: &Baseline) -> bool {
    resp.status == baseline.status &&
    resp.content_length == baseline.content_length &&
    resp.body_hash == baseline.body_hash
}
