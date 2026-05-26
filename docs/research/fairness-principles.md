# Fairness And Appropriateness Principles

Status: draft principles.

Retrieval date: 2026-05-26.

## Purpose

ZONES needs a public standard for judging candidate time-zone maps before it
optimizes anything. These principles are not final policy recommendations. They
define what evidence a candidate must expose to be treated as fair, appropriate,
or ready for review.

## Principles

1. Legal clarity comes first.
   Current law, historical law, proposed law, and counterfactual scenarios must
   be separate objects with separate source trails.

2. Solar fit is a measurement, not a mandate.
   Mean solar noon error is central to the project, but it cannot by itself
   justify a change under current US DOT practice.

3. Civic boundaries deserve respect.
   Plans should prefer state, county, municipal, and comparable administrative
   boundaries when doing so avoids confusing local splits.

4. Contiguity and graph integrity are required.
   A candidate zone should be contiguous unless a documented island, exclave, or
   jurisdiction-specific exception makes non-contiguity legitimate.

5. Minimize disruption unless evidence supports change.
   Improvements in solar fit should be balanced against population moved,
   cross-zone commuting, media markets, schools, health care access,
   transportation access, shopping patterns, and local government preference.

6. Population impact must be visible.
   A low-count unit score is not enough. Reports should show population-weighted
   effects and identify who benefits or bears disruption.

7. Boundary precision must match the claim.
   Internal-point scans can discover issues. Published rankings and proposals
   need legal-boundary and representative-point uncertainty checks.

8. DST and standard time are separate choices.
   A plan that improves standard-time solar fit may still worsen DST-period
   clock-noon alignment or morning-light conditions.

9. Global claims need local authority.
   Non-US and historical analyses must use country-specific legal context where
   available and label reconstructions honestly.

10. Reproducibility is part of fairness.
    Source manifests, vintages, hashes, and scoring code must be reproducible
    before a result is used to support a policy claim.

## Candidate Review Gates

Before a candidate map can be called a reform candidate, it must pass:

- legal-baseline review by `civil-time-policy-reviewer`;
- boundary/source review by `boundary-data-steward`;
- solar metric review by `solar-time-methodologist`;
- contiguity and graph review by `graph-optimization-reviewer`;
- public-readability review by `public-map-editor`.

## Open Work

- Convert these principles into machine-checkable scorecard fields.
- Define acceptable thresholds for solar error, moved population, and boundary
  disruption.
- Decide whether thresholds should differ for exploratory, paper, and public
  map outputs.
