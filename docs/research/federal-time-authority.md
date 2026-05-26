# Federal Time Authority Research Note

Status: seed note.

Retrieval date: 2026-05-26.

## Working Facts To Verify And Expand

- The US Department of Transportation oversees US time zones, and 49 CFR part 71
  contains the official time-zone listing.
- Under the Uniform Time Act, either Congress or the Secretary of
  Transportation can change a time-zone boundary.
- DOT oversees uniform DST observance but does not have power to repeal or
  change DST.
- States may exempt themselves from observing DST by state law, but current DOT
  guidance says states do not have authority to choose permanent DST.
- If a state observes DST, it must use federally mandated start and end dates.
- Active federal bills such as the Sunshine Protection Act must be treated as
  proposals, not current law, until enacted.

## Open Questions

- What exact DOT boundary-change criteria and petition process should ZONES
  encode as feasibility notes?
- Which Federal Register proceedings provide examples of county-level or
  locality-level time-zone boundary changes?
- Which legal boundary source best matches 49 CFR part 71 for geospatial
  scoring?

## Implementation Impact

ZONES should score at least three scenario families separately:

- current legal standard time,
- current legal DST-observing clock time during the DST period,
- candidate standard-zone reforms.

Permanent-DST scenarios may be modeled for comparison, but they must be labeled
as legislative scenarios under current federal constraints.
