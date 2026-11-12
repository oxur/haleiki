# CC closing report — Arc 05 Slice 01, iteration 01

## Assignment, ownership and intake state

Current assignment: `arc05-news-blog-content-type/slice01-news-model-and-post/cc-prompt-iteration01.md`, read in full (lines 1–376; the initially truncated read was recovered with contiguous ranges 1–94, 95–210 and 211–376). SHA-256: `2e78d1481c57c89054ab47f756d78c4564a5fb838c43f023086212f2f6740002`. It succeeds the preserved, unchanged `cc-prompt.md`; this is the first executed implementation iteration, not a completed correction pass. The issuance commit is `2d5e1486e2c9ba5bc111b42603bce9ab4a46c736`.

- **Source:** `/Users/oubiwann/lab/oxur/haleiki`, branch `framework-base/tiny-http`, intake HEAD `75d2593d51c986346f0239bed36a248ac5b4e6f7`, clean; no dirty paths.
- **Planning:** `/Users/oubiwann/lab/oxur/haleiki/.worktrees/planning`, branch `planning`, HEAD `2d5e1486e2c9ba5bc111b42603bce9ab4a46c736`, clean; issuance commit resolves to that same HEAD.
- **Contributor attribution:** Operator/dispatch requested OpenAI `gpt-6-luna`, reasoning `high`; contributor tool accepted/selected those settings. Actual context ID: `01a1ccf7-3e7c-7ab2-ae59-72c9be8f457b`, nickname Kierkegaard. Backend identity is unreported. CDC Locke: requested/tool-selected OpenAI `gpt-6.1-sol / high`, context `01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`; backend identity is unreported.
- **Mutation ownership:** CC owns source edits, generation, source staging and gates during this assignment, plus only this slice's `closing-report.md`. CDC owns planning staging/commits, ledgers and acceptance. No other source writer was observed during intake. No worktree, external message, push, merge, publish or deploy was performed.

### Required-reading record

All paths below resolve under the assignment's exact SOURCE, PLANNING and SKILLS roots. The source tree was clean at the baseline above; planning text was read at the clean issuance HEAD. Every initially truncated combined output used for the required packet was recovered using the bounded ranges below. These are CC reading attestations, not independent comprehension or acceptance.

| Item | Loaded extent and state |
|---|---|
| 1. Current assignment, slice plan and slice ledger | Assignment lines 1–376 in contiguous bounded reads; `slice-plan.md` full (127 lines); `ledger.md` full (31 lines), issuance state. |
| 2. Instructions and planning migration boundary | Source `AGENTS.md` full (120); planning `AGENTS.md` full (24); planning `README.md` full (63), intake state. |
| 3. Parent plans and ledgers | Project plan full, lines 1–265 in two bounded reads; project ledger full (24); Arc 05 plan full (185); Arc 05 ledger full (17), issuance state. |
| 4. Preserved history | `slice-doc.md` full (98) and predecessor `cc-prompt.md` full (86); unchanged history. The canonical decision log and current prompt resolve their historical discussion gate without rewriting either file. |
| 5. Collaboration and planning mechanics | Collaboration Framework `SKILL.md` v3.0.1 full, lines 1–574 in three bounded reads; Project Management `guides/README.md` full, lines 1–232; canonical planning guide sections “Existing records and supporting evidence” (89–113), “The per-slice documents and artifact home” (156–195), “Slice iteration filenames and preservation” (302–end); top-down guide section “Issuing and executing an iteration” (202–256). |
| 6. Roles, autonomy and intake contract | Engineering Methodology lines 26–153 and 326–550; Prompt Authoring lines 297–397, including the complete intake and review-evidence contract. |
| 7. Contributor boundaries and return | Agent Coordination `01-when-to-delegate.md` full (146; named Autonomous exception section), `02-context-packets.md` full (119; named Autonomous packets section), and `03-result-integration.md` full (92; named Autonomous returns section). |
| 8. Rust guidance | Repository `assets/ai/rust/SKILL.md` v2.0.0 full, lines 1–611; its symlink resolved to `/Users/oubiwann/lab/billosys/ai-engineering/knowledge/rust` (no substitution). `11-anti-patterns.md` full, all 2,188 lines in contiguous bounded reads. Also loaded error-handling lines 1–44, 168–328, 444–493; type-design 43–209; core-idioms 191–220 and 745–780; API-design 33–88. |
| 9. Testing and verification | Testing `SKILL.md` (35), testing guides 01 (91) and 03 (132), work-verification `SKILL.md` (51), and guides 01 (115), 02 (72), 03 (81), 04 (59), 05 (147), each full. |
| 10. Binding templates, parser and package/dependency model | Templates README (119), news-post template (51), base template (50), parser (527), error (46), workspace Cargo manifest (3), tools manifest (66), all full at source baseline. |
| 11. Render and call-site wiring | article.rs 1–218 and 363–403; render.rs 1–43 and 339–393; markdown.rs 1–115 and 415–435; server.rs 1–125 and 560–750; export.rs 1–438; search.rs 1–81 and 229–307; main.rs 1–28 and 126–206; validator.rs 140–270; graph.rs 97–130 and 635–659; generator.rs 610–740; demo/build.rs 235–255; export_cli.rs full (34). Complete named ranges and enclosing definitions were loaded. |
| 12. Gate behavior and pinned dependencies | Makefile lines 1–33, 125–171 and 186–328. Cargo.lock's complete matching package blocks and direct selection: serde_yaml 0.9.34+deprecated (alongside an unrelated 0.8.26 lock entry), minijinja 2.21.0, comrak 0.54.0, Pagefind 1.5.2; `tools/Cargo.toml` direct dependency block was read in full. No dependency change is authorized. |
| 13. Full data queries | Exact commands/results and denominators are recorded below; separate from prose-reading extents. |

### Required-data query receipts

1. A Python query enumerated every recursively discovered regular `*.md` in `demo/` and `content/`; for direct authored files it listed every path in `demo/sources/*.md`, `demo/concepts/*.md`, `demo/news/*.md`, and matching directories under content. It scanned each Markdown file's initial YAML frontmatter for `type` or `page_type` whose value is `news` or `blog`. Exit 0. Population: demo 752 Markdown files, 91 direct sources, 186 direct concepts, 0 direct news; content 0 Markdown and 0 direct source/concept/news. Matches: 0. Read/decoding errors: 0. Direct-file listing was fully returned. The recursive scan included every discovered Markdown file; no generated directory was silently excluded. Generated copies were not counted as independent authored posts; the zero direct-news count is the relevant authoring population.
2. `rg -n 'Corpus \\{' tools/src tools/tests` (run against absolute source paths) exited 0 and returned 18 textual matches: 13 actual literals (parser production constructor; graph 1; generator 1; validator 10), plus the Corpus declaration and four helper-function declarations/signatures in graph/generator/article/search. All literal enclosing expressions were inspected. The authorized fixture impact is limited to the graph/generator/validator tests, while the parser production constructor receives the populated news map.
3. Repository status and HEAD checks exited 0 in both worktrees; both porcelain outputs were empty. Prompt SHA-256 and issuance commit identity checks exited 0.

## Contract readback before implementation

The source parser currently owns only source/concept maps (`tools/src/parser.rs:16–25,218–352`), and the existing consumers deliberately treat those as the wiki population: graph/statistics/validators and Pagefind custom records do not automatically include another content type (`tools/src/graph.rs:97–130`, `render.rs:339–393`, `search.rs:56–79,229–284`). Slice 01 therefore adds an independent news map and one route while preserving those existing populations. Its one approved file is maintainer demo content, not a product index or a seed for a recommendation policy (canonical decision log; F-1/F-2).

The news parser must be a strict boundary: only direct regular Markdown files, sorted before insertion; a missing news directory is empty, but directory/file I/O and malformed input fail with path/field context. News frontmatter is checked at byte zero for the opening fence. Required values are exact strings/sequences, not YAML-coerced strings; dates must be canonical, real Gregorian dates; identity comes from the filename and shared Unicode-aware slugifier. Duplicate normalized slugs and empty derived slugs fail before map overwrite (prompt steps 1–2; F-3/F-4/F-6). Existing source/concept parsing stays intact.

The new renderer feeds owned model values into the existing forced-HTML-autoescape environment and actual `pages/news-post.jinja` (`render.rs:26–31`; template contract). Only the Markdown body goes through the existing renderer and the template's existing `post.body | safe`; title, author, category and tags remain normal escaped strings. Excerpt is stored but not synthesized or made visible without a template contract. The sole post serializes `prev: null`, `next: null`, and `related: []`; absent byline stays absent. No trust bar, template redesign, news listing, archive/taxonomy routes, feed or news search is part of this slice (F-5/F-7 and held-gate section).

`server::render_routes` is the shared sink: export writes its route map, while serve resolves the same rendered bytes (`server.rs:56–124`, `export.rs:30–56`). Add the post there once, with collision detection against existing route keys before insertion. Existing Pagefind construction is a separate custom-record pipeline and remains concepts plus sources (`search.rs:56–79,229–284)); this increment must prove the new route works through resolve/export without accidentally indexing it. A malformed corpus must fail before a partial successful site is returned.

The acceptance set remains all eight open rows in `ledger.md`. F-1/F-2 need the canonical approval and fresh corpus audit; F-3/F-4 need strict parser controls; F-5/F-7 need real-template, serialization and escaping evidence; F-6 needs exact shared-route set, collision rejection, resolve/export path and search/wiki preservation; F-8 requires the exact Make gate plus the named targeted commands. CC evidence will be attested/proposed-done only until Locke independently checks it. Browser gates are not triggered by this Rust/template-context/docs-only change because no browser assets or harness behavior is changed; their Arc-level obligations remain.

No plan/source conflict or unresolved consequential design choice was found at intake. The approved assignment and existing source contracts are coherent; implementation proceeds within the enumerated allowlist.

## Implementation, self-review and proposed disposition

**CC status: proposed-done / attested, pending CDC reproduction and acceptance.** This section records the stable committed candidate. It is not a ledger update or an acceptance claim.

- Final source: `/Users/oubiwann/lab/oxur/haleiki`, branch `framework-base/tiny-http`, HEAD `cb31dc92caa667ce6f3a7133163c3da8749549b1`; parent `75d2593d51c986346f0239bed36a248ac5b4e6f7`. Source `git status --short` was empty after commit. Commit subject: `feat(news): add strict single-post model and routes`. It has the required contiguous Codex and Billo AI trailers.
- Planning remained at issuance HEAD `2d5e1486e2c9ba5bc111b42603bce9ab4a46c736`; the only CC-owned planning path is this `closing-report.md`. No ledger or other planning path was changed or staged. The current assignment hash remains `2e78d1481c57c89054ab47f756d78c4564a5fb838c43f023086212f2f6740002`.
- Tool configuration and identity: requested and tool-selected OpenAI `gpt-6-luna`, reasoning `high`; context `01a1ccf7-3e7c-7ab2-ae59-72c9be8f457b`, nickname Kierkegaard. Backend identity was not reported. CDC Locke was requested/tool-selected OpenAI `gpt-6.1-sol / high`, context `01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`; backend identity was not reported.
- Changed source paths (all and only the authorized allowlist): `AGENTS.md`, `README.md`, `demo/news/search-lands-in-the-demo-wiki.md`, `tools/src/generator.rs`, `tools/src/graph.rs`, `tools/src/main.rs`, `tools/src/news.rs`, `tools/src/parser.rs`, `tools/src/search.rs`, `tools/src/server.rs`, `tools/src/validator.rs`, `tools/tests/export_cli.rs`. No templates, browser files, package pins, original sources/concepts, migration checker, or issued prompts changed.

The implementation adds an independent `Corpus.news` map. News ingestion reads sorted direct regular Markdown files, treats only a missing directory as empty, and carries read/frontmatter/field context on errors. Required YAML values are checked without scalar coercion; dates are real Gregorian calendar dates; category/tag/post slugs use the existing slugifier; filename identity is stable across date changes; duplicate normalized slugs fail with both paths. Optional blank author/excerpt values normalize to absence. Body Markdown remains separate from escaped metadata.

`news::render_post` supplies the real existing post template through the shared renderer. Only the rendered Markdown body reaches the template's existing `safe` boundary. The post context serializes null previous/next and an empty related list; absent author produces no byline; the news template carries no trust bar. `render_routes` adds the sorted post route once and errors before replacement on collision. Existing concept and source namespaces remain valid even when their bare slug matches the post slug. Export and serve consume the same shared route map. Graph/wiki statistics and Pagefind records remain based on the prior concept/source populations.

### F-1 through F-8 walk (each attested, proposed-done)

| Row | CC evidence and disposition |
|---|---|
| F-1 | The canonical decision log and current issuance assignment record the Operator's four approved Slice01 choices; readback completed before editing. One approved demo article was added at the exact path and with the approved values/body. Approval provenance is recorded, not re-decided here. **Proposed-done.** |
| F-2 | Full post-edit audit: 753 Markdown files recursively in `demo/` (the baseline 752 plus this post), 91 direct source files, 186 direct concept files, exactly one direct news file; `content/` has zero Markdown files. Exactly one news/blog type/page_type frontmatter match exists, this approved post. UTF-8 read errors: zero. No generated paths were excluded from the recursive scan; generated copies were not counted as independent editorial files. The pre-edit full listing and query are in intake receipts. **Proposed-done.** |
| F-3 | Parser model tests assert the approved fields/body, date/URL/filename identity, category/tag slugs, absent-directory behavior, empty tags/optional fields, and date-independent URL. Full route/export runs parse the complete corpus; existing source/concept body-preservation tests and suite remain green. **Proposed-done.** |
| F-4 | Strict required-field missing/null/wrong scalar/sequence/mapping controls; exact type, blank title/category, invalid tag elements, blank/slug-empty tag/category/filename controls; valid empty tags; leap-century/month/day and canonical-format matrix; opening/closing fence and invalid UTF-8 controls; duplicate normalized names report both paths. Final parser selector: 19 passed, 0 failed, 1 pre-existing ignored. **Proposed-done.** |
| F-5 | The existing `pages/news-post.jinja` renders approved fields and display date. Tests check category and all three tag URLs, byline/body, individually escaped title/category/tag/author, safe Markdown structural HTML, explicit serialized null/null/[], absent byline/navigation/related markup, and no trust bar. Final news selector: 4 passed, 0 failed. **Proposed-done.** |
| F-6 | Route-key equality extends the pre-existing wiki set by exactly the one post. Resolve returns the post and matches the exported file bytes; `export_cli` writes the path/content. A legacy segmented concept slug colliding with `/news/<slug>/` fails with that route in the diagnostic. A separate isolated test proves same-bare-slug concept `/slug/`, source `/source/slug/`, and news `/news/slug/` all coexist. Search tests retain exactly concepts + sources and no `/news/` URL. Existing graph/statistics/render tests remain green. **Proposed-done.** |
| F-7 | Shared route map means serve/export use identical rendered bytes; the server parity oracle includes the news URL. No recommendation policy, news index, archive, category/tag route, feed or Pagefind record was added. Existing whole-suite determinism/search/wiki preservation controls passed. **Proposed-done.** |
| F-8 | Final-candidate `make check` exit 0: dependency freshness, Clippy, format, debug build and all workspace/integration tests. Direct `cargo outdated --root-deps-only --ignore-external-rel` exit 0. Final-candidate no-default workspace suite exit 0. Named focused commands and populations are in the receipts below. Browser gates were not triggered: this change affects Rust parsing/render context, tests and prose only; no browser asset/harness/shared browser behavior changed. **Proposed-done.** |

### Final-candidate validation receipts

All commands below used the explicit source workdir. “Final candidate” means the committed tree at `cb31dc92caa667ce6f3a7133163c3da8749549b1`; focused parser and route namespace selectors were rerun after their final test additions. The last full `make check` and no-default run occurred after all source changes.

| Command | Exit and observed population |
|---|---|
| `cargo test --all-features --workspace parser::tests::` | 0; 19 passed, 0 failed, 1 ignored. |
| `cargo test --all-features --workspace news::tests::` | 0; 4 passed, 0 failed. |
| `cargo test --all-features --workspace server::tests::test_render_routes` | 0 on the route-set/collision selector; 2 passed, 0 failed on that receipt. The additional final namespace selector below also passed and is included in final `make check`. |
| `cargo test --all-features --workspace server::tests::test_news_route_coexists_with_concept_and_source_sharing_bare_slug` | 0; 1 passed, 0 failed. |
| `cargo test --all-features --workspace search::tests::test_build_records` | 0; 2 passed, 0 failed. |
| `cargo test --all-features --workspace --test export_cli` | 0; 1 passed, 0 failed. |
| `cargo test --no-default-features --workspace` | 0 on the final candidate; unit suite 132 passed, 0 failed, 1 ignored; export and search CLI integration tests each 1 passed. It emitted existing unused-import warnings in `tools/tests/demo_fetch.rs` and `tools/tests/demo_status.rs`; those out-of-scope files were left untouched. |
| `cargo test --all-features --workspace -- --quiet` | 0 on the committed candidate; unit suite 416 passed, 0 failed, 8 ignored; `demo_fetch` 14 passed/3 ignored, `demo_status` 3 passed, `export_cli` 1 passed, `search_cli` 1 passed. |
| `cargo outdated --root-deps-only --ignore-external-rel` | 0. Reported newer root versions for clap, globset, minijinja, reqwest, scraper, serde_json, thiserror and tokio; no pins were changed. The Makefile wrapper separately printed its own “All dependencies up to date” message while listing newer packages; the direct command is the retained receipt. |
| `make check` | 0 on the final candidate. Clippy and format passed; debug build succeeded; all-feature unit suite reported 416 passed, 0 failed, 8 ignored; `demo_fetch` 14 passed/3 ignored, `demo_status` 3 passed, `export_cli` 1 passed, `search_cli` 1 passed. The command ended “All checks passed (build + lint + test)”. |
| `git diff --check` and staged diff check | 0. Final commit contains exactly the 12 listed source paths. |

`make check` removes `bin/`. Before the first run, the pre-existing ignored `bin/haleiki` was copied to `/private/tmp/haleiki-slice01-original-bin`; after the final run it was restored. Both hashes were `4df7d22f32bbeb7bcde62c62e100917987199fc71729179f85b877a1c3405f6e`. The temp backup is operational evidence only and is not a repository artifact. Test-created temporary output directories were cleaned by their tests.

### Failures and corrections retained

1. Initial parser compile failed (exit 101): a local closure returning a borrowed line ending did not satisfy the inferred lifetime. Replaced it with a small helper. Clippy later flagged the helper's local placement; it was moved to module scope.
2. Parser fixture run failed five tests (exit 101): negative controls inspected only the outer anyhow display instead of the full error chain, and the date-change fixture omitted its opening/closing fences. Corrected the assertions to retain the chain and fixed the fixture. The next parser run exposed one more assertion that inspected only outer context for an empty filename slug; corrected it. Final parser gate passed. Later added empty-tag-slug and present-non-directory I/O controls; final parser suite is 19/1.
3. Initial news-render test failed one assertion because CommonMark retained the source line break in the output. The semantic assertion now checks both paragraph fragments while separate body-safety tests remain structural. Final news suite passed.
4. Initial route-collision fixture failed before reaching the collision because its concept did not satisfy the real concept template. Replaced it with a complete demo concept. The next attempt correctly surfaced dangling provenance in the isolated corpus; cleared that fixture-only provenance and the collision oracle passed. Added and passed a separate bare-slug namespace test.
5. First `make check` stopped at Clippy (exit 2) on seven implementation lints. The first direct Clippy rerun found seven follow-up needless-borrow lints (exit 101). Corrected doc markup, Copy date receiver style, helper placement, mapping-key borrows and parser ownership; direct Clippy then passed. Subsequent full checks passed. No failure was hidden or converted into a passing receipt.

### Applied-guideline self-review and limits

The private post/date model centralizes validated invariants (TD-02/03/05, AP-06); strict frontmatter and slug/route errors return contextual `Result`s rather than coercing, panicking or silently overwriting (EH-01/05/08/12, AP-09/25/33). Rendering borrows shared environment/site/corpus and returns owned HTML (API-02, ID-08); optional links/byline use `Option`, related uses an empty `Vec`, and serialization is tested (ID-30, AP-67). Template autoescaping stays enabled; only the existing rendered-body safe slot is used. Input and route order are deterministic. Clippy and format are green. No dependency, background task, mutable global, custom validator, cache or new product surface was introduced.

The existing post template intentionally links to future `/news/`, category and tag pages; these links are **pending surfaces**, not full-link-parity passes. Archive behavior, taxonomy, feed/link treatment, news search participation, full arc composition and browser validation remain held/later work. CC has not accepted any row on CDC's behalf.

## Bubble-up to Arc 05

- **Delivered capability:** strict authoring model for one dated post; stable filename route; safely rendered post template; one demo example; same route bytes through local serve/export; explicit collision handling and source/concept namespace preservation.
- **Discoveries:** no authored news/blog frontmatter existed at baseline. The repository's existing source/concept graph and Pagefind custom-record pipeline stay separate from the added post map. The shipping template contains future-surface links, so the single-post capability is usable through its direct URL while navigation taxonomy/archive behavior remains unresolved by design.
- **Scope-as-specified / scope-as-delivered:** delivered the approved single-post increment within the explicit file allowlist. No listing, archive, taxonomy, feed, search inclusion, trust bar, visual redesign or Arc06 cleanup was added. One route collision with a legacy segmented concept slug is now diagnosed rather than overwritten.
- **Handoff:** source mutation ownership is now returned explicitly to CDC Locke through the parent relay. Source HEAD and tree are stable and clean at `cb31dc92caa667ce6f3a7133163c3da8749549b1`. CDC owns independent reproduction, acceptance, planning staging/commit and all later ledger/retrospective updates. This report remains the only CC-created planning change and is unstaged.

## Iteration 02 — R1 explicit-null test correction (2026-11-11)

### Assignment and intake

Current assignment: `cc-prompt-iteration02.md`, read in full (160 lines), SHA-256 `af48b57b82ba72758bbab5f2b673ef5ea3a2baaedcffd888a7a75a91a618854f`. It is the active, bounded test-only correction. `cc-prompt-iteration01.md`, its report above, and the review record remain unchanged.

- At intake, SOURCE `/Users/oubiwann/lab/oxur/haleiki` was branch `framework-base/tiny-http`, HEAD `cb31dc92caa667ce6f3a7133163c3da8749549b1`, clean. PLANNING `/Users/oubiwann/lab/oxur/haleiki/.worktrees/planning` was branch `planning`, HEAD `685a4fa5e64927e5664fe399428160f92e950ca4`, clean. The only change allowed to source in this pass was the existing serialization test in `tools/src/news.rs`; planning ownership is limited to appending this report.
- Dispatch and tool selection remain OpenAI `gpt-6-luna / high`, actual CC context `01a1ccf7-3e7c-7ab2-ae59-72c9be8f457b` (Kierkegaard). CDC Locke remains `gpt-6.1-sol / high`, context `01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`. Backend identities were not reported.

### Required reading and retained findings

| Required item | Loaded extent/state |
|---|---|
| Iteration-02 assignment | Full 160 lines at planning issuance `685a4fa5e64927e5664fe399428160f92e950ca4`. Its single-path allowlist, four commands, binary-preservation condition, R1 repair, retained-evidence rule and all-row-open status governed this pass. |
| Slice plan and ledger | `slice-plan.md` full, 156 lines; `ledger.md` full, 35 lines, both at iteration-02 issuance. All F-1…F-8 statuses remain open. |
| CDC independent review | `cdc-verification.md` full, 140 lines at issuance. R1 is a test false-positive only: bracket indexing returns a static JSON Null for a missing key; CDC found the production `PostContext` fields present. Its independent parser/render/route/search/export/CLI/permission evidence and limitations were read as prior-revision evidence, not recast as fresh CC runs. |
| Iteration-01 report and assignment | `closing-report.md` full, 117 lines, recovered in contiguous bounded reads after a truncated combined read; iteration-01 assignment full, lines 1–376, recovered/read across ranges 1–200 and 201–376. The prior report/failure history is preserved verbatim above. |
| Repository instructions | Source `AGENTS.md` full, 122 lines; planning `AGENTS.md` full, 24 lines. Existing source branch, model, project and gate boundaries remain binding. |
| Test and pinned source | `tools/src/news.rs` full, 195 lines at `cb31dc92`; Cargo.lock's complete `serde_json` 1.0.151 package block, lines 3001–3012; pinned registry `value/index.rs` lines 178–220 and `value/mod.rs` get API/doc section lines 260–330. The index API returns `&NULL` when the key lookup is absent; `Value::get` returns `None`, whereas an explicitly serialized null is `Some(&Value::Null)`. |
| Corrective/intake guidance | Prompt-authoring guide “Required reading and CC intake” lines 297–398 and “Corrective iterations” 441–474; independent-verification “Evidence During Correction Review” lines 80–106; validation-gates “Proportionate Proof” lines 27–50. Each full named section was loaded. |
| Retained iteration-01 normative reading | Still available in this same CC context and recorded in the report above: Collaboration Framework v3.0.1; Autonomous/model/ownership and agent-coordination contracts; Rust v2.0 entrypoint and full 2,188-line anti-patterns chapter plus applicable Rust guides; testing and work-verification guides; project-management planning/iteration contracts. These are retained as prior reads, not claimed as reread in this correction. |

The intake readback is narrow: only `value.get("prev")`/`get("next")` paired with `Some(&serde_json::Value::Null)` proves that both keys exist and contain null. If either key is omitted, `get` yields `None`, which cannot equal `Some(Null)`. Square-bracket indexing cannot establish this because its pinned behavior intentionally maps the missing-key case to `Null`. The correction keeps serialization of the real `PostContext::from_post` fixture and leaves the `related == []` assertion and rendered no-byline/no-navigation/no-related checks intact. Production behavior, context struct, fixtures and product decisions do not change.

### Correction, commit and current state

Changed exactly the two assertions in the named existing test. Source commit: `5fe3cf46fa1ea673b0b84f09167e36de8611d3ee`, parent `cb31dc92caa667ce6f3a7133163c3da8749549b1`, subject `test(news): assert explicit null context keys`; exact changed path: `tools/src/news.rs` only (2 insertions, 2 deletions). Required Codex and Billo AI trailers are contiguous. Source is clean at that commit. No production code, fixture, template, dependency or other path changed. The assignment hash remains as listed above; planning issuance remains `685a4fa5e64927e5664fe399428160f92e950ca4` until CDC commits its report update.

### Fresh gates and retained independent evidence

| Check on the corrected candidate | Result |
|---|---|
| `cargo test --all-features --workspace news::tests::` | Exit 0; all 4 renderer tests passed, 0 failed, 420 filtered. This includes the actual-context Some(Null) assertions, unchanged `related=[]`, and real-template absence controls. |
| `cargo test --no-default-features --workspace` | Exit 0; unit tests 132 passed, 0 failed, 1 ignored. `demo_fetch` and `demo_status` selected 0 each under these features; `export_cli` and `search_cli` each ran 1 and passed. Existing unused-import warnings in the two zero-selected integration targets remain outside this one-file scope. |
| `cargo outdated --root-deps-only --ignore-external-rel` | Exit 0. Stderr: `warning: Feature loader of package minijinja has been obsolete in version 3.1.0`. Stdout lists newer root versions of clap, comrak, ego-tree, globset, htmd, minijinja, pagefind, reqwest, scraper, serde_json, thiserror and tokio. No dependency was changed. The Make wrapper's “All dependencies up to date” banner does not cancel this direct version report. |
| Pre-`make check` binary preservation check | Both `bin/haleiki` and `/private/tmp/haleiki-slice01-original-bin` were regular non-symlink files, uid 501, mode 0755; both SHA-256 values matched the required `4df7d22f32bbeb7bcde62c62e100917987199fc71729179f85b877a1c3405f6e`. |
| `make check` | Exit 0. Clippy, formatting and debug build passed; all-feature unit tests 416 passed, 0 failed, 8 ignored; `demo_fetch` 14 passed/3 ignored, `demo_status` 3 passed, `export_cli` 1 passed and `search_cli` 1 passed. Ended “All checks passed (build + lint + test)”. |
| Post-run binary restoration | Rechecked the backup as regular, non-symlink, uid 501 and the expected hash before copying it back. Restored `bin/haleiki`; both files again hash to the expected value. |
| `git diff --check` | Exit 0 before commit and on the clean committed tree. Cached diff check also exited 0 and contained only `tools/src/news.rs`. |

No test, compile, lint, permission or sandbox failure occurred in iteration 02; the combined reading output truncation was recovered with bounded reads. The earlier implementation and first-gate failures remain documented in the preceding iteration-01 section and were not erased or counted as failures of this correction.

CDC's recorded evidence at `cb31dc92` remains a prior independent review, not a fresh CC run: strict parser and data results (F-1…F-4), unchanged route and namespace behavior (F-6), and its exact real-export/CLI/permission controls are not invalidated by this two-assertion test-only change. Their original source revision, methods, populations and limits remain in `cdc-verification.md`. The renderer/context claim (F-5/F-7) was rerun here with the corrected oracle; F-8's no-default, direct dependency query and full `make check` were freshly run on the corrected candidate. CDC decides whether the unaffected evidence remains applicable after inspecting this diff.

### F-row accounting, scope and handback

All eight rows remain open; these are CC attestations only.

- **F-1/F-2:** retain CDC's independent approval-log and full-corpus audit at `cb31dc92`; this test-only correction changes neither approval nor authored content.
- **F-3/F-4:** retain CDC's independent parser/data evidence at `cb31dc92`; no parser or fixture changed.
- **F-5:** corrected serialized-context test and the four-test renderer suite passed on this candidate; CDC's prior real-template/escaping evidence is retained as recorded.
- **F-6:** retain CDC's independent route-set, collision, positive namespace, search and real-export evidence; route/source behavior is unchanged.
- **F-7:** `get` now rejects omission for both navigation keys; explicit null/null plus the unchanged empty related list and rendered absence checks passed fresh. The independent reviewer must inspect the actual correction.
- **F-8:** no-default suite, direct outdated query and required `make check` passed fresh; full counts and binary preservation are recorded above.

The applied change follows ID-30's distinction between `Option` absence and explicit serialized null, using the pinned `Value::get` API. It repairs a discriminating oracle in the existing fixture, not production behavior. The correction is bounded to the review finding; no new behavior, fixture, tool, or evidence artifact was introduced. News listing/archive/taxonomy/feed/search participation and all later discussion gates remain held. Browser checks were not triggered because no browser assets or harness behavior changed.

Iteration-02 artifact inventory: no new artifact beyond this appended canonical report section. Source ownership is returned to CDC Locke through the parent relay; the stable source candidate is clean at `5fe3cf46fa1ea673b0b84f09167e36de8611d3ee`. Planning remains at `685a4fa5e64927e5664fe399428160f92e950ca4` with only this report modified by CC. CDC owns staging/committing the report, independent reproduction, acceptance and ledger updates. No F row is declared accepted here.

**Arc bubble-up:** Slice01's single-post capability and scope are unchanged. The correction closes a test-oracle ambiguity about explicit nulls; CDC's review found no production defect. Delivery remains exactly the approved single post and route. The later news navigation surfaces and all Arc05 composition gates remain open.

## CDC closing disposition — 2026-11-11 (Locke)

Independent acceptance: F-1…F-8 done at source
`5fe3cf46fa1ea673b0b84f09167e36de8611d3ee`; R1 resolved. The full
verification/retained-evidence assessment is in [cdc-verification.md](./cdc-verification.md).
The required closing analysis is [cdc-retrospective.md](./cdc-retrospective.md).
This attributed CDC appendix does not rewrite or promote the CC attestations
above. Source and candidate ownership have returned to CDC; there is no
active CC assignment. Planning input to this final review was `685a4fa`;
the local close commit containing this appendix is reported separately.

Final supporting-artifact inventory: `artifacts/cdc-review01-focused.log`,
`artifacts/cdc-final-gates.log`, `artifacts/gate-preview.html`,
`artifacts/gate-preview.png`, `artifacts/gate-contexts.json`, and
`artifacts/render-gate-examples.py`. The two logs retain independent
verification; the four draft-design outputs make the reserved archive/taxonomy
and visible-feed-link choices concrete. No illustration is a product route,
editorial post or composition acceptance. Canonical prompts/close documents
remain in the slice root and are excluded from this supporting inventory.

Bubble-up confirmed: the single-post capability is delivered; arc structure
and scope remain unchanged. E-1 rolls up the accepted child. Other arc and
project rows remain open pending their own work/composition. Current next
owner is the Operator via parent relay for the existing discussion gates;
recommendations and concrete renders are in the arc-plan packet.
