# Arc 05 — News / Blog Content Type

> The arc plan-of-record: add Haleiki's news/blog surface to the tiny_http pivot.
> Part of the tiny_http project ([`../project-plan.md`](../project-plan.md)).
> Status: **active** · 2026-08-21.

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
| **01 — news model + post route** | Define the news authoring/frontmatter model, parse demo news posts, render one real post through `news-post.jinja`, and choose the post URL/date/display conventions with operator confirmation from concrete examples. | 02, 03, arc close |
| **02 — news index + archives + taxonomy views** | Assemble the `news.*` context for `news-index`, `news-archive`, `news-tags`, and `news-categories`; implement date grouping, category/tag counts, and listing routes. Use rendered examples to settle archive pagination, taxonomy descriptions, and visual/content copy questions. | 03, arc close |
| **03 — sink parity + feed/search disposition + arc close** | Prove the full news/blog surface renders in static export and `haleiki serve`; decide and implement or defer `/news/feed.xml`; decide whether news posts participate in Pagefind search now or in Arc 06; run project gates and write the arc closing report. | project close |

## 3. Decisions and Discussion Gates

Confirmed:

- News is blog: dated maintainer updates and related news items for the wiki.
- News is a new content kind added alongside concepts and sources.
- News remains read-only.
- Trust bars stay off news posts unless the operator explicitly reverses the
  design-review note in `templates/README.md`.

Discussion-gated:

- Authoring location and post URL model, after CC shows concrete sample paths.
- Minimum required frontmatter and optional fields, after CC shows concrete demo
  posts.
- Whether demo news fixtures are existing content or minimal test/demo fixtures.
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

Opens here; closes in `closing-report.md` at arc close. Class (a) = slices
closed; class (b) = composition reproduced at arc scale; class (c) = bubble-up
findings dispositioned.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| E-1 | Slice 01 (news model + post route) closed | ptr: `slice01-news-model-and-post/cdc-verification.md` | correctness | arc-plan | open | |
| E-2 | Slice 02 (news index + archives + taxonomy views) closed | ptr: slice02 cdc-verification | correctness | arc-plan | open | |
| E-3 | Slice 03 (sink parity + feed/search disposition + arc close) closed | ptr: slice03 cdc-verification | correctness | arc-plan | open | |
| E-4 | The full news/blog route set renders from real/demo posts: `/news/`, at least one post route, `/news/archive/`, `/news/tags/`, and `/news/categories/` | reproduce at arc scale: build/export, inspect route files and rendered HTML | serious | project-plan P-10 | open | |
| E-5 | Both sinks serve identical news pages for the implemented route set | reproduce: compare static export files against live `haleiki serve` responses for news routes | serious | project-plan P-10 | open | |
| E-6 | News metadata composes correctly: date displays, category/tag links, prev/next, related posts or an explicit no-related policy | reproduce: inspect rendered post and list pages; run targeted tests for context assembly | correctness | templates/README.md | open | |
| E-7 | RSS/feed and search participation are dispositioned with operator-visible evidence | inspect Slice 03 decision log; verify implemented route(s) or documented deferral/re-entry | correctness | template contract / Arc 04 bubble-up | open | |

## 6. Version History

### v1.0 — 2026-08-21
Initial Arc 05 plan. Records the operator decision that News is Haleiki's blog:
wiki updates and related news items. Opens a three-slice path: news model + post
route, news listing/archive/taxonomy surfaces, then sink parity plus feed/search
disposition and arc close. Discussion gates are explicit so product/design
choices happen with rendered examples in hand.
