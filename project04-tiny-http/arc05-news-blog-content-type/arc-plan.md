# Arc 05 — News / Blog Content Type

> The arc plan-of-record: add Haleiki's news/blog surface to the tiny_http pivot.
> Part of the tiny_http project ([`../project-plan.md`](../project-plan.md)).
> Status: **active**, Slice 01 closed; G1–G4 approved, Slice 02 ready for CC dispatch · v1.3 · 2026-11-11; initial plan 2026-08-21.

---

## 0. Direction note

For Haleiki, **News is blog**: a dated stream of wiki updates and related news
items that make sense in the context of a maintained knowledge base. It is not a
general CMS feature, not comments, not editorial workflow, and not a rewrite of
the existing concept/source model.

The design refactor already ships five news templates:

- `templates/pages/news-index.jinja`
- `templates/pages/news-post.jinja`
- `templates/pages/news-archive.jinja`
- `templates/pages/news-tags.jinja`
- `templates/pages/news-categories.jinja`

The binding contract in `templates/README.md` names the required `post.*` and
`news.*` context shapes. Arc 05 implements those shapes conservatively, with
operator discussion gates when a rendered example is needed to make the product
call well.

## 1. Capability

The demo site gains a first-class dated news/blog content type. Authored news
Markdown parses into the corpus, renders individual posts through
`news-post.jinja`, renders the news index/archive/tags/categories templates, and
appears in both sinks: static export and `haleiki serve`.

The delivered surface should feel like a maintainer log for a wiki: release
notes, corpus updates, editorial announcements, and related project news. The
reader can browse the latest posts, open a post, move between posts, and browse
posts by date, tag, or category once those views are implemented.

## 2. Slice Breakdown

Three slices. Slice 01 creates the load-bearing content model and proves one
post can render. Slice 02 builds the list/archive/taxonomy surfaces after the
team has a rendered example in hand. Slice 03 composes the full news surface
across export/server/search/feed decisions and closes the arc.

| Slice | Scope | Load-bearing for |
|-------|-------|------------------|
| **01 — news model + post route (closed 2026-11-11)** | Define the news authoring/frontmatter model, parse demo news posts, render one real post through `news-post.jinja`, and choose the post URL/date/display conventions with operator confirmation from concrete examples. | 02, 03, arc close |
| **02 — news index + archives + taxonomy views (open)** | Assemble the four shipped `news.*` listing contexts, approved unpaged month/year archive, category/tag counts and plural/singular routes; implement ordered post navigation and explicit no-related policy. G1/G2 approval is recorded; hide the absent-feed link under G3. | 03, arc close |
| **03 — sink parity + feed/search disposition + arc close** | Include news in Pagefind under G4 without wiki tier/trust invention; prove full static/server/search parity and approved G3 RSS deferral/re-entry/hidden-link handling; run fresh arc gates and independent composition review before closing the arc. | project close |

## 3. Decisions and Discussion Gates

Confirmed:

- News is blog: dated maintainer updates and related news items for the wiki.
- News is a new content kind added alongside concepts and sources.
- News remains read-only.
- Trust bars stay off news posts unless the operator explicitly reverses the
  design-review note in `templates/README.md`.

Resolved on 2026-11-11 (was discussion-gated): authoring location and post URL
model; minimum/optional frontmatter; and existing content versus minimal demo
fixtures. The canonical Slice 01 decision log retains the approved example
and the four Operator decisions; original issued history stays unchanged.

The following gates were reserved and are now **resolved by actual Operator
approval on 2026-11-11**; see the authoritative approval record below. Their
original subjects remain visible for provenance:

- Archive pagination shape and date granularity, after the first archive render.
- Tag/category vocabulary and descriptions, after the first taxonomy render.
- Whether `/news/feed.xml` is delivered in Arc 05 or deferred with the visible
  template link handled.
- Whether Pagefind includes news posts in Arc 05 or whether search integration
  waits for Arc 06/project recomposition.

## 4. Dependencies

Consumes closed Arcs 01–04:

- Arc 01 render core and minijinja environment.
- Arc 02 route/static export machinery.
- Arc 03 `haleiki serve` route/server machinery.
- Arc 04 Pagefind search gates and browser/e2e harness, if news search
  participation is accepted in Slice 03.

Leaves for Arc 06:

- Final visual parity pass against the design review.
- Default CI/browser gate decisions.
- CLI reconciliation and removal of Cobalt/Liquid remnants.
- Project-scale recomposition of DoD rows.

## 5. Arc Ledger


See the sibling [ledger.md](./ledger.md). All original row IDs, criteria,
statuses and evidence are retained there.

## 6. Version History

### v1.3 — 2026-11-11 (actual Operator approval of G1–G4)

The Operator answered “Approve all four recommendations” to the exact
G1–G4 decision packet. Record that approval before dependent Slice 02 work.
Archive and taxonomy implementation is unblocked; RSS remains explicitly
deferred with link handling, and news Pagefind inclusion is assigned to
Slice 03. Existing source/model selections, local-only authority, independent
composition gates and the end-of-Arc-05 endpoint remain unchanged.
Slice 02 now has a complete source-grounded open set and current assignment;
its ten rows remain open for implementation and independent acceptance.

### v1.2 — 2026-11-11 (Slice 01 accepted; next discussion gates)

Slice 01 closed at source 5fe3cf46 after independent correction review, full
native gates and CDC retrospective. E-1 now records the accepted child; all
composition and remaining slice rows stay open. No structural/scope change is
needed. Native-template draft renders make the existing archive/taxonomy
discussion gates concrete; feed/search dispositions are proposed, not approved.

### v1.1 — 2026-11-11 (Slice 01 assignment preparation)

Retained the parent's Autonomous authorization and Slice 01 approvals,
reconciled the initial gate list, and issued the source-grounded numbered
assignment. Existing E-1…E-7 and F-1…F-8 remain open. No implementation or
composition acceptance is inferred from this preparation.


### Migration — 2026-11-11

Moved from `docs/tiny-http/` to orphan branch `planning`, worktree
`.worktrees/planning/project04-tiny-http/`. The embedded ledger now lives in sibling
`ledger.md`; row text and acceptance states are unchanged. See the
[byte-preserved August plan](../../archive/docs/tiny-http/arc05-news-blog-content-type/arc-plan.md). Existing issued prompts, ledgers, close reports,
verification records and `slice-doc.md` documents retain their bytes.
Canonical `slice-plan.md` entrypoints link the original slice documents.
Historical repository-relative paths resolve via the root migration manifest;
source paths such as `tools/`, `templates/` and `demo/` refer to the implementation
checkout, not this planning-only worktree. No feature verification was rerun.

### v1.0 — 2026-08-21
Initial Arc 05 plan. Records the operator decision that News is Haleiki's blog:
wiki updates and related news items. Opens a three-slice path: news model + post
route, news listing/archive/taxonomy surfaces, then sink parity plus feed/search
disposition and arc close. Discussion gates are explicit so product/design
choices happen with rendered examples in hand.

## Autonomous execution authorization — 2026-11-11

The Operator approved all four Slice 01 content-model recommendations and
authorized Autonomous Mode through the end of Arc 05, including slice iterations,
independent review, local scoped commits, arc composition and CDC retrospectives.
Stop at unresolved product/design discussion gates; do not open Arc 06.
No pushing, merging, publishing, external messages or deployment is authorized.

| Role | Explicitly approved provider/model/settings | Assignment |
|------|--------------------------------------------|------------|
| CDC, planning and independent implementation review | OpenAI contributor tool, `gpt-6.1-sol`, reasoning `high` | Dedicated coordinator context; context ID recorded at handoff |
| CC, source implementation and attested evidence | OpenAI contributor tool, `gpt-6-luna`, reasoning `high` | Dedicated implementation context, assigned by CDC |

Both exact model overrides and High reasoning are advertised by the live
contributor tool. No substitutions/default inheritance are authorized. The
parent chat relays progress and Operator decisions; substantive coordination is
handed to the explicitly selected CDC context. CDC reviews CC's implementation;
self-repair by a reviewer requires a separate verifier for affected changes.
Use the existing `cdc-verification.md` convention. Arc-scale acceptance requires
fresh composition evidence and the applicable independent/Operator gate.

Source checkout: `/Users/oubiwann/lab/oxur/haleiki`, suitable dedicated non-default
branch `framework-base/tiny-http`, clean baseline
`75d2593d51c986346f0239bed36a248ac5b4e6f7`.
Planning checkout: `/Users/oubiwann/lab/oxur/haleiki/.worktrees/planning`, branch
`planning`; baseline before this authorization `89213a1`.
CDC owns planning mutation/staging; CC owns source mutation/staging only while
assigned. Serialize source writes and gates against a stable candidate. Retain
all prompts and original evidence, existing outputs of unknown ownership, and
all current acceptance obligations. Do not run push/publish/deploy targets.

Slice 01 decisions are confirmed in its canonical `slice-plan.md`; the original
issued prompt and `slice-doc.md` remain preserved. CDC must prepare a source-
grounded `cc-prompt-iteration01.md` under the current prompt-authoring/intake
contract before assigning CC. Later archive pagination/date grouping, taxonomy
vocabulary/descriptions, RSS/feed delivery and news search-participation gates
were open at this initial authorization. The actual G1–G4 approval below now
resolves them; the current Slice 02 assignment governs dependent execution.

## Contributor attribution and dispatch — 2026-11-11

CDC: Locke, actual context `01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`.
Operator requested OpenAI `gpt-6.1-sol`, reasoning `high`; the parent
reports its spawn request explicitly accepted those settings. This is
requested/tool-selected attribution, not a claim of independently
reported backend identity; that identity is unreported.

This CDC context has no native subagent spawn tool. The parent explicitly
authorized dispatch relay: it will spawn CC as `gpt-6-luna / high` from
the durable assignment and return actual CC ID/settings/results to Locke.
No alternate model or user-owned chat route is used. CC is not yet
dispatched at issuance; the parent owns the next dispatch action.

Historical initial-dispatch assignment: `slice01-news-model-and-post/cc-prompt-iteration01.md`.
The current assignment is named in the Slice 02 section below.
CC owns source plus its designated `closing-report.md` handoff only; CDC
owns planning/index work and independently reviews source after CC returns
mutation ownership. Arc authority remains active across this phase return.

## Slice 01 review history — 2026-11-11 (superseded by final acceptance)

CC Kierkegaard, context `01a1ccf7-3e7c-7ab2-ae59-72c9be8f457b`, explicitly
requested/tool-selected `gpt-6-luna / high`, returned clean source `cb31dc92`
and its attested closing report. CDC Locke independently reviewed and
reproduced the focused one-post behavior; one test-oracle gap (R1, explicit
null key presence) is returned in numbered iteration 02. No source behavior
repair or arc scope change is indicated. E-1 and all other open arc rows stay
open until actual slice acceptance/composition. Later product gates remain
unchanged. Actual attribution and candidate evidence are in the slice records;
backend identities remain unreported.

## Final Slice 01 acceptance and original decision packet — 2026-11-11 (now approved below)

Slice 01 is independently accepted at source
`5fe3cf46fa1ea673b0b84f09167e36de8611d3ee` (CC source commits cb31dc92 and
5fe3cf46). All F-1…F-8 are done, R1 resolved, and the slice retrospective is
complete. E-1 is done; E-2…E-7 and project P-5/P-10 remain open. No arc close,
arc retrospective or fresh independent arc composition gate is claimed.

The already approved authoring/URL/frontmatter/one-fixture decisions are not
being reopened. The following are the reserved section-3 decisions, now with
concrete native minijinja renders of the unchanged shipped templates:

- [Open all decision previews](./slice01-news-model-and-post/artifacts/gate-preview.html)
- [Chromium capture](./slice01-news-model-and-post/artifacts/gate-preview.png)
- [Exact illustrative contexts](./slice01-news-model-and-post/artifacts/gate-contexts.json)
- [Render reproduction script](./slice01-news-model-and-post/artifacts/render-gate-examples.py)

**These are draft context renders, not implemented routes or approval.** The
unpaged archive and taxonomy use only the approved post. The paged alternative
uses a clearly labelled synthetic 41-post scenario: page 2 has the real post
plus 19 layout-only rows. No synthetic Markdown entered the corpus. The index
preview exposes the current unconditional RSS link. The screenshot was visually
inspected in cached Chromium (six frames including the surrounding review
document); this is preview QA, not the required product browser/composition gate.

| Gate | CDC recommendation, awaiting Operator decision | Concrete alternative / tradeoff |
|---|---|---|
| G1 — archive pagination/date granularity | `/news/archive/` lists all posts on one page, newest first, grouped by calendar month with year in the heading (e.g. August 2026); `pagination: null`, so no pager. This is Archive A and fits the current one-post maintainer log. | Archive B uses 20 posts per page, monthly headings within each page, page 1 at `/news/archive/` and later pages at `/news/archive/page/<n>/`. B bounds page size but adds route/boundary behavior. Neither changes stable post URLs. |
| G2 — taxonomy vocabulary/descriptions | Keep author-driven, open-ended news labels, separate from wiki taxonomy. Demo currently has only category **Project Updates**, tags **search**, **pagefind**, **tiny-http**; do not seed extra labels. Proposed category description: **Changes to Haleiki, its demo wiki, and supporting tools.** Curated descriptions are optional; absent descriptions stay empty. Tags use their existing names without invented descriptions. | A controlled vocabulary/description catalogue can be explicitly selected, but would add authoring policy beyond today's required nonempty category/string-list tags. The preview shows exactly the current one-category/three-tag result. |
| G3 — RSS/feed disposition | Defer feed delivery with a documented re-entry condition: choose a canonical published site URL and implement/test feed output. Hide the visible **Subscribe via RSS** link while no feed exists. The current profile supplies no canonical public base URL, and the index template currently renders that link unconditionally. | Deliver `/news/feed.xml` in Slice 03 after the Operator supplies/chooses the canonical site URL and intended RSS contract. Keeping a visible dead feed link is not a recommended disposition. |
| G4 — Pagefind news participation | Include news posts in Arc 05 Slice 03, with `type=news` and news category metadata. Do not invent wiki tier or epistemic-trust classifications for announcements; omit those news-only inapplicable filters. Existing source/concept records and facets must remain intact. | Defer news search to Arc 06, explicitly retaining source/concept-only results until that later authorized work. The current post is rendered but intentionally absent from Pagefind. |

Historical packet status before the actual answer: all four recommendations
were unapproved. G1/G2 decisions were needed before
source implementation of Slice 02. G3/G4 can be answered now or retained for
their later Slice 03 gate; approval of G1/G2 alone does not settle them. A feed
deferral requires the visible-link handling above (or an explicit alternative).

After G1/G2 resolution, CDC will record the Operator's choices and prepare the
source-grounded Slice 02 open set/prompt, then parent resumes the selected CC
`gpt-6-luna / high`. Existing singular category/tag post links must acquire real
matching targets alongside the plural index views. The literal “1 posts” copy
visible in the unchanged templates should receive ordinary singular/plural
handling in that slice; these previews do not silently repair it. Prev/next
and related-post multi-post policy and complete sink composition still need
their assigned later evidence. Browser/native gates and a separately attributed
independent arc composition review remain mandatory before any arc close.

No dependent source default has been selected. Arc 05 authority remains active
across this discussion boundary; no Arc 06, push, merge, publication or deployment
is authorized. Next action owner: Operator through parent relay. CDC owns the
stable source/planning trees while awaiting that decision.

## Actual Operator approval — G1–G4, 2026-11-11

The parent relay received the actual answer **“Approve all four
recommendations”** to the exact G1–G4 async question grounded in the packet
and reviewed previews above. This is a received Operator decision, not elapsed
time, parent preference or an inferred default. All four reserved product gates
are resolved; do not ask to reconfirm them or unchanged contributor settings.

1. **G1:** `/news/archive/` lists all posts on one page, newest first, grouped
   by calendar month/year; no pagination, `pagination: null`.
2. **G2:** news labels remain author-driven/open-ended, separate from wiki
   taxonomy. Demo keeps category **Project Updates** and tags **search**,
   **pagefind**, **tiny-http** only. Optional curated category description:
   **Changes to Haleiki, its demo wiki, and supporting tools.** Missing
   descriptions are empty; do not invent tag descriptions or editorial labels.
3. **G3:** defer RSS until a canonical public site URL is chosen and the feed
   contract is implemented/tested. Hide **Subscribe via RSS** while the feed
   is absent. Preserve the explicit disposition and re-entry condition.
4. **G4:** include news in Pagefind during Arc 05 Slice 03 with news type and
   category metadata; no invented wiki tier/trust values. Existing source/
   concept records and facets remain intact.

Execution resumes under the same Autonomous authority through end of Arc 05.
CDC Locke (`01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`) remains explicitly requested/
tool-selected OpenAI `gpt-6.1-sol / high`; CC Kierkegaard
(`01a1ccf7-3e7c-7ab2-ae59-72c9be8f457b`) resumes as explicitly selected
`gpt-6-luna / high` through parent dispatch. Backend identity remains
unreported. Closing inactive contexts released slots; it did not authorize a
model substitution or change the independent implementation-review split.

Stable entry states: source `5fe3cf46fa1ea673b0b84f09167e36de8611d3ee`, clean
`framework-base/tiny-http`; planning `e9c2b05272f6f42c9921eb5588e4cd265e6b675e`,
clean before this approval update. CDC owns planning and source inspection;
CC receives source mutation/gates only after its new durable Slice 02 packet.
No CC source work is active during preparation. Parent remains dispatch/relay.
No push, merge, publication, deployment or Arc 06 opening is authorized.
Fresh independent arc composition and the arc retrospective remain mandatory;
no project acceptance is inherited from child closes.

## Current Slice 02 assignment — 2026-11-11

The actual G1–G4 approval is recorded above. The existing roadmap is retained;
Slice 02 is now open and ready for parent to resume CC from the durable packet:

`arc05-news-blog-content-type/slice02-news-index-archives-taxonomy/cc-prompt.md`

Read its complete open set (plan, ledger, prompt). Ten F rows are open; E-2…E-7
and all remaining project rows stay open. No source change or passing candidate
gate is claimed by packet preparation. Source remains clean 5fe3cf46; the
containing planning issuance commit records both actual approval and open set.

This slice implements four listing templates, all matching singular taxonomy
targets, ordering/grouping/counts/optional descriptions, chronological neighbors,
explicit no-related policy, empty states/escaping/count copy, and hides the absent
RSS link. Extra population fixtures stay isolated tests, not demo editorial
posts. News Pagefind inclusion remains Slice 03, followed by complete sink/search/
feed-deferral composition, fresh independent arc review and CDC retrospective.
Routine implementation policy is fully specified in the prompt; no G1/G2 gate
remains. Source README/AGENTS reconciliation belongs to CC's narrow allowlist.

CDC Locke remains gpt-6.1-sol/high; CC Kierkegaard resumes the actual existing
ID as gpt-6-luna/high, requested/tool-selected identities with backend identity
unreported. Parent owns dispatch only. On dispatch CC exclusively owns source
mutation/index/gates plus its designated Slice 02 closing report and cc-* evidence;
CDC keeps those stable and owns remaining planning/index. After explicit handback
CDC independently reviews, verifies, closes/bubbles up and prepares Slice 03.
Authority stays active through end of Arc 05; no new arc or external action.
