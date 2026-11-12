# Slice 01 — news model + post route · Ledger

> Grep-verifiable acceptance criteria. Per the collaboration-framework's
> `LEDGER-DISCIPLINE.md`, every row reaches a final status (`done` /
> `deferred` / `no-op`) before the slice advances; `done` requires reproduced
> evidence. Status: **open** · 2026-08-21.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | The slice records the operator-confirmed authoring location, URL model, and minimum/optional frontmatter fields after concrete examples are presented | read `slice-doc.md` decision log / closing report; verify examples are included | serious | operator | open | | Discussion gate; do not implement on unstated assumptions. |
| F-2 | The current demo corpus is audited for existing news/blog content; if minimal demo fixtures are added, the closing report explains why this is test/demo content rather than product editorial scope creep | `find demo -maxdepth 3 -type f` / `rg -n "type: news|category:|tags:" demo`; inspect fixture paths | correctness | project boundary | open | | |
| F-3 | News posts parse into a first-class model with title, date, category, tags, optional author, optional excerpt, slug/url, and Markdown body | targeted parser unit test(s) | serious | templates/README.md | open | | |
| F-4 | Invalid news frontmatter fails clearly for missing/invalid required fields, especially title/date/category | targeted parser/validator tests | serious | robustness | open | | |
| F-5 | The real `templates/pages/news-post.jinja` template renders one parsed post with the expected `post.*` context and keeps only `post.body` safe HTML | targeted render test; grep rendered HTML for title/date/category/tags/body | serious | template contract | open | | |
| F-6 | The post route is present in the shared route set with the confirmed URL model and no collision with concept/source/static routes | route/render test; grep route output | correctness | Arc 02/03 architecture | open | | |
| F-7 | `prev`, `next`, and `related` are either populated correctly for available demo posts or explicitly empty/null with a documented policy | targeted context test; inspect rendered post navigation/related section | correctness | template contract | open | | |
| F-8 | The default project gate remains green | `make check` | correctness | repo convention | open | | |

## What Worked

_Filled at close._

## Current authority note — 2026-11-11

F-1's original reference to the slice-doc decision log now resolves through
the canonical `slice-plan.md` Operator decision log and numbered assignment;
the issued `slice-doc.md` remains unchanged history. All eight row criteria,
statuses and original verification descriptions are preserved. Preparation
does not independently close F-1 or any implementation/gate row.

Independent review 01: see `cdc-verification.md`. R1 affects F-5/F-7
test discrimination; full F-8 reproduction awaits the corrected candidate.
Current assignment is `cc-prompt-iteration02.md`; every F row remains open.
