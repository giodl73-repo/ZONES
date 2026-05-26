# ZONES Research Program

ZONES studies whether civil time-zone boundaries can be redesigned to reduce
solar-time mismatch while respecting legal boundaries, institutional authority,
and community ties.

The project should follow BISECT's evidence discipline: implementation waves
produce reproducible data packages first; papers come after the data can support
claims.

## What We Know Now

- Solar mean noon is the cleanest first astronomical anchor.
- Standard-time solar error can be computed from longitude and legal UTC offset:
  `abs(longitude * 4 - standard_offset_minutes)`.
- DST must be modeled separately from standard-time fit because it deliberately
  shifts clock time later by the DST delta.
- DOT's current US justification standard is convenience of commerce, not solar
  accuracy.
- US county-level analysis is feasible using Census/TIGER/GEOID conventions and
  RPLAN contexts.
- Global and historical analysis requires time-versioned boundary units,
  time-zone regimes, and offset rules.

## What We Do Not Know Yet

- Which legal time-zone boundary geospatial source best matches DOT/49 CFR part
  71 for US scoring.
- How much of the current US deviation is a boundary problem versus a DST policy
  problem.
- Whether population centers materially change the worst-county list compared
  with geometric internal points.
- Which convenience-of-commerce proxies best approximate DOT's actual standard:
  commuting, media markets, airports, schools, health care, shopping, and
  economic ties.
- Whether global historical time-zone analysis can rely on IANA tzdb alone or
  needs national legal-source supplements.
- What tradeoff rule is fair enough to call a plan better rather than merely
  more solar-aligned.

## Evidence Ladder

1. **Seed fixture**: synthetic counties prove the evaluator contract.
2. **Internal-point scan**: rough county ranking from Census gazetteer points and
   timezone lookup; useful for triage only.
3. **Legal-boundary baseline**: county graph, legal time-zone assignment, and
   population/longitude sources packaged as RPLAN context.
4. **Current-plan scorecard**: standard-time and DST-period errors for current
   law.
5. **DOT-convenience layer**: source-backed disruption and commerce proxies.
6. **Candidate families**: minimum-change, strict-state, strict-county,
   solar-first, and DOT-weighted scenarios.
7. **Role-reviewed publications**: claims separated into measurement, method,
   policy feasibility, and scenario recommendations.

## Research Tracks

- Track A: Measurement and Baseline
  ([module](tracks/A-measurement-baseline/))
- Track B: Source and Temporal Data Model
  ([module](tracks/B-source-temporal-model/))
- Track C: Optimization and Scenario Families
  ([module](tracks/C-optimization-scenarios/))
- Track D: Law, DOT Convenience, and Public Policy
  ([module](tracks/D-law-policy/))
- Track E: Global and Historical Time-Zone Systems
  ([module](tracks/E-global-historical/))

Each track is configured in `.claude/panel.json` so `panel:module` can review
the module before individual papers move from hypothesis to drafting.

## Publication Rule

No paper should argue for a specific reform until the baseline data, scoring
formula, DOT-convenience layer, and role review all exist. Earlier papers should
be framed as methods, measurements, or source audits.
