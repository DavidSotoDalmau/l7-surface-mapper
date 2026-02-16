
#[derive(Debug, Clone)]
pub enum WafKind {
    Waf,
    Cdn,
    ReverseProxy,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct WafAnalysis {
    pub detected: bool,
    pub vendor: Option<String>,
    pub kind: WafKind,
    pub confidence: u8,
    pub signals: Vec<String>,
}

impl WafAnalysis {
    pub fn new() -> Self {
        Self {
            detected: false,
            vendor: None,
            kind: WafKind::Unknown,
            confidence: 0,
            signals: Vec::new(),
        }
    }

    pub fn add_signal(&mut self, vendor: &str, kind: WafKind, score: u8, signal: &str) {
        self.detected = true;
        self.vendor = Some(vendor.to_string());
        self.kind = kind;
        self.confidence = (self.confidence + score).min(100);
        self.signals.push(signal.to_string());
    }
}
