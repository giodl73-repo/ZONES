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
- `git diff --check`

## Status

Ready.
