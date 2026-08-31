# ZONES Invariants

## ZONES-I-01: CLI Startup Must Not Exhaust The Windows Main Stack

**Claim:** `zones-cli` argument parsing and command dispatch run on a large
enough stack for the full Clap command surface on Windows.

**Status:** VERIFIED

**Why it matters:** A public command reference is useless if `--help`, `status`,
or seed commands crash before domain logic runs.

**Test:** `cargo run -p zones-cli -- --help`, `cargo run -p zones-cli --
status`, and `cargo run -p zones-cli -- seed-report`.

## ZONES-I-02: Seed Evaluation Remains Deterministic

**Claim:** The seed fixture reports 4 units, 2 zones, 2 boundary edges,
connected zones, and stable weighted/max solar-error metrics.

**Status:** VERIFIED

**Why it matters:** The seed evaluator is the first proof of the scoring
contract before source-derived county baselines are public evidence.

**Test:** `cargo run -p zones-cli -- seed-report` and
`cargo test --workspace`.

## ZONES-I-03: Source Reference Coverage Is Machine-Visible

**Claim:** Source-reference reports expose complete, missing, and caveated
per-unit source coverage before a plan input is treated as publishable evidence.

**Status:** VERIFIED

**Why it matters:** ZONES must distinguish source-backed county seed inputs
from synthetic or smoke fixtures.

**Test:** `cargo run -p zones-cli -- source-ref-report
data/plan-inputs/us-county-baseline-seed.json`.

## ZONES-I-04: Candidate Packets Keep The Recommendation Gate Closed

**Claim:** Candidate comparison and map-packet outputs carry explicit closed
recommendation-gate language while reporting baseline and candidate metrics.

**Status:** VERIFIED

**Why it matters:** Local visual inspection should not harden into a public
preferred-map or enactment claim.

**Test:** `cargo run -p zones-cli -- compare-offset-candidates
data/plan-inputs/us-county-baseline-seed.json --output
target/zones/us-county-baseline-seed/candidate-comparison.json` and
`cargo run -p zones-cli -- write-offset-candidate-maps
data/plan-inputs/us-county-baseline-seed.json --output-dir
target/zones/us-county-baseline-seed/candidate-maps`.

## ZONES-I-05: Strict Static Analysis Is Part Of The Public CLI Gate

**Claim:** The workspace passes clippy with all targets and warnings denied.

**Status:** VERIFIED

**Why it matters:** Public-facing policy tools need maintainable command and
renderer code, not only passing unit tests.

**Test:** `cargo clippy --workspace --all-targets -- -D warnings`.

## ZONES-I-06: Public Authority Boundaries Are Machine-Checked

**Claim:** Candidate recommendation, national-baseline, and solar-objective
risks are encoded as a versioned authority-boundary contract and parsed by
repo-local tests.

**Status:** VERIFIED

**Why it matters:** ZONES can produce polished maps and concrete score deltas
before it has legal authority, complete source coverage, public preference, or
implementation tradeoff evidence. The strongest outputs need equally explicit
negative authority.

**Test:** `crates/zones-cli/tests/pitfall_policy.rs` parses
`docs/authority-boundaries.v1.json` and verifies the forbidden claims,
required upgrade evidence, and required visible tradeoffs for `ZONES-PF-01`,
`ZONES-PF-02`, and `ZONES-PF-03`.
