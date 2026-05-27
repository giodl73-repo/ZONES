# US Boundary Sources Research Note

Status: draft inventory.

Retrieval date: 2026-05-26.

## Primary Sources

| Source | Use | Contract | Caveats |
|---|---|---|---|
| Census TIGER/Line counties, 2024 | County and county-equivalent geometry | Legal/statistical boundary geometry with GEOID/FIPS keys | Not a time-zone source; legal boundaries and names are as of the vintage date |
| Census Gazetteer counties, 2024 | Exploratory internal points, land/water area, compact source for early scans | Tabular county records with internal latitude/longitude and GEOID fields | Internal point is not population weighted and can be misleading for large or irregular counties |
| RPLAN contexts | Local boundary graph carrier | Units, adjacency, population, and plan assignments can reuse election-redistricting machinery | ZONES must add time-zone meaning outside RPLAN, not mutate RPLAN semantics |
| BISECT conventions | Implementation precedent | Shows how to structure boundary ingestion, graph contracts, and research artifacts | BISECT election concepts should be copied only when the boundary/data shape matches |

Useful source URLs:

- https://www.census.gov/geographies/mapping-files/time-series/geo/tiger-line-file.2024.html
- https://www.census.gov/cgi-bin/geo/shapefiles/index.php?layergroup=Counties&year=2024
- https://www.census.gov/geographies/reference-files/2024/geo/gazetter-file.html
- https://www.census.gov/programs-surveys/geography/technical-documentation/complete-technical-documentation/tiger-geo-line/2024.html

## ZONES Source Contract

Every boundary unit used in a published score must carry:

- stable unit id, preferably Census GEOID for US counties;
- display name and parent jurisdiction;
- boundary vintage and source URL, represented in plan inputs by
  `source_refs.boundary_source_id`;
- representative point method and source, represented by
  `source_refs.representative_point_source_id`;
- adjacency source and build date, either through an RPLAN context or an
  equivalent ZONES source reference;
- population source and vintage when population-weighted scoring is used,
  represented by `source_refs.population_source_id`;
- any known split between legal time-zone boundary and chosen administrative
  unit.

## Representative Point Levels

ZONES should support at least four representative-point methods:

| Method | Use | Publication status |
|---|---|---|
| Census internal point | Fast exploratory scans and smoke tests | Allowed only with caveat |
| Geometric centroid | Geometry QA comparison | Allowed only with caveat |
| Population-weighted centroid | Main county-level baseline | Required before strong claims |
| Multi-point population sample | Large/split counties and uncertainty intervals | Required for high-risk cases |

## Open Work

- Use an RPLAN-produced county context as the preferred intake artifact when it
  is available; otherwise build a ZONES smoke fixture directly from documented
  Census fields and keep it clearly caveated.
- Reconcile county adjacency from the RPLAN artifact with any ZONES-specific
  boundary graph smoke fixtures before publishing scores.
- Add source manifests with file hashes before committing derived national score
  tables.
- Identify counties where current legal time-zone boundaries split the county or
  conflict with a county-level simplification.
