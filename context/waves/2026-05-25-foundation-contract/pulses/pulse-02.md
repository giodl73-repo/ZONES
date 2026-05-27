# Pulse 02: US county source intake

## Goal

Create the first reproducible path from documented US boundary and time-zone
sources into a county-level ZONES plan input.

## Starting point

The repo already has the seed evaluator, source manifest, zone catalog, temporal
fixture, source-limitation matrix, module-boundary contract, offset-fit reports,
map/GeoJSON outputs, geometry joins, and candidate-plan generation. These remain
fixture-level until a county source intake can produce auditable inputs.

## Scope

- Select the first county source path: prefer an RPLAN-produced county context
  backed by Census TIGER/Line and Gazetteer sources, with ZONES retaining the
  time-zone assignment and scoring fields.
- Define current legal time-zone assignments as county-level records that cite
  49 CFR Part 71 and/or reconciled DOT map evidence, with split-county caveats.
- Add a small county smoke fixture that uses GEOID-shaped ids without committing
  raw national GIS cache bytes.
- Keep ZONES policy fields in ZONES; keep portable boundary graph/context shape
  aligned with RPLAN.

## Out of scope

- No full national scorecard yet.
- No final recommendations.
- No raw GIS cache committed to git.
- No hidden county-to-zone inference without a source/caveat field.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p zones-cli -- evaluate-plan`
- `cargo run -p zones-cli -- evaluate-plan-detail`
- `cargo run -p zones-cli -- source-report`
- `cargo run -p zones-cli -- source-ref-report data/plan-inputs/us-county-smoke.json`
- `cargo run -p zones-cli -- source-gate-report`
- `cargo run -p zones-cli -- offset-fit`
- `cargo run -p zones-cli -- write-offset-candidate-plan`
- `git diff --check`

## Status

In progress.

## Pulse log

- Added `data/plan-inputs/us-county-smoke.json` as the first county-shaped
  evaluator input. It intentionally uses GEOID-shaped ids, placeholder weights,
  and explicit caveats rather than claiming source-derived legal assignments.
- Extended the US foundation source manifest with the selected Census population
  source and DOT geospatial map layer needed by county intake.
- Added per-unit `source_refs` so county-shaped inputs can cite boundary,
  representative point, population, legal assignment, and time-zone geometry
  sources, with split-county caveats carried into detailed score output.
- Added `source-ref-report` to summarize source-reference coverage and caveat
  counts for county-shaped inputs before they can graduate from smoke fixture to
  publishable scorecard.
- Added missing-reference counts and `publishable_source_ref_coverage` to make
  source-reference QA usable as a pre-publication gate.
- Added `data/source-gates/us-foundation-source-gate.json` and
  `source-gate-report` so every source has acquisition mode, cache posture,
  rights posture, expected artifact, hash requirement, and gate notes before
  broad ingestion.
