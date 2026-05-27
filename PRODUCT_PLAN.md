# ZONES Product Plan

## Thesis

Time-zone reform can be made concrete by scoring the mismatch between legal
clock time and local solar time, then constraining proposed changes to familiar
government boundaries. ZONES should produce defensible alternatives, not a
single vibes-based map.

## Product surface

The first useful artifact is a command-line evaluator that compares a current
time-zone plan against one or more candidate plans and emits:

- weighted local-time error,
- disconnected-zone failures,
- boundary-cut counts,
- units moved across current legal zones,
- population moved across current legal zones,
- state/county boundary respect,
- source and assumption notes,
- JSON and per-unit/per-zone CSV evaluation artifacts under ignored output paths.

## Waves

### Wave 1: Foundation Contract

Create the repo, evaluator contract, seed fixture, and validation path. Prove
that RPLAN supplies the legal-boundary unit graph/context contract, RLINE
supplies graph boundary/connectivity metrics, and ZONES owns time-zone domain
policy.

### Wave 2: Source Inventory

Inventory US county/state boundaries, time-zone legal boundaries, longitude,
population, and international-administrative equivalents. Decide which sources
are fetched through FLETCH and which are referenced only as documented inputs.
Reuse BISECT's proven Census/TIGER/GEOID conventions where applicable, but route
the portable boundary contract through RPLAN contexts.

### Wave 3: US County Baseline

Build a reproducible US county graph with current legal time-zone assignments,
solar offset estimates, and first current-plan scorecards.

### Wave 4: Temporal Global Model

Generalize the US baseline into country-neutral, time-versioned boundary units,
time-zone regimes, offset rules, DST transitions, and source vintages. Prove the
model on one non-US pilot before broad international claims.

### Wave 5: Candidate Search

Add constrained candidate generation using RLINE graph/stat kernels and
product-owned scoring. Compare strict state-boundary, county-boundary, and
minimum-change plan families.

### Wave 6: Publication Packets

Publish map/report packets with tradeoff tables, source receipts, and review
findings. Keep any final recommendations clearly separated from score output.

## Non-goals

- No real-world scheduling, legal, or emergency guidance.
- No raw GIS cache bytes committed to the repo.
- No dependency from RLINE back into ZONES.
- No broad international expansion until the US county baseline and a non-US
  temporal pilot prove the schema.
- No hidden political objective function; all weights must be named and auditable.
- No final reform recommendation until source, score, civil-time, and public-map
  review gates pass.

## Initial validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p zones-cli -- seed-report
cargo run -p zones-cli -- evaluate-plan
cargo run -p zones-cli -- evaluate-plan-detail
cargo run -p zones-cli -- write-evaluation
cargo run -p zones-cli -- source-report
cargo run -p zones-cli -- source-ref-report
cargo run -p zones-cli -- source-gate-report
cargo run -p zones-cli -- rplan-context-report
cargo run -p zones-cli -- zone-catalog-report
cargo run -p zones-cli -- temporal-dataset-report
cargo run -p zones-cli -- source-limitation-report
cargo run -p zones-cli -- module-boundary-report
cargo run -p zones-cli -- offset-fit
cargo run -p zones-cli -- write-offset-fit
cargo run -p zones-cli -- write-offset-maps
cargo run -p zones-cli -- write-offset-atlas
cargo run -p zones-cli -- write-offset-geojson
cargo run -p zones-cli -- write-offset-candidate-plan
git diff --check
```
