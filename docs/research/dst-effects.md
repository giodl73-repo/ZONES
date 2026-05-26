# DST Effects Research Note

Status: seed note.

Retrieval date: 2026-05-26.

## Working Facts To Verify And Expand

- NIST describes DST as moving clocks one hour ahead during the DST period.
- In 2026, DST is in effect from March 8 at 2 a.m. local time to November 1 at
  2 a.m. local time.
- The spring transition moves one hour of daylight from morning to evening; the
  fall transition moves it back from evening to morning.
- DST is not observed in Hawaii, American Samoa, Guam, Northern Mariana Islands,
  Puerto Rico, the Virgin Islands, and most of Arizona.

## Scoring Implications

Standard solar error should be separate from DST-shifted clock error. DST may
make evening daylight feel more convenient while worsening clock-noon alignment.
Those are different claims and should not be collapsed into one score.

Required scenario labels:

- `standard-time`: legal standard offset only.
- `dst-period`: legal standard offset plus 60 minutes where DST is observed.
- `no-dst`: standard offset year round.
- `permanent-dst-legislative-scenario`: standard offset plus 60 minutes year
  round, labeled as a scenario rather than current legal authority.

## Open Questions

- Should ZONES score sunrise/sunset exposure for representative dates in
  addition to solar-noon error?
- Which latitude/season combinations create unacceptable morning-darkness risk?
- Should school start times, commute windows, or media-market alignment appear
  only as later optional disruption metrics?
