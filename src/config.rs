use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    #[arg(long)]
    pub target: String,

    #[arg(long)]
    pub wordlist: String,

    #[arg(long, default_value_t = 100)]
    pub concurrency: usize,
	
	#[arg(long, default_value = "GET")]
    pub method: String,
	
	#[arg(long)]
	pub data: Option<String>,
	
	// =============================
    // Infra-aware & Adaptive flags
    // =============================

    #[arg(long, default_value_t = false)]
    pub infra_aware: bool,

    #[arg(long, default_value_t = false)]
    pub infra_report: bool,

    #[arg(long, default_value_t = false)]
    pub stop_on_block: bool,

    #[arg(long, default_value = "balanced")]
    pub adaptive_mode: String,

    #[arg(long, default_value_t = 5000)]
    pub max_concurrency: usize,
	
	#[arg(long, default_value_t = false)]
	pub fingerprint_only: bool,
}
