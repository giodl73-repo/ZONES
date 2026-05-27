# Wave: Foundation Contract

## Goal

Create the ZONES repo foundation and prove the first plan-evaluation contract
over a tiny administrative-unit graph.

## Thesis

ZONES should feel like BISECT for time-zone boundaries, but the domain policy
belongs here. RPLAN supplies the portable legal-boundary unit graph/context
contract, RLINE supplies graph metrics and connectivity checks, and ZONES owns
solar-time error, jurisdiction stability, source assumptions, and map outputs.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Workspace foundation | done | Create repo skeleton, docs, skills, and first tested RPLAN-backed evaluator. |
| 02 | US county source intake | done | Prove the source-gated county intake and baseline-smoke scorecard path without publication or recommendation claims. |
| 03 | US county seed graph | ready | Replace smoke fixtures with the first source-derived county graph, assignment, point, and population inputs. |
| 04 | Candidate scoring report | pending | Compare current and proposed plans with explicit tradeoff weights. |

## Current implementation state

After Pulse 01, the foundation contract has expanded beyond the original seed
evaluator. The repo now has source manifests, zone catalogs, a temporal pilot
fixture, a source-limitation matrix, a ZONES/RPLAN/RLINE module-boundary
contract, offset-fit scoring, ranked JSON/CSV outputs, SVG map and atlas
rendering, GeoJSON export, boundary-geometry joins, and nearest-offset candidate
plan generation.

Pulse 02 should not add recommendation language. It should turn the documented
US source inventory into a small, auditable county-level input path that can feed
the existing evaluator and map/report commands.

Pulse 02 is complete as a smoke rehearsal: source gate policy, RPLAN context
gate, assignment evidence gate, representative-point gate, baseline-smoke plan
input, ignored scorecard artifacts, and role review are in place. Pulse 03 is
the first source-derived county graph/current-plan pass and remains blocked from
publication until placeholder legal assignments, placeholder population weights,
and exploratory point methods are replaced or explicitly reviewed.

## Success criteria

- README explains the repo purpose and first command.
- Product plan names waves and non-goals.
- Wave/pulse scaffolding exists.
- Skills exist for future wave, pulse, and research execution.
- `zones-core` exposes a tested plan evaluator backed by RLINE graph metrics.
- Validation commands pass.
