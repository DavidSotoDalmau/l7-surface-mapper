use super::InfraProfile;

pub struct InfraReport {
    pub summary: Vec<String>,
    pub confidence: u8,
}

pub fn generate(profile: &InfraProfile) -> InfraReport {

    let mut confidence = 0;
    let mut summary = Vec::new();

    if profile.edge.is_some() {
        confidence += 20;
    }

    if profile.backend_server.is_some() {
        confidence += 15;
    }

    if profile.framework.is_some() {
        confidence += 15;
    }

    if profile.language.is_some() {
        confidence += 15;
    }

    if profile.tls_issuer.is_some() {
        confidence += 15;
    }

    if profile.cold_start_detected {
        confidence += 10;
    }

    if profile.estimated_ceiling.is_some() {
        confidence += 10;
    }

    if let Some(edge) = &profile.edge {
        summary.push(format!("Edge: {}", edge));
    }

    if let Some(hosting) = &profile.hosting {
        summary.push(format!("Hosting: {}", hosting));
    }

    if let Some(framework) = &profile.framework {
        summary.push(format!("Framework: {}", framework));
    }

    if let Some(lang) = &profile.language {
        summary.push(format!("Language: {}", lang));
    }

    if let Some(tls) = &profile.tls_issuer {
        summary.push(format!("TLS: {}", tls));
    }

    InfraReport {
        summary,
        confidence: confidence.min(100),
    }
}
