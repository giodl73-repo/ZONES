# Legal Time-Zone Boundary Sources Research Note

Status: draft inventory.

Retrieval date: 2026-05-26.

## Current US Authority And Boundary Sources

The US legal boundary baseline must be grounded in DOT authority and the
official federal time-zone description, not in a convenience point lookup.

| Source | Use | Contract | Caveats |
|---|---|---|---|
| 49 CFR part 71 | Official US time-zone boundary listing | Legal text for zone boundaries | Text must be converted to geometry or administrative assignments before scoring |
| DOT Time Zones Geospatial Map | Federal geospatial reference for US time zones | Public map layer linked from DOT time-zone guidance | Must verify metadata, vintage, and whether it is authoritative enough for county scoring |
| DOT time-zone boundary procedure | Current petition and evaluation process | Explains DOT convenience-of-commerce standard and process | It is process guidance, not a scoring dataset |
| Federal Register proceedings | Case evidence for how DOT applies criteria | Examples of accepted/rejected boundary changes | Needs careful selection; examples are not universal rules |

Useful source URLs:

- https://www.transportation.gov/regulations/time-act
- https://www.transportation.gov/regulations/procedure-moving-area-one-time-zone-another
- https://www.transportation.gov/sites/dot.gov/files/2023-12/TIME_ZONE_GUIDANCE.pdf
- https://www.ecfr.gov/current/title-49/subtitle-A/part-71

## Legal Baseline Rules

- Treat current law, proposed legislation, and analytic counterfactuals as
  separate regimes.
- Store the legal source at the assignment level, not only at the dataset level.
- In county-shaped plan inputs, use `source_refs.time_zone_assignment_source_id`
  for legal assignment evidence and `source_refs.time_zone_geometry_source_id`
  for reconciled map or polygon evidence.
- Do not infer county assignment solely from an internal point when a legal
  boundary crosses the county.
- For DST scoring, attach observance as an offset rule, not as a separate
  geographic boundary unless the law creates a different observance area.

## County-Level Simplification Policy

County-level analysis is acceptable as an early baseline if it marks counties
whose legal time-zone treatment is more detailed than a county assignment. A
county-level score should expose an uncertainty flag when:

- the county is split by a legal time-zone boundary;
- the representative point is close to a legal boundary;
- source geometry and CFR text disagree;
- offshore/island or dateline geometry creates longitude ambiguity.

## Open Work

- Inventory 49 CFR part 71 text into machine-readable boundary clauses that can
  cite the source clause for each county-level assignment.
- Compare DOT geospatial map outputs with county-level assignments and mark
  split or uncertain counties before they feed score tables.
- Create a split-county review table before publishing top-deviation rankings.
- Select Federal Register proceedings for the D.0 convenience-of-commerce paper.

Run `cargo run -p zones-cli -- county-assignment-report <assignment-set>` before
feeding current-law assignments into a baseline. Placeholder or uncertain rows
must keep the report not ready until every county assignment has clause-level
49 CFR evidence and reconciled geometry notes.

The Pulse 03 four-county seed replaces placeholder rows with clause-cited
evidence for the selected counties. Alabama seed counties cite `49 CFR 71.5(e)`
and `49 CFR 71.6(a)` for central time. Florida seed counties cite `49 CFR 71.4`
and `49 CFR 71.5(f)` for eastern time. The seed remains blocked from publication
until county-level DOT geometry reconciliation is complete.

Run `cargo run -p zones-cli -- geometry-reconciliation-report <reconciliation>`
to check the DOT geometry gate. `geometry_reconciliation_ready: false` blocks
publication even when legal clause evidence is present.

The BTS/NTAD ArcGIS feature service for time zones is recorded at
`data/source-endpoints/dot-time-zones-arcgis.json`. It exposes polygon geometry
with `zone` and `utc` fields at
`https://services.arcgis.com/xOi1kZaI0eWDREZv/arcgis/rest/services/NTAD_Time_Zones/FeatureServer/0`.
Only endpoint metadata is committed; raw geometry belongs in ignored cache until
the geometry reconciliation step is implemented.

Pulse 03 verifies the four seed representative points against the BTS/NTAD
Eastern and Central polygons. This is useful evidence that the assigned zone is
consistent with the point location, but it is not full county-polygon
reconciliation and does not clear the publication gate.
