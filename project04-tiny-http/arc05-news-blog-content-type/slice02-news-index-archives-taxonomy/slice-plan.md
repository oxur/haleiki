# Slice 02 — News index, archives and taxonomy

Status: **open / ready for CC dispatch** · v1.0 · 2026-11-11.
Parent: [Arc 05](../arc-plan.md); acceptance: [ledger.md](./ledger.md).
Current assignment: [cc-prompt.md](./cc-prompt.md).

## Goal and approved inputs

A reader can enter `/news/`, open the approved post, browse all posts by
month/year, category or tag, and follow every news category/tag link to a
matching real page. Both sinks consume the same complete route map. This
delivers the existing roadmap's second increment; Slice 03 then adds news to
Pagefind and composes the arc, including the approved RSS deferral.

Slice 01 is accepted at source `5fe3cf46fa1ea673b0b84f09167e36de8611d3ee`.
The Operator's actual 2026-11-11 answer **“Approve all four recommendations”**
resolves G1–G4; [the arc approval record](../arc-plan.md#actual-operator-approval--g1g4-2026-11-11)
governs. No reconfirmation or new product gate is needed for this assignment.
The one approved authored post and strict model remain intact.

## Binding behavior

- Four permanent listing routes: `/news/`, `/news/archive/`, `/news/tags/`,
  `/news/categories/`, including an empty news corpus. Add observed-label
  targets `/news/category/<slug>/` and `/news/tag/<slug>/`, each showing exactly
  its matching posts; unknown labels remain 404. Existing post URLs are stable.
- All posts, including future-dated valid posts, participate. No scheduler.
  Order is publication date descending, filename slug ascending on equal dates.
  Index hero = first, secondary = next two, Recent = remainder. `date_display`
  stays `August 21, 2026`; `date_short` is `Aug 21`, matching the reviewed preview.
- Archive contains every post once, month/year groups newest first, same post
  order inside groups; example label `August 2026`. `pagination` is present and
  null. No pagination/date-bucket routes or synthetic editorial content.
- News taxonomy is separate from wiki taxonomy. Categories and tags appear in
  slug order, counts are distinct matching posts. Repeated identical tags within
  a post count/render once, preserving first authored occurrence. Distinct label
  names with the same normalized slug fail collection assembly with an actionable
  error naming namespace, slug and both labels; do not silently merge them.
  Category and tag namespaces may share the same slug. This is route ambiguity
  handling, not a controlled editorial vocabulary.
- Optional `profile.news.category_descriptions` maps canonical category slugs
  to plain strings, with type-preserving validation of actual string keys/
  values (no YAML scalar coercion). Default missing configuration/map/entry
  is empty; explicit null/wrong-shaped maps or values fail. Unused
  entries do not seed labels. The demo adds only `project-updates` with approved
  text **Changes to Haleiki, its demo wiki, and supporting tools.** No tag
  descriptions or extra demo labels. Each category's Recent is its first three
  posts. Tag size: count 1 small, 2–4 medium, 5+ large (named presentation policy).
- Prev = immediate older post; next = immediate newer post in the total order.
  Ends serialize explicit null keys. Related is always an explicit empty list
  for Arc 05: no automatic recommendation policy. One demo post retains null/
  null/[] and omits navigation/related blocks. This supplies E-6's permitted
  explicit no-related disposition, without changing the one-post approval.
- Only rendered Markdown body is marked safe. Title, excerpt, author, labels,
  descriptions and neighboring titles stay autoescaped. Absent excerpt omits
  its paragraph; absent description stays empty and omits its paragraph.
  Empty listings show “No news posts yet.”; copy uses 1 post/0 posts/2 posts
  and the corresponding tag singular/plural forms.
- Remove the visible RSS subscription link while the feed is absent. RSS stays
  deferred until a canonical public URL is chosen and a feed contract is
  implemented/tested. News Pagefind inclusion remains assigned to Slice 03;
  existing source/concept search records and wiki rendering must survive here.
- Detect every exact news route collision with another news or existing wiki
  route before overwrite. Shared bare slugs across distinct namespaces remain
  valid. Rendering errors occur before export output cleanup/publication of a
  route map; do not claim rollback for later filesystem/Pagefind failures.

## Scope and delivery

Source changes are limited to the explicit packet allowlist: news collection/
date helpers, render profile configuration, shared routes, four listing
templates, sample/demo profile descriptions, relevant tests and narrow docs.
One existing Deno/Playwright suite gains real news link walks against both sinks.
No new dependency, infrastructure, shipped browser JS/CSS, authored demo posts,
wiki schema/graph change, RSS implementation, Pagefind production change or
Arc 06 work. Native shared-route rendering makes the result usable immediately
in both sinks; source commits are proposed-done until CDC independently accepts.

Artifacts: supporting logs and any needed native captures under `artifacts/`
inside this slice. No close-set placeholders are created at opening. CC writes
only this slice's `closing-report.md` plus `artifacts/cc-*` in planning, does not
stage planning, and preserves all attempts. CDC owns the remaining planning
files/index, reviews source after handback, writes `cdc-verification.md` and
mandatory `cdc-retrospective.md`, walks all F rows and bubbles up to the arc.

## Source, authority and ownership

SOURCE `/Users/oubiwann/lab/oxur/haleiki`, clean `framework-base/tiny-http`
at `5fe3cf46fa1ea673b0b84f09167e36de8611d3ee`.
PLANNING `/Users/oubiwann/lab/oxur/haleiki/.worktrees/planning`, branch
`planning`, entry `e9c2b05272f6f42c9921eb5588e4cd265e6b675e`; this open set
and actual approval are committed before parent dispatch. Intake records the
containing issuance HEAD and prompt hash, rather than a self-referential hash.

Unchanged Autonomous scope: through end of Arc 05, local scoped commits only,
stop for an actual new material discussion gate. No push/merge/publish/deploy,
external messages, user-owned chats, worktree churn, Arc 06 or project closure.
CDC Locke `01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`: explicitly requested/tool-
selected OpenAI `gpt-6.1-sol / high`. CC Kierkegaard
`01a1ccf7-3e7c-7ab2-ae59-72c9be8f457b`: explicitly selected
`gpt-6-luna / high`, to be resumed by parent with those settings. Backend
identity remains unreported for both. No extra roles or substitutions.

CDC owns stable source inspection and planning until dispatch. On dispatch CC
exclusively owns source mutation/index/builds/scanners/servers and its designated
report/artifacts; CDC/parent keep those inputs stable. After a clean scoped source
commit and explicit ownership return, CDC performs independent review/gates.
Repairs go through a preserved numbered prompt and CC, never CDC self-repair/
self-acceptance. Retain the five-refinement limit; repeated failures require
revisiting cause/method and escalation rather than silently resetting the loop.

## Verification and exit

The prompt maps F-1…F-10 to discriminating fixtures and real paths. Required:
focused news/parser/profile/route/export/search preservation tests, full
`make check`, `cargo test --workspace --no-default-features`, direct dependency
query provenance, and `make check-browser-js`, `make lint-e2e`, `make test-e2e`.
Browser steps prove actual links/counts/no RSS in both sinks, not multi-post
ordering; isolated Rust fixtures prove that ordering and failure behavior.
Derive route/corpus/test populations from the candidate, not a prior export
count. Preserve unknown outputs; verify original ignored `bin/haleiki` and its
backup ownership/hash before Make cleanup and restore byte-identically after.
Environment bind/loading failures stay distinct from project failures; authorized
privileged reruns are recorded. Frozen migration checks are not development gates.
Do not waive or weaken any required gate to close.

Exit only after all rows have evidence and CDC disposition; CC attestation is
proposed-done. Subsequent owner CDC opens Slice 03 under the approved roadmap,
then fresh arc composition/independent gate/retrospective and project bubble-up.
Slice 02 does not close Arc 05 or project rows by inheritance.

## Author readiness — Locke, 2026-11-11

Applied engineering-methods `07-implementation-prompt-authoring.md` in full
(524 lines), current planning/Autonomous/coordination/test/verification contracts,
repo Rust v2.0.0 entrypoint and full anti-patterns plus scoped idiom/type/error/
API guidance. Deno skill/test guidance informs added existing-suite browser
steps; Haleiki's Deno gate overrides the skill's default Biome setup. All source
and template references in the prompt were inspected at the stated baseline;
no production edits were made by CDC. A mistaken `templates/partials/macros.jinja`
lookup was recovered by fully reading actual `templates/macros.jinja`; no missing
dependency remains. Oversized guide output was recovered with bounded reads.

Readiness: source-grounded (packet §§3–5), design-complete (§4), guideline-applied
(§6), executable (§5 with native wiring), falsifiable (§7 exact fixture oracles),
coherent/portable (§§1–2,8–9). This is routine approved implementation preparation,
not a separately commissioned investigation. Required reading budget is approximately
120–160k tokens including the assignment and projected data: the 14 mandatory
full shared skill/guide texts alone contain 39,660 words / 293,227 characters;
posture adds 7,195 words, then bounded source/template/planning sections and
complete data projections. Reserve implementation/recovery headroom by excluding
old assignments and wiki body prose, not by skimming required contracts. If a
receiving context cannot fit the complete packet with implementation headroom,
return that concrete limit for re-slicing before dependent edits. No unsupported public API or unresolved
binding choice is delegated to CC; private names/mechanics remain local discretion.

Fresh author data query: all direct demo Markdown frontmatter records, sources
91, concepts 186, news 1; 10 wiki categories, one news category/three news tags.
PyYAML used only for inventory, not as the Rust parser/type oracle. Original
binary and backup are regular nonsymlink uid501/mode0755 and SHA-256
`4df7d22f32bbeb7bcde62c62e100917987199fc71729179f85b877a1c3405f6e`.
These are preparation observations, not passing candidate gates.

## Assignment history

| Assignment | Issued | Predecessor / reason | Disposition |
|---|---|---|---|
| `cc-prompt.md` | 2026-11-11 | New roadmap Slice 02, accepted Slice 01 and actual G1–G4 approval | Ready for parent to resume selected CC; implementation not yet started |

Current project-relative path:
`arc05-news-blog-content-type/slice02-news-index-archives-taxonomy/cc-prompt.md`.
Once issued, preserve it; corrections use the next unused `cc-prompt-iterationNN.md`.

## Version history

- v1.0 — 2026-11-11: full open set after actual G1–G4 approval; no ledger row
  closed at preparation. Existing three-slice roadmap retained.
