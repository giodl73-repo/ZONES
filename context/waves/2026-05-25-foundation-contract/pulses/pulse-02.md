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

- Select the first county source path: Census TIGER/Line or an RPLAN-produced
  county context, plus the source manifest fields needed to cite it.
- Define how current legal time-zone assignments will be represented at county
  level, including split-county caveats.
- Add or document a small county smoke fixture that uses real GEOID-shaped ids
  without committing raw national GIS cache bytes.
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
- `cargo run -p zones-cli -- offset-fit`
- `cargo run -p zones-cli -- write-offset-candidate-plan`
- `git diff --check`

## Status

Ready.
