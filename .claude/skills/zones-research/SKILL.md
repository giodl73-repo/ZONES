---
name: zones-research
description: Ground ZONES source, benchmark, policy, or design decisions before broad integration.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# ZONES Research

Use this skill before adopting new source data, scoring weights, or country
expansions.

## Output

Write a short note under `docs/research/` that records:

- decision question,
- sources reviewed,
- rights/cache posture,
- assumptions,
- rejected alternatives,
- follow-up implementation pulse.

## Rules

- Prefer primary government, standards, or academic sources.
- Distinguish legal time, civil boundary, solar-time estimate, and population
  data.
- Do not cache raw source data until the source policy is explicit.
- Do not turn a scoring assumption into a recommendation without review.
