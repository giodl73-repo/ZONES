# Pulse 05: PITFALL use-case integration

## Result

Added use-case-first fields and retained policy-test coverage for ZONES' three
open candidate-recommendation, national-baseline, and solar-objective pitfalls.

## Covered pitfalls

- `ZONES-PF-01` keeps lower-error candidate maps and polished packets from
  becoming preferred, best, or ready-to-adopt time-zone recommendations before
  review gates close.
- `ZONES-PF-02` keeps the four-county source-derived seed and boundary-backed
  local map packets from becoming a national current-law scorecard.
- `ZONES-PF-03` keeps solar-time error visible as one metric rather than the
  whole policy objective before DOT convenience, disruption, legal-process, and
  implementation tradeoffs are available.

## Validation

```powershell
cargo fmt --check
cargo test -p zones-cli --test pitfall_policy
cargo test --workspace
cargo run -p zones-cli -- --help
```
