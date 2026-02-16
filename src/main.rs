mod config;
mod http_engine;
mod baseline;
mod analyzer;
mod rate_limit;
mod models;

use clap::Parser;
use memmap2::Mmap;
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::Mutex;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};

use hyper::Client;
use hyper_rustls::HttpsConnectorBuilder;

use config::Config;
use rate_limit::RateLimiterDetector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    // 🔥 Hyper HTTPS Connector
    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build();

    let client: Client<_> = Client::builder()
    .http2_adaptive_window(true)   // 👈 aquí activamos soporte HTTP2
    .pool_max_idle_per_host(1000)
    .build(https);

    // mmap wordlist
    let file = File::open(&config.wordlist)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let wordlist = std::str::from_utf8(&mmap)?
        .lines()
        .map(|s| s.trim())
        .filter(|s| {
    let trimmed = s.trim();
    !trimmed.is_empty() &&
    !trimmed.starts_with('#') &&
    trimmed.is_ascii()
})

        .collect::<Vec<_>>();

    let total = wordlist.len() as u64;

    // 📊 Progress bar
    let pb = Arc::new(ProgressBar::new(total));
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)"
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    let counter = Arc::new(AtomicUsize::new(0));

    // 🔍 Baseline detection
    let random_path = "this_path_should_not_exist_123456";
   let baseline_resp =
    http_engine::fetch(&client, &config.target, random_path).await?;

let baseline_resp = match baseline_resp {
    Some(r) => r,
    None => {
        panic!("Baseline request failed due to invalid URI");
    }
};

let baseline = baseline::build_baseline(&baseline_resp);

    println!("Baseline established: {:?}", baseline);

    let rate_detector = Arc::new(Mutex::new(RateLimiterDetector::new(200)));

    stream::iter(wordlist)
        .for_each_concurrent(config.concurrency, |path| {
            let client = client.clone();
            let baseline = baseline.clone();
            let target = config.target.clone();
            let rate_detector = rate_detector.clone();
            let counter = counter.clone();
            let pb = pb.clone();

            async move {
                if let Ok(Some(resp)) = http_engine::fetch(&client, &target, path).await {
                    {
                        let mut guard = rate_detector.lock().await;
                        guard.record(resp.status);

                        if guard.rate_limited() {
                            pb.println("[!] Rate limiting detected.");
                        }
                    }

                    if let Some(finding) =
                        analyzer::analyze(&resp, &baseline)
                    {
                        pb.println(format!(
                            "[+] {} (status: {}, {} bytes)",
                            finding.path,
                            finding.status,
                            finding.content_length
                        ));
                    }
                }

                let prev = counter.fetch_add(1, Ordering::Relaxed);
                pb.set_position((prev + 1) as u64);
            }
        })
        .await;

    pb.finish_with_message("Completed");

    Ok(())
}
