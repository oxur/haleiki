# Slice 02 ledger

All rows open at issuance. CC returns per-row proposed evidence; CDC closes
only after independent review at the exact candidate. Composition remains in
the arc ledger, without inheriting child evidence as arc acceptance.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|---|---|---|---|---|---|---|
| F-1 | Real news index uses every post in date-descending/slug-ascending order, hero/next-two/remainder slots and optional excerpt behavior | five-post fixture with opposing date/slug order and same-day ties; actual index template assertions | correctness | roadmap Slice 02 | open | |
| F-2 | All-post archive has newest month/year groups, each post once, explicit pagination null and no pager/date-page routes | exact multi-year fixture groups/counts plus presence-aware serialization and real HTML | correctness | approved G1 | open | |
| F-3 | News categories have distinct-post counts, slug order, optional curated descriptions and first-three recent posts, separate from wiki taxonomy | fixture exact category vectors/recent links; approved demo description; absent/unused config controls | correctness | approved G2 | open | |
| F-4 | News tags have distinct-post counts, slug order, documented small/medium/large sizes and correct ordered sections; repeated identical tags render/count once | five-post exact tag vectors and size boundaries; conflicting normalized-label rejection; cross-namespace coexistence | correctness | approved G2 | open | |
| F-5 | Every singular category/tag news link resolves to a real matching target, alongside four plural/global routes; unknown labels 404 | exact route-set oracle, selected-only category/tag HTML, live/static browser link walks | serious | existing post/index template links | open | |
| F-6 | Prev points older, next newer, boundaries are present null keys and related is always explicit empty under the documented no-related policy | exact five-post neighbor table including ties/ends, one-post null/null/[] and real markup | correctness | arc E-6 | open | |
| F-7 | Exact route collisions fail before overwrite and before export cleanup; same bare slugs on distinct routes remain usable | reserved news filenames, legacy full-path fixture, output sentinel and positive namespace controls | correctness | Slice 01 overwrite guard | open | |
| F-8 | Empty corpus and absent optional data render honest empty states; metadata is autoescaped; only rendered Markdown body is safe; count copy is grammatical | zero/one/many fixtures, hostile metadata/description/neighbor strings, real templates and presence-aware context checks | correctness | template contract / preview findings | open | |
| F-9 | Approved demo post and all wiki source/concept content, graph/profile behavior and source/concept-only Pagefind remain intact; no dead RSS link | corpus/route/search preservation tests and full derived export population; byte-preserved authored fixture; no feed link/output | serious | approved G3 / Slice 03 boundary | open | |
| F-10 | Candidate passes native/full-feature/no-default/browser gates with exact receipts, scoped source commit, docs and intake/self-review/return evidence | make check; no-default workspace; three browser gates; actual test populations/exits; closing-report; binary preservation and clean scoped commit | correctness | quality floor / Autonomous authority | open | |
