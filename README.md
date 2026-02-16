# L7 Surface Mapper

High-performance HTTP endpoint and parameter discovery tool written in
Rust.

Built on **Hyper + Tokio** for maximum throughput and minimal overhead.

Designed for authorized security testing, red team exercises and lab
environments.

------------------------------------------------------------------------

## 🚀 Features

-   ⚡ Hyper-based HTTP engine (no reqwest overhead)
-   🔥 High throughput (70k+ req/s local benchmark)
-   🎯 Endpoint fuzzing
-   🎯 Parameter fuzzing (GET & POST)
-   🔁 `FUZZ` placeholder support in:
    -   URL
    -   Query string
    -   POST body
-   🧠 Baseline response filtering
-   📉 Basic rate-limit detection
-   📊 Concurrent progress bar
-   🛠 Configurable HTTP methods
-   📦 Optimized release builds (LTO + strip + panic abort)
-   🧵 Async architecture (Tokio)

------------------------------------------------------------------------

## 📦 Installation

Clone the repository:

``` bash
git clone https://github.com/youruser/l7-surface-mapper.git
cd l7-surface-mapper
```

Build release:

``` bash
cargo build --release
```

Binary will be available at:

    target/release/l7_surface_mapper

Or download precompiled binaries from the **Releases** section.

------------------------------------------------------------------------

## 🛠 Basic Usage

``` bash
l7_surface_mapper \
  --target http://127.0.0.1:8080 \
  --wordlist wordlist.txt \
  --concurrency 500
```

------------------------------------------------------------------------

# ⚙️ Parameters

  Flag              Description
  ----------------- ---------------------------------
  `--target`        Target base URL
  `--wordlist`      Path to wordlist file
  `--concurrency`   Number of concurrent requests
  `--method`        HTTP method (default: GET)
  `--data`          Request body (for POST/PUT/etc)

------------------------------------------------------------------------

# 🔁 FUZZ Placeholder Logic

The tool supports intelligent `FUZZ` replacement.

### Case 1 -- FUZZ in URL

``` bash
--target http://127.0.0.1:8080/FUZZ
```

Result:

    GET /admin
    GET /login
    GET /api

------------------------------------------------------------------------

### Case 2 -- FUZZ in Query Parameter

``` bash
--target http://127.0.0.1:8080/?param=FUZZ
```

Result:

    GET /?param=admin
    GET /?param=login
    GET /?param=api

------------------------------------------------------------------------

### Case 3 -- FUZZ in POST Body

``` bash
--method POST \
--data "username=FUZZ&password=123"
```

Result:

    POST /  (body: username=admin&password=123)
    POST /  (body: username=login&password=123)

------------------------------------------------------------------------

### Case 4 -- FUZZ in Both URL and Body

``` bash
--target http://127.0.0.1:8080/FUZZ \
--method POST \
--data "param=FUZZ"
```

Both URL and body occurrences are replaced.

------------------------------------------------------------------------

### Case 5 -- No FUZZ Present

If `FUZZ` is not present in URL or body:

-   Wordlist entries are appended to the URL path.

``` bash
--target http://127.0.0.1:8080
```

Result:

    GET /admin
    GET /login
    GET /api

------------------------------------------------------------------------

# 🔥 HTTP Methods

Supported methods:

-   GET (default)
-   POST
-   PUT
-   DELETE
-   HEAD
-   OPTIONS

Example:

``` bash
--method HEAD
```

------------------------------------------------------------------------

# 🧠 Baseline Filtering

Before fuzzing begins, the tool performs a baseline request using a
random path.

Responses matching the baseline (status + content-length) are ignored to
reduce noise.

------------------------------------------------------------------------

# 📊 Rate Limit Detection

The tool maintains a sliding window of response codes.

If excessive `429` responses are detected, a warning is displayed.

------------------------------------------------------------------------

# 🧪 Benchmarks

Local testing results (body disabled):

  Target                 Throughput
  ---------------------- ----------------
  Azure Web App          \~460 req/s
  httpbin                \~1470 req/s
  Docker nginx (local)   \~73,000 req/s

------------------------------------------------------------------------

# 🏗 Architecture

-   Hyper HTTP client
-   Rustls TLS
-   Tokio async runtime
-   Zero-body download mode
-   Atomic progress tracking
-   Sliding window rate-limit detection

------------------------------------------------------------------------

# 📦 Releases

Releases are built automatically using GitHub Actions.

Artifacts include:

-   Windows x86_64
-   Linux x86_64
-   macOS x86_64

Each release is versioned:

    l7_surface_mapper-vX.Y.Z-platform

------------------------------------------------------------------------

# ⚠️ Disclaimer

This tool is intended for:

-   Authorized penetration testing
-   Red team exercises
-   Lab environments
-   CTF practice

Always obtain proper authorization before testing any system.

The author assumes no responsibility for misuse.

------------------------------------------------------------------------

# 📄 License

MIT License
