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
- `cargo run -p zones-cli -- write-offset-candidate-maps data/plan-inputs/us-county-baseline-seed.json --geojson data/boundaries/us-county-seed-boundaries.geojson --require-all-units --output-dir target/zones/us-county-baseline-seed/candidate-boundary-maps`
- `cargo run -p zones-cli -- write-offset-candidate-maps data/plan-inputs/seed-plan.json --geojson data/boundaries/seed-boundaries.geojson --require-all-units --output-dir target/zones/seed-boundary-candidate-maps`
- `.\scripts\write-national-exploratory-county-maps.ps1`
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
- Extended the candidate map packet path to accept an optional boundary
  GeoJSON join and render SVG polygons when plan geometry is present. This makes
  the option-map packet geometry-backed for fixtures with committed boundaries
  while the US county seed remains gated by source-derived county boundary
  availability.
- Added a small generalized Census TIGERweb county-boundary fixture for the four
  source-derived seed GEOIDs, so the seed candidate packet can be generated with
  boundary-backed SVG/GeoJSON maps while raw national GIS cache data stays out of
  git.
- Added `scripts/write-national-exploratory-county-maps.ps1`, which fetches
  generalized Census TIGERweb county boundaries into ignored `target/` artifacts
  and renders a full-national exploratory candidate map packet. The packet keeps
  the recommendation gate closed and is explicitly not current law, legal advice,
  a population-weighted scorecard, or a publication-ready national claim.
- Fixed candidate-plan assignment generation so units still point to the correct
  UTC offsets after candidate zones are sorted for deterministic output. The
  national exploratory packet now shows plausible option deltas instead of
  inflated comparison errors, and comparison caveats now follow the supplied
  scenario instead of hard-coding the four-county seed scope.
- Added a comparison-summary table to the candidate packet index so local review
  can see baseline and candidate weighted-error, moved-unit, and moved-population
  metrics without opening the JSON first. The table text keeps lower-error
  deltas framed as measurements, not recommendations.
