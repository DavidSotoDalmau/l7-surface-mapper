use super::InfraProfile;

pub fn correlate(mut profile: InfraProfile) -> InfraProfile {

    // ==========================
    // Infer hosting by patterns
    // ==========================

    if let Some(edge) = &profile.edge {
        if edge.contains("Vercel") {
            profile.hosting = Some("Vercel Platform".into());
            profile.language = Some("Node.js".into());
        }

        if edge.contains("Cloudflare") && profile.backend_server.is_none() {
            profile.hosting = Some("Cloudflare proxied origin".into());
        }
    }

    // ==========================
    // Infer stack coherence
    // ==========================

    if let Some(framework) = &profile.framework {
        if framework.contains("Next.js") {
            profile.language = Some("Node.js".into());
        }

        if framework.contains("Laravel") || framework.contains("WordPress") {
            profile.language = Some("PHP".into());
        }
    }

    // ==========================
    // TLS correlation
    // ==========================

    if let Some(issuer) = &profile.tls_issuer {
        if issuer.contains("Let's Encrypt") {
            if profile.hosting.is_none() {
                profile.hosting = Some("Likely PaaS or VPS (Let's Encrypt managed)".into());
            }
        }
    }

    profile
}
