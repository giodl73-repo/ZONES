# Track B - Source And Temporal Data Model

Papers in this track:

- B.0+temporal-zone-model
- B.1+boundary-source-audit
- B.2+global-time-rules

## Module Question

What data model and source chain can represent civil time-zone boundaries,
offset rules, DST transitions, and administrative units around the world over
time?

## Hypothesis Gate

Before drafting, each paper must pass a `/researchprewrite`-style check:

- state which source contract is being validated,
- name the boundary or time-rule sources,
- list legal/source caveats,
- define what source mismatch would falsify the proposed contract,
- identify whether the output belongs in ZONES, RPLAN, RLINE, FLETCH, or a
  published artifact.

## Current Status

Active methods track. The Rust temporal model exists as a foundation, but
source audit and non-US pilot evidence are still missing.
