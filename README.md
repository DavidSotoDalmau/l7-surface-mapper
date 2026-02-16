# L7 Surface Mapper

High-performance HTTP endpoint discovery tool written in Rust.

## Features

- Hyper-based HTTP engine
- HTTP/1.1 + HTTP/2
- Baseline filtering
- Rate limit detection
- Progress bar
- No body download mode
- High throughput (70k+ req/s local)

## Usage

```bash
l7_surface_mapper.exe \
  --target http://127.0.0.1:8080 \
  --wordlist wordlist.txt \
  --concurrency 500
