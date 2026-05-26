# ZONES Research Paper Portfolio

Status: planning.

The paper map follows the BISECT pattern: track heads tell the whole story, and
sub-papers provide detailed evidence. ZONES is not ready for formal papers yet;
this file records the intended structure so implementation waves know what
evidence they are building toward.

## Track A - Measurement And Baseline

Audience: geography, public policy, time standards, and civic-data readers.

Module path: [`tracks/A-measurement-baseline/`](tracks/A-measurement-baseline/).

| Track ID | Slug | Working Title | Status | Evidence Needed |
|---|---|---|---|---|
| A.0 | solar-noon-baseline | Mapping Civil-Time Error: A County-Level Baseline for US Time Zones | planned | US legal-boundary baseline, standard-time scorecard, DST-period scorecard |
| A.1 | county-deviation-atlas | The Counties Most Misaligned With Solar Noon | planned | County scores, population-weighted variants, uncertainty notes |
| A.2 | dst-shift-analysis | Daylight Saving Time Versus Solar-Time Fit | planned | Standard vs DST-period comparisons and seasonal framing |

## Track B - Source And Temporal Data Model

Audience: GIS, data engineering, reproducibility, and historical-time readers.

Module path: [`tracks/B-source-temporal-model/`](tracks/B-source-temporal-model/).

| Track ID | Slug | Working Title | Status | Evidence Needed |
|---|---|---|---|---|
| B.0 | temporal-zone-model | A Temporal Data Model for Civil Time-Zone Boundaries | planned | ZONES temporal model, RPLAN context mapping, IANA/national-source audit |
| B.1 | boundary-source-audit | Legal Boundary Sources for Time-Zone Analysis | inventory | Census/TIGER, legal timezone layers, source hashes, cache posture |
| B.2 | global-time-rules | Global Time-Zone Rules Over Time: Source Limits and Data Contracts | inventory | IANA tzdb assessment plus national legal-source examples |

## Track C - Optimization And Scenario Families

Audience: algorithms, operations research, geography, and civic optimization.

Module path: [`tracks/C-optimization-scenarios/`](tracks/C-optimization-scenarios/).

| Track ID | Slug | Working Title | Status | Evidence Needed |
|---|---|---|---|---|
| C.0 | timezone-redistricting | Time-Zone Redistricting Under Civic Boundary Constraints | planned | Candidate generator, RLINE/RPLAN validation, scenario comparison |
| C.1 | minimum-change-zones | Minimum-Change Time-Zone Reform | planned | Move counts, boundary cuts, solar-error improvements |
| C.2 | state-county-tradeoffs | State Versus County Boundaries in Time-Zone Reform | planned | Strict-state and county-level scenario families |

## Track D - Law And DOT Convenience

Audience: transportation policy, administrative law, local government.

Module path: [`tracks/D-law-policy/`](tracks/D-law-policy/).

| Track ID | Slug | Working Title | Status | Evidence Needed |
|---|---|---|---|---|
| D.0 | convenience-of-commerce | Solar Fit Is Not Enough: DOT Convenience-of-Commerce Constraints | planned | DOT guidance, Federal Register examples, proxy dataset design |
| D.1 | public-disruption-metrics | Measuring Time-Zone Change Disruption | planned | Commuting, media market, airport, health care, school, shopping proxies |
| D.2 | reform-pathways | Legal Pathways for Time-Zone Boundary Reform | inventory | US federal/state authority, active legislation, boundary-change case studies |

## Track E - Global And Historical Systems

Audience: comparative policy, historical GIS, standards/data systems.

Module path: [`tracks/E-global-historical/`](tracks/E-global-historical/).

| Track ID | Slug | Working Title | Status | Evidence Needed |
|---|---|---|---|---|
| E.0 | global-zone-systems | Time-Zone Boundaries Around the World: A Comparative Solar-Fit Baseline | planned | Non-US pilot, global source audit, comparable scoring |
| E.1 | historical-zone-change | Historical Time-Zone Changes as Boundary Regimes | planned | Time-versioned regimes and historical source examples |
| E.2 | country-boundary-pilots | Non-US Administrative Boundary Pilots | planned | At least one reproducible non-US pilot |

## Track Head Story

- A.0 says what is misaligned now.
- B.0 says how the data model makes the measurement reproducible over time.
- C.0 says how candidate plans are generated and compared.
- D.0 says why solar fit is only one constraint under current law.
- E.0 says whether the method generalizes beyond the US.
