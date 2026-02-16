
pub mod types;
pub mod passive;
pub mod active;
pub mod behavioral;
pub mod scoring;

use hyper::Client;


use types::WafAnalysis;
use passive::detect_from_headers;
use active::active_probe;
use behavioral::behavioral_analysis;
use scoring::merge;

pub async fn analyze(
    client: &Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    target: &str,
    resp: &crate::models::ResponseInfo,
    ratio_403: f64,
    ratio_429: f64,
) -> WafAnalysis {

    let passive = detect_from_headers(&resp.headers);



	
    let active = active_probe(client, target).await;
    let behavioral = behavioral_analysis(ratio_403, ratio_429);

    let mut combined = merge(passive, active);
	if let Some(body) = &resp.body_sample {
		if let Some((vendor, score, signal)) =
			passive::detect_from_body(body)
		{
			combined.add_signal(&vendor, types::WafKind::Waf, score, &signal);
		}
	}
    combined = merge(combined, behavioral);
	
	combined
}
