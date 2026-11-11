# Slice 03 — browser JS tooling policy · CDC Verification

> Independent verification of CC's proposed-done Slice 03 evidence.
> Reviewer: CDC/Codex. Date: 2026-08-21. Status: **CDC-closed**.

## Scope

This verification covers Slice 03's tooling-policy follow-up for Arc 04. The
slice settles the quality gate for shipped browser JavaScript under
`assets/js/*.js`, keeps the Deno/Playwright harness policy scoped to
`tests/e2e/`, confirms the browser search evidence still reproduces, and checks
that no Node/npm workflow was introduced.

Implementation commits reviewed:

- `367799e` — Arc04 close slice02 and open slice03
- `384c046` — Arc04 Slice03 define browser JS gate
- `e194a0a` — Arc04 Slice03 record implementation evidence

All three commits include the required assistant trailers:

- `Co-authored-by: Codex <noreply@openai.com>`
- `Co-authored-by: Billo AI <ai-engineering@billo.systems>`

## Result

Slice 03 is **CDC-closed**. Ledger rows I-1 through I-7 reproduce at the claimed
strength. The only non-elevated failures observed were the expected sandbox
localhost-bind failures; the same checks passed when rerun with localhost
permissions.

## Ledger Walk

| ID | Result | CDC Evidence |
|----|--------|--------------|
| I-1 | reproduced | `AGENTS.md`, `slice-doc.md`, and `Makefile` document `make check-browser-js` as the canonical browser-asset gate. The docs explicitly state that Deno's default lint/fmt policy does not directly govern shipped `assets/js/*.js`; Deno lint/fmt remains scoped to `tests/e2e/`. |
| I-2 | reproduced | `Makefile` exposes `check-browser-js` and invokes `deno run --allow-read=assets/js --allow-run=git tests/e2e/browser_js_gate.js` from the repo root. `check` remains `common-checks test`; `check-e2e` remains `lint-e2e test-e2e`, so the browser/e2e gates are still explicit entry points. |
| I-3 | reproduced | `make check-browser-js` passed. The gate covered the tracked shipped browser assets returned by `git ls-files 'assets/js/*.js'`: `assets/js/browse-scrollspy.js`, `assets/js/math.js`, `assets/js/search.js`, `assets/js/theme.js`, and `assets/js/toc-scrollspy.js`. |
| I-4 | reproduced | `make lint-e2e` passed. Deno lint checked 3 files and Deno fmt checked 4 files under `tests/e2e/`. |
| I-5 | reproduced | Non-elevated `make test-e2e` failed only at the known sandbox localhost readiness boundary. Rerun with localhost permissions, it passed: 1 Deno test, 12 steps, 0 failures. |
| I-6 | reproduced | Non-elevated `make check` failed only at four Rust live-server tests blocked from binding localhost (`PermissionDenied`). Rerun with localhost permissions, it passed: dependency check, clippy, rustfmt, build, 397 unit/bin tests passed with 8 ignored, and integration suites passed (14/3/1/1). |
| I-7 | reproduced | `find . -maxdepth 3 \( -name package.json -o -name package-lock.json -o -name node_modules \) -print` produced no output. Policy/documentation grep showed no Node/npm workflow addition; mentions are the explicit forbidden-pattern policy. |

## Commands Reproduced

```
git status --short
git log --oneline -5
git show --stat --format=fuller 367799e 384c046 e194a0a
git ls-files 'assets/js/*.js'
test ! -e docs/tiny-http/arc04-pagefind-search/slice03-browser-js-tooling-policy/cdc-verification.md
rg -n "check-browser-js|canonical browser-asset gate|Deno's default lint/fmt|assets/js/\*\.js|tests/e2e" AGENTS.md docs/tiny-http/arc04-pagefind-search/slice03-browser-js-tooling-policy Makefile
rg -n "^(check|check-e2e|check-browser-js|test-e2e|lint-e2e):|test-e2e|check-browser-js" Makefile
find . -maxdepth 3 \( -name package.json -o -name package-lock.json -o -name node_modules \) -print
make check-browser-js
make lint-e2e
make test-e2e
make check
```

## Notes

- `make test-e2e` and `make check` require localhost permissions in the Codex
  sandbox because both exercise live local server behavior.
- `make check-browser-js` is a targeted browser-asset policy gate: it verifies
  the tracked shipped assets parse under the repo's current classic-script
  assumptions and rejects Node/CommonJS/Deno/build-step patterns. It is not a
  replacement for future full browser compatibility testing; Arc 06 still owns
  default/CI wiring decisions.

