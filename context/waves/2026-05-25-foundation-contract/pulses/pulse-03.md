# Pulse 03: US county seed graph

## Goal

Replace the Pulse 02 smoke fixtures with the first source-derived county graph
and current-plan baseline input.

## Starting point

Pulse 02 proved the shape of the intake path:

- `source-gate-report` for source/cache/rights posture,
- `rplan-context-report` for county context readiness,
- `county-assignment-report` for current-law assignment evidence readiness,
- `representative-point-report` for point and solar-offset method readiness,
- `source-ref-report` for per-unit source-reference coverage,
- `data/plan-inputs/us-county-baseline-smoke.json` for assembled baseline input,
- ignored `target/zones/us-county-baseline-smoke/` scorecard artifacts,
- role review that keeps publication and recommendation gates closed.

## Scope

- Produce or consume the first source-derived RPLAN county context.
- Replace placeholder population weights with source-derived county weights.
- Replace placeholder assignment rows with county-level current-law evidence
  rows that cite 49 CFR Part 71 and DOT geometry reconciliation notes.
- Keep Census internal points explicitly exploratory unless a stronger point
  method lands in this pulse.
- Generate a current-plan baseline input and scorecard artifacts under ignored
  output paths.

## Out of scope

- No recommendation language.
- No public scorecard promotion.
- No raw national GIS/cache bytes committed to git.
- No time-zone policy in RPLAN, RLINE, or BISECT.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p zones-cli -- source-gate-report`
- `cargo run -p zones-cli -- rplan-context-report <source-derived-context>`
- `cargo run -p zones-cli -- county-assignment-report <source-derived-assignments>`
- `cargo run -p zones-cli -- representative-point-report <source-derived-points>`
- `cargo run -p zones-cli -- source-ref-report <baseline-plan-input>`
- `cargo run -p zones-cli -- evaluate-plan <baseline-plan-input>`
- `cargo run -p zones-cli -- evaluate-plan-detail <baseline-plan-input>`
- `git diff --check`

## Status

In progress.

## Pulse log

- Added source-derived four-county seed fixtures from Census public data:
  `data/rplan-contexts/us-county-seed-rplan-context.json`,
  `data/representative-points/us-county-seed-gazetteer.json`, and
  `data/plan-inputs/us-county-baseline-seed.json`.
- The seed replaces placeholder population weights and approximate coordinates
  with Census 2024 county population estimates and Census 2024 Gazetteer internal
  points for the four county-shaped rows.
- The seed still uses smoke adjacency and placeholder current-law assignments,
  so it remains blocked from publication.
