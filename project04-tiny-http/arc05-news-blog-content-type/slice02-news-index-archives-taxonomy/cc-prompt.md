# CC assignment — Arc 05 Slice 02: news index, archive and taxonomy

Issued 2026-11-11 by CDC Locke `01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`.
Implementation assignment; return proposed-done evidence to Locke for independent
review. Project-relative path (copy/paste):

`arc05-news-blog-content-type/slice02-news-index-archives-taxonomy/cc-prompt.md`

## 1. Authority, stable inputs and ownership

The Operator actually answered **“Approve all four recommendations”** to G1–G4.
That answer is recorded in the governing arc/project plans before this assignment.
Continue Autonomous Mode through end of Arc 05, local scoped commits, unchanged
two-contributor split. No reconfirmation of approved choices or model settings.

- CDC Locke: explicitly requested/tool-selected OpenAI `gpt-6.1-sol`, reasoning
  `high`, actual context above. Owns planning/design/independent acceptance.
- CC Kierkegaard: parent resumes actual `01a1ccf7-3e7c-7ab2-ae59-72c9be8f457b`
  explicitly as OpenAI `gpt-6-luna`, reasoning `high`. Record actual resume
  settings/identity receipt; backend identity unreported. No substitutions or
  extra roles. If selection cannot be honored, hold dependent implementation.

Resolve these exact absolute roots; inherited cwd may be absent:

```text
SOURCE=/Users/oubiwann/lab/oxur/haleiki
PLANNING=/Users/oubiwann/lab/oxur/haleiki/.worktrees/planning
PROJECT=$PLANNING/project04-tiny-http
ARC=$PROJECT/arc05-news-blog-content-type
SLICE=$ARC/slice02-news-index-archives-taxonomy
SKILLS=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills
```

SOURCE is clean `framework-base/tiny-http` at
`5fe3cf46fa1ea673b0b84f09167e36de8611d3ee`. Accepted Slice 01 source commits:
`cb31dc92caa667ce6f3a7133163c3da8749549b1` and the test-only correction at
5fe3cf46. PLANNING is `planning`, entry
`e9c2b05272f6f42c9921eb5588e4cd265e6b675e`; CDC commits approval/open set
before dispatch. Record actual containing issuance HEAD and prompt SHA-256 at
intake. If the source/ownership state differs, report drift before dependent work.

On dispatch CC exclusively owns source writes/staging/builds/install/scanner/
server operations, plus **only** `$SLICE/closing-report.md` and supporting
`$SLICE/artifacts/cc-*` files in planning. CDC/parent will keep these inputs stable
until handback. CC never stages/commits planning. CDC owns all other planning
mutations/index and later independent gates. No overlapping source work/gates.
Use absolute shell working directories. Necessary writes/builds/local binds
outside writable roots may use `sandbox_permissions=require_escalated` under
existing scope. Preserve denied attempts, privileged reruns and real failures.

No push/merge/publish/deploy/external messages, user-owned chats, extra worktrees,
old Claude-worktree edits, Arc 06, project closure or self-acceptance. The five-
refinement limit remains; do not invent iterations or silently reset the budget.
Challenge a mistaken design before affected edits; routine private mechanics
are local discretion, consequential behavior in §4 is binding.

## 2. Required-reading manifest and intake

Load complete required content **before dependent edits**. Paths expand from §1;
all source ranges below identify 5fe3cf46, planning identifies the containing
CDC issuance commit, skill files identify the installed versions read at intake.
Shared documents are deliberately bounded to current obligations. A filename
without an explicit section scope means required-full. Recover truncated tool
output with contiguous bounded reads; hashes/headings are identity, not reading.

| Order | Class, exact material and loaded extent | Purpose / when |
|---|---|---|
| 1 | Required-full: this `cc-prompt.md`, `$SLICE/slice-plan.md`, `$SLICE/ledger.md`, `$SOURCE/AGENTS.md`, `$PLANNING/AGENTS.md` | Active assignment, ten rows, authority and repo constraints before edits |
| 2 | Required-section: `$PROJECT/project-plan.md` sections 1–3 and “Workflow, paths and resume gate”; `$ARC/arc-plan.md` sections 0–4, “Autonomous execution authorization”, “Contributor attribution and dispatch”, and “Actual Operator approval — G1–G4, 2026-11-11” plus current Slice 02 assignment section; required-full `$PROJECT/ledger.md` and `$ARC/ledger.md` | Product/model approval, endpoint and higher-scale gates; original packet is history, final approval/current assignment govern |
| 3 | Required-section: `$ARC/slice01-news-model-and-post/slice-plan.md` “Operator decision log — 2026-11-11” and “Final closure and next gate — 2026-11-11”; `cdc-verification.md` “Correction review 02 and Slice 01 acceptance — 2026-11-11” including all its subsections through EOF; reference-only its issued prompts, old previews and raw logs | Preserve accepted strict model and fixture, presence-aware null lesson; later G1–G4 approval supersedes the old held-gate prose, not its acceptance evidence |
| 4 | Required-full: `$SKILLS/collaboration-framework/SKILL.md` v3.0.1, `$SKILLS/project-management/guides/README.md`; required-section: project-management `02-canonical-planning-worktree.md` “Existing records and supporting evidence”, “The per-slice documents and artifact home”, “Slice iteration filenames and preservation”; `03-planning-top-down.md` “The per-slice open set”, “Turn the slice into an implementation-ready assignment”, “Issuing and executing an iteration”; required-full `04-closing-slices.md` | Current workflow, immutable issued history, evidence and handback |
| 4a | Required-full: `$SKILLS/collaboration-framework/knowledge/collaboration-framework/guides/01-posture-and-ethics.md`, `02-structural-pulls.md`, `03-collaborative-rights.md`, `04-component-route-table.md` (same guide directory) | Explicit shared posture dependencies, not an unbounded historical chain |
| 5 | Required-section: `$SKILLS/engineering-methods/guides/01-engineering-methodology.md` “Notes for Codex”, “Roles and Shared Invariants”, “Two-Contributor Workflow”, “Workflow Selection and Transitions”, “Codex Interpretation and Authority”, “Autonomous Mode” (all subsections, including mandatory selection/iteration limits); `07-implementation-prompt-authoring.md` “Required reading and CC intake”, “Corrective iterations”; agent-coordination `guides/02-context-packets.md` “Autonomous Mode contributor packets” and `guides/03-result-integration.md` “Autonomous Mode contributor returns” | Apply authority/intake, preserve requested vs reported settings, return evidence without accepting it |
| 6 | Required-full: `$SOURCE/assets/ai/rust/SKILL.md` v2.0.0 and `guides/11-anti-patterns.md` (2188 lines at author read); required-section: guides `01-core-idioms.md` ID-08 and ID-30; `02-api-design.md` API-02/03; `03-error-handling.md` EH-01,05–08,12; `05-type-design.md` TD-02–05 (complete entries and definitions). New Rust mechanics needing additional guidance trigger the relevant complete chapter section before use | Invariants, owned immutable collection, borrowing and actionable fallible errors. Repo-local path is the installed symlink; fallback installed rust-guidelines only if missing, disclose exact substitution |
| 7 | Required-full: `$SKILLS/testing/SKILL.md`, `guides/01-testing-discipline.md`, `guides/03-validation-gates.md`; `$SKILLS/work-verification/SKILL.md`, `guides/01-ledger-discipline.md`, `guides/03-row-closure.md`, `guides/05-independent-verification.md` | Discriminating real checks, denominators, CC attestation versus CDC acceptance |
| 8 | Required-full: `$SKILLS/javascript-deno-guidelines/SKILL.md` and `guides/09-anti-patterns.md`; required-section: its `guides/12-deno/12-02-testing.md` ID-04–06,12,15,23. This applies to **Deno test harness only**; Haleiki's existing Deno lint/fmt gates override the skill's default Biome setup. Do not introduce Biome config, Node/npm infrastructure or convert shipped classic browser scripts | Await real browser assertions, reuse existing lifecycle, cleanup. Relevant extension of existing suite, not runtime/tooling migration |
| 9 | Required-full: `$SOURCE/templates/README.md`, `templates/pages/news-{index,archive,tags,categories,post}.jinja` (expand all five paths), `templates/macros.jinja`, `demo/profile.yaml`, `templates/profile.sample.yaml`, approved `demo/news/search-lands-in-the-demo-wiki.md`, `tools/src/news.rs`, `tools/src/error.rs` | Actual contracts/copy/design, profile default compatibility, accepted post renderer |
| 10 | Required-section: `$SOURCE/tools/src/parser.rs` lines 1–217,310–379,448–650,843–1203; `render.rs` 26–180,339–393,613–655,722–757; `server.rs` 1–130,582–735,826–858,1107–1195; required-full `tools/src/export.rs`, `tools/tests/export_cli.rs`; required-section `tools/src/search.rs` 1–155,284–320; required-full `tests/e2e/harness.js`, `tests/e2e/search_test.js`, `tests/e2e/deno.json`; Makefile complete build/clean/test/lint/browser/common-check/check-deps targets (125–153,188–249,273–322) | Symbols, callers, existing fixtures and real shared sinks; no guessed API or partial path wiring |
| 11 | Required-data: `$SOURCE/Cargo.lock` complete package blocks for minijinja, serde, serde_json, serde_yaml, comrak and Pagefind, plus `tools/Cargo.toml` full text; `tests/e2e/deno.lock` locked Playwright version. Required-data: all direct `demo/{sources,concepts,news}/*.md` frontmatter projections (filename, kind, slug/title where present, category, news date/tags), complete file/count/category populations; baseline/source status and binary identity query from §8 | Pin dependencies; derive all relevant populations, not a sample or inherited 294-file count. Retain commands/exits/exclusions/results; data projection is not full semantic reading of all wiki prose |
| 11a | Required-section: `/Users/oubiwann/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_yaml-0.9.34+deprecated/src/de.rs` 1472–1500 and `src/value/de.rs` 1–100; locate exact same pinned package if registry root differs | Typed String YAML scalar coercion versus Value type preservation; strict new optional description map, before its deserializer |
| 12 | Conditional: before any closing claim/retrospective, `$SKILLS/methodology-stewardship/SKILL.md` and `guides/01-retrospectives-and-follow-through.md` required-full; CDC owns retrospective, CC supplies observations. Conditional: load newly encountered binding guide dependencies before affected work and record them | No recursive historical read mandate; no unavailable authority hidden by summaries |

At preflight write a concise intake table in **this slice's `closing-report.md`**:
each required item/state, full or exact section/range loaded, tool output IDs or
visible ranges where available, recovered omissions; structured queries and
inspected complete projections separately. Attest triggered/not-applicable
conditional items with reason. In a resumed context reload unavailable required
contracts; a prior receipt/compaction summary is not original authority.

Then write a short contract readback in your own words: how F rows connect to
source, especially date/slug ties, real singular targets, distinct-post counts,
no-related/explicit-null, RSS hide and later search inclusion, collision-before-
cleanup, one approved authored fixture, feature/browser gates and ownership.
Identify disagreements before edits; otherwise continue without a new approval.
Reading receipts remain attestation, not independent acceptance.

## 3. What changes, and what survives

Today `news.rs` renders only `news-post.jinja`, `server::render_routes` adds
individual news URLs after wiki routes, and prev/next/related stay empty. All
four listing templates already exist but lack assembled contexts/routes. The
index hardcodes `/news/feed.xml`. Existing post/index category/tag links are
singular paths; implementing only plural indices would leave them broken.

`parser::NewsDate` privately validates real Gregorian dates; `NewsPost` owns
body/metadata/filename identity with borrowed accessors. Keep those guarantees.
`render::Profile` resolves template configuration separately from legacy
`parser::Profile`; do not conflate them. `assemble_site_context` computes wiki
stats/graph data only. Both export and serve use `render_routes`. `export_site`
renders **before** deleting output. Pagefind uses explicit source/concept custom
records, not a crawl of the new HTML. This slice leaves its production path intact.

Author's fresh full inventory: 91 sources, 186 concepts, 10 wiki categories,
one news post, one news category and three news tags. These are baseline
observations, not future constants. Preserve all authored wiki and news bytes.
Use isolated temporary fixtures for extra cases, including future dates.

## 4. Binding design and engineering policy

**Approved product decisions:** G1 all-post monthly/yearly unpaged archive at
`/news/archive/`, explicit `pagination: null`; G2 open author-driven labels
separate from wiki, only current demo labels and optional approved description;
G3 RSS deferred and subscription link hidden until canonical public URL and a
tested feed contract exist; G4 news Pagefind delivered in Slice 03 with news type/
category, no wiki tier/trust invention. Do not reopen these gates or implement
news search/feed early in this packet.

**Collection and order.** Prepare one immutable owned `NewsCollection` per
`render_routes` call from `&Corpus` and `&render::Profile` news config, before
rendering news pages. Sort once by `(publication date descending, filename slug
ascending)`. No current-time filter/scheduler: every parsed post participates.
Preserve stable filename-based URLs. Avoid sorting/scanning the whole corpus
per post; store slug→position indices for constant-time neighbor lookup. Own
summary strings, not self-referential structs or cloned Markdown bodies. Reuse
`NewsDate`; add `Ord/PartialOrd` and crate-visible month-key/label/short-display
helpers without opening its fields/constructor. Human dates are English:
`August 21, 2026`, `Aug 21`, `August 2026`. No new crate/dependency.

**Contexts.** Named serializable plain-data structs, with fields consumed by
`templates/README.md`. A summary has title,url,date_display,date_short,category,
excerpt Option<String>; internal slug/index may be skipped from serialization.
Retain original metadata as plain strings; only post Markdown body is rendered
HTML and marked safe. Absence has typed Option/empty collections as appropriate.
Archive pagination must serialize present-null, not an omitted key or `{}`.
No pager routes. Each post appears once in the index and archive; hero first,
secondary next two, Recent remainder. Archive months use reverse chronological
`(year,month)` groups with the same post order inside each group.

**Taxonomy.** Categories and tags use separate maps keyed by derived slug,
rendered in ascending slug order. Counts are distinct matching post counts.
For repeated identical tags within one post, preserve the first authored
occurrence in post badges and membership; count that post once. Retain parser
model data rather than silently rewriting authored tag lists. Distinct exact
label names resolving to one slug within the same namespace are ambiguous:
collection construction returns `Error::Config` naming category/tag, slug and
both labels. It must not silently pick a name/merge labels. A category and a
tag sharing a slug are valid because their route namespaces differ. No fixed
vocabulary requirement, taxonomy.yaml edits or title casing normalization.

Add optional render-profile configuration:

```yaml
news:
  category_descriptions:
    project-updates: "Changes to Haleiki, its demo wiki, and supporting tools."
```

Use `#[serde(default)]` on the new profile member and description map; missing
config/map/entry gives empty descriptions. Map values are plain strings, not
HTML; wrong-shaped supplied configuration fails normal profile deserialization.
Require actual string keys/values, rejecting numeric/bool/null/container values.
Pinned serde_yaml 0.9.34's typed String deserializer visits scalar text without
type discrimination (`de.rs:1472–1500`), so a bare BTreeMap<String,String> derive
would coerce some wrong types. Use a small `deserialize_with` helper for the new
map: `Value::deserialize(deserializer)?`, require Mapping, require String keys
and values via Value matching/as_str, then collect to BTreeMap; turn failures
into `serde::de::Error::custom` with the configuration field/key context. Missing
map uses serde default; explicitly supplied null map is an error. Do not change
the existing profile's unrelated string fields or strict post parser.
Unused keys do not create categories or routes. Demo adds only the approved
entry above. Sample config may document the optional empty map, no invented copy.
Leave legacy `parser::Profile`/wiki taxonomy unchanged. Category recent = first
three matching chronological summaries. Tag cloud size policy: 1 small, 2–4
medium, >=5 large, represented by a closed serializable enum or equivalent
private helper. Document the thresholds; no speculative scaling machinery.

**Navigation/no-related policy.** In the global sorted list, post at position i
has prev = i+1 (older), next = i-1 (newer); total-order ties follow the same
rule. Boundaries serialize present-null (`get(...) == Some(Null)` tests).
Neighbor fields title,url,date_short come from the matching summary. `related`
always serializes explicit `[]` for all Arc 05 posts, a documented no-related
policy satisfying E-6. No inferred category/tag recommender. One demo post still
has null/null/[] with no navigation/related markup or trust bar.

**Routes.** Always create `/news/`, `/news/archive/`, `/news/tags/`,
`/news/categories/`; also each post URL and each observed singular
`/news/category/<slug>/`, `/news/tag/<slug>/`. Singular views reuse their shipped
plural template with context filtered to exactly one matching category/tag and
its posts. Add optional `news.selected_name` for a clear singular page/document
title and breadcrumb ending in that escaped label; plural views retain their
existing titles. Singular tag total_posts is its distinct matched count, not
whole-corpus count. Unknown labels are not generated and resolve 404; direct
unknown-label render helpers return Config error rather than global content.
The index's Categories/Tags headings link to the plural indices so these views
are reachable. Keep existing namespaces/slugs and no redirect framework.

Use an insertion helper for **every** news route that checks exact-key presence
before insertion into the shared map, including news-vs-news reserved paths.
News filename `archive.md`, `tags.md`, `categories.md` collides and fails; do not
silently displace a listing or post. A legacy concept whose full path is
`news/archive` or `news/category/project-updates` also fails. A concept `archive`
at `/archive/` may coexist with `/news/archive/`. Prefix relationships without
an identical full path are valid. Collision reports include the actual URL and
news origin. No route map is returned on error. Since render precedes export
cleanup, preserve pre-existing output on rendering/collision failure. Do not
extend this guarantee to later output I/O or Pagefind failures.

**Templates.** Remove the hardcoded RSS anchor now that index becomes usable;
do not add an unimplemented configurable feed switch. Preserve styles/chrome,
five real templates and body-only-safe boundary. Guard absent excerpt and empty
description paragraphs; no `None`, `none`, `null` or invented excerpt/byline.
Use ordinary singular/plural handling everywhere news counts say post/posts or
tag/tags: 1 post, 0/2 posts; 1 tag, 0/2 tags. Every zero-post listing renders
“No news posts yet.” and empty collections without phantom cards/months/labels.
Keep `pagination` null, no pager markup. No shipped CSS/JS changes.

**Failure/resources.** Collection validation and direct unknown helpers use
existing typed Config errors; template errors propagate through `?`, server
adds context via existing anyhow conventions. No panic for authored conflicts,
missing labels or malformed optional config. No background tasks, mutable cache,
new lock/Arc wrapper or cancellation semantics: collection is local immutable
startup/build data, rendered strings retain existing ownership. Complexity is
one O(N log N) sort plus taxonomy membership/group assembly and unavoidable
rendered-output work. No benchmark/optimization project is needed.

## 5. Concrete implementation spine (recommended mechanics)

The following is a coherent recipe; names/private layout may differ with a
clear return explanation. All observable policy in §4 stays binding. Sketches
below describe shapes, **not compiled drop-in code**.

1. `tools/src/parser.rs::NewsDate`: derive natural calendar ordering (fields
   already year,month,day); reuse a single month-name table in display helpers;
   add `pub(crate)` month_key→(u16,u8), month_label→String and short_display→String
   for dates above. Keep strict parse and private invariant boundaries unchanged.
2. `tools/src/render.rs::Profile`: add `#[serde(default)] pub news: NewsConfig`,
   passive Debug/Clone/PartialEq/Default/Deserialize/Serialize configuration with
   default BTreeMap<String,String> category_descriptions with the scoped strict
   Value-based map deserializer above. Existing profiles
   without `news` must continue to resolve. Add only approved demo config and
   optional empty-map sample documentation. Profile resolver still owns YAML I/O.
3. `tools/src/news.rs`: extend the existing module with private summary/group/
   category/tag/context records and crate-visible immutable `NewsCollection`.
   Recommend `NewsCollection::new(&Corpus, &NewsConfig) -> crate::error::Result<Self>`.
   Collect borrowed posts, sort once, validate label→slug uniqueness in separate
   maps, create summaries/positions and deduplicated per-post tags, group months
   and category/tag member positions in that order. Derive category recent and
   counts from those memberships; never scan wiki labels or seed unused config.
   Store indices into owned summaries or clone small summaries for serializable
   views, without self-borrows. Do not render/copy every Markdown body here.
4. Add `render_index(env,site,&collection)`, `render_archive(...)`,
   `render_tags(env,site,&collection,selected_slug:Option<&str>)` and
   `render_categories(...)` returning existing Result<String>; use fixed real
   template names and existing `site => {data:site}`, `page`, `news` wrapper.
   None selects plural, Some selects only that existing label. Give server
   borrowed category/tag slug iteration without exposing mutable invariant fields.
5. Update `render_post(env,site,corpus,&collection,slug)` and its callers/tests.
   Lookup the actual NewsPost, obtain neighbors by indexed position, render that
   one Markdown body and deduplicated tag badges. Keep PostContext fields and
   explicit null/[] serialization. No independently rebuilt order per post.
6. `server::render_routes`: after profile/site resolution construct one
   collection with `&site.profile.news`. Preserve wiki assembly, then render/
   checked-insert all global news pages, sorted observed singular routes and
   individual post pages using that same collection. The helper checks before
   overwrite; failed construction/collision returns before export cleanup.
   No per-request renderer change and no export/server duplication.
7. Update four listing templates for RSS absence, optional paragraph guards,
   singular/plural copy, empty states, plural-index links and selected-label
   title/breadcrumb. Reuse existing `post_row`. Post template safe/nav guards
   should not need behavior changes beyond provided correct context.
8. Extend inline tests using actual parse_corpus + temporary Markdown with real
   profile/taxonomy, the existing route-collision fixtures, export/CLI tests and
   existing `tests/e2e/search_test.js` lifecycle. Name Rust additions
   `test_news_collection_*` or `test_render_news_*` so focused commands select
   real nonzero populations. Keep old post/parser/search controls meaningful.
   Update full-route/export counts by independently constructed expected route
   sets, not by comparing a function to itself or hardcoding previous totals.
9. Narrowly update `templates/README.md` contract, source README authoring/routes/
   defaults and AGENTS Arc 05 authority/current scope. Document order, collisions,
   optional descriptions, nav and no-related, empty behavior and feed re-entry.
   State Pagefind news is next Slice 03 until implemented. No old archive/prompt
   rewrites, unrelated cleanup, CLI reconciliation, new deps or generated site commit.

## 6. Applied guideline decisions

Paths are repo-local `assets/ai/rust/guides/` unless marked installed.

| Guide / verified strength | Decision and rationale | Symbol / proof |
|---|---|---|
| `05-type-design.md` TD-02/03 SHOULD, TD-05 MUST; `11-anti-patterns.md` AP-06 AVOID | Preserve validated NewsDate and private collection invariants; add natural ordering/helpers rather than reparsing raw date strings in templates | NewsDate/NewsCollection; exact date/tie/month oracles and strict parser regressions |
| `01-core-idioms.md` ID-08 CONSIDER; API-02 SHOULD; AP-12/18 AVOID | Borrow Corpus/config inputs; own small serializable summaries, index them once, do not clone bodies or create self-references | constructor/render_post; code inspection plus all views sharing exact expected order |
| ID-30 MUST; AP-67 SHOULD | Use Option for nav/excerpt/pagination, empty collections for no results/related; preserve presence-aware Some(Null) assertions | context serialization and absence markup; omitted-key implementation fails |
| `03-error-handling.md` EH-01/08 MUST, EH-12 SHOULD, EH-05 CONSIDER | Authored label/route conflict is recoverable; existing Config/anyhow layers preserve actionable context, no panic/silent overwrite | collection/insertion; real collision and sentinel export tests |
| AP-08/23 AVOID, AP-35 AVOID | One sort/preparation, named Recent limit and tag-size thresholds; no speculative cache/optimization framework | shared render_routes; code review and boundary fixtures |
| Installed testing `03-validation-gates.md` Proportionate Proof | Existing inline/CLI/e2e paths cover behavior; add bounded real news link steps, no new verifier/harness/dependency | exact oracles below; mandatory native gates/exits/populations |
| Installed JS Deno testing ID-04/06 MUST, ID-12/15/23 SHOULD; anti-pattern ID-20 MUST-AVOID | Await browser actions and structural assertions; reuse single live/static/browser setup and finally cleanup; sequential link navigation is dependent work | existing suite extension, both-sink link assertions; Deno lint/fmt/test |

No default Biome/Node/runtime/migration policy from a generic skill overrides
Haleiki's explicit Deno harness versus classic-browser-script boundary.

## 7. Observable tests and oracles (F-1…F-10)

Use isolated authored Markdown parsed by the real parser, not hand-built private
NewsPost literals or fixtures added to demo. A five-post collection is the main
discrimination fixture; create files in shuffled order and vary titles against
filename order. Names below identify fixture titles as well as filename stems:

| Slug/title marker | Date | Category | Tags |
|---|---|---|---|
| z-newest | 2027-01-02 | Project Updates | common |
| a-tie | 2026-08-21 | Project Updates | common, common, pair, search |
| b-tie | 2026-08-21 | Project Updates | common, pair |
| m-older | 2026-08-01 | Project Updates | common, rare |
| a-oldest | 2025-12-31 | Docs | common |

1. **F-1/F-2:** exact order `[z-newest,a-tie,b-tie,m-older,a-oldest]`,
   deliberately disagreeing with slug/date-display-string order. Hero z-newest,
   secondary a-tie,b-tie; Recent m-older,a-oldest, no loss/duplication. Archive
   exact groups `January 2027:[z-newest]`, `August 2026:[a-tie,b-tie,m-older]`,
   `December 2025:[a-oldest]`, total5, present-null pagination and no `.pagination`
   markup. All five include future-date fixture, no current-time exclusion.
   Check actual template sections, not just that titles occur somewhere.
2. **F-3/F-4:** category order Docs(count1),Project Updates(count4); Project
   Updates Recent exactly first three, omits m-older. Tag order common(count5,
   large),pair(2,medium),rare(1,small),search(1,small). common section has five
   distinct posts once each in global order despite duplicate tag; a-tie post
   has one common badge, then pair,search. Exercise count4→medium boundary as
   well with an isolated membership variant; all size policy boundaries observed.
   Only Project Updates gets configured description, Docs empty; absent config
   resolves; empty map resolves; unused key creates no label/route; wrong value/
   map shape returns YAML/profile error, including numeric/bool/null/list/map
  values and non-string keys. Escape hostile description strings.
3. **F-4/F-7:** two labels `C++` and `C#` (actual slugify→c) in the same tag
   namespace fail with both labels/slug; category equivalents also fail. Test
   both across posts and tag conflict within one post. Repeated identical
   labels succeed; category `Shared` and tag `Shared` coexist at distinct paths.
   Do not require parse_corpus to reject collection-stage conflicts; call the
   real assembly/render_routes boundary. Error not panic or silent merge.
4. **F-5:** exact route set from fixture = unchanged wiki set + four globals +
   five posts + observed two category/four tag paths. Assert singular Docs shows
   only a-oldest, common allfive, pair only a-tie/b-tie, rare only m-older,
   search only a-tie. Selected name/title/breadcrumb/counts correct; all plural
   views retained. Unknown paths 404, unknown direct helper Config error. Actual
   browser link walks below reject a template-only fake/inert href implementation.
5. **F-6:** neighbor table (prev older / next newer): z-newest→a-tie/null;
   a-tie→b-tie/z-newest; b-tie→m-older/a-tie; m-older→a-oldest/b-tie;
   a-oldest→null/m-older. Check URLs and titles/date_short through serialization
   and representative rendered hrefs. All related present-empty. Keep one-post
   test `get("prev/next") == Some(Null)` (two separate get calls), no nav markup.
   Missing-key fallback Index checks alone cannot satisfy explicit-null criteria.
6. **F-7:** `archive.md`, `tags.md`, `categories.md` each fail with its exact
   URL; legacy concept full news listing and singular category paths conflict
   through render_routes (reuse complete concept fixture, avoid unrelated
   validation errors). Existing same-bare-slug concept/source/news positive
   case remains. Export collision on temp outdir with a sentinel file returns
   error without deleting/changing sentinel; no directory cleanup before render.
7. **F-8:** empty corpus (missing and empty news directory) yields exactly four
   news globals, no posts/singular targets, total0, months/tags/categories/sections
   empty and pagination Some(Null); all four real templates show honest empty
   state, no phantom card/None. One/many count HTML uses grammatical post/tag
   forms. Absent excerpt removes hero/secondary paragraph (test both positions),
   absent author still omits byline, absent description omits empty paragraph.
   Hostile title/category/tag/author/excerpt/description/neighbor metadata stays
   escaped in real views, while `**Body**` renders `<strong>Body</strong>` in
   post body; no metadata `safe` additions or trust bar. Assertions must scope
   absent markup to the fixture, not incidental chrome words.
8. **F-9:** derive and assert full candidate corpus/route/HTML counts and exact
   expected paths using independent expected-set construction. The unchanged demo
   implies 5 wiki singletons+10 wiki categories+186 concepts+91 sources, plus
   4 news globals+1 post+1 news category+3 tags = **301 map pages / 302 exported
   HTML files including 404**, if fresh inventories agree. This is a checkable
   derived prediction, not permission to add/remove content to hit a count.
   Compare source/concept memberships, representative body/metadata/graph stats
   and existing search records; search length remains sources+concepts with no
   news URL in Slice 02. News production search inclusion belongs Slice 03.
   All demo news Markdown bytes unchanged. No feed file/route/link; re-entry
   documented. Existing search tests keep real positive wiki results/facets.
9. **F-5/F-8/F-9/F-10 real browser:** extend the single existing suite in
   `tests/e2e/search_test.js`, reusing its live/static/browser lifecycle and
   sequential `t.step` blocks (avoid port collisions/new module setup). For
   **each sink** enter `/news/` (200), assert hero approved title/date and no RSS
   anchor, click actual hero link, read rendered body and expected category/
   three tags. Click each actual post taxonomy link and assert successful
   selected-label page/matching post link; no invented news trust. Return to
   index and use actual plural-heading links to categories/tags, assert approved
   description and three real tag sections/counts. Click View all posts, assert
   August2026 group, `1 post`, post link and no pager. Unknown news label 404.
   Assert real DOM/href/response, not source string presence alone. Preserve all
   prior search scenarios; no news search expectations until Slice 03. Record
   actual parent tests and substeps/exits. Rust fixtures supply many-post coverage
   unavailable from the intentionally single authored demo post.

## 8. Required gates, evidence and output preservation

Run on a stable final candidate, **serially**, SOURCE absolute cwd unless noted.
Record actual selector populations and skips/ignores; exit0 with zero selected
tests is not evidence. Focused tests first, then full/native gates once after
final edits; rerun affected checks after any repair. Exact focused commands:

```text
cargo test --workspace --all-features news::tests::
cargo test --workspace --all-features parser::tests::test_parse_news
cargo test --workspace --all-features render::tests::test_resolve_profile
cargo test --workspace --all-features server::tests::test_render_routes
cargo test --workspace --all-features server::tests::test_news
cargo test --workspace --all-features export::tests::
cargo test --workspace --all-features search::tests::
cargo test --workspace --all-features --test export_cli
cargo test --workspace --no-default-features
cargo outdated --root-deps-only --ignore-external-rel
make check
make check-browser-js
make lint-e2e
make test-e2e
```

If new test names need a focused selector add it and record exact command/nonzero
selection; do not omit named obligations. Gates cover code/HTML/browser behavior,
not yet the fresh independent arc-scale composition gate. Existing four no-default
unused-import warnings in demo_fetch/status are baseline-source warnings per
Slice 01 independent comparison; disclose actual observations and substantiate
new classifications. Fix new real warnings/failures in scope. No broad warning
allow, gate weakening, dependency update or silent skip. `cargo outdated` exits0
with a version report; that is not “no upgrades.” Make check-deps masks query
stderr/status, hence retain the direct query provenance. Frozen migration checker
expects unchanged migration-baseline source/ledgers and is not applicable here.

**Before any Make build/cleanup:** inspect ignored `bin/` contents, lstat owner/
mode/type and SHA-256 of `bin/haleiki` and
`/private/tmp/haleiki-slice01-original-bin`. Author found only that regular
nonsymlink binary, uid501 mode0755, both SHA-256
`4df7d22f32bbeb7bcde62c62e100917987199fc71729179f85b877a1c3405f6e`.
Verify current ownership/content, do not blindly overwrite changed/unknown data.
Preserve original before Make's `rm -rf bin`; restore only the verified owned
original byte-identically afterward and record hashes. No cargo clean-all or
clobbering unknown output. Do not commit bin/target/generated _site/Pagefind.

Use the current target/debug binary for a fresh real export to a unique owned
temporary output, inspect all derived news files and rendered counts/links,
capture exact command/exits/population in the report. Native browser assertions
above provide point-of-use evidence; save only useful logs/captures under the
designated artifacts home. No new reusable validator, synthetic screenshots or
fixture posts masquerading as authored content. Distinguish sandbox bind denial/
dependency loading failure from project failure; retain attempt and rerun under
existing escalation scope. Never invent a passing receipt or discard initial
compile/assertion/fixture/lint failures.

## 9. File allowlists, commit and handback

SOURCE allowlist (explicit paths; narrow edits within described roles):

```text
tools/src/parser.rs
tools/src/news.rs
tools/src/render.rs
tools/src/server.rs
tools/src/export.rs
tools/tests/export_cli.rs
templates/pages/news-index.jinja
templates/pages/news-archive.jinja
templates/pages/news-tags.jinja
templates/pages/news-categories.jinja
templates/README.md
templates/profile.sample.yaml
demo/profile.yaml
tests/e2e/search_test.js
README.md
AGENTS.md
```

Export changes should be tests/doc only, shared production pipeline retained.
No source search.rs/JS/CSS/Markdown corpus edit, new crate, lockfile/toolchain,
generic routing refactor, existing archives or other worktree. If a genuine
required integration path falls outside this list, return the exact reason to
CDC before that edit; do not silently expand or drop scope. Private helper/type
names, exact struct layout/test-helper factoring are local discretion within
the contracts; explain substantive deviations from the recommended recipe.

Commit proposed-done source locally after gates using explicit changed paths
from the allowlist, `git diff --check`, review staged diff, and scoped
`git commit --only -- <exact changed paths>`. No blanket add/stage or planning
commit. Every assistant-authored commit has these contiguous trailers:

```text
Co-authored-by: Codex <noreply@openai.com>
Co-authored-by: Billo AI <ai-engineering@billo.systems>
```

Return `$SLICE/closing-report.md` with:

- current assignment identity/hash, source and planning HEADs/dirty states,
  actual requested/tool-selected model/context versus unreported backend;
  complete intake/readback (not merely “all read”);
- concrete implementation/recipe deviations, applied-guideline self-review;
- per-row F-1…F-10 walk as **attested/proposed-done**, exact source/tests/files/
  commands/exits/selected counts/ignored counts; failures, fixes and repeated
  checks retained, artifact paths resolvable; no claim CDC/arc acceptance;
- fresh full corpus/route/export/browser observations and scope limits,
  warnings/query provenance and verified binary preservation; source commit(s)
  with exact hashes, clean source status and changed-path allowlist check;
- bubble-up answers: findings/change implications/retrospective observations,
  Slice 03 Pagefind and RSS-deferral/composition obligations retained. E-2…E-7
  and project rows remain CDC-owned and open pending real acceptance;
- explicit source mutation/gate/index **ownership returned to CDC**, all local
  processes stopped, no CC running in parallel, remaining blockers if any.

After handback remain idle for review. CDC independently inspects source/report,
reproduces applicable required gates and meaningful controls, closes ten rows
only if justified, writes verification/retrospective and scoped planning commit,
then opens Slice 03. Any correction uses a new numbered durable packet. Arc 05
continues through fresh composition and a separately attributed independent
arc gate; no Arc 06 or project closure by inheritance.
