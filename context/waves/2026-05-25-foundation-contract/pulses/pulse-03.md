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
- Replaced placeholder current-law assignment rows for the four seed counties
  with clause-cited 49 CFR evidence:
  - Alabama seed counties cite `49 CFR 71.5(e); 49 CFR 71.6(a)` for central time.
  - Florida seed counties cite `49 CFR 71.4; 49 CFR 71.5(f)` for eastern time.
- Replaced smoke adjacency with TIGER-derived adjacency for the four seed
  counties. The selected counties have no boundary adjacencies among themselves,
  so the seed current-plan report correctly has zero boundary edges and
  disconnected zones.
- Added `data/geometry-reconciliation/us-county-seed-dot-reconciliation.json`
  and `geometry-reconciliation-report` to track DOT geometry reconciliation as a
  separate publication gate. All four seed rows are currently pending
  reconciliation, so the seed remains blocked from publication.
- Added `data/source-endpoints/dot-time-zones-arcgis.json` to record the BTS
  Time Zones ArcGIS FeatureServer endpoint, fields, zone rows, license posture,
  and raw-cache caveats for the next geometry join.
- Used the BTS/NTAD Time Zones FeatureServer to verify the four seed
  representative points against their expected zone polygons. The geometry gate
  now reports `representative_point_matched_count: 4`, but remains not ready
  because representative-point matching is not full county-polygon
  reconciliation.
