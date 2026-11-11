# CC assignment — Arc 05 Slice 01, iteration 01: news model and one post

Issued 2026-11-11 by CDC Locke, context `01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`.
Project: `project04-tiny-http`. Current project-relative assignment:
`arc05-news-blog-content-type/slice01-news-model-and-post/cc-prompt-iteration01.md`.
Predecessor: sibling `cc-prompt.md`, preserved unchanged. This replaces an
unstarted, discussion-gated assignment; it is not a completed correction round.
No source work has yet been accepted; F-1 through F-8 all remain open.

## Authority, roots, state and ownership

This is bounded **CC source implementation**, followed by CC self-review,
attested evidence and a local scoped commit. The Operator approved the four
Slice 01 decisions on 2026-11-11 and Autonomous Mode through **end of Arc 05**,
stopping at discussion gates. The selected arrangement is CDC + CC.

- `SOURCE=/Users/oubiwann/lab/oxur/haleiki`, branch `framework-base/tiny-http`,
  clean baseline `75d2593d51c986346f0239bed36a248ac5b4e6f7`.
- `PLANNING=/Users/oubiwann/lab/oxur/haleiki/.worktrees/planning`, branch
  `planning`. Preparation began at `89213a1696eee2c317ff7156dec0931bc405a1f2`
  with parent authorization edits to this slice plan and the arc plan. Those
  edits are retained in the CDC issuance commit. Record the actual issuance
  HEAD and prompt hash at intake; the commit cannot name its own hash here.
- `PROJECT=$PLANNING/project04-tiny-http` and
  `SLICE=$PROJECT/arc05-news-blog-content-type/slice01-news-model-and-post`.
- `SKILLS=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
- CC must be explicitly dispatched through the contributor tool as OpenAI
  `gpt-6-luna`, `reasoning_effort=high`; no substitution or inherited default.
  Record requested settings, tool-selected settings and actual context ID.
  Backend identity remains unreported unless the tool reports it.
- CDC Locke was requested and tool-selected as OpenAI `gpt-6.1-sol / high`
  according to the parent dispatch receipt; backend identity is unreported.
  Locke independently reviews source and owns acceptance, planning/index
  mutations, `cdc-verification.md`, and closing retrospectives.
- Parent performs dispatch/relay only. CC exclusively owns source mutation,
  generation and source staging while assigned. No overlapping source checks,
  edits, installs, exports or scanners; hand the stable candidate back to CDC.
- CC may write **only** `$SLICE/closing-report.md` in planning, as its designated
  handoff/evidence record. CDC will not edit it while CC owns it. CDC owns
  planning staging/commits and all ledgers; leave their statuses untouched.
  Use the report for intake, reading extents, command receipts and the F-row
  walk; create it only when recording actual work, never as a success stub.

Use absolute working directories on every shell call. The inherited app cwd is
stale/absent. These live roots may require `sandbox_permissions=require_escalated`
for writes, Git, builds and local binding. Existing authorization covers scoped
local work; retain denied attempts and rerun through the available escalation
flow. Preserve unrelated edits, outputs of unknown ownership, and all five old
Claude worktrees. Do not create worktrees or extra contributor roles.
No push, merge, publication, deployment, external messages or user-owned chats.
Do not open Arc 06. Local validation must not trigger external publication.

## Required reading and intake

Path prefixes above resolve exact absolute paths; ranges below refer to the
inspected source baseline. Record drift before acting. Planning text means the
CDC issuance state containing the retained parent edits, not the sealed archive.
Read in order before dependent edits; recover truncated content in contiguous
bounded reads. Source inspection and compilation do not replace normative text.

| Order | Exact path(s), source state and required scope | Purpose |
|---|---|---|
| 1 | This prompt; `$SLICE/slice-plan.md`; `$SLICE/ledger.md` — **required-full**, issuance state | Current authority, ownership, all eight rows |
| 2 | `$SOURCE/AGENTS.md`; `$PLANNING/AGENTS.md`; `$PLANNING/README.md` — **required-full**, intake state | Repo instructions and migration boundary; the source's old pending-decision sentence is superseded by the approved canonical log |
| 3 | `$PROJECT/project-plan.md` and `$PROJECT/ledger.md`; `$PROJECT/arc05-news-blog-content-type/arc-plan.md` and its `ledger.md` — **required-full**, issuance state | Scope, higher-scale obligations, unresolved later gates |
| 4 | `$SLICE/slice-doc.md`; `$SLICE/cc-prompt.md` — **required-full**, preserved history | Retained original scope; their initial gate is resolved only by the current decision log, not by rewriting history |
| 5 | `$SKILLS/collaboration-framework/SKILL.md` v3.0.1 — **required-full**; `$SKILLS/project-management/guides/README.md` — **required-full**, then `02-canonical-planning-worktree.md` sections “Existing records and supporting evidence”, “The per-slice documents and artifact home”, and “Slice iteration filenames and preservation”; `03-planning-top-down.md` section “Issuing and executing an iteration” — **required-section**, full named sections | Mode, history and handoff mechanics |
| 6 | `$SKILLS/engineering-methods/guides/01-engineering-methodology.md` lines 26–153 and 326–550 — **required-section**; `07-implementation-prompt-authoring.md` lines 297–397 (“Required reading and CC intake”, through the review-evidence paragraph) — **required-section** | Roles, Autonomous authority/model settings, stable-input and intake contracts |
| 7 | `$SKILLS/agent-coordination/guides/01-when-to-delegate.md` section “Autonomous Mode exception”; `02-context-packets.md` section “Autonomous Mode contributor packets”; `03-result-integration.md` section “Autonomous Mode contributor returns” — **required-section**, full named sections | Assigned contributor responsibility and return limits |
| 8 | `$SOURCE/assets/ai/rust/SKILL.md` v2.0.0; `guides/11-anti-patterns.md` — **required-full** before Rust writing/review. Same guide root: `03-error-handling.md` lines 1–44,168–328,444–493; `05-type-design.md` 43–209; `01-core-idioms.md` 191–220,745–780; `02-api-design.md` 33–88 — **required-section** | Applied rules below. Repo symlink resolves to `/Users/oubiwann/lab/billosys/ai-engineering/knowledge/rust`; if absent, use the installed `$SKILLS/rust-guidelines` counterparts and record the substitution of document source, not model |
| 9 | `$SKILLS/testing/SKILL.md`, its `guides/01-testing-discipline.md` and `03-validation-gates.md`; `$SKILLS/work-verification/SKILL.md`, its `guides/01-ledger-discipline.md`, `02-evidence-strength.md`, `03-row-closure.md`, `04-silent-drop-checks.md`, `05-independent-verification.md` — **required-full** | Behavioral proof, mandatory gates, evidence strength, close/independence |
| 10 | `$SOURCE/templates/README.md`; `$SOURCE/templates/pages/news-post.jinja`; `$SOURCE/templates/base.jinja`; `$SOURCE/tools/src/parser.rs`; `$SOURCE/tools/src/error.rs`; `$SOURCE/Cargo.toml`; `$SOURCE/tools/Cargo.toml` — **required-full**, source baseline | Binding context and existing parser/error/feature model |
| 11 | `$SOURCE/tools/src/article.rs` 1–218,363–403; `render.rs` 1–43,339–393; `markdown.rs` 1–115,415–435; `server.rs` 1–125,560–750; `export.rs` 1–438; `search.rs` 1–81,229–307; `main.rs` 1–28,126–206; `validator.rs` 140–270; `graph.rs` 97–130,635–659; `generator.rs` 610–740; `demo/build.rs` 235–255; `$SOURCE/tools/tests/export_cli.rs` — **required-section**, complete ranges (integration test required-full) | Reuse and caller/test wiring. Read enclosing definitions when newly edited code depends on them |
| 12 | `$SOURCE/Makefile` 1–33,125–171,186–328 — **required-section**; `$SOURCE/Cargo.lock` dependency entries for direct `serde_yaml` 0.9.34, minijinja 2.21.0, comrak 0.54.0 and Pagefind — **required-data**, inspect entire matching package blocks and direct dependency selection in `tools/Cargo.toml` | Gate side effects and pinned APIs; do not upgrade dependencies |
| 13 | Required-data queries in next paragraph, over the entire named population at intake | Verify author observations and all Corpus construction impact sites |

For item 13, enumerate direct authored `demo/sources/*.md`,
`demo/concepts/*.md`, `demo/news/*.md` and their counts; scan frontmatter (not
body mentions) in every Markdown file under `demo/` and `content/` for `type`
or `page_type` equal to news/blog. State any generated-tree exclusions and
read errors; generated copies are not independent authored posts. CDC observed
752 Markdown files recursively under demo, of which 91 direct sources and 186
direct concepts, zero direct news; content had zero Markdown. No news/blog
frontmatter match was found. This is an observation to recheck, not a required
answer. Run `rg -n 'Corpus \{' tools/src tools/tests` and inspect every real
struct construction's enclosing expression; distinguish declarations/functions
from literals. Query the full relevant paths, not a sample. Record commands,
exits, denominators and complete results separately from text-reading extents.

Before implementation, put a concise intake/readback in `closing-report.md`:
actual roots/branches/HEADs/dirty paths; chosen tool settings/ID; each required
item's loaded extent/state and any recovered truncation; data-query coverage;
then explain the binding scope, parser failures, sink/search separation and
the safe-HTML boundary in your own words with source/row citations. Resolve
contradictions with CDC; otherwise execute without another approval dialogue.
The reading packet is roughly 10,000 lines including source and guides, bounded
to this slice with implementation/recovery headroom; references outside named
normative dependencies are background, not an unbounded history-reading task.
Reload governing contracts no longer reliably available after recovery.

## Approved behavior and held decisions

**Binding, Operator 2026-11-11:** author `news/*.md` below the selected content
root (`demo/news/*.md` beside sources/concepts); filename-derived stable
`/news/<slug>/` independent of date. Required: exact `type: news`, nonempty
string title, real strict `YYYY-MM-DD` publication date, one nonempty string
category, and a string-list tags field (the list may be empty). Optional string
author/excerpt; absent author gives no byline. Markdown body is rendered HTML.
Derive category/tag URL slugs. News has no trust bar.

Add exactly one realistic demo post, with these approved values and body:

```markdown
---
type: news
title: "Search Lands in the Demo Wiki"
date: 2026-08-21
category: "Project Updates"
tags: ["search", "pagefind", "tiny-http"]
author: "Haleiki Maintainers"
excerpt: "Client-side Pagefind search is now wired into the demo wiki."
---

Search now supports highlighted results and category, tier,
content-type, and trust filters.
```

Path: `demo/news/search-lands-in-the-demo-wiki.md`.
With this one post, `prev` and `next` serialize explicitly as null and `related`
as `[]`. Isolated tests may create extra posts; do not seed extra editorial
content. Slice 01 supplies the single-post context, not a recommendation engine.
Do not invent a multi-post navigation/recommendation policy here; report a
changed authored population to CDC before dependent rendering changes.

**Held for Operator discussion:** archive pagination/date grouping after an
archive example; taxonomy vocabulary/descriptions after taxonomy render;
feed delivery versus documented deferral with its visible link handled; news
Pagefind participation now versus Arc 06. No `/news/` index, archives, taxonomy
routes, feed, news indexing, or visual redesign in this assignment. The real
post template's breadcrumb/category/tag links intentionally point at later
surfaces; disclose them as pending, not passing full-link parity. Full live
news-surface parity and all arc composition remain later obligations.

## Source-grounded implementation spine

CDC inspected baseline `75d2593d51c986346f0239bed36a248ac5b4e6f7`:
`parser.rs:16–25,218–352` loads only source/concept maps, taxonomy/profile;
`article.rs:159–216` illustrates real template rendering via the shared forced
HTML-escape environment (`render.rs:26–31`). `markdown.rs:67–115` supplies
Markdown HTML and the existing slug convention. `server.rs:56–124` assembles
the BTreeMap used by export and serve. `export.rs:30–56` writes that map and
then invokes a separate Pagefind builder; `search.rs:56–79,229–284` creates
custom records only for source/concept content, without crawling exported HTML.
The graph, wiki stats and validators inspect the two original maps explicitly.
`Corpus` literal additions are required in graph/generator/validator tests.

Implement in this order. Shapes below are recommended, **uncompiled sketches**;
private helper names may change. Binding outcomes/failures are not discretionary.

1. **`tools/src/parser.rs`: additive model and corpus loading.** Add
   `Corpus.news: HashMap<String, NewsPost>`, loaded by `parse_news` in
   `parse_corpus`. Absent news directory means empty map; a present directory
   read/parse error fails with path/field context. Read direct regular `*.md`
   files only, in sorted pathname order. Do not recurse into builds or consume
   `.gitkeep`/other extensions. Keep the existing source/concept parser paths.
   Add a validated `NewsPost` (title, date, category, tags, optional author and
   excerpt, filename-derived slug/url, original Markdown body), with owned data,
   private invariant-bearing fields and crate-visible read accessors. Add a
   small validated `NewsDate` with private numeric components and ISO/display
   accessors; no new date dependency is needed.

2. **Validate before insertion.** Reuse `parse_frontmatter` into
   `serde_yaml::Value`, then inspect string and sequence values strictly via
   `as_str`/`as_sequence` (existing use: `demo/build.rs:242–255`). Do not rely on
   a permissive `Deserialize<String>` to distinguish numbers/bools from strings.
   Require `type`, `title`, `date`, `category`, `tags`, and exact `news` type. Missing, null,
   scalar/sequence/mapping mismatches and whitespace-only title/category must
   return errors with field and file context. Every tag element must be a
   string with nonempty derived slug; empty tags sequence is valid. Optional
   author/excerpt may be absent/null; present non-null values must be strings.
   Normalize whitespace-only optional strings to absence. Do not synthesize an
   excerpt or byline. Ignore unrelated frontmatter fields; a supplied `slug`
   must never override the filename. Check the initial frontmatter fence for
   news instead of finding a later body fence; leave the legacy parser alone.

   Date acceptance: exactly ten ASCII characters, hyphens at offsets 4/7,
   digits elsewhere, year 0001–9999, month 1–12, and day valid for that month
   under Gregorian leap rules (`year % 4 == 0` except centuries unless `% 400
   == 0`). Reject timestamps, suffixes, non-padded fields, surrounding spaces
   and impossible days. Preserve canonical ISO date; display the approved
   example as **August 21, 2026**. Use English month names and an unpadded day;
   this post-display choice does not decide archive granularity.

   Derive post slug with existing `markdown::slugify(file_stem)`; the approved
   filename is unchanged. Reject empty results. Derive category/tag slugs with
   the same convention (`Project Updates` → `project-updates`, `tiny-http`
   unchanged); reject empty results. Detect duplicate normalized news slugs
   before HashMap overwrite, return both paths in the error. No silent winner.
   URL is exactly `/news/{slug}/`; dates and authored `slug` cannot change it.
   Retain the shared Unicode-aware slug convention rather than introducing a
   separate ASCII-only normalizer. Encoded non-ASCII HTTP URL handling is not
   newly promised here; preserve existing route behavior and flag limitations.

3. **`tools/src/news.rs` plus `main.rs` module registration.** Add a dedicated
   post renderer analogous to `article.rs`, using:
   `render_post(env: &Environment<'_>, site: &SiteContext, corpus: &Corpus,
   slug: &str) -> crate::error::Result<String>`.
   Missing slug returns `Error::Config`; propagate template errors. Assemble a
   serializable `PostContext` with `title`, `body`, `date_display`, category,
   category_slug, `tags: Vec<{name,slug}>`, author, prev, next, related. Use
   `markdown::render_markdown(post.body()).html` once for `body` and render
   **the existing** `pages/news-post.jinja`. Supply site `{data: site}` and a
   minimal page `{title,slug,data:{}}` for base-template access. Pass normal
   strings into minijinja; only the template's existing `post.body | safe`
   bypasses escaping. No global-safe strings, preescaped metadata, or autoescape
   changes. `prev: Option<PostLink>=None`, `next=None`, `related: Vec<_>=[]`
   are concrete serializable values; test their serialized shape directly.

4. **`server.rs::render_routes`: integrate once.** Iterate sorted news keys
   and call `news::render_post` with the existing environment/site/corpus; insert
   `/news/{slug}/` into the same pages map **with collision checking** before
   replacing any existing entry. A collision returns a useful route diagnostic.
   This needs no second exporter/server renderer or network architecture change.
   Check real existing concept/source/static page collisions, including legacy
   concept slugs containing path segments. Do not silently rewrite other kinds'
   slugs or broaden the job into fixing unrelated legacy route collisions.
   Do not build or reserve later product routes as if their design was approved.

5. **Construction/test integration and documentation.** Add empty news maps
   to every existing `Corpus` literal in graph/generator/validator tests; only
   those mechanical fixture fields change there. Extend server's full-route-set
   oracle and export CLI checks for the new post, preserving every existing
   route assertion. Keep `search::build_records` source/concept-only and add a
   regression assertion that news is not inadvertently indexed. Add the one
   demo file. In source `AGENTS.md`, narrowly replace the stale initial-decision
   hold with the current canonical approval/assignment and later held gates.
   Add concise news authoring/use instructions to source `README.md` including
   `haleiki export --content-dir demo ...` and the directly usable post URL,
   required fields and current missing listing surfaces. Do not describe
   legacy `build` as the new exporter or do Arc 06 CLI cleanup.

### Guidance actually applied by CDC

| Source and strength | Choice and reason | Observable proof |
|---|---|---|
| Rust TD-02/03 SHOULD, TD-05 MUST; AP-06 AVOID | Validated date and private news invariants prevent bypass through ordinary construction; Corpus maps fit existing ownership | Strict parse boundary controls, private date fields |
| EH-01 MUST, EH-05 CONSIDER, EH-08 MUST, EH-12 SHOULD; AP-09/25/33 AVOID | Existing CLI `anyhow` parser boundary gets contextual errors; renderer reuses typed Error. Expected content errors do not panic or silently disappear | Invalid fields identify path/field; duplicate and route conflicts fail |
| API-02 SHOULD; ID-08 CONSIDER | Borrow corpus/environment, return owned HTML; own parsed strings rather than proliferating lifetimes | Real parse → renderer → route path |
| ID-30 MUST; AP-67 SHOULD | Optional fields/neighbor links are Options; related is an empty vector, not missing or sentinel metadata | Serialized null/null/[]; absent author omits byline |
| Testing “Proportionate Proof”; work-verification independence | Behavioral inline tests and the existing real export/shared route prove this increment; no new reusable validator | Oracles below plus make check; CDC independently reproduces |

Synchronous startup parsing/rendering is already the lifecycle boundary. No new
tasks, mutexes, cancellation, cache, migrations or feature-dependent news type
are needed. Parsing completes before routes or export cleanup; input failures
must not yield a partial successful corpus/site. Existing export cleanup and
server request policy remain in force. Sorted file/route iteration avoids
filesystem-order winners. Runtime performance is linear parsing plus sorting;
no speculative optimization or new dependency/toolchain changes are assigned.

## Discriminating checks and validation

Keep tests in parser/news/server/search inline modules where private helpers
are accessible, and extend `tools/tests/export_cli.rs` for public CLI use.
Use unique `assert_fs::TempDir` fixtures with minimal taxonomy/profile (copy the
real demo profile when full rendering needs it); never insert test specimens
into the live demo or alter unknown-owner outputs. Additional fixture files
may live under `tools/tests/fixtures/news-slice01/` only if inline fixtures are
insufficient. Each negative case must use the same parser as its positive.

| Rows | Setup/action and exact oracle | Incorrect behavior rejected |
|---|---|---|
| F-1/F-2 | Recheck current approved log and the full frontmatter query population; account for the sole added demo post | Invented approval or reuse of generated copies as editorial sources |
| F-3/F-4 | Load the approved demo through `parse_corpus`; assert every authored field/body, exact filename slug/URL, ISO date; load isolated valid `tags: []` with omitted author/excerpt; absent news dir yields zero posts | Model absent from corpus, defaults masking missing required fields |
| F-4 | For **each** required field, remove it, set null, and use a wrong scalar/sequence/mapping type; test blank/whitespace title/category, `type: source`, tags scalar, `[1]`, `[true]`, and a mapping element. Assert failure identifies filename and field. Empty tag names/slug-empty category or filename also fail; `tags: []` succeeds | String coercion, treating missing tags as empty, truthy type checks |
| F-4 | Accept `2024-02-29`, `2000-02-29`, `2026-04-30`; reject `2026-02-29`, `1900-02-29`, `2026-04-31`, `2026-00-01`, `2026-13-01`, `2026-01-00`, `0000-01-01`, `2026-8-21`, `2026-08-21T00:00:00Z`, and padded date strings | Regex-only dates, incorrect century/leap/padding rules |
| F-3/F-6 | Change only date in the same named fixture: URL identical; add conflicting frontmatter slug: URL still filename-derived; `My Post.md` plus `my-post.md` fail with both paths | Date permalinks, trusting authored slug, HashMap last-wins |
| F-5 | Real `news-post.jinja` renders approved fields, `August 21, 2026`, category `/news/category/project-updates/`, all three tag hrefs, author and rendered paragraph; parse a fixture with `<b>Title</b>`/`<i>Author</i>` metadata and `**Body**` Markdown: metadata escaped, `<strong>Body</strong>` structural HTML. Check title/category/tag/author individually, not only one field | Missing real-template wiring, escaped Markdown, global-safe metadata |
| F-5/F-7 | Serialize context: prev=null,next=null,related=[]; absent optional fields omit `.post-author`, `.post-nav`, `.related-posts`; no trust-bar markup. Excerpt remains model data, not invented visible output | Undefined fields masquerading as explicit null, synthetic byline/nav/trust |
| F-6 | Full shared-route key set = previous wiki set + exactly one news route; no listing/feed/taxonomy routes. Real `resolve` of post is 200 text/html and matches route bytes; export writes `news/search-lands-in-the-demo-wiki/index.html` with same post content | Renderer unreachable from sink, accidental out-of-scope routes |
| F-6 | Isolated legacy concept with slug `news/search-lands-in-the-demo-wiki` plus this news post: `render_routes` fails on that exact collision; a normal concept/source sharing the bare news slug keeps its distinct existing path; assets remain unchanged | News insertion overwrites concept output or rejects legitimate namespacing |
| F-3/F-6 preservation | Recheck 91 authored source and 186 concept parses (derive current counts); retain Vimalamitra body “Attributed works”, Abelian Group “Quick Definition”, graph/wiki statistics and all existing source/concept routes; existing wiki tests remain green. Search records remain exactly concepts+sources, with no `/news/` URL | News changes graph totals/wiki taxonomy, existing body loss, accidental search inclusion |
| F-8 | `make check` on the final stable candidate, exact phases/exits/populations in report | Described, stale, zero-selected or skipped gate presented as passing |

Slash entities (`&#x2f;`) in fully interpolated hrefs are expected; reuse the
existing literal-or-escaped comparison or HTML-decode attributes. Do not turn
escaping off to satisfy a raw-text matcher. Test malformed delimiters and
unreadable/invalid news files as errors; keep the frontmatter/body boundary real.

Commands run with cwd exactly `$SOURCE` (substitute the absolute root):

```sh
cargo test --all-features --workspace parser::tests::
cargo test --all-features --workspace news::tests::
cargo test --all-features --workspace server::tests::test_render_routes
cargo test --all-features --workspace search::tests::test_build_records
cargo test --all-features --workspace --test export_cli
cargo test --no-default-features --workspace
cargo outdated --root-deps-only --ignore-external-rel
make check
```

Name any additional focused selectors and actual selected counts; zero tests is
not a pass. No baseline make/test run was performed during CDC authoring, so no
historical green receipt is inherited. The direct `cargo outdated` query must
retain its actual exit and stderr: Makefile's command substitution suppresses
stderr and can mask query failure. A green wrapper cannot prove that query
succeeded. `make check` includes dependency freshness,
lint, build (cleans `bin/`) and the all-feature suite. Inspect/preserve unknown
bin outputs first. Dependency downloads/tool setup for these named gates may
need escalation; do not run `make deps`, modify Makefile, upgrade pins or hide a
failed freshness query to obtain green. Report such blockers to CDC with full
underlying cause before proposing a policy or dependency change.

Run `make check-browser-js`, `make lint-e2e`, `make test-e2e` if browser assets,
their harness, shared browser behavior or search rendering changes; otherwise
record why not triggered for this Rust post-only increment. They remain required
at arc composition. Keep denied local binds distinct from test failures and
retain privileged reruns. Do not run the frozen `verify-migration.py` as a new
development gate or change it: it checks original live ledgers/source at the
migration baseline, and legitimately ceases to apply after development.

## Allowed changes, commit and return

Source allowlist: `tools/src/parser.rs`, new `tools/src/news.rs`,
`tools/src/main.rs`, `tools/src/server.rs`, `tools/src/export.rs` (tests only),
`tools/src/search.rs` (regression tests only), `tools/src/graph.rs`,
`tools/src/generator.rs`, `tools/src/validator.rs` (the last three: mechanical
Corpus test-fixture field additions only), `tools/tests/export_cli.rs`, optional
`tools/tests/fixtures/news-slice01/`, the one `demo/news/*.md` above, `README.md`
and `AGENTS.md`. Other paths/dependency changes need CDC reconciliation first.
Keep existing templates/browser JS/CSS, authored sources/concepts, archives,
issued prompts and migration checker unchanged.

After implementation/self-review and required gates, enumerate actual changed
source paths explicitly, stage only them, and use `git commit --only -- <exact
paths>`; no blanket add/commit. Include these **contiguous trailers**:

```text
Co-authored-by: Codex <noreply@openai.com>
Co-authored-by: Billo AI <ai-engineering@billo.systems>
```

Return to CDC via parent: current prompt and source/planning hashes; requested
and tool-selected model/settings plus context ID; actual changed paths and
source commit; intake/readback reference; F-1…F-8 row walk as **attested,
proposed-done**, including all failures/unrun checks and actual test populations;
artifact inventory (none expected beyond the canonical report unless a concrete
evidence need arises); applied-guideline self-review; deviations and remaining
limitations; “Bubble-up to the arc” answering delivered capability, discoveries
and scope-as-specified versus scope-as-delivered. Do not predeclare CDC/arc
acceptance or retrospective completion. If blocked, return precise cause,
candidate/dirty state, affected rows and next-action owner; preserve every
attempt. Hand back source mutation ownership explicitly, leaving the candidate
stable for independent CDC reproduction. CDC stages/commits the report and
updates planning only after inspecting it.

## CDC author readiness

CDC performed the full 524-line implementation-prompt-authoring guide load,
required collaboration/layout/iteration/closing/verification loads, repo Rust
entrypoint and full 2,188-line anti-patterns chapter, and the domain sections
above before issuing. Truncated early tool reads were recovered with bounded
reads; file existence/counts were not treated as reading. Source and test
reconnaissance is cited in the spine and manifest, with the corpus query
population kept separate. Pinned dependencies are read from Cargo.lock; existing
repository calls establish Value string/sequence API use. No new dependency API
or compiled-code claim is made for the illustrative shapes.

Readiness: source-grounded (existing shared route/search separation and every
Corpus construction accounted for); design-complete for the approved one-post
increment (strict fields/dates, filename identity, collision/error policy,
explicit null/empty context); guideline-applied (table); executable (ordered
symbols, construction sites, real sink path and module-local test access);
falsifiable (coercion/calendar/overwrite/escaping controls and positive wiki
preservation); coherent/portable (exact roots/ranges, intake contract and held
later decisions). This is author self-review, not implementation verification.
