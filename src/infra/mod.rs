pub mod edge;
pub mod hosting;
pub mod backend;
pub mod framework;
pub mod tls;
pub mod behavior;
pub mod tls_deep;
pub mod correlation;
pub mod report;
use crate::infra::tls_deep::classify_issuer;
use crate::models::ResponseInfo;
use crate::infra::correlation::correlate;

#[derive(Debug, Clone)]
pub struct InfraProfile {
    pub edge: Option<String>,
    pub hosting: Option<String>,
    pub backend_server: Option<String>,
    pub language: Option<String>,
    pub framework: Option<String>,
    pub tls_issuer: Option<String>,
    pub tls_version: Option<String>,
    pub tls_alpn: Option<String>,
    pub tls_subject: Option<String>,
    pub wildcard_cert: bool,
    pub cold_start_detected: bool,
    pub estimated_ceiling: Option<usize>,
    pub mitigation: Option<String>,
}

impl InfraProfile {
    pub fn new() -> Self {
        Self {
            edge: None,
            hosting: None,
            backend_server: None,
            language: None,
            framework: None,
            tls_issuer: None,
            tls_version: None,
            tls_alpn: None,
            tls_subject: None,
            wildcard_cert: false,
            cold_start_detected: false,
            estimated_ceiling: None,
            mitigation: None,
        }
    }
}

pub async fn analyze(
    resp: &ResponseInfo,
    avg_latency: f64,
    baseline_latency: f64,
    estimated_ceiling: Option<usize>,
    host: &str,
) -> InfraProfile {

    let mut profile = InfraProfile::new();

    profile.edge = edge::detect_edge(&resp.headers);
    profile.hosting = hosting::detect_hosting(&resp.headers);
    profile.backend_server = backend::detect_server(&resp.headers);
    profile.language = backend::detect_language(&resp.headers, resp.body_sample.as_deref());
    profile.framework = framework::detect_framework(resp.body_sample.as_deref());
    profile.tls_issuer = tls::detect_tls(&resp.headers);
    profile.cold_start_detected = behavior::analyze_behavior(avg_latency, baseline_latency);
    profile.estimated_ceiling = estimated_ceiling;

    if let Some(tls) = tls_deep::analyze_tls(host).await {
        profile.tls_version = tls.tls_version;
        profile.tls_alpn = match tls.alpn.as_deref() {
			Some("h2") => Some("HTTP/2".into()),
			Some("http/1.1") => Some("HTTP/1.1".into()),
			Some(v) => Some(v.to_string()),
			None => None,
		};
        profile.tls_issuer = tls.issuer.map(|i| classify_issuer(&i));
        profile.tls_subject = tls.subject;
        profile.wildcard_cert = tls.is_wildcard;
    }

    if resp.status == 403 {
        profile.mitigation = Some("Infrastructure-level blocking detected".into());
    }
	
	let profile = correlate(profile);
    profile
}
