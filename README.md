# ZONES

**Time-zone redistricting along real civic boundaries.**

ZONES is a civic-design and optimization project for proposing better time-zone
maps. It treats counties, states, provinces, or similar administrative units as
graph nodes, scores how far each unit's clock is from local solar time, and then
searches for contiguous zone plans that reduce avoidable clock error without
cutting across recognizable government boundaries.

**Series:** Applied Systems.

## Why ZONES

Modern time-zone borders are a mix of geography, law, history, politics, and
accident. Some places keep clock time that is visibly misaligned with local
sunrise, noon, and sunset. ZONES makes that mismatch measurable, then asks a
redistricting-style question: what zone boundaries would be more accurate if
they had to follow state, county, or equivalent boundaries?

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

## First command

```powershell
cargo run -p zones-cli -- seed-report
cargo run -p zones-cli -- evaluate-plan
cargo run -p zones-cli -- evaluate-plan-detail
cargo run -p zones-cli -- write-evaluation
cargo run -p zones-cli -- source-report
cargo run -p zones-cli -- zone-catalog-report
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
When plan units include `map_geometry` polygons or multipolygons, GeoJSON emits
those shapes. If only `map_point` coordinates are present, it emits
representative points; otherwise it falls back to schematic points derived from
solar offset.
`attach-geojson-geometries` joins a boundary FeatureCollection into a plan input
by unit id, which lets county/state boundary exports from BISECT or RPLAN feed
the same offset-fit reports and maps.
`data/plan-inputs/seed-plan-map-points.json` is the seed fixture for
coordinate-aware map rendering.

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
PROOF for report validation, CROP/PEBBLE for portable evidence packs, and ROLES
for domain review panels. BISECT remains the reference implementation for proven
Census/TIGER/GEOID handling; reusable boundary packages should flow through
RPLAN rather than through BISECT application internals.

## License

MIT. See [`LICENSE`](LICENSE).
