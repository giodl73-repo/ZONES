# ZONES

**Time-zone redistricting along real civic boundaries.**

**Clock borders should be explainable in sunlight, geography, and civic boundaries.**

ZONES is a civic-design and optimization project for proposing better time-zone
maps. It treats counties, states, provinces, or similar administrative units as
graph nodes, scores how far each unit's clock is from local solar time, and then
searches for contiguous zone plans that reduce avoidable clock error without
cutting across recognizable government boundaries.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

## Infrastructure 2.0 family

ZONES is the civic-boundary member of a shared evidence-first family:

```text
PUBLIC SOURCES → CORPUS → SCORE → SERVICE PROMISE → GAP MAP
                                                     ↓
                                      CONCEPT → REVIEW → DESIGN
```

| Lane | Repositories |
|------|--------------|
| Movement | [ROUTE](https://github.com/giodl73-repo/ROUTE), [GAUGE](https://github.com/giodl73-repo/GAUGE), [TARMAC](https://github.com/giodl73-repo/TARMAC), [HARBOR](https://github.com/giodl73-repo/HARBOR) |
| Lifelines | [PYLON](https://github.com/giodl73-repo/PYLON), [PACKET](https://github.com/giodl73-repo/PACKET), [BASIN](https://github.com/giodl73-repo/BASIN), [DRAIN](https://github.com/giodl73-repo/DRAIN) |
| Public access | [SHIELD](https://github.com/giodl73-repo/SHIELD), [SLATE](https://github.com/giodl73-repo/SLATE) |
| Civic boundaries | [ZONES](https://github.com/giodl73-repo/ZONES) |

The family shares evidence labels, explicit scale and demand bases, T1–T4
service promises where meaningful, adversarial review, and acceptance of a
rigorous null result. Each repository owns its domain semantics and safety
boundary.

## Use ZONES

ZONES is public and open to use as a reference model, civic-boundary scoring
system, research corpus, map-generation fixture, or local adaptation starting
point.

If you want to apply it to a state, province, country, county-level plan,
current-law review, or civil-time policy question, start with
[`docs/adoption/README.md`](docs/adoption/README.md). It lays out safe reuse,
first adaptation steps, reviewer targets, and claim boundaries.

## Portfolio reuse posture

ZONES is open for reference review and bounded local adaptation, but it is
intentionally a specialist civic-policy product rather than a shared portfolio
dependency. The draft adoption guide describes how to reproduce the method
without turning an exploratory score into legal or operational advice; it is
not a versioned crate, schema, or dataset contract, and no downstream manifest
currently records direct ZONES adoption.

ZONES keeps civil-time scoring, source gates, legal-assignment evidence,
candidate plans, and publication claims local. Reusable graph and boundary
primitives flow through RPLAN and RLINE. A future direct dependency requires an
explicit versioned surface, a pinned downstream manifest, and consumer-owned
compatibility and claim-boundary tests.

## Why ZONES

Modern time-zone borders are a mix of geography, law, history, politics, and
accident. Some places keep clock time that is visibly misaligned with local
sunrise, noon, and sunset. ZONES makes that mismatch measurable, then asks a
redistricting-style question: what zone boundaries would be more accurate if
they had to follow state, county, or equivalent boundaries?

The transferable principle is: **optimize a public boundary against measurable
error without pretending the metric is the whole policy decision.**

## Method

- Build an adjacency graph from accepted boundary units.
- Attach each unit's solar-time offset, population, jurisdiction, and source
  provenance.
- Evaluate existing and proposed zone plans for contiguity, boundary cuts, and
  weighted local-time error.
- Search candidate plans with reusable graph, statistics, and optimization
  kernels from RLINE.
- Publish auditable map, score, and tradeoff packets rather than one magic map.

## Specs And Roles

- [`docs/specs/SPEC_INDEX.md`](docs/specs/SPEC_INDEX.md) tracks the foundation
  spec and dependency boundaries.
- [`.roles/ROLE.md`](.roles/ROLE.md) defines the review panel for civil-time
  policy, boundary data, solar-time scoring, graph optimization, and public maps.
- [`research/RESEARCH.md`](research/RESEARCH.md) tracks what is known, unknown,
  and publishable as the evidence base grows.

## Quick start

Start with the fixture, inspect its evidence gates, then generate the first
candidate comparison:

```powershell
cargo run -p zones-cli -- seed-report
cargo run -p zones-cli -- source-gate-report
cargo run -p zones-cli -- compare-offset-candidates
```

## Command reference

```powershell
cargo run -p zones-cli -- seed-report
cargo run -p zones-cli -- evaluate-plan
cargo run -p zones-cli -- evaluate-plan-detail
cargo run -p zones-cli -- write-evaluation
cargo run -p zones-cli -- source-report
cargo run -p zones-cli -- source-ref-report
cargo run -p zones-cli -- source-gate-report
cargo run -p zones-cli -- rplan-context-report
cargo run -p zones-cli -- county-assignment-report
cargo run -p zones-cli -- geometry-reconciliation-report
cargo run -p zones-cli -- representative-point-report
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
cargo run -p zones-cli -- compare-offset-candidates
cargo run -p zones-cli -- evaluate-plan data/plan-inputs/us-county-smoke.json
cargo run -p zones-cli -- evaluate-plan data/plan-inputs/us-county-baseline-smoke.json
cargo run -p zones-cli -- source-ref-report data/plan-inputs/us-county-smoke.json
```

The seed report runs a tiny four-county fixture through the first plan evaluator.
It is not a real proposal; it proves the scoring contract and RLINE dependency
shape. The evaluate-plan command runs the same contract from a JSON input file,
checks that the named source manifest and zone catalog match, and then scores
the plan. The detail variant also emits per-unit error rows and propagated
caveats. The source report validates the first committed source manifest and
summarizes which source categories are currently covered. Generated evaluation
artifacts are written under `target/` by default and are intentionally not
committed. `write-evaluation` writes a full JSON packet, a per-unit CSV score
table, and a per-zone summary CSV. When a plan input includes
`reference_assignment`, reports include moved unit and moved population counts
against that reference.

Plan inputs carry an explicit scenario label and kind, such as `current-law`,
`historical-law`, `proposed-scenario`, or `analytic-counterfactual`.
Current-law and historical-law scenarios must cite an authority source from the
source manifest.

The seed zone catalog proves ZONES can represent whole-hour, half-hour, and
45-minute offsets. It is not a complete list of legal time zones.

`data/temporal-fixtures/non-us-pilot.json` is a synthetic global/temporal model
fixture. It validates the serializable contract for jurisdictions, boundary
units, graph versions, time-zone regimes, offset rules, DST deltas, and
evaluation contexts without claiming to be a legal dataset.
`data/source-limitation-matrix/global-source-claims.json` records which source
families can support offset-history, legal-boundary, administrative-boundary,
metadata, population, and representative-point claims.
`data/source-gates/us-foundation-source-gate.json` records the source-gate
policy for the US baseline: acquisition mode, cache posture, rights posture,
expected artifact, hash requirement, and gate notes for every source in the US
foundation manifest.
`data/rplan-contexts/us-county-smoke-rplan-context.json` is the first committed
RPLAN county-context smoke fixture. It proves the target shape for GEOID-sorted
county units, adjacency, populations, geometry source context, source hashes, and
context-hash validation before a national county context is generated.
`data/legal-assignments/us-county-smoke-current-law.json` is the first committed
current-law assignment evidence smoke fixture. It carries legal source, clause,
DOT geometry source, status, and caveats per county-shaped unit; the report stays
not ready while assignments are placeholders.
`data/representative-points/us-county-smoke-gazetteer.json` is the first
committed representative-point smoke fixture. It proves the solar-offset
derivation from longitude and keeps the method exploratory until
population-center or stronger point evidence is available.
`data/module-boundaries/zones-rplan-rline.json` records which responsibilities
belong in ZONES, RPLAN, RLINE, and BISECT reference material.

`offset-fit` compares current assigned offsets against the nearest whole-hour,
half-hour, and quarter-hour offset for each unit. It also reports DST-shifted
clock error with a configurable `--dst-delta-minutes` value.
`write-offset-fit` writes the same report plus a ranked per-unit CSV under
`target/zones/` by default.
`write-offset-maps` uses the Rust SVG renderer to write schematic maps for
current standard time, current DST-period clock time, and best whole-hour,
half-hour, and quarter-hour options.
`write-offset-atlas` writes those maps plus a local `index.html` comparison
page.
`write-offset-geojson` exports the same offset-fit fields as GeoJSON for GIS
inspection and later boundary joins.
`write-offset-candidate-plan` materializes nearest-offset alternatives as real
plan inputs for whole-hour, half-hour, or quarter-hour grids.
`write-offset-candidate-maps` writes a full local map packet for current law and
the whole-hour, half-hour, and quarter-hour candidate grids. Each option gets a
plan input, offset-fit JSON, GeoJSON, SVG maps, and an atlas page under
`target/zones/` by default; the packet index keeps the recommendation gate
closed and includes a comparison-summary table with baseline and candidate
weighted-error, moved-unit, and moved-population metrics plus inline SVG
previews for quick visual inspection. Pass
`--geojson <FeatureCollection>` to join boundary geometry before
rendering, so SVG maps draw filled unit polygons instead of point markers when
geometry is available.
When plan units include `map_geometry` polygons or multipolygons, GeoJSON emits
those shapes. If only `map_point` coordinates are present, it emits
representative points; otherwise it falls back to schematic points derived from
solar offset.
`attach-geojson-geometries` joins a boundary FeatureCollection into a plan input
by unit id, which lets county/state boundary exports from BISECT or RPLAN feed
the same offset-fit reports and maps.
`data/plan-inputs/seed-plan-map-points.json` is the seed fixture for
coordinate-aware map rendering.
`data/plan-inputs/us-county-smoke.json` is the first county-shaped intake smoke
fixture. It uses GEOID-shaped ids and explicit caveats to prove the evaluator
contract without claiming a source-derived national county scorecard. Its
per-unit `source_refs` fields make boundary, point, population, time-zone
assignment, geometry, and split-county caveats machine-readable.
`data/plan-inputs/us-county-baseline-smoke.json` is assembled from the committed
RPLAN context, current-law assignment evidence, representative-point fixture, and
zone catalog. It proves the baseline input assembly path while remaining a smoke
fixture because legal assignments and point methods are not strong-claim ready.
`data/plan-inputs/us-county-baseline-seed.json` is the first Pulse 03
source-derived seed input. It replaces approximate point and population
placeholders with Census Gazetteer internal points and 2024 county population
estimates for the four seed units, and it replaces placeholder legal assignment
rows with 49 CFR clause-cited seed evidence for Alabama and Florida counties.
It also replaces smoke adjacency with TIGER-derived adjacency for the selected
four-county set; those counties have no boundary adjacencies among themselves, so
the seed report is intentionally disconnected.
`data/boundaries/us-county-seed-boundaries.geojson` is a small generalized
Census TIGERweb county-boundary fixture for those four GEOIDs, used to generate
boundary-backed local candidate map packets without committing raw national GIS
cache data.
`data/geometry-reconciliation/us-county-seed-dot-reconciliation.json` tracks DOT
geometry reconciliation as a separate publication gate; the seed rows are
polygon-reconciled, with a source-precision caveat on Baldwin County.
`data/source-endpoints/dot-time-zones-arcgis.json` records the BTS/NTAD Time
Zones ArcGIS FeatureServer endpoint and query shape for the next reconciliation
step without committing raw geometry.
`source-ref-report` summarizes that per-unit source-reference coverage, missing
reference counts, and caveat coverage so smoke fixtures and future county intakes
can be checked before publishing scores.
`compare-offset-candidates` writes a candidate comparison report for nearest
whole-hour, half-hour, and quarter-hour offset grids. It reports score deltas,
moved units/population by zone id, caveats, and `recommendation_gate_closed`.
For the source-derived seed candidate map packet:

```powershell
cargo run -p zones-cli -- write-offset-candidate-maps data/plan-inputs/us-county-baseline-seed.json --output-dir target/zones/us-county-baseline-seed/candidate-maps
cargo run -p zones-cli -- write-offset-candidate-maps data/plan-inputs/us-county-baseline-seed.json --geojson data/boundaries/us-county-seed-boundaries.geojson --require-all-units --output-dir target/zones/us-county-baseline-seed/candidate-boundary-maps
cargo run -p zones-cli -- write-offset-candidate-maps data/plan-inputs/seed-plan.json --geojson data/boundaries/seed-boundaries.geojson --require-all-units --output-dir target/zones/seed-boundary-candidate-maps
```

For a local full-national exploratory county map packet, fetch generalized Census
TIGERweb county boundaries into ignored `target/` artifacts and render the same
candidate-map packet shape:

```powershell
.\scripts\write-national-exploratory-county-maps.ps1
```

This packet is for visual inspection only: it uses equal unit weights,
longitude-derived analytic offsets, generalized county geometry, and an empty
adjacency graph. It is not current law, a legal assignment map, a
population-weighted scorecard, or a recommendation.

The baseline smoke scorecard can be generated under ignored output paths with:

```powershell
cargo run -p zones-cli -- write-evaluation data/plan-inputs/us-county-baseline-smoke.json --output target/zones/us-county-baseline-smoke/evaluation.json --unit-scores-csv target/zones/us-county-baseline-smoke/unit-scores.csv --zone-summaries-csv target/zones/us-county-baseline-smoke/zone-summaries.csv
cargo run -p zones-cli -- write-offset-fit data/plan-inputs/us-county-baseline-smoke.json --output target/zones/us-county-baseline-smoke/offset-fit.json --unit-scores-csv target/zones/us-county-baseline-smoke/offset-fit-units.csv
cargo run -p zones-cli -- write-offset-geojson data/plan-inputs/us-county-baseline-smoke.json --output target/zones/us-county-baseline-smoke/offset-fit.geojson
cargo run -p zones-cli -- write-offset-maps data/plan-inputs/us-county-baseline-smoke.json --output-dir target/zones/us-county-baseline-smoke/maps
cargo run -p zones-cli -- write-offset-atlas data/plan-inputs/us-county-baseline-smoke.json --output-dir target/zones/us-county-baseline-smoke/atlas
cargo run -p zones-cli -- write-offset-candidate-plan data/plan-inputs/us-county-baseline-smoke.json --output target/zones/us-county-baseline-smoke/offset-candidate-plan.json
```

## Non-goals

- ZONES is not a legal time-zone authority.
- ZONES does not give operational scheduling advice.
- ZONES does not optimize only for solar accuracy; administrative stability,
  commerce, travel, public preference, and implementation cost are explicit
  tradeoffs.
- ZONES does not put product-specific scoring into RLINE.

## Dependencies

ZONES depends on `rplan-core` for portable legal-boundary unit graph/context
contracts and `rgraph-core` for boundary and contiguity metrics. Future waves
should consider `ropt-core` for candidate search, FLETCH for source acquisition,
MDLOOM for report validation, MDCROP/MDPORT for portable evidence records, and ROLES
for domain review panels. BISECT remains the reference implementation for proven
Census/TIGER/GEOID handling; reusable boundary packages should flow through
RPLAN rather than through BISECT application internals.

## License

MIT. See [`LICENSE`](LICENSE).
