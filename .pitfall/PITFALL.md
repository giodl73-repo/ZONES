# ZONES PITFALL Index

ZONES uses PITFALL to preserve doctrine for civil-time boundary scoring, source
gates, solar-error metrics, candidate-map packets, public recommendation
boundaries, and RPLAN/RLINE ownership separation.

| Namespace | Kind | Path | Owner |
|---|---|---|---|
| `zones` | `principles` | [zones-principles.md](zones-principles.md) | ZONES maintainer |
| `zones` | `invariants` | [zones-invariants.md](zones-invariants.md) | ZONES maintainer |
| `zones` | `pitfalls` | [zones-pitfalls.md](zones-pitfalls.md) | ZONES maintainer |

## Integration

- ROLES: `.roles/ROLE.md` covers civil-time policy, boundary data, solar-time
  methodology, graph optimization, and public-map review gates.
- VTRACE: ZONES does not currently carry a repo-local VTRACE matrix; PITFALL
  entries cite specs, research tracking, role gates, wave notes, source
  manifests, and executable validation until a trace slice exists.
- Tests: Rust tests, clippy, source-gate reports, source-ref reports,
  candidate-map commands, and PITFALL validators are the evidence hooks.
