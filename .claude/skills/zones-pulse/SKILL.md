---
name: zones-pulse
description: Execute one ZONES pulse with docs, tests, and validation.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# ZONES Pulse

1. Read `context/waves/PHASES.md`.
2. Read the active wave and target pulse.
3. Make the smallest complete implementation and documentation change.
4. Add or update focused tests for scoring, graph, or source-contract behavior.
5. Run `cargo fmt`, `cargo test --workspace`, and any pulse-specific command.
6. Record the validation result in the pulse before commit.
