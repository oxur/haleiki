# Project ledger

> Extracted without row changes on 2026-11-11 from the August plan. Existing dispositions/evidence are retained, not freshly reproduced by the migration.

Opens here; closes (per-row walk) in the project's `closing-report.md` at project
close. Rows are the definition-of-done composition criteria (class **b**) and the
arcs-closed roll-up (class **a**), per the collaboration-framework's
`LEDGER-DISCIPLINE.md` Section C. Composition rows are **reproduced at project
scale** (an end-to-end demonstration), never inherited from arc attestations.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| P-1 | Arc 01 (render core) closed + composed | ptr: `arc01-render-core/closing-report.md` | correctness | project-plan | done | reproduced at arc scale — Arc 01 closed 2026-07-28; A-4…A-7 green in clean build |
| P-2 | Arc 02 (routes & static export) closed + composed | ptr: `arc02-routes-static-export/closing-report.md` | correctness | project-plan | done | reproduced at arc scale — Arc 02 closed 2026-07-28; real 293-file `_site/` export, B-5…B-8 green |
| P-3 | Arc 03 (tiny_http server) closed + composed | ptr: `arc03-tiny-http-server/closing-report.md` | correctness | project-plan | done | reproduced at arc scale — Arc 03 closed 2026-07-29; C-1…C-5 green against a live hardened `haleiki serve` (route sweep, HEAD/405/nosniff, panic isolation, export parity) |
| P-4 | Arc 04 (search / Pagefind) closed + composed | ptr: `arc04-pagefind-search/closing-report.md` | correctness | project-plan | done | reproduced at arc scale — Arc 04 closed 2026-08-21; D-3...D-6 green against a fresh export, live `haleiki serve`, and Chromium (`garbage` query, live facets, static/live parity) |
| P-5 | Arc 05 (news content type) closed + composed | ptr: `arc05-news-blog-content-type/closing-report.md` | correctness | project-plan | open | |
| P-6 | Arc 06 (polish/parity/deploy) closed + composed | ptr: arc06 closing-report | correctness | project-plan | open | |
| P-7 | `haleiki build` emits a static `_site/` that renders the demo **wiki** with parity to the design review | reproduce: build, open in browser, diff vs review mockups | serious | DoD | open | |
| P-8 | `haleiki serve` serves the same site over tiny_http with configurable bind/port, structured errors + logging | reproduce: serve, curl routes + assets, kill/restart | serious | DoD | open | implemented in Arc 03 (`haleiki serve`: bind/port, HEAD/405, nosniff, access log, panic isolation — audited live); stays open pending project-scale reproduction in Arc 06 |
| P-9 | Client-side search returns faceted results via Pagefind | reproduce: query in browser, exercise each facet | serious | DoD | open | implemented in Arc 04 and reproduced at arc scale (`garbage` query returned 14 highlighted metadata-bearing results; live Category/Tier/Type/Trust counts rendered; query+category composition narrowed/restored results; static/live URL parity held). Stays open pending project-scale recomposition in Arc 06/final close rather than inheriting arc evidence into a DoD row. |
| P-10 | The **news** section renders end-to-end in both static export and server | reproduce: build + serve, walk index/post/archive/tags/categories | serious | DoD | open | |
| P-11 | No Cobalt / Liquid references remain on the branch (Pagefind is now intended — decision #3 reversed, v1.4) | `grep -ri -e cobalt -e liquid` returns only historical/doc mentions | correctness | DoD | open | |
| P-12 | Visual + functional parity with the design review accepted by the operator | operator acceptance judgment, recorded | serious | DoD | open | |
