# Arc 04 — Search (Pagefind) · Closing Report

> Arc-level close for `arc-plan.md` §4, per `PROJECT-MANAGEMENT.md` Part V and
> `LEDGER-DISCIPLINE.md` Section B. Slice rows are closed by pointer; composition
> rows D-3...D-6 were reproduced at arc scale against a fresh export, a live
> `haleiki serve`, and a real headless Chromium browser. Status: **CLOSED** ·
> 2026-08-21.

---

## Capability Verdict

Arc 04's capability — client-side Pagefind search with category / tier /
page-type / trust facets, hosted by both the static export and the tiny_http
server — is **delivered and reproduced at arc scale**.

The close evidence built the current `target/debug/haleiki`, exported the demo
site to a temporary `_site`, started both a static-file sink and a live
`haleiki serve`, and drove a browser query for `garbage`. The Pagefind bundle
contained 336 files; both sinks returned the JS entry, WASM, JSON entry, index,
and fragment files with the expected content types. The browser returned 14
results with title/url/type/category/tier metadata and `<mark>` excerpts, live
facet counts, query+facet narrowing, and identical result URLs across sinks.

## Slice Walk

Three slices, all CDC-closed with reproduced evidence:

| Slice | Outcome | Close evidence |
|-------|---------|----------------|
| 01 — index build + bundle in both sinks | delivered | `slice01-index-and-bundle/cdc-verification.md` — 8/8 rows reproduced; in-process Pagefind bundle, export emission, live serving, content types, and traversal guard. |
| 02 — client wiring + facets + composition | delivered | `slice02-client-and-facets/cdc-verification.md` — 8/8 rows reproduced; `search.js` hydrates the Pagefind API, browser query/facets/static-vs-live parity reproduced. |
| 03 — browser JS tooling policy | delivered | `slice03-browser-js-tooling-policy/cdc-verification.md` — 7/7 rows reproduced; `make check-browser-js` is the shipped browser-JS gate, Deno lint/fmt remains scoped to `tests/e2e/`, and no Node/npm workflow was introduced. |

No slice is missing from the arc-plan breakdown. No arc-scale silent drop was
found.

## Composition Check

The composition rows were reproduced after the three slice closes, not treated
as inherited from the slice reports. The targeted arc-close probe used the
existing `tests/e2e/harness.js` to build/export/start both sinks and drove
Chromium over HTTP:

```bash
deno run --config tests/e2e/deno.json --allow-all /private/tmp/arc04-close-probe.js
```

The normal sandbox run failed at the localhost boundary:

```text
Error: server at http://127.0.0.1:18080/ never became ready
```

The same probe passed with localhost permissions. This is a sandbox permission
boundary, not a project failure.

| ID | Criterion | Verdict | Arc-scale reproduction |
|----|-----------|---------|------------------------|
| D-1 | Slice 01 closed | done | CDC-closed 2026-07-29, 8/8 rows; the bundle is built in-process and served live. |
| D-2 | Slice 02 closed | done | CDC-closed 2026-08-21, 8/8 rows; live browser search/facets/sink parity reproduced. |
| D-3 | Both sinks host the Pagefind bundle with correct file kinds/content types | done | Fresh export produced `_site/pagefind/` with 336 files. Static and live fetches returned `200` for `pagefind.js` (`text/javascript`, 45555 bytes), `wasm.en.pagefind` (`application/wasm`, 72209 bytes), `pagefind-entry.json` (`application/json`, 173 bytes), `index/en_13a9ba1.pf_index` (`application/octet-stream`, 36267 bytes), and `fragment/en_1174e95.pf_fragment` (`application/octet-stream`, 1734 bytes). |
| D-4 | Real-browser query returns expected metadata and highlighted excerpt | done | Chromium loaded live `/search/?q=garbage` and rendered 14 results. First result: `Garbage Collection`, `/garbage-collection/`, type `Concept`, category `programming-languages`, tier `foundational`, excerpt containing Pagefind `<mark>` highlights. Result set metadata included types `Concept` and `Source`, category `programming-languages`, tiers `advanced`, `foundational`, and `intermediate`; every result had a title, site-relative href matching `.result-url`, type/category/tier badges, and a marked excerpt. |
| D-5 | Facets work with live counts; query + facet composition narrows correctly | done | Browser facet groups rendered exactly `Category`, `Tier`, `Type`, `Trust`. For query `garbage`, live counts included category `Programming Languages: 14`, tier `Advanced: 1`, `Foundational: 4`, `Intermediate: 9`, type `Concept: 9`, `Source: 5`, trust `Credible: 14`. Selecting category `Mathematics` with the query narrowed to 0 results and showed `.search-empty`; selecting `Programming Languages` restored 14 results, all with category `programming-languages`. |
| D-6 | Static export and tiny_http server return the same result URLs | done | The same browser query against both sinks returned the identical sorted URL set: `/actor-model/`, `/beam-vm/`, `/borrow-checker/`, `/erlang/`, `/garbage-collection/`, `/john-mccarthy-computer-scientist/`, `/lifetimes-rust/`, `/lisp/`, `/ownership-rust/`, `/source/erlang-language/`, `/source/john-mccarthy/`, `/source/lfe-language/`, `/source/lisp-language/`, `/source/rust-language/`. |
| D-7 | Slice 03 closed | done | CDC-closed 2026-08-21, 7/7 rows; browser-JS tooling policy is settled and reproduced. |

All seven rows are now final; D-3...D-6 are closed by arc-scale reproduction.

## Gates Run

| Command | Outcome |
|---------|---------|
| `make check-browser-js` | pass — tracked shipped assets covered: `browse-scrollspy.js`, `math.js`, `search.js`, `theme.js`, `toc-scrollspy.js`. |
| `make lint-e2e` | pass — `deno lint` checked 3 files; `deno fmt --check` checked 4 files. |
| `deno run --config tests/e2e/deno.json --allow-all /private/tmp/arc04-close-probe.js` | sandbox run failed at localhost readiness; rerun with localhost permissions passed and produced the D-3...D-6 evidence above. |
| `make test-e2e` | sandbox run failed at localhost readiness; rerun with localhost permissions passed: 1 test, 12 steps, 0 failures. |
| `make check` | sandbox run failed only on four localhost-bind Rust tests (`PermissionDenied` binding ephemeral ports); rerun with localhost permissions passed: clippy, fmt, build, 397 unit/bin tests passed with 8 ignored, and integration suites passed (`demo_fetch` 14/3, `demo_status` 3, `export_cli` 1, `search_cli` 1). |

## Bubble-Up To The Project

- **P-4 is satisfied.** Arc 04 delivered the roadmap capability: Pagefind index
  built from the rendered corpus, `/pagefind/` hosted in both sinks, custom
  `search.js` browser wiring, live facets, and live/static sink parity.
- **P-9 remains open.** This arc supplies strong implementation evidence for
  client-side Pagefind search, and the project plan now records that evidence.
  The project ledger says DoD rows close at project-scale recomposition rather
  than by inheriting arc attestations; additionally, Arc 06 is still the planned
  final parity/CI/deployment recomposition point. P-9 should be closed there by
  rerunning the browser search/facet evidence in the full project context.
- **No roadmap reshaping is forced.** Arc 05 (news) and Arc 06
  (polish/parity/deploy) remain in the existing order.
- **Forward caveat:** browser/e2e default-CI wiring remains deliberately
  deferred to Arc 06, consistent with Slice 03. The project now has the Make
  gates (`check-browser-js`, `lint-e2e`, `test-e2e`) needed for that decision.

## What Worked

- Keeping search client-only made sink parity directly testable: the same
  browser query over the same Pagefind bundle yielded an exact URL-set match.
- The Slice 03 tooling split paid off at arc close: shipped browser JS has its
  own gate, while the Deno/Playwright harness remains lint/fmt clean without
  imposing Deno module style on classic browser assets.

## Verdict

**Arc 04 — Search (Pagefind) is CLOSED and composed.** Rows D-1...D-7 are done;
the remaining composition rows D-3...D-6 were reproduced at arc scale. Project
ledger row **P-4** is satisfied; **P-9** is annotated with the arc evidence and
left open for project-scale recomposition.
