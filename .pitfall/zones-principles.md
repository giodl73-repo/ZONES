# ZONES Principles

## ZONES-P-01: Measurements Are Not Recommendations

**Rationale:** Lower solar-error or candidate-map deltas can look like policy
advice even when the source, public-map, and civil-time review gates remain
closed.

**Decision rule:** Any candidate output must state whether the recommendation
gate is closed and must frame lower-error deltas as measurements, not preferred
maps.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`context/waves/2026-05-25-foundation-contract/pulses/pulse-04.md`, and
`cargo run -p zones-cli -- compare-offset-candidates`.

## ZONES-P-02: Legal Authority Stays Outside The Score

**Rationale:** ZONES can evaluate current-law and counterfactual plans, but it
cannot enact, certify, or advise operational scheduling.

**Decision rule:** Current-law and historical-law scenarios must cite authority
sources; proposed and analytic counterfactual scenarios must remain explicitly
labeled and non-authoritative.

**Evidence:** `docs/specs/zones-foundation.md`, `.roles/ROLE.md`, and
`cargo test --workspace`.

## ZONES-P-03: Source Gates Precede Public Baselines

**Rationale:** Boundary, population, point, assignment, and time-zone geometry
sources have different rights, cache, and evidence strengths.

**Decision rule:** A public baseline claim needs manifest coverage, source-gate
coverage, complete per-unit source references, and explicit caveats before it
can move beyond fixture or seed status.

**Evidence:** `data/source-manifests/us-foundation.json`,
`data/source-gates/us-foundation-source-gate.json`, and
`cargo run -p zones-cli -- source-gate-report`.

## ZONES-P-04: Solar Fit Is One Tradeoff

**Rationale:** Solar-time error is measurable, but civil time also involves
commerce, public preference, administration, travel, media, schools, emergency
services, and implementation cost.

**Decision rule:** Scoring changes must keep weights and caveats visible and
must not hide disruption or convenience-of-commerce work behind a single
solar-fit number.

**Evidence:** `research/RESEARCH.md`, `research/TRACKING.md`,
`docs/research/fairness-principles.md`, and
`cargo run -p zones-cli -- evaluate-plan-detail`.

## ZONES-P-05: Shared Kernels Do Not Own Time-Zone Policy

**Rationale:** RPLAN and RLINE provide portable graph/context mechanics, but
ZONES owns civil-time semantics, source assumptions, map outputs, and public
recommendation caveats.

**Decision rule:** Promote only reusable boundary/context or graph/stat
mechanics upstream; keep time-zone policy, scoring language, and map packet
claims local to ZONES.

**Evidence:** `docs/specs/SPEC_INDEX.md`,
`data/module-boundaries/zones-rplan-rline.json`, and
`cargo run -p zones-cli -- module-boundary-report`.
