# Time Offset Rules Research Note

Status: draft inventory.

Retrieval date: 2026-05-26.

## Working Rule

ZONES stores civil-time offsets as minutes from UTC. It must not assume that
time zones are whole-hour offsets.

## Why

Current and historical civil time includes non-whole-hour offsets. Half-hour
offsets such as UTC+05:30 and UTC+09:30 exist, and 45-minute offsets such as
UTC+05:45 exist. The IANA time-zone database also treats a time zone as a rule
history for a location or fixed offset, not just as a whole-hour offset bucket.

## Implementation Impact

- `ZoneSpec.utc_offset_minutes` remains an `i32`.
- Foundation validation rejects only implausible offsets outside UTC-14:00 to
  UTC+14:00.
- ZONES should not ship a closed built-in list of "all time zones" as the legal
  truth. Legal regimes and IANA/national-source rules should supply the current
  zone/rule catalog for a given analysis vintage.
- Candidate scenarios may use whole-hour, half-hour, or quarter-hour offsets if
  the scenario names and caveats are explicit.

## Sources

- https://ftp.iana.org/tz/tzdb-2025b/theory.html
- https://www.timeanddate.com/time/time-zones-interesting.html
- https://www.timeanddate.com/time/current-number-time-zones.html
