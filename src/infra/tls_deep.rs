use tokio_rustls::TlsConnector;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use webpki_roots::TLS_SERVER_ROOTS;
use tokio::net::TcpStream;
use std::sync::Arc;
use x509_parser::prelude::*;

pub struct TlsDeepProfile {
    pub tls_version: Option<String>,
    pub alpn: Option<String>,
    pub issuer: Option<String>,
    pub subject: Option<String>,
    pub is_wildcard: bool,
}

pub fn classify_issuer(issuer: &str) -> String {
    let lower = issuer.to_lowercase();

    if lower.contains("let's encrypt") || lower.contains("r3") || lower.contains("wr1") {
        return "Let's Encrypt (DV Certificate)".into();
    }

    if lower.contains("digicert") {
        return "DigiCert (Commercial CA)".into();
    }

    if lower.contains("cloudflare") {
        return "Cloudflare Origin CA".into();
    }

    if lower.contains("amazon") {
        return "Amazon Trust Services".into();
    }

    if lower.contains("globalsign") {
        return "GlobalSign CA".into();
    }

    issuer.to_string()
}

pub async fn analyze_tls(host: &str) -> Option<TlsDeepProfile> {

    let addr = format!("{}:443", host);
    let stream = TcpStream::connect(addr).await.ok()?;

    let mut root_store = RootCertStore::empty();
    root_store.extend(TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));

    let server_name = rustls::pki_types::ServerName::try_from(host.to_string()).ok()?;
    let tls_stream = connector.connect(server_name, stream).await.ok()?;

    let (_, session) = tls_stream.get_ref();

    let tls_version = session.protocol_version().map(|v| format!("{:?}", v));
    let alpn = session.alpn_protocol().map(|v| String::from_utf8_lossy(v).to_string());

    let certs = session.peer_certificates()?;
    let first_cert = certs.first()?;

    let (_, parsed_cert) = parse_x509_certificate(first_cert.as_ref()).ok()?;

    let issuer_cn = parsed_cert
		.issuer()
		.iter_common_name()
		.next()
		.and_then(|cn| cn.as_str().ok())
		.map(|s| s.to_string());

	let issuer_org = parsed_cert
		.issuer()
		.iter_organization()
		.next()
		.and_then(|o| o.as_str().ok())
		.map(|s| s.to_string());

	let issuer = match (issuer_cn, issuer_org) {
		(Some(cn), Some(org)) => Some(format!("{} ({})", org, cn)),
		(Some(cn), None) => Some(cn),
		_ => None,
	};


    let subject = parsed_cert.subject().iter_common_name().next()
        .map(|cn| cn.as_str().unwrap_or("").to_string());

    let is_wildcard = subject.as_deref().map(|s| s.starts_with("*.")).unwrap_or(false);

    Some(TlsDeepProfile {
        tls_version,
        alpn,
        issuer,
        subject,
        is_wildcard,
    })
}
