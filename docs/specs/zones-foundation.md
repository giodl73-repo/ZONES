# ZONES Foundation Spec

## Purpose

ZONES evaluates time-zone plans over legal civic boundary units. The first
target is a US county-level graph, with later support for state, province,
municipality, or equivalent administrative units where source quality permits.
The data model must support global jurisdictions and historical/future versions
of boundaries, UTC offsets, and daylight-saving rules.

## Boundary Unit Contract

Foundation US inputs should be representable as an RPLAN context:

- `PlanUnitIndex.unit_kind`: `county` for the first US baseline.
- `PlanUnitIndex.unit_ids`: canonical five-digit county FIPS where applicable.
- `UnitGraph`: undirected adjacency over legal boundary units.
- `populations`: one non-negative population value per unit.
- `GeometryContext`: source id, CRS, and optional per-unit geometry hashes.
- `SourceHashes`: hashes or identifiers for source inputs.

ZONES adds country-neutral and time-versioned domain fields that RPLAN should
not own:

- jurisdiction code and administrative-unit level,
- validity interval for the boundary unit record,
- standard meridian offset in minutes from UTC,
- solar offset estimate in minutes from UTC,
- solar-error formula and sign convention,
- current legal time-zone assignment with validity interval,
- candidate time-zone assignment with scenario validity interval,
- time-zone offset in minutes from UTC,
- daylight saving rule or scenario with effective dates,
- civil-time source notes,
- scoring weights and caveats.

## Temporal Model

ZONES must treat time zones as regimes over time, not as one static map.

Core entities:

- **Boundary unit**: a legal or administrative area with an id, name,
  jurisdiction, unit level, geometry/source reference, and validity interval.
- **Boundary graph version**: adjacency for a boundary-unit universe at a
  specific source vintage or validity interval.
- **Time-zone regime**: an authority-backed or scenario-backed set of zone
  assignments and offset rules valid over a time interval.
- **Offset rule**: standard UTC offset plus optional daylight-saving transitions
  or fixed seasonal adjustments.
- **Scenario**: a candidate or counterfactual regime, explicitly marked as not
  current law unless sourced as enacted.

All scores must name the evaluation date or evaluation period. A county,
province, or municipality can change geometry, parent jurisdiction, or time-zone
assignment over time; historical comparisons must not silently reuse current
boundaries.

## Guiding Principles

ZONES should evaluate candidate plans against explicit principles rather than a
single hidden objective:

- **Solar fit**: civil noon should not drift far from local solar noon without a
  documented tradeoff.
- **Boundary legitimacy**: plans should follow recognizable legal boundaries
  such as states, counties, provinces, or municipalities unless a source and
  implementation note justifies otherwise.
- **Contiguity**: each zone should be geographically connected unless a known
  island, ferry, or administrative exception is recorded in the graph.
- **Minimum disruption**: plans should report how many people and units move
  from current legal zones.
- **Institutional feasibility**: ZONES reports scenarios and tradeoffs; it does
  not claim authority to enact a map.
- **Transparency**: every score must name its source vintage, weighting rule,
  daylight-saving assumption, and caveats.
- **Comparability**: current legal zones, standard-time-only scenarios,
  daylight-saving scenarios, and candidate reforms must be scored with the same
  formulas when compared.
- **Temporal honesty**: historical, current, and proposed maps must carry their
  own validity dates and source vintages.

## Scoring Contract

Every evaluated plan must report at least:

- unit count,
- zone count,
- boundary-cut count,
- per-zone contiguity status,
- population-weighted mean absolute solar-time error,
- maximum absolute solar-time error,
- moved unit count and moved population when a reference assignment is supplied.

The portable JSON plan-input contract contains:

- `input_id`;
- `source_manifest_id`;
- `units`, each with id, name, solar offset, and population;
- `adjacency`, as zero-based unit-index neighbors;
- `plan`, with zone specs and one assignment index per unit;
- `reference_assignment`, optionally naming the current/baseline zone index for
  each unit;
- `caveats`, for fixture/source limitations.

`evaluate-plan` must validate the referenced source manifest id before scoring.
This is intentionally shallow at the foundation layer: later county baselines
must add per-field source references, but even the seed path should reject a
plan file whose manifest identity does not match the supplied manifest.
`evaluate-plan-detail` emits the same aggregate report plus per-unit error rows,
input caveats, and source caveats so county baselines can publish both summary
and inspection artifacts from the same input.
`write-evaluation` writes that detailed packet plus per-unit and per-zone CSV
score tables to ignored `target/` paths by default; committed source fixtures
and ignored generated outputs must remain separate.

Foundation scoring uses minutes as the unit. A unit's **standard solar error**
is:

```text
abs(unit_solar_offset_minutes - zone_standard_offset_minutes)
```

where `unit_solar_offset_minutes` is the longitude-derived local mean solar
offset from UTC and `zone_standard_offset_minutes` is the standard offset active
for the named regime. Offsets are minute-precise, not whole-hour-only:
half-hour and 45-minute civil offsets must be valid. A daylight-saving scenario
uses the offset rule active on the evaluation date before computing
daylight-shifted error. In the US this is often `standard offset + 60` during
the DST period; globally this must not be hardcoded because daylight-saving
deltas and transition rules vary. Later specs may add seasonal sunrise/sunset
exposure metrics, but those must be named separately from standard solar error.

Zone catalogs are analysis inputs, not universal legal truth. A catalog should
name its source manifest, generation date, offsets in minutes, and caveats. The
seed catalog exists only to prove the representation of whole-hour, half-hour,
and 45-minute offsets.

Later reports may add:

- population moved from current legal zone,
- state or county split counts,
- commute/media-market disruption proxies,
- daylight saving assumptions,
- source freshness and uncertainty flags.

## Source Contract

The source inventory pulse must classify each source before ingestion:

- legal boundary source, starting with Census TIGER/Line county boundaries for
  the US baseline,
- legal time-zone boundary source, starting with DOT/NIST federal time-zone and
  DST references plus any geospatial boundary source selected for the baseline,
- longitude/centroid source, including whether centroids come from geometry,
  Census gazetteers, or another derived file,
- population source, starting with a Census county population source for the US
  baseline,
- update cadence,
- rights/cache posture,
- expected FLETCH registry or local fixture path.
- temporal validity fields available from the source, including effective dates,
  publication dates, or known historical caveats.

BISECT's Census/TIGER/GEOID conventions are the first reference point for US
county boundaries. Reusable boundary payloads should be expressed as RPLAN
contexts or future RPLAN packages.

## Research Requirements

Before the US county baseline is treated as evidence, add `docs/research/`
notes for:

- federal time-zone authority and the process for boundary changes,
- daylight saving law, exceptions, current federal constraints, and active
  legislative proposals,
- global time-zone database/source options, including IANA tzdb and national
  legal sources where available,
- current legal time-zone boundary data sources and their geometry quality,
- Census/TIGER county boundary and GEOID handling inherited from BISECT/RPLAN,
- solar-time math, longitude/centroid choice, and DST scenario effects,
- fairness and appropriateness principles for scoring tradeoffs.

## Non-Goals

- No legal claim that a proposed map can or should be enacted.
- No operational scheduling advice.
- No raw national GIS cache committed to git.
- No time-zone policy added to RPLAN, RLINE, or BISECT.
- No broad international conclusions before the US county baseline and at least
  one non-US boundary/time-rule pilot are reproducible.
- No recommendation language before role review confirms that scores,
  assumptions, and legal caveats are visible.

## Validation Contract

Foundation validation:

```powershell
cargo fmt
cargo test --workspace
cargo run -p zones-cli -- seed-report
cargo run -p zones-cli -- evaluate-plan
cargo run -p zones-cli -- evaluate-plan-detail
cargo run -p zones-cli -- write-evaluation
cargo run -p zones-cli -- source-report
cargo run -p zones-cli -- zone-catalog-report
git diff --check
```

Source-inventory validation:

```powershell
git grep -n "source" -- README.md PRODUCT_PLAN.md docs/specs context/waves
```
