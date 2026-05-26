# ZONES Roles

ZONES uses repo-local review roles to keep civil-time reform claims, source
handling, optimization, and public maps honest.

All role outputs are AI-simulated review artifacts unless explicitly replaced by
human review. They are design and quality-control aids, not expert testimony.

## Parliament

| Role | Focus |
|---|---|
| [civil-time-policy-reviewer](parliament/civil-time-policy-reviewer.md) | Legal/civic scope, time-zone authority, and recommendation caveats. |
| [boundary-data-steward](parliament/boundary-data-steward.md) | Administrative boundary units, GEOID/FIPS handling, source provenance, and cache policy. |
| [solar-time-methodologist](parliament/solar-time-methodologist.md) | Solar offset estimates, longitude assumptions, daylight-saving framing, and error metrics. |
| [graph-optimization-reviewer](parliament/graph-optimization-reviewer.md) | Contiguity, boundary cuts, partition search, and RPLAN/RLINE dependency boundaries. |
| [public-map-editor](parliament/public-map-editor.md) | Map/report language, visual caveats, and public communication risk. |

## Review Gates

- Foundation gate: specs, roles, product plan, wave docs, and seed evaluator
  agree on dependency boundaries.
- Source gate: every source has rights/cache posture before broad ingestion.
- Score gate: every candidate map reports tradeoffs without implying legal
  authority.
- Recommendation gate: no candidate may be described as preferred, best, or
  ready for adoption until civil-time, source, scoring, optimization, and
  public-map roles have reviewed the packet.
- Publication gate: public copy distinguishes evidence, design alternatives,
  and recommendations.
