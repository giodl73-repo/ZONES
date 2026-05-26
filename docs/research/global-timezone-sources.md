# Global Time-Zone Sources Research Note

Status: draft inventory.

Retrieval date: 2026-05-26.

## Source Layers

| Source | Use | Contract | Caveats |
|---|---|---|---|
| IANA tzdb | Offset and DST rule history for representative locations | Widely used civil-time rule database | It does not record full legal boundaries and is incomplete for many pre-1970 details |
| CLDR / Unicode timezone metadata | Names, aliases, display metadata, Windows/IANA mappings | User-facing metadata and interoperability | Not a legal geospatial source |
| National statutes and official maps | Country-specific legal boundary evidence | Best authority for legal claims | Availability, language, licensing, and historical coverage vary |
| Open geospatial timezone layers | Engineering helper for point-to-zone lookup | Useful for exploratory joins | Must be audited; may derive from IANA zones and community-maintained polygons |

Useful source URLs:

- https://ftp.iana.org/tz/theory.html
- https://www.iana.org/time-zones
- https://cldr.unicode.org/

## Model Implications

ZONES should store time-zone data as regimes, not as one timeless map. The
minimum global/historical contract is:

- jurisdiction and source authority;
- boundary unit validity interval;
- boundary graph version;
- time-zone assignment validity interval;
- offset rule validity interval;
- source confidence and caveats;
- whether the record is legal, historical reconstruction, or analytic scenario.

## Non-US Pilot Selection Criteria

A first non-US pilot should have:

- accessible administrative boundaries;
- a documented current legal time-zone framework;
- either DST/history complexity or a boundary-administration issue that tests
  the model;
- permissive enough licensing for reproducible derived artifacts;
- enough English-language or translatable primary documentation for review.

## Open Work

- Pick one pilot country and one fallback country.
- Audit whether source licensing allows derived zone-assignment tables.
- Add fixtures that prove the Rust temporal model can represent non-US rules
  without US-specific fields.
- Document which claims rely on IANA rules versus national legal sources.
