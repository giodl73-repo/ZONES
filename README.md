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
```

The seed report runs a tiny four-county fixture through the first plan evaluator.
It is not a real proposal; it proves the scoring contract and RLINE dependency
shape. The evaluate-plan command runs the same contract from a JSON input file,
checks that the named source manifest matches, and then scores the plan. The
detail variant also emits per-unit error rows and propagated caveats. The source
report validates the first committed source manifest and summarizes which source
categories are currently covered. Generated evaluation artifacts are written
under `target/` by default and are intentionally not committed.

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
