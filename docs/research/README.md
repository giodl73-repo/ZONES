# ZONES Research Backlog

ZONES needs research before it can make claims beyond seed fixtures. Research
notes should cite primary sources where possible and should distinguish facts,
assumptions, and scoring choices.

## Required Notes

| Note | Status | Role gate | Purpose |
|---|---|---|---|
| `federal-time-authority.md` | pending | civil-time-policy-reviewer | Summarize US authority for time-zone boundaries, DST observance, and boundary-change procedures. |
| `dst-effects.md` | pending | solar-time-methodologist | Define standard-time, daylight-saving, and seasonal daylight-exposure scenarios. |
| `us-boundary-sources.md` | draft | boundary-data-steward | Inventory Census/TIGER county boundaries, GEOID/FIPS handling, population sources, and BISECT/RPLAN reuse points. |
| `legal-time-zone-boundaries.md` | draft | boundary-data-steward | Inventory current legal time-zone boundary geospatial sources and quality limits. |
| `global-timezone-sources.md` | draft | boundary-data-steward | Inventory IANA tzdb, CLDR, national legal sources, and historical time-zone data limits. |
| `time-offset-rules.md` | draft | solar-time-methodologist | Confirm minute-precision offsets, including half-hour and quarter-hour civil time. |
| `fairness-principles.md` | draft | full parliament | Define how solar fit, disruption, state/county respect, contiguity, and feasibility trade off. |

## Source Rules

- Use DOT, NIST, Congress.gov, Census, Federal Register, or equivalent primary
  sources first.
- Record retrieval date for legal or regulatory sources.
- Do not cache raw GIS/source data until the source policy is explicit.
- Do not treat active legislation as current law.
- Do not treat IANA tzdb as a complete legal-boundary source without documenting
  what it does and does not encode.
- Keep recommendations out of research notes unless the recommendation gate has
  passed.
