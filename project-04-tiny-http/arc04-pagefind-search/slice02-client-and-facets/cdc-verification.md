# Slice 02 — client wiring + facets + arc composition · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md)
> against the artifacts, per `LEDGER-DISCIPLINE.md` Section A. Verdict:
> **CLOSED**, with the browser-JS tooling policy routed to Slice 03 ·
> 2026-08-21.

## Verifier and method

Independent Codex Desktop review of commit
`ae82a12311a7b47fd7c2e5d57120e0eea43ad9ed` (`Arc04 Slice02 wire client search
facets`), not the implementing CC context. I read the slice doc, ledger, closing
report, `arc-plan.md`, the shipped browser asset, the search template, the
Makefile targets, and the Deno/Playwright harness under `tests/e2e/`.

I reproduced the browser harness and project checks rather than accepting the
closing report's claims. Localhost-binding commands require elevated permission
in the Codex sandbox; the non-elevated failures were `PermissionDenied` /
server-not-ready at bind time, and the same commands passed with localhost
permissions.

## Row-count check

Ledger has **8** rows (H-1…H-8); the closing report addresses all 8. No silent
drops. No deferred rows. H-8 was amended by operator decision on 2026-08-21:
the browser-JS tooling-policy mismatch is now Slice 03, not a blocker for this
functional slice.

## Per-row verification

| ID | CDC verdict | Strength | How reproduced |
|----|-------------|----------|----------------|
| H-1 | **confirmed** | reproduced | `make test-e2e` passed with localhost permissions. The harness step `unit: parseQuery...` verified the exposed parser, and `e2e (live): ?q=garbage renders results...` verified `?q=` seeding plus initial search in a real browser. |
| H-2 | **confirmed** | reproduced | The same e2e step asserted rendered `.search-result` titles, hrefs, visible URL text, and real Pagefind `<mark>` highlights. Grep of `assets/js/search.js` found one `innerHTML` write, only for `[data-field="excerpt"]`; title, URL, badges, facet labels, and status text use `textContent`. |
| H-3 | **confirmed** | reproduced | `make test-e2e` passed the no-match step using `日本語テスト`, and the clear-query step verified the result list empties and `.search-empty` returns hidden for the initial empty-query state. |
| H-4 | **confirmed** | reproduced | The e2e facet-sidebar step verified the four groups render as `Category`, `Tier`, `Type`, `Trust`, with non-zero counts from the real bundle. The pure `formatCount`/`humanizeLabel` step also passed. |
| H-5 | **confirmed** | reproduced | The e2e facet-toggle step narrowed `garbage` results to zero with `Mathematics`, then restored the original count with `Programming Languages`. `assets/js/search.js` converts multi-value groups to Pagefind's `{any: [...]}` shape for within-group OR; multiple group keys remain Pagefind's native across-group AND. |
| H-6 | **confirmed** | reproduced | The e2e URL-state step typed `garbage`, observed `q=garbage`, opened the reflected URL in a fresh page, and reproduced results. Facet URL persistence remains intentionally out of scope. |
| H-7 | **confirmed** | reproduced | The sink-parity e2e step drove both a live `haleiki serve` and a statically hosted export, then compared sorted `.result-url` sets for the same query. The sets were non-empty and identical. |
| H-8 | **confirmed under amended scope** | reproduced + routed | `make lint-e2e` passed (`deno lint` checked 2 files, `deno fmt --check` checked 3 files under `tests/e2e/`). `make test-e2e` passed with 1 Deno test / 12 steps / 0 failures, proving the shipped `search.js` is browser-loadable with no build step. `make check` passed with localhost permissions: 397 unit/bin tests plus the integration suites passed. Grep confirms no `require`, no static `import`, and only the required dynamic `import('/pagefind/pagefind.js')` in the shipped asset. |

## H-8 amendment

My first CDC pass did **not** close the slice because the original H-8 wording
said `assets/js/search.js` was Deno lint/fmt-clean. Direct reproduction proved
otherwise:

```
deno lint assets/js/search.js
# failed: 52 findings, mainly no-var/no-window, plus no-this-alias/no-inner-declarations

deno fmt --check assets/js/search.js
# failed: Deno would reformat the browser asset
```

The operator decided to close the functional search slice and open a new slice
for the tooling issue. That is the right disposition: Deno is a good runner for
the committed Playwright harness, but Deno's default lint rules are not yet an
approved policy for classic browser assets that intentionally use `window` and
match existing house style. Slice 03 now owns the question: what is Haleiki's
canonical quality gate for shipped browser JavaScript?

## Path-move verification

The e2e harness now lives under `tests/e2e/`, not a top-level `e2e/` directory.
I confirmed:

- `tests/e2e/harness.js` computes `REPO_ROOT` with `new URL("../..",
  import.meta.url)`, so paths to `tools`, `demo`, `templates`, and `assets`
  resolve correctly from the moved location.
- `make test-e2e`, `make lint-e2e`, and `make check-e2e` run from
  `tests/e2e/`.
- A stale-reference grep found no bare `e2e/` references after the move.

## Commands reproduced

```
make lint-e2e
# passed: deno lint checked 2 files; deno fmt --check checked 3 files

make test-e2e
# passed with localhost permissions: 1 test, 12 steps, 0 failures

make check
# passed with localhost permissions: build + lint + test;
# 397 unit/bin tests passed, 8 ignored; integration suites 14/3/1/1 passed
```

Non-elevated `make test-e2e` and `make check` fail in this Codex sandbox only
because tests bind localhost ports. Those same checks passed with localhost
permissions, matching CC's report.

## Verdict

**Slice 02 is CLOSED.** The functional capability is reproduced: `/search/`
loads Pagefind in a real browser, seeds `?q=`, renders results into the bespoke
markup with the intended escaping boundary, exposes and verifies the pure helper
seam, renders all four facets with live counts, applies query+facet filtering,
handles no-match and cleared-query states, reflects `q` in the URL, and returns
identical result URL sets against live-server and static-export sinks.

The only non-closing issue is not a product behavior defect: the repo lacks a
settled browser-JS lint/format policy. That is now explicitly opened as Slice
03 (`../slice03-browser-js-tooling-policy/`). Arc 04 should not close until
Slice 03 closes and the arc-level D-3…D-6 composition rows are reproduced.

## What worked

- The Deno/Playwright harness earned its keep: it reproduced both the browser
  behavior and the two-sink parity in one command, and it remains separate from
  `make check` until CI policy is settled.
- Splitting the tooling dispute into a follow-up slice preserved the functional
  close without laundering an unreproduced lint/fmt claim.
