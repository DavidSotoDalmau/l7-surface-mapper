
use hyper::Client;
use crate::waf::types::{WafAnalysis, WafKind};
use crate::http_engine;

pub async fn active_probe(
    client: &Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    target: &str,
) -> WafAnalysis {

    let mut result = WafAnalysis::new();

    let probe_path = "__waf_probe_test__";

    if let Ok(Some(resp)) =
        http_engine::fetch(client, target, probe_path, "GET", None).await
    {
        if resp.status == 403 {
            result.add_signal(
                "Generic WAF",
                WafKind::Waf,
                50,
                "403 on probe path",
            );
        }
    }

    result
}
