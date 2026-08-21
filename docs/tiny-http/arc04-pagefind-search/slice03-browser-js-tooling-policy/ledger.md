# Slice 03 — browser JS tooling policy · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. Status: **open** · 2026-08-21.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| I-1 | The repo documents the canonical quality gate for shipped browser JS (`assets/js/*.js`), including whether Deno lint/fmt applies directly, applies through browser-specific config, or does not apply to browser assets | read the added/updated docs; grep for the policy text and the target names it names | correctness | slice-doc | open | | |
| I-2 | The policy is encoded in runnable tooling, preferably Make-backed, so future CC/CDC contexts do not need to remember ad hoc commands | run the documented Make/browser-JS target(s); confirm they invoke the chosen gate from the repo root | correctness | slice-doc | open | | |
| I-3 | The chosen browser-JS gate passes on all tracked shipped browser assets, including `assets/js/search.js`, `assets/js/theme.js`, and `assets/js/browse-scrollspy.js` | run the browser-JS gate; list the tracked `assets/js/*.js` files it covers | correctness | slice-doc | open | | |
| I-4 | The Deno/Playwright harness remains Deno lint/fmt-clean and separate from the shipped browser-asset policy | `make lint-e2e` | serious | slice02 CDC | open | | |
| I-5 | The browser search evidence still reproduces after the tooling changes | `make test-e2e` | serious | slice02 CDC | open | | |
| I-6 | The Rust suite / default project gate stays green; browser harness CI/default wiring remains deferred to Arc 06 | `make check`; inspect Makefile to confirm `check` does not depend on `test-e2e` unless Arc 06 has explicitly changed policy | correctness | slice-doc | open | | |
| I-7 | No Node/npm project infrastructure is introduced | `find . -maxdepth 3 \( -name package.json -o -name package-lock.json -o -name node_modules \) -print`; grep docs/Makefile for `npm install`/`npm run` workflow additions | correctness | slice-doc | open | | |

## What Worked

_Filled at close._

