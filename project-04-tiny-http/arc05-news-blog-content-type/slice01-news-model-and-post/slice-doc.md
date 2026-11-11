# Slice 01 — news model + post route

> Plan-of-record for the first slice of Arc 05 (News / Blog Content Type).
> Open set: this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** · 2026-08-21.

---

## Goal

Make the smallest real news/blog path work end to end: an authored news post is
parsed as a new content kind, assembled into the `post.*` context required by
`templates/pages/news-post.jinja`, and rendered at a stable route.

This slice establishes the load-bearing model. It intentionally stops before
building the full news index/archive/tag/category surfaces.

## Definition

In Haleiki, **News is blog**: dated wiki-maintainer updates and related news
items. Example subjects include release notes, corpus additions, editorial
changes, site improvements, and related project announcements.

News posts are read-only content, like concepts and sources. They are not user
comments, not an admin UI, and not a CMS workflow.

## Discussion Gate

Before implementation, CC must present concrete examples and get operator
confirmation for:

- the authoring location;
- the post URL model;
- the minimum frontmatter fields and optional fields;
- whether this slice uses existing demo content if found, or adds minimal
  demo/test fixtures if no news posts exist.

The recommended starting shape to discuss is:

```yaml
---
type: news
title: "Search Lands in the Demo Wiki"
date: 2026-08-21
category: "Project Updates"
tags: ["search", "pagefind", "tiny-http"]
author: "Haleiki Maintainers"
excerpt: "Client-side Pagefind search is now wired into the demo wiki."
---
```

The recommended starting URL shape to discuss is one of:

- `/news/<slug>/`
- `/news/<yyyy>/<mm>/<dd>/<slug>/`

Do not treat either as final until the operator confirms.

## Scope

### In

- Audit the current demo corpus for any existing news/blog content.
- Add a `NewsPost`/equivalent model only as needed for the parsed content.
- Parse news frontmatter and body from the chosen authoring location.
- Render one real/demo post through `templates/pages/news-post.jinja`.
- Add the post route to the shared route set so export/server can use the same
  renderer later.
- Implement enough date formatting, category slugging, tag slugging, and
  `post.body | safe` Markdown rendering to satisfy the single-post template.
- Set `prev`, `next`, and `related` to correct values if multiple posts exist;
  otherwise use explicit empty/null values that the template handles cleanly.
- Focused unit/integration tests for parsing and rendering the first post.

### Out

- The news index (`/news/`), archive, tag index, and category index.
- RSS/feed generation.
- Pagefind/search integration for news posts.
- Full static export/server parity for the news route set.
- Trust bars on news posts.
- Broad visual polish against the design review.

## Verification Approach

- Grep/read the operator-confirmed decision log in this slice doc.
- Run targeted parser/render tests for the news model and one post route.
- Render the post with minijinja and assert required fields appear:
  title, date, category link, tag links, author when present, and HTML body.
- Run `make check` before close unless there is a documented environment
  boundary.

## Exit Criteria

Ledger rows F-1...F-8 reach final status. One news/blog post renders from parsed
Markdown through the real `news-post.jinja` template. The model and route
choices are recorded, and the slice leaves enough concrete rendered evidence for
Slice 02's index/archive/taxonomy discussions.
