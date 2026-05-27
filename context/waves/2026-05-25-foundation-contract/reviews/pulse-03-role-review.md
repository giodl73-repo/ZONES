# Pulse 03 Role Review: US County Seed Graph

## Scope

Review the Pulse 03 source-derived four-county seed:

- Census 2024 Gazetteer representative points,
- Census 2024 county population estimates,
- TIGER-derived seed adjacency,
- 49 CFR clause-cited current-law assignment evidence,
- BTS/NTAD Time Zones polygon reconciliation,
- assembled `data/plan-inputs/us-county-baseline-seed.json`,
- generated scorecard commands under ignored `target/zones/us-county-baseline-seed/`.

All findings are AI-simulated role checks. They are design and quality-control
aids, not expert testimony.

## Verdict

Pulse 03 is acceptable as the first **source-derived seed graph/current-plan
baseline rehearsal**. It is not acceptable as a national baseline, public
scorecard, or recommendation packet.

## Role findings

| Role | Finding | Gate result |
|---|---|---|
| civil-time-policy-reviewer | Seed assignments cite 49 CFR clauses and avoid enactment/recommendation language. | Pass for seed; recommendation gate closed. |
| boundary-data-steward | Population, points, adjacency, and geometry reconciliation now cite source-derived inputs and hashes. Raw source caches remain ignored. | Pass for four-county seed; national graph still required. |
| solar-time-methodologist | Solar offsets are source-derived from Census Gazetteer longitudes. Internal points are still explicitly exploratory. | Pass for exploratory seed; strong claims blocked. |
| graph-optimization-reviewer | TIGER-derived seed adjacency has zero edges for the selected county set, and disconnected zones are surfaced rather than hidden. | Pass. |
| public-map-editor | Scorecard artifacts remain ignored measurement outputs. Caveats are visible and no public-copy promotion occurred. | Pass for internal inspection; publication blocked. |

## Blocking conditions before publication

- Scale from four seed counties to a national or explicitly scoped regional
  county universe.
- Replace Census internal points with stronger point methodology, or keep all
  outputs clearly exploratory.
- Review Baldwin County's tiny BTS/NTAD residual source-boundary sliver before
  using it in public claims.
- Run a publication packet review with public-map copy and source caveats.

## Recommendation gate

Closed. Pulse 03 measures seed evidence only; it does not prefer or recommend a
time-zone plan.

