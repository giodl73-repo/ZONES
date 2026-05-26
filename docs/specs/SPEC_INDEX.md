# ZONES Spec Index

ZONES specs define the product boundary before implementation expands.

| Spec | Owner | Status | Purpose |
|---|---|---|---|
| [zones-foundation.md](zones-foundation.md) | ZONES | draft | Defines the first boundary-unit, time-zone plan, scoring, source, and dependency contracts. |
| [zones-temporal-global-model.md](zones-temporal-global-model.md) | ZONES | draft | Defines country-neutral, time-versioned entities for worldwide time-zone analysis. |

Research notes live under [`docs/research/`](../research/) and must be filled
before US county baseline claims move beyond seed-fixture status.

## Spec Rules

- RPLAN owns portable unit graph/context contracts.
- RLINE owns reusable graph/stat/optimization kernels.
- ZONES owns civil-time policy, solar-error scoring, source assumptions, map
  outputs, and recommendation caveats.
- BISECT may be used as precedent for Census/TIGER/GEOID handling, but ZONES
  must not depend on BISECT application internals.
- Raw GIS/source cache bytes stay out of git unless a source policy explicitly
  allows a small committed fixture.

The checked contract for these boundaries lives at
`data/module-boundaries/zones-rplan-rline.json` and can be summarized with
`zones module-boundary-report`.
