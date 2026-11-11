# CC Prompt — Arc 05 Slice 01: news model + post route

You are implementing **Arc 05 Slice 01 — news model + post route** in Haleiki.

Read first:

- `AGENTS.md`
- `docs/tiny-http/project-plan.md`
- `docs/tiny-http/arc05-news-blog-content-type/arc-plan.md`
- `docs/tiny-http/arc05-news-blog-content-type/slice01-news-model-and-post/slice-doc.md`
- `docs/tiny-http/arc05-news-blog-content-type/slice01-news-model-and-post/ledger.md`
- `templates/README.md`
- `templates/pages/news-post.jinja`

## Core Direction

In Haleiki, **News is blog**: dated wiki-maintainer updates and related news
items. This slice should make one parsed news post render through the real
`news-post.jinja` template. Do not build the full news index/archive/tag/category
surface yet.

## Required Discussion Gate

Before coding, audit the existing demo corpus for news/blog content and present
concrete examples to the operator. Get confirmation for:

- authoring location;
- post URL model;
- required and optional frontmatter fields;
- whether to add minimal demo/test fixtures if no suitable posts already exist.

Reasonable starting proposal:

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

Possible URL models to discuss:

- `/news/<slug>/`
- `/news/<yyyy>/<mm>/<dd>/<slug>/`

Do not silently choose if the operator has not confirmed.

## Implementation Boundaries

In scope:

- parse a news/blog content kind;
- render one post through `templates/pages/news-post.jinja`;
- add focused tests for parsing, validation, context assembly, and the route;
- keep the shared renderer/export/server architecture compatible with later
  slices.

Out of scope:

- `/news/`, `/news/archive/`, `/news/tags/`, `/news/categories/`;
- RSS/feed generation;
- Pagefind/search integration for news;
- trust bars on news posts;
- visual polish beyond making the real template render correctly.

## Ledger Discipline

Work row-by-row against `ledger.md`. If a row is wrong, ask for an amendment
rather than working around it. The closing report must walk F-1...F-8 with
evidence for each row.

Run `make check` before close. If localhost permissions are required for live
server tests in this environment, say so explicitly and distinguish sandbox
permission failures from project failures.

Commit only when the ledger is proposed-done, and include:

```
Co-authored-by: Codex <noreply@openai.com>
Co-authored-by: Billo AI <ai-engineering@billo.systems>
```
