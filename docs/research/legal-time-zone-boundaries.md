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
