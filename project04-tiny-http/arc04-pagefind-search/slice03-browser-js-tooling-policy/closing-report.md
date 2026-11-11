# Slice 03 — browser JS tooling policy · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per ledger discipline.
> Status: **CDC-closed** — all 7 rows reproduced by independent verification in
> [`cdc-verification.md`](./cdc-verification.md). Implementation commit:
> `384c046`.

## Policy Decision

Shipped browser JavaScript under `assets/js/*.js` is governed by
`make check-browser-js`, not by Deno's default lint/fmt rules. These assets are
classic browser scripts loaded by ordinary `<script>` tags; house-style `var`,
IIFEs, `window`, and `document` remain allowed.

The new gate parses every tracked `assets/js/*.js` file as classic JavaScript
and rejects Node/CommonJS/Deno/build-step dependency patterns: `require`,
`module.exports`, `process.`, `Deno.`, `node:`, `npm:`, `jsr:`, `package.json`,
`node_modules`, `npm install`, and `npm run`.

Deno lint/fmt remains the policy for the Deno/Playwright harness under
`tests/e2e/`, via `make lint-e2e`. Browser/e2e default-CI wiring remains
deferred to Arc 06.

## Ledger Walk

| ID | Status | Evidence |
|----|--------|----------|
| I-1 | done | `AGENTS.md` and `slice-doc.md` document the canonical browser-asset gate as `make check-browser-js` and explicitly separate it from Deno default lint/fmt. Grep of `AGENTS.md`, this slice directory, and `Makefile` finds the policy text and target names. |
| I-2 | done | `Makefile` exposes `check-browser-js`, which runs `deno run --allow-read=assets/js --allow-run=git tests/e2e/browser_js_gate.js` from the repo root. `check` remains `common-checks test`; `check-e2e` remains `lint-e2e test-e2e`. |
| I-3 | done | `make check-browser-js` passed and listed all tracked shipped assets: `assets/js/browse-scrollspy.js`, `assets/js/math.js`, `assets/js/search.js`, `assets/js/theme.js`, `assets/js/toc-scrollspy.js`. |
| I-4 | done | `make lint-e2e` passed: `deno lint` checked 3 files and `deno fmt --check` checked 4 files under `tests/e2e/`. |
| I-5 | done | `make test-e2e` passed with localhost permissions: 1 Deno test, 12 steps, 0 failures. The non-elevated run failed only at the known sandbox localhost-bind boundary. |
| I-6 | done | `make check` passed with localhost permissions: dependency check, clippy, rustfmt, build, 397 unit/bin tests passed with 8 ignored, and integration suites passed. The non-elevated run failed only at live-server tests blocked from binding localhost. |
| I-7 | done | `find . -maxdepth 3 \( -name package.json -o -name package-lock.json -o -name node_modules \) -print` produced no output. Diff grep found only policy text forbidding `package.json`/`node_modules`/`npm install`/`npm run`, not a workflow addition. |

## Commands Run

```
make check-browser-js
# passed; covered:
# assets/js/browse-scrollspy.js
# assets/js/math.js
# assets/js/search.js
# assets/js/theme.js
# assets/js/toc-scrollspy.js

make lint-e2e
# passed: deno lint checked 3 files; deno fmt --check checked 4 files

make test-e2e
# non-elevated: failed at localhost bind readiness
# with localhost permissions: passed, 1 test / 12 steps / 0 failures

make check
# non-elevated: failed only at localhost-binding live-server tests
# with localhost permissions: passed, 397 unit/bin tests + integration suites

find . -maxdepth 3 \( -name package.json -o -name package-lock.json -o -name node_modules \) -print
# no output
```

## Caveat

This slice does not choose a future CI policy for browser/e2e checks. Re-entry:
Arc 06 may decide whether `make check-browser-js`, `make lint-e2e`, or
`make test-e2e` become default CI gates.

## What Worked

- Keeping the browser-asset gate targeted avoided formatting churn in shipped
  classic scripts while still making the no-build/no-Node contract executable.
- The same Deno runtime can host both gates as long as the repo names their
  different authority clearly: Deno lint/fmt for harness code, targeted classic
  script checks for shipped browser assets.
