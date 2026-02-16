use crate::models::{ResponseInfo, Baseline, Finding};

pub fn analyze(resp: &ResponseInfo, baseline: &Baseline) -> Option<Finding> {
    if resp.status == 200 && resp.content_length != baseline.content_length {
        return Some(Finding {
            path: resp.path.clone(),
            status: resp.status,
            content_length: resp.content_length,
        });
    }

    if matches!(resp.status, 401 | 403 | 405 | 500) {
        return Some(Finding {
            path: resp.path.clone(),
            status: resp.status,
            content_length: resp.content_length,
        });
    }

    None
}