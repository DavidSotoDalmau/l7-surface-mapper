# L7 Surface Mapper

High-performance HTTP endpoint and parameter discovery tool written in Rust.

Built with **Hyper** for maximum throughput and minimal overhead.

---

## 🚀 Features

- ⚡ Hyper-based HTTP engine (no reqwest overhead)
- 🔥 High throughput (70k+ req/s local benchmark)
- 🎯 Endpoint fuzzing
- 🎯 Parameter fuzzing (GET & POST)
- 🔁 `FUZZ` placeholder support in:
  - URL
  - Query string
  - POST body
- 📊 Progress bar
- 🧠 Baseline response filtering
- 📉 Basic rate limit detection
- 🧵 Concurrent async architecture (Tokio)
- 🛠 Configurable HTTP methods:
  - GET
  - POST
  - HEAD
  - PUT
  - DELETE
  - OPTIONS
- 📦 Optimized release build (LTO + strip + panic abort)

---

## 📦 Installation

Clone the repository:

```bash
git clone https://github.com/youruser/l7-surface-mapper.git
cd l7-surface-mapper
