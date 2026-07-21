# Contributing

Keep ZONES source-backed, auditable, and explicit about the difference between
analytic counterfactuals and legal or policy recommendations.

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p zones-cli -- seed-report
```

Do not commit raw restricted GIS caches, credentials, local generated packets,
or claims that a candidate map is legally or politically adopted.
