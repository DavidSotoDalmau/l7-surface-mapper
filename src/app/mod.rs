
pub mod language;
pub mod framework;
pub mod api;
pub mod frontend;
pub mod cms;
pub mod inference;
pub mod backend;
use serde::Serialize;
use crate::models::ResponseInfo;

#[derive(Debug, Clone, Serialize)]
pub struct AppProfile {
    pub language: Option<String>,
    pub framework: Option<String>,
    pub frontend_type: Option<String>,
    pub api_type: Option<String>,
    pub cms: Option<String>,
    pub confidence: u8,
	pub architecture: Option<String>,
}

impl AppProfile {
    pub fn new() -> Self {
        Self {
            language: None,
            framework: None,
            frontend_type: None,
            api_type: None,
            cms: None,
            confidence: 0,
			 architecture: None,
        }
    }
}

pub fn analyze(resp: &ResponseInfo) -> AppProfile {
    let mut profile = AppProfile::new();

    profile.language = language::detect_language(resp);
    profile.framework = framework::detect_framework(resp);
    profile.frontend_type = frontend::detect_frontend(resp);
    profile.api_type = api::detect_api(resp);
    profile.cms = cms::detect_cms(resp);
	profile.framework = framework::detect_framework(resp)
		.or_else(|| backend::detect_backend_framework(resp));
    profile = inference::infer(profile);

    profile
}
