# ZONES Pitfalls

## ZONES-PF-01: Candidate Score Becomes Time-Zone Recommendation

**Status:** OPEN

**Pattern:** A lower-error candidate grid, map packet, or comparison table is
described as the preferred, best, or ready-to-adopt time-zone plan.

**Domain:** Candidate maps, README examples, research papers, customer demos,
public-policy copy, and local adaptation worksheets.

**Actor:** Public-map author, civic-policy reviewer, local-adaptation user,
customer-demo author, or future release agent.

**Task:** Present candidate-map scores, map packets, or comparison tables
without turning lower solar error into a recommended or ready-to-adopt
time-zone plan.

**Surface:** README candidate-map commands, generated packet index, adoption
guide, role gates, public-map copy, and research paper drafts.

**Likely mistake:** Treat polished maps, score deltas, moved-population metrics,
or `recommendation_gate_closed` as if the civil-time, source, optimization, and
public-map reviews have selected a preferred plan.

**Consequence:** ZONES can overstep into time-zone recommendation or public
policy advice before legal authority, source completeness, tradeoff review, and
public communication gates are ready.

**Owner:** ZONES civil-time-policy reviewer and public-map editor.

**Detection difficulty:** The generated packet is visually polished and reports
concrete improvements, so recommendation language can slip in even when the
gate is closed.

**Structural solution:** Keep recommendation-gate language in generated packet
indexes and require civil-time, source, optimization, and public-map review
before any preferred-map claim.

**Evidence:** `.roles/ROLE.md`,
`context/waves/2026-05-25-foundation-contract/pulses/pulse-04.md`, and
`cargo run -p zones-cli -- write-offset-candidate-maps
data/plan-inputs/us-county-baseline-seed.json --output-dir
target/zones/us-county-baseline-seed/candidate-maps`.

**Test:** `cargo test -p zones-cli --test pitfall_policy`.

## ZONES-PF-02: Source-Derived Seed Becomes National Baseline

**Status:** OPEN

**Pattern:** The four-county source-derived seed, boundary-backed local maps,
or exploratory national visual packet is presented as a national current-law
scorecard.

**Domain:** US county baseline, map packets, research tracking, adoption guide,
and public claims.

**Actor:** Research author, map-packet publisher, adoption-guide user,
portfolio reviewer, or public reader.

**Task:** Explain the four-county source-derived seed and exploratory map
packet without presenting it as a national current-law scorecard.

**Surface:** README, research tracking, seed input, source-ref reports,
boundary-backed map packet, and adoption guide.

**Likely mistake:** Treat real Census/DOT-derived rows, source references,
GEOID-shaped IDs, and boundary-backed local maps as national baseline coverage.

**Consequence:** A four-county seed can become an apparent U.S. scorecard,
overstating legal-assignment completeness, county context coverage, and source
readiness.

**Owner:** ZONES boundary-data steward and research owner.

**Detection difficulty:** The seed has real Census/DOT-derived inputs and
complete source references, so it can sound stronger than its four-county
scope.

**Structural solution:** Preserve seed scope, exploratory point-method caveats,
and source-gate evidence until the national county context and legal assignment
baseline are complete.

**Evidence:** `README.md`, `research/TRACKING.md`,
`data/plan-inputs/us-county-baseline-seed.json`, and
`cargo run -p zones-cli -- source-ref-report
data/plan-inputs/us-county-baseline-seed.json`.

**Test:** `cargo test -p zones-cli --test pitfall_policy`.

## ZONES-PF-03: Solar Error Becomes Whole Policy Objective

**Status:** OPEN

**Pattern:** Solar-time fit is optimized or summarized without naming
convenience-of-commerce, disruption, public preference, legal process, or
implementation-cost tradeoffs.

**Domain:** Scoring formulas, research papers, candidate comparisons, public
maps, and DOT-policy discussion.

**Actor:** Research author, scoring editor, civic-policy reviewer, map author,
or public explainer.

**Task:** Discuss solar-time fit and candidate comparisons while keeping
commerce, disruption, preference, legal process, and implementation-cost
tradeoffs visible.

**Surface:** Research program, tracking table, fairness principles, federal
time authority note, scoring formulas, candidate comparisons, and public maps.

**Likely mistake:** Let mean solar noon error become the whole objective because
it is the clearest first metric and easiest score to optimize.

**Consequence:** ZONES can imply a plan is better or fairer without DOT
convenience-of-commerce evidence, disruption metrics, public preference,
implementation cost, or legal-process review.

**Owner:** ZONES solar-time methodologist, civil-time-policy reviewer, and
public-map editor.

**Detection difficulty:** Solar offset is the clearest first metric, so it can
dominate language before the DOT convenience layer exists.

**Structural solution:** Keep fairness and DOT-convenience research blocks open
until candidate families can report named tradeoff weights.

**Evidence:** `research/RESEARCH.md`, `research/TRACKING.md`,
`docs/research/fairness-principles.md`, and
`docs/research/federal-time-authority.md`.

**Test:** `cargo test -p zones-cli --test pitfall_policy`.

## ZONES-PF-04: CLI Command Surface Overflows Before Validation

**Status:** MITIGATED

**Pattern:** The large Clap command graph overflows the default Windows main
thread stack, so even `zones-cli --help` and `zones-cli status` crash before
domain validation can run.

**Domain:** CLI startup, public command reference, validation automation,
Windows portfolio runs, and customer demos.

**Detection difficulty:** Unit tests exercise core contracts and command helper
logic but do not execute the compiled binary's argument parser on Windows.

**Structural solution:** Run CLI parsing and dispatch inside a named thread with
an explicit larger stack, and keep binary startup smokes in PITFALL validation.

**Evidence:** `crates/zones-cli/src/main.rs`, `cargo run -p zones-cli --
--help`, `cargo run -p zones-cli -- status`, and `cargo run -p zones-cli --
seed-report`.

## ZONES-PF-05: Unit Tests Hide Renderer And CLI Shape Debt

**Status:** MITIGATED

**Pattern:** `cargo test --workspace` passes while strict clippy catches
wide-argument SVG helpers, manual coordinate range checks, or CLI enum naming
debt.

**Domain:** SVG map rendering, coordinate validation, CLI argument enums,
public map packets, and release validation.

**Detection difficulty:** The behavior is correct and covered by tests, so the
debt appears only when `-D warnings` is part of the validation surface.

**Structural solution:** Keep clippy with all targets and denied warnings in
the ZONES PITFALL gate.

**Evidence:** `crates/zones-core/src/lib.rs`,
`crates/zones-cli/src/main.rs`, and `cargo clippy --workspace --all-targets
-- -D warnings`.
