# Pulse 02 Role Review: US County Baseline Smoke Scorecard

## Scope

Review the Pulse 02 source-intake and baseline-smoke scorecard path:

- source manifest and source gate policy,
- RPLAN county context smoke fixture,
- current-law assignment evidence smoke fixture,
- representative-point smoke fixture,
- assembled baseline smoke plan input,
- generated scorecard commands under ignored `target/zones/us-county-baseline-smoke/`.

All findings are AI-simulated role checks. They are design and quality-control
aids, not expert testimony.

## Verdict

Pulse 02 is acceptable as a **smoke-scorecard and source-gate rehearsal**. It is
not acceptable as a real county baseline, public evidence packet, or
recommendation packet.

## Role findings

| Role | Finding | Gate result |
|---|---|---|
| civil-time-policy-reviewer | Current-law and legal authority are labeled, and recommendation language is absent. Placeholder assignments correctly prevent legal-validity claims. | Pass for smoke; publication/recommendation blocked. |
| boundary-data-steward | RPLAN context shape, GEOID ordering, source gate policy, ignored-cache posture, and source hashes are present. National raw GIS/cache bytes are not committed. | Pass for smoke; real county graph/source hashes still required. |
| solar-time-methodologist | Solar offsets are minute-based and derived from longitude. Census internal points are explicitly exploratory. | Pass for smoke; strong claims blocked until stronger representative-point method. |
| graph-optimization-reviewer | RPLAN owns unit graph/context shape; RLINE remains reusable graph-metric support; ZONES owns civil-time scoring. No optimizer claims are made. | Pass. |
| public-map-editor | Scorecard commands are measurement outputs under ignored paths. Caveats are visible in plan input and detailed output. | Pass for internal inspection; public map packet blocked. |

## Blocking conditions before publication

- Replace placeholder county time-zone assignments with clause-level 49 CFR
  evidence and reconciled DOT geometry status.
- Mark split, uncertain, or legally ambiguous counties before rankings.
- Replace placeholder population weights with source-derived county weights.
- Keep Census internal points labeled exploratory, or promote to a stronger
  representative-point method before strong claims.
- Produce role-reviewed public copy that distinguishes evidence, design
  alternatives, and recommendations.

## Recommendation gate

Closed. No ZONES output in this wave may describe a candidate plan as preferred,
best, adoption-ready, or legally valid.

