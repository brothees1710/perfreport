# PerfReport — IS2083 Lab 3

Rust migration of a Windows-only C++ performance reporting utility.

## What it reports
- Logical processors
- Total and available memory, and memory utilization
- System uptime
- Disk capacity and utilization
- An efficiency score with GOOD / FAIR / POOR rating

## Rust
```bash
cargo run
cargo test
```

## Docker
```bash
docker build -t perfreport .
docker run --rm perfreport
```

## GitHub Actions
The workflow in `.github/workflows/ci.yml` checks formatting, lints, builds, tests, and validates Docker image builds on pushes to `main`.
