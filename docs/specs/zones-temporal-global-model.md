# ZONES Temporal Global Model

Status: draft.

## Goal

ZONES must support time-zone analysis around the world and over time. The US
county baseline is the first implementation target, not the limit of the model.

## Entity Sketch

### SourceManifest

- `manifest_id`
- `generated_on`
- `sources`: source citations for legal text, boundary geometry, time rules,
  population, representative points, derived manifests, and research notes.

### SourceCitation

- `source_id`
- `title`
- `kind`
- `url`
- `retrieved_on`
- `vintage`
- `content_hash`
- `caveats`

### Jurisdiction

- `jurisdiction_id`: stable code such as `US`, `CA`, `MX`, or source-specific
  country/territory code.
- `name`
- `parent_jurisdiction_id`
- `source_id`
- `valid_from`
- `valid_to`

### BoundaryUnit

- `unit_id`: source-stable id where available.
- `jurisdiction_id`
- `unit_level`: country, state, province, county, municipality, district,
  imported.
- `name`
- `geometry_ref` or geometry hash.
- `representative_point`: centroid, internal point, population center, or
  source-provided point.
- `valid_from`
- `valid_to`

### BoundaryGraphVersion

- `graph_id`
- `unit_universe_id`
- `source_id`
- `valid_from`
- `valid_to`
- `adjacency`
- `edge_kind`: land-boundary, water-boundary, ferry, bridge, point-touch,
  administrative-exception.

### TimeZoneRegime

- `regime_id`
- `authority`: current-law, historical-law, proposed-scenario,
  analytic-counterfactual.
- `jurisdiction_scope`
- `source_id`
- `valid_from`
- `valid_to`
- `assignments`: boundary unit to zone id.
- `offset_rules`: zone id to one or more offset rules.

### OffsetRule

- `zone_id`
- `standard_offset_minutes`
- `effective_from`
- `effective_to`
- `dst_delta_minutes`
- `transition_rule_ref`
- `observance_notes`

### TemporalDataset

- `dataset_id`
- embedded `source_manifest`
- `jurisdictions`
- `units`
- `boundary_graphs`
- `regimes`
- `evaluation_contexts`
- `caveats`

The dataset wrapper is the portable fixture and interchange contract. Validation
requires unique ids, known source references, valid graph dimensions, known
assignment units and zones, and evaluation contexts that reference known graph
and regime ids.

## Evaluation Requirements

Every score must include:

- evaluation date or period,
- source manifest id,
- boundary graph version,
- time-zone regime id,
- offset rule id,
- representative-point method,
- population or weighting source,
- source vintage.

## Open Questions

- Whether IANA tzdb is sufficient for global historical offset rules, and where
  it needs national legal-source supplementation.
- Whether source manifests should live beside derived artifacts, embedded in
  artifacts, or both.
- How to represent jurisdictions that change civil-time law without matching
  administrative boundary units.
- Whether population centers should replace geometric internal points for
  fairness scoring.
- How to compare regimes when boundary units split, merge, or change codes.
