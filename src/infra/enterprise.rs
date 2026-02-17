use crate::models::ResponseInfo;

pub fn detect_enterprise_edge(resp: &ResponseInfo) -> Vec<String> {

    let mut findings = Vec::new();
    let headers = &resp.headers;

    // 🔥 Cloudflare
    if headers.contains_key("cf-ray") ||
       headers.contains_key("cf-cache-status") {
        findings.push("Cloudflare CDN / WAF".into());
    }

    // 🔥 Akamai
    if headers.contains_key("akamai-origin-hop") ||
       headers.contains_key("x-akamai-transformed") {
        findings.push("Akamai CDN".into());
    }

    // 🔥 Fastly
    if headers.contains_key("x-served-by") &&
       headers.contains_key("x-cache") {
        findings.push("Fastly CDN".into());
    }

    // 🔥 AWS CloudFront
    if headers.contains_key("x-amz-cf-id") ||
       headers.contains_key("x-amz-cf-pop") {
        findings.push("AWS CloudFront".into());
    }

    // 🔥 AWS ALB
    if headers.contains_key("x-amzn-trace-id") {
        findings.push("AWS Application Load Balancer".into());
    }

    // 🔥 Azure Front Door
    if headers.contains_key("x-azure-ref") {
        findings.push("Azure Front Door".into());
    }

    // 🔥 Google Frontend
    if headers.contains_key("x-cloud-trace-context") ||
       headers.contains_key("server-timing") {
        findings.push("Google Frontend / GCP Edge".into());
    }

    // 🔥 F5 BigIP
    if headers.contains_key("x-waf-event") ||
       headers.contains_key("x-f5-request-id") {
        findings.push("F5 BigIP".into());
    }

    // 🔥 Imperva
    if headers.contains_key("x-iinfo") {
        findings.push("Imperva WAF".into());
    }

    // 🔥 Envoy / Istio
    if headers.contains_key("x-envoy-upstream-service-time") {
        findings.push("Envoy / Service Mesh".into());
    }

    // 🔥 HAProxy
    if headers.contains_key("x-haproxy-server-state") {
        findings.push("HAProxy".into());
    }

    // 🔥 Varnish
    if headers.contains_key("x-varnish") {
        findings.push("Varnish Cache".into());
    }

    findings
}
pub fn estimate_proxy_depth(resp: &ResponseInfo) -> usize {

    let mut depth = 0;
    let headers = &resp.headers;

    if headers.contains_key("via") { depth += 1; }
    if headers.contains_key("x-forwarded-for") { depth += 1; }
    if headers.contains_key("x-envoy-upstream-service-time") { depth += 1; }
    if headers.contains_key("x-amzn-trace-id") { depth += 1; }

    depth
}