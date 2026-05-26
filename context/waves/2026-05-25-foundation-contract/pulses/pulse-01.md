# Pulse 01: Workspace foundation

## Goal

Create the repository foundation and first tested time-zone plan evaluator.

## Changes

- Add Rust workspace with `zones-core` and `zones-cli`.
- Add README, product plan, wave docs, and repo-specific skills.
- Add seed fixtures that evaluate contiguity, boundary cuts, and weighted
  solar-time error from direct adjacency and from an RPLAN legal-boundary
  context.
- Record TRACKER dependency intake and RLINE usage.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- `cargo run -p zones-cli -- seed-report`
- `git diff --check`

## Status

Done.
