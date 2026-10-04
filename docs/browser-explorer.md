# Synthetic clock workbench

`zones-web` wraps the existing seed, native plan/offset evaluators, and SVG
renderer in a worker. The adapter explicitly labels every input as an analytical
counterfactual and clears legal authority. Existing source-derived scorecards
and their publication gates are unchanged. All four units, population weights,
coordinates, and polygon geometry are synthetic.

Two offsets accept quarter-hour steps from UTC−12 to UTC+14. Each sample unit
can choose A or B; daylight display adds 0 or 60 minutes. Connectivity, edge
cuts, moved population, and standard solar error remain native computations.
The daylight view displays shifted error separately from the standard-plan
score. Empty unused zones are allowed; connectivity evaluates zones in use.
Shared URLs contain only these bounded controls. Results export JSON.

Build with `python tools/build-pages.py`; wasm-bindgen-cli is pinned to 0.2.127.
`npm ci && npm test` exercises actual WASM, assignment/disconnection, shared
reload, download, invalid URL, mobile width, and failed WASM recovery. Actions
validates native tests, scoped clippy/fmt, release WASM, and a 5 MB output gate
before default-branch Pages deployment.
