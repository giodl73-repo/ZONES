# Paper A.0: Mapping Civil-Time Error

## Research Question

Where do current US civil time-zone assignments deviate most from local mean
solar noon, and how much does the answer change when the metric uses standard
time, DST-period clock time, internal points, and population weighting?

### Hypotheses / Claims

- **H1**: Current US standard-time zones contain county-level units with mean
  solar-noon error greater than 45 minutes.
- **H2**: DST-period clock time increases mean solar-noon error by roughly 60
  minutes in DST-observing counties, changing the interpretation of which
  places are most misaligned.
- **H3**: The highest-error counties identified by Census internal points remain
  high-error after population-weighted representative points are used, but their
  rank order changes.

Falsifiers:

- H1 fails if the legal-boundary baseline finds no county with standard-time
  absolute solar error above 45 minutes.
- H2 fails if DST-period scoring does not shift DST-observing counties by the
  expected offset rule, or if DST rules cannot be represented reproducibly.
- H3 fails if population-weighted points move most exploratory high-error
  counties below a moderate-error threshold.

### Scope Boundary

- **In scope**: US counties and county equivalents, current-law standard offsets,
  DST-period offset rules, Census/TIGER/GEOID-compatible county units.
- **Out of scope**: final reform recommendations, non-US claims, historical
  boundary changes, apparent solar noon by date.
- **Generalizability claim**: This paper can establish a US baseline method. It
  does not claim the same source stack works globally.

## Target Venue

- Journal: geography / public policy / GIS venue after evidence exists.
- Expected contribution type: empirical measurement baseline.

## Methodology

- Build an RPLAN-compatible county context for US county units.
- Attach legal time-zone assignments and standard UTC offsets.
- Attach representative point and population source metadata.
- Compute standard solar error:

```text
abs(longitude * 4 - standard_offset_minutes)
```

- Compute DST-period clock error using the active offset rule for the scenario.

## Evaluation

- **Baselines**:
  - current legal standard time,
  - current legal DST-period clock time,
  - exploratory Census internal-point scan.
- **Success criteria**:
  - every scored county has a source-backed unit id, representative point,
    legal time-zone assignment, offset rule, population value, and source
    vintage;
  - results reproduce from committed scripts and source manifests;
  - role review accepts the distinction between measurement and recommendation.
- **Failure modes to test**:
  - county split by a time-zone boundary,
  - county representative point falls near or across a legal boundary,
  - no DST observance,
  - Alaska/Aleutian dateline behavior.

## Experiments

- [ ] Build US county RPLAN context.
- [ ] Join current legal time-zone assignment to counties.
- [x] Compute standard-time solar error.
- [x] Compute DST-period clock error.
- [ ] Compare internal-point and population-weighted representative points.
- [ ] Produce top-deviation and population-weighted summary tables.

## Figures

- [ ] US county solar-error map under standard time.
- [ ] US county DST-period shift map.
- [ ] Top 25 counties by standard error.
- [ ] Internal-point versus population-weighted scatter.

## Tables

- [ ] Source manifest table.
- [ ] Top deviation counties.
- [ ] Zone-level summary statistics.
- [ ] DST-observing versus non-observing summary.

## Quality Checkpoints

- [ ] Hypotheses are falsifiable.
- [x] Standard-time and DST-period scenarios are separated.
- [ ] DOT/legal authority caveat appears before any interpretation.
- [ ] No candidate reform recommendation appears in the paper.
- [ ] Source uncertainty and representative-point method are visible in every
  result table.
