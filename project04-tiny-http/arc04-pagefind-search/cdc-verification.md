# Arc 04 — Search (Pagefind) · CDC Verification

> Independent verification of CC's Arc 04 close report.
> Reviewer: CDC/Codex. Date: 2026-08-21. Status: **CDC-verified**.

## Scope

This verification covers commit `344b56e` (`Arc04 close Pagefind search`) and
the arc-level close set:

- `docs/tiny-http/arc04-pagefind-search/closing-report.md`
- `docs/tiny-http/arc04-pagefind-search/arc-plan.md`
- `docs/tiny-http/project-plan.md`

The commit includes the required assistant trailers:

- `Co-authored-by: Codex <noreply@openai.com>`
- `Co-authored-by: Billo AI <ai-engineering@billo.systems>`

## Result

Arc 04 is **CDC-verified closed**. The documentation roll-up is consistent, and
ledger rows D-3 through D-6 reproduce at arc scale with a fresh export, live
`haleiki serve`, static-file sink, and Chromium browser.

Project ledger row P-4 is correctly marked done. Project ledger row P-9 is
correctly left open with an evidence note because project DoD rows close during
project-scale recomposition, not by inheriting arc evidence.

## Arc Ledger Walk

| ID | Result | CDC Evidence |
|----|--------|--------------|
| D-1 | confirmed by pointer | Slice 01 remains CDC-closed via `slice01-index-and-bundle/cdc-verification.md`. |
| D-2 | confirmed by pointer | Slice 02 remains CDC-closed via `slice02-client-and-facets/cdc-verification.md`. |
| D-3 | reproduced | A fresh export produced `_site/pagefind/` with 336 files. Static and live sinks both returned `200` with non-empty bodies for `pagefind.js` (`text/javascript`, 45555 bytes), `wasm.en.pagefind` (`application/wasm`, 72209 bytes), `pagefind-entry.json` (`application/json`, 173 bytes), `index/en_13a9ba1.pf_index` (`application/octet-stream`, 36267 bytes), and `fragment/en_1174e95.pf_fragment` (`application/octet-stream`, 1734 bytes). |
| D-4 | reproduced | Chromium loaded live `/search/?q=garbage` and rendered 14 results. The first result was `Garbage Collection`, `/garbage-collection/`, with badges `Concept`, `programming-languages`, `foundational`, and a Pagefind `<mark>` excerpt. |
| D-5 | reproduced | Live facet groups rendered as `Category`, `Tier`, `Type`, and `Trust`. Query-specific counts matched the close report: `Programming Languages: 14`, `Advanced: 1`, `Foundational: 4`, `Intermediate: 9`, `Concept: 9`, `Source: 5`, and `Credible: 14`. Selecting `Mathematics` narrowed the `garbage` query to 0 and showed the empty state; selecting `Programming Languages` restored 14 results. |
| D-6 | reproduced | The same Chromium query against the static sink and live sink returned the identical sorted 14-result URL set: `/actor-model/`, `/beam-vm/`, `/borrow-checker/`, `/erlang/`, `/garbage-collection/`, `/john-mccarthy-computer-scientist/`, `/lifetimes-rust/`, `/lisp/`, `/ownership-rust/`, `/source/erlang-language/`, `/source/john-mccarthy/`, `/source/lfe-language/`, `/source/lisp-language/`, `/source/rust-language/`. |
| D-7 | confirmed by pointer | Slice 03 remains CDC-closed via `slice03-browser-js-tooling-policy/cdc-verification.md`. |

## Commands Reproduced

```
git status --short
git log --oneline -6
git show --stat --format=fuller 344b56e
sed -n '1,260p' docs/tiny-http/arc04-pagefind-search/closing-report.md
sed -n '1,190p' docs/tiny-http/arc04-pagefind-search/arc-plan.md
sed -n '72,145p' docs/tiny-http/project-plan.md
make check-browser-js
make lint-e2e
deno run --config tests/e2e/deno.json --allow-all /private/tmp/arc04-cdc-probe.js
make test-e2e
make check
```

## Gate Results

| Command | Outcome |
|---------|---------|
| `make check-browser-js` | passed; covered `assets/js/browse-scrollspy.js`, `assets/js/math.js`, `assets/js/search.js`, `assets/js/theme.js`, and `assets/js/toc-scrollspy.js`. |
| `make lint-e2e` | passed; Deno lint checked 3 files and Deno fmt checked 4 files. |
| `deno run --config tests/e2e/deno.json --allow-all /private/tmp/arc04-cdc-probe.js` | non-elevated run built and exported, then failed at the expected localhost readiness boundary; rerun with localhost permissions passed and produced the D-3...D-6 evidence above. |
| `make test-e2e` | passed with localhost permissions: 1 Deno test, 12 steps, 0 failures. |
| `make check` | passed with localhost permissions: clippy, fmt, build, 397 unit/bin tests passed with 8 ignored, and integration suites passed (`demo_fetch` 14/3, `demo_status` 3, `export_cli` 1, `search_cli` 1). |

## Notes

- Localhost permissions are required in the Codex sandbox for the live
  server/browser evidence. The observed sandbox failure was a permission
  boundary, not a project failure.
- Browser/e2e default-CI wiring remains deferred to Arc 06, consistent with
  Slice 03 and the Arc 04 close report.

