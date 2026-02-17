
use super::AppProfile;

pub fn infer(mut profile: AppProfile) -> AppProfile {

    let mut score = 0;

    if profile.language.is_some() { score += 20; }
    if profile.framework.is_some() { score += 25; }
    if profile.frontend_type.is_some() { score += 15; }
    if profile.api_type.is_some() { score += 15; }
    if profile.cms.is_some() { score += 25; }
	if profile.framework.as_deref() == Some("Angular") {
		profile.frontend_type = Some("SPA (Client-Side Rendering)".into());
	}
	if profile.framework.is_none() &&
	   profile.cms.is_none() &&
	   profile.language.is_none() {

		profile.architecture = Some(
			"Likely proprietary / custom high-scale architecture".into()
		);
	}
    profile.confidence = score.min(100);

    profile
}
