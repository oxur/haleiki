# Arc 02 — Routes & Static Export · Closing Report

> The arc-level close, per `PROJECT-MANAGEMENT.md` Part V and
> `LEDGER-DISCIPLINE.md` Section B: capability verdict, slice walk, the
> composition check (arc ledger B-5…B-8 **reproduced at arc scale**), the
> change log, and the bubble-up to the project. Status: **CLOSED** ·
> 2026-07-28.

## 1. Capability — restated and verdict

**Capability (from `arc-plan.md`):** deliver the full wiki route set and a
complete static `_site/` from the demo corpus — home, browse, per-category,
about, attribution, 404, and the search shell — with all category grouping
resolved in Rust, and write a deployable `_site/`.

**Verdict: delivered.** `haleiki export` renders the demo corpus to a **293-file
`_site/`** (6 singletons + 10 categories + 91 sources + 186 concepts), copies the
site-asset allowlist, and produces a tree whose internal links resolve to real
files. Verified by running the real export in an independent build, not only by
tests.

## 2. Slice walk

Four slices, all CDC-closed against independent reproduction. Count matches the
`arc-plan.md` breakdown (4); no arc-scale silent drop.

| Slice | Outcome | Close evidence |
|-------|---------|----------------|
| 01 — site-index + home | delivered | `slice01-site-index-home/cdc-verification.md` — 7/7 reproduced. |
| 02 — browse + category | delivered | `slice02-browse-category/cdc-verification.md` — 7/7; the `_uncategorized` silent-drop bug caught in-slice by an exact-count test and fixed. |
| 03 — about + attribution + search shell + 404 | delivered | `slice03-standalone-pages/cdc-verification.md` — 8/8; required-`Profile.attribution` ripple handled proactively (both YAMLs). |
| 04 — static export writer | delivered | `slice04-static-export/cdc-verification.md` — 8/8; real `_site/` export reproduced at arc scale. |

## 3. Composition check (arc ledger, reproduced at arc scale)

Reproduced by **running the real export** against the full demo corpus in a
clean independent build and inspecting the artifact:

| ID | Criterion | Verdict | Evidence (reproduced) |
|----|-----------|---------|-----------------------|
| B-1 | Slice 01 closed | done | ptr: slice01 cdc-verification |
| B-2 | Slice 02 closed | done | ptr: slice02 cdc-verification |
| B-3 | Slice 03 closed | done | ptr: slice03 cdc-verification |
| B-4 | Slice 04 closed | done | ptr: slice04 cdc-verification |
| B-5 | Complete `_site/` at contract paths | done | 293 HTML files: home/browse/about/attribution/search `index.html`, `404.html` (not `404/index.html`), 10 `category/{slug}/`, 186 `{slug}/`, 91 `source/{slug}/`. |
| B-6 | Assets copied | done | `_site/assets/{css,fonts,icons,images,js,vendor}` present (`haleiki.css` + 8 fonts); `ai`/`mockups` excluded. |
| B-7 | Grouping resolved in Rust | done | Delivered in slice 02 (browse-by-subcategory, category-by-tier; templates declarative); confirmed present in the written browse/category pages. |
| B-8 | Internal links resolve | done | home → `/category/mathematics/` (file exists); `abelian-group` breadcrumb + see-also → `/source/group-mathematics/`, `/ring-mathematics/` (files exist). |

Composition verdict: **the slices compose into the capability.** Nothing the arc
promised is missing. Gate reviewed by the independent CDC context that ran the
export (distinct from the implementer).

## 4. Change log

The `arc-plan.md` stayed at **v1.0** — no mid-arc plan changes were forced; each
slice delivered its assigned piece. Two findings surfaced during the arc that did
not change the plan but are carried forward:

- **Tier silent-drop robustness** (slice 02 CDC): `build_category_page_data`
  would omit an article whose tier is outside the taxonomy — does not fire on the
  demo (all tiers valid; validator-guarded), logged for a future arc.
- **Recurring `&#x2f;` href escaping** (Arc 01 + slices 03, 04): functionally
  harmless (browsers decode), but worth a permanent note — added at this close
  (project-level gotcha).

## 5. Bubble-up to the project

Answered against `project-plan.md`, per Part V:

1. **Did Arc 02 deliver its capability as the roadmap defined it?** Yes — the
   full route set + a complete deployable `_site/`, grouping in Rust. No
   project-plan change forced (a valid, recorded outcome).
2. **What did Arc 02 reveal the project plan did not anticipate?**
   - **Command reconciliation for Arc 06:** the static export landed as a new,
     additive `export` subcommand; `build` still writes the legacy `_data` JSON.
     Arc 06 should reconcile these (make `build` the site build / drop the JSON
     writer / remove Cobalt-Liquid), as the design-delta intends.
   - **The href-escaping gotcha** now has a project-level note (CLAUDE.md), so
     the next template/renderer author doesn't rediscover it.
   - **Search shell only:** `/search/` renders its static shell; facets, the
     JSON index, and `search.js` behavior are Arc 04 — as sequenced.
3. **Silent-drop diff at arc scale:** none. Every roadmap expectation of Arc 02
   landed; the items above are forward-looking, not gaps.

## 6. Arc-close decision

**Go.** Arc 02 closes. Project ledger row **P-2** (Arc 02 closed + composed) is
satisfied; `project-plan.md` is updated (P-2 done; status: Arc 03 — tiny_http
server — is next to plan deep). No remediation slice required.
