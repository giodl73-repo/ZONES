# Pulse 04: Candidate scoring report

## Goal

Compare current and candidate plan families over the source-derived four-county
seed with explicit tradeoff weights, without recommendation language.

## Starting point

Pulse 03 produced a source-derived four-county seed with:

- source-gated Census and DOT inputs,
- TIGER-derived adjacency,
- Census 2024 population estimates,
- Census Gazetteer internal points,
- 49 CFR clause-cited current-law assignments,
- BTS/NTAD polygon reconciliation,
- ignored seed scorecard artifacts.

The seed remains non-publishable as a national baseline because the scope is four
counties and the representative-point method is exploratory.

## Scope

- Compare the current assignment against one or more generated candidate plan
  inputs.
- Keep weights explicit and auditable.
- Report solar fit, moved population, moved units, contiguity, source caveats,
  and point/geometry method caveats.
- Keep candidate outputs under ignored paths unless a later publication packet
  explicitly promotes them.

## Out of scope

- No preferred map.
- No enactment or scheduling advice.
- No public scorecard promotion.
- No national baseline claim.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p zones-cli -- evaluate-plan data/plan-inputs/us-county-baseline-seed.json`
- `cargo run -p zones-cli -- source-ref-report data/plan-inputs/us-county-baseline-seed.json`
- `cargo run -p zones-cli -- geometry-reconciliation-report`
- `cargo run -p zones-cli -- compare-offset-candidates data/plan-inputs/us-county-baseline-seed.json --output target/zones/us-county-baseline-seed/candidate-comparison.json`
- `cargo run -p zones-cli -- write-offset-candidate-maps data/plan-inputs/us-county-baseline-seed.json --output-dir target/zones/us-county-baseline-seed/candidate-maps`
- `git diff --check`

## Status

In progress.

## Pulse log

- Added `compare-offset-candidates`, which compares the current seed assignment
  against nearest whole-hour, half-hour, and quarter-hour analytic
  counterfactuals.
- The comparison report is written under ignored
  `target/zones/us-county-baseline-seed/candidate-comparison.json`.
- Moved-unit and moved-population counts compare zone ids, not internal zone
  indexes, so equivalent whole-hour assignments do not count as moved.
- The report keeps `recommendation_gate_closed: true`; lower error deltas are
  measurements, not recommendations.
- Added `write-offset-candidate-maps`, which emits an ignored local map packet
  for the current-law seed and each generated offset-grid counterfactual. Each
  option includes the materialized plan input, offset-fit JSON, GeoJSON, SVG map
  set, and an atlas page, with the packet index explicitly keeping the
  recommendation gate closed.
