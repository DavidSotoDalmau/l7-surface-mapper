mod config;
mod http_engine;
mod baseline;
mod analyzer;
mod rate_limit;
mod models;
mod waf;

use std::time::Duration;
use clap::Parser;
use memmap2::Mmap;
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering, AtomicBool};
use tokio::sync::Mutex;

use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{HashMap,HashSet};
use hyper::Client;
use hyper_rustls::HttpsConnectorBuilder;
use waf::analyze as waf_analyze;

use config::Config;
use rate_limit::RateLimiterDetector;

struct RuntimeStats {
    total_requests: AtomicUsize,
    total_429: AtomicUsize,
	total_403: AtomicUsize,   // 👈 NUEVO
    total_latency: AtomicU64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let stop_flag = Arc::new(AtomicBool::new(false));
    let config = Config::parse();
	let results: Arc<Mutex<HashMap<u16, HashSet<String>>>> =
    Arc::new(Mutex::new(HashMap::new()));
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
	let stats = Arc::new(RuntimeStats {
		total_requests: AtomicUsize::new(0),
		total_429: AtomicUsize::new(0),
		 total_403: AtomicUsize::new(0),   // 👈 NUEVO
		total_latency: AtomicU64::new(0),
	});
	let current_concurrency = Arc::new(AtomicUsize::new(config.concurrency));
	let active_requests = Arc::new(AtomicUsize::new(0));
	let stats_clone = stats.clone();
	let current_clone = current_concurrency.clone();
	let stop_flag_clone = stop_flag.clone();  
	tokio::spawn(async move {
		let mut baseline_latency: Option<f64> = None;
		let mut phase: u8 = 1; // 1=fast, 2=medium, 3=fine
		let mut ceiling_estimate: usize = current_clone.load(Ordering::Relaxed);

		// parámetros configurables
		let fast_step = 200;
		let medium_step = 50;
		let fine_step = 5;

		loop {
			tokio::time::sleep(Duration::from_secs(3)).await;

			let total = stats_clone.total_requests.load(Ordering::Relaxed);
			if total < 20 {
				continue;
			}

			let r429 = stats_clone.total_429.load(Ordering::Relaxed);
			let latency_sum = stats_clone.total_latency.load(Ordering::Relaxed);

			let avg_latency = latency_sum as f64 / total as f64;
			let ratio_429 = r429 as f64 / total as f64;
			let r403 = stats_clone.total_403.load(Ordering::Relaxed);
			let ratio_403 = r403 as f64 / total as f64;
			let current = current_clone.load(Ordering::Relaxed);
			let mut new = current;

			// Establecer baseline inicial
			if baseline_latency.is_none() && ratio_429 == 0.0 {
				baseline_latency = Some(avg_latency);
			}

			// 🔻 BAJADA FUERTE POR 429
			if ratio_429 > 0.05 {
				phase = 2;
				new = (current as f64 * 0.6) as usize;
				ceiling_estimate = new;
			}
			// 🔻 BAJADA MODERADA POR LATENCIA
			else if let Some(base) = baseline_latency {
				if avg_latency > base * 2.0 {
					phase = 2;
					new = (current as f64 * 0.75) as usize;
					ceiling_estimate = new;
				}
				else {
					// 🔺 ESCALADO SEGÚN FASE
					match phase {
						1 => {
							// Fase rápida
							new = current + fast_step;
						}
						2 => {
							if current >= ceiling_estimate {
								phase = 3;
								new = current + fine_step;
							} else {
								new = current + medium_step;
							}
						}
						3 => {
							new = current + fine_step;
						}
						_ => {}
					}
				}
			}

			// 🔻 Ajuste fino hacia abajo si hay algo de 429 leve
			if ratio_429 > 0.0 && ratio_429 <= 0.05 {
				new = current.saturating_sub(fine_step);
			}
			if ratio_429 > 0.3 || ratio_403 > 0.4 {
				println!("\n[WAF] High block ratio detected. Aborting.");
				stop_flag_clone.store(true, Ordering::Relaxed);
				current_clone.store(0, Ordering::Relaxed);
			}
			// Clamp de seguridad
			new = new.clamp(1, 5000);

			if new != current {
				current_clone.store(new, Ordering::Relaxed);

				println!(
					"[Adaptive] Phase: {} | Concurrency: {} | 429 ratio: {:.3} | Avg latency: {:.0}ms",
					phase,
					new,
					ratio_429,
					avg_latency
				);
			}

			// Reset ventana
			stats_clone.total_requests.store(0, Ordering::Relaxed);
			stats_clone.total_429.store(0, Ordering::Relaxed);
			stats_clone.total_latency.store(0, Ordering::Relaxed);
			stats_clone.total_403.store(0, Ordering::Relaxed);
		}
	});

	
    // mmap wordlist
    let file = File::open(&config.wordlist)?;
    let mmap = unsafe { Mmap::map(&file)? };

let wordlist: Vec<String> = std::str::from_utf8(&mmap)?
    .lines()
    .map(|s| s.trim())
    .filter(|s| {
        let trimmed = s.trim();
        !trimmed.is_empty() &&
        !trimmed.starts_with('#') &&
        trimmed.is_ascii()
    })
    .map(|s| s.to_string())
    .collect();

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
    http_engine::fetch(&client, &config.target, random_path,&config.method, config.data.as_deref()).await?;

	let baseline_resp = match baseline_resp {
		Some(r) => r,
		None => {
			panic!("Baseline request failed due to invalid URI");
		}
	};

	let baseline = baseline::build_baseline(&baseline_resp);

    println!("Baseline established: {:?}", baseline);
	// --- WAF Detection ---
	let waf_result = waf_analyze(
		&client,
		&config.target,
		&baseline_resp,
		0.0, // ratio_403 inicial
		0.0, // ratio_429 inicial
	).await;

	if waf_result.detected {
		println!(
			"\n[WAF] Detected: {:?} | Vendor: {:?} | Confidence: {}%",
			waf_result.kind,
			waf_result.vendor,
			waf_result.confidence
		);

		for s in waf_result.signals {
			println!("  └─ {}", s);
		}
	} else {
		println!("\n[WAF] No obvious WAF detected");
	}
	if waf_result.detected && waf_result.confidence > 60 {
		println!("[WAF] Switching adaptive controller to conservative mode");

		current_concurrency.store(
			(config.concurrency as f64 * 0.6) as usize,
			Ordering::Relaxed,
		);
	}
	if waf_result.detected && waf_result.confidence > 80 {
		println!("\n[WAF] Blocking behavior confirmed. Stopping fuzz.");
		stop_flag.store(true, Ordering::Relaxed);
	}
    let rate_detector = Arc::new(Mutex::new(RateLimiterDetector::new(200)));

    let wordlist = Arc::new(wordlist);
let mut handles = Vec::new();

// número base de workers
let base_workers = config.concurrency;
let index = Arc::new(AtomicUsize::new(0));
for _ in 0..base_workers {
    let wordlist = wordlist.clone();
    let stop_flag = stop_flag.clone();
    let client = client.clone();
    let baseline = baseline.clone();
    let target = config.target.clone();
    let rate_detector = rate_detector.clone();
    let counter = counter.clone();
    let pb = pb.clone();
    let method = config.method.clone();
    let data = config.data.clone();
    let current_concurrency = current_concurrency.clone();
    let active_requests = active_requests.clone();
    let stats = stats.clone();
    let results = results.clone();
let index = index.clone();
    let handle = tokio::spawn(async move {
       
        loop {
			if stop_flag.load(Ordering::Relaxed) {
				break;
			}

			let i = index.fetch_add(1, Ordering::Relaxed);

			if i >= wordlist.len() {
				break;
			}

			let path = &wordlist[i];
          
            while active_requests.load(Ordering::Relaxed)
                >= current_concurrency.load(Ordering::Relaxed)
            {
                if stop_flag.load(Ordering::Relaxed) {
                    return;
                }
                tokio::time::sleep(Duration::from_millis(1)).await;
            }

            active_requests.fetch_add(1, Ordering::Relaxed);

            let result = http_engine::fetch(
                &client,
                &target,
                &path,
                &method,
                data.as_deref()
            ).await;

            stats.total_requests.fetch_add(1, Ordering::Relaxed);

            if let Ok(Some(resp)) = result {
                if resp.status == 429 {
                    stats.total_429.fetch_add(1, Ordering::Relaxed);
                }
                if resp.status == 403 {
                    stats.total_403.fetch_add(1, Ordering::Relaxed);
                }

                stats.total_latency
                    .fetch_add(resp.latency_ms as u64, Ordering::Relaxed);

                {
                    let mut guard = rate_detector.lock().await;
                    guard.record(resp.status);
                }

                if let Some(finding) = analyzer::analyze(&resp, &baseline) {
					 pb.println(format!(
						"[+] {} (status: {}, {} bytes)",
						finding.path,
						finding.status,
						finding.content_length
					));
                    let mut guard = results.lock().await;
                    guard
                        .entry(finding.status)
                        .or_insert_with(HashSet::new)
                        .insert(finding.path.clone());
                }
            }

            let prev = counter.fetch_add(1, Ordering::Relaxed);
            pb.set_position((prev + 1) as u64);

            active_requests.fetch_sub(1, Ordering::Relaxed);
        }
    });

    handles.push(handle);
}

// esperar a todos los workers
for h in handles {
    let _ = h.await;
}

    pb.finish_with_message("Completed");
	println!("\n========== Clean Results ==========\n");

	let guard = results.lock().await;

	// Orden de prioridad manual
	let priority = vec![
		200, 201, 204,
		301, 302,
		401, 403,
		405,
		500, 502, 503,
	];

	for status in &priority {
		if let Some(paths) = guard.get(status) {
			if !paths.is_empty() {
				println!("--- Status {} ---", status);

				let mut sorted: Vec<String> = paths.iter().cloned().collect();
				sorted.sort(); // 🔥 orden alfabético

				for p in sorted {
					println!("{}", p);
				}

				println!();
			}
		}
	}

	// Mostrar cualquier otro status no listado
	for (status, paths) in guard.iter() {
		if !priority.contains(status) {
			println!("--- Status {} ---", status);

			let mut sorted: Vec<String> = paths.iter().cloned().collect();
			sorted.sort();

			for p in sorted {
				println!("{}", p);
			}

			println!();
		}
	}
    Ok(())
}
