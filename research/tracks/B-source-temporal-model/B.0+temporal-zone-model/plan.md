# Paper B.0: A Temporal Data Model For Civil Time-Zone Boundaries

## Research Question

What minimal data model can represent administrative boundary units, time-zone
assignments, UTC offset rules, DST transitions, and candidate scenarios across
countries and historical periods without conflating current law, historical law,
and counterfactual analysis?

### Hypotheses / Claims

- **H1**: A regime model with boundary-unit validity intervals, graph versions,
  offset rules, and assignment intervals can represent current US county-level
  time-zone scoring without time-zone-specific extensions to RPLAN.
- **H2**: The same model can represent at least one non-US pilot with DST or
  historical offset rules by changing source data, not code structure.
- **H3**: IANA tzdb is useful for offset-rule history but insufficient as a
  complete legal-boundary source; national/legal geospatial sources or
  source-specific caveats are still required.

Falsifiers:

- H1 fails if RPLAN contexts cannot carry the required US boundary/unit graph
  contract without embedding time-zone policy.
- H2 fails if a non-US pilot requires new model fields not anticipated by the
  temporal model.
- H3 fails if source audit shows a single source fully covers both legal
  boundaries and historical offset rules with adequate legal provenance.

### Scope Boundary

- **In scope**: data model, validation constraints, US pilot, one non-US pilot,
  source limitations.
- **Out of scope**: optimizing candidate maps, recommending reforms, complete
  global historical database.
- **Generalizability claim**: The model should support global/historical
  extension, but only tested pilots are evidence.

## Target Venue

- Venue: GIS/data systems/software paper after non-US pilot.
- Expected contribution type: data model and reproducibility contract.

## Methodology

- Formalize entities: jurisdiction, boundary unit, boundary graph version,
  time-zone regime, offset rule, assignment, evaluation context.
- Validate model constraints in Rust.
- Map US county baseline to RPLAN + ZONES model.
- Select one non-US pilot and document source limitations.

## Evaluation

- **Baselines**:
  - static current-law map model,
  - IANA-only time-zone lookup,
  - RPLAN-only context without ZONES temporal fields.
- **Success criteria**:
  - model validates US county baseline;
  - model validates one non-US temporal pilot;
  - source audit identifies which claims each source can and cannot support.
- **Failure modes to test**:
  - boundary unit changes over time,
  - DST transition rule changes,
  - legal zone not aligned to chosen administrative unit,
  - source conflict between time-rule and geospatial data.

## Experiments

- [x] Serialize/deserialize temporal model fixtures.
- [ ] Validate US county baseline context.
- [x] Validate one non-US pilot.
- [ ] Compare IANA lookup to selected legal-boundary source.
- [x] Produce source limitation matrix.

## Quality Checkpoints

- [x] Model separates current law, historical law, proposed scenarios, and
  analytic counterfactuals.
- [ ] RPLAN/RLINE/ZONES boundaries are explicit.
- [x] IANA is not described as complete legal-boundary evidence.
- [x] Non-US pilot caveats are visible.
