# slice01-news-model-and-post — plan entrypoint

The preserved [slice-doc.md](./slice-doc.md) is this slice's original plan.
Read it in full with [ledger.md](./ledger.md) and [cc-prompt.md](./cc-prompt.md).
This canonical filename is an entrypoint, not a replacement or reduced contract.

Status: open; four content-model decisions approved by the Operator on 2026-11-11.
Source checkout: `/Users/oubiwann/lab/oxur/haleiki`, branch
`framework-base/tiny-http`; migration baseline `ffe2dd78994e`.
Historical `docs/tiny-http/` references map to this project's root. Source-code,
demo and template paths refer to the implementation checkout. Other old document
paths resolve through the planning root's migration manifest.

## Assignment and evidence preservation

The initial `cc-prompt.md` remains preserved with its existing scope and gates.
No new assignment or iteration was issued by the migration. Existing close and
verification records, where present, retain their authorship and evidence strength.
New follow-ups use `cc-prompt-iterationNN.md`; never silently rewrite the initial prompt.
Artifacts: retain existing locations; new supporting outputs default to `artifacts/`.

## Version history

- 2026-11-11: added canonical plan entrypoint during the Operator-authorized migration.
- 2026-11-11: retained Operator decision log and issued source-grounded iteration 01;
  recorded actual CDC attribution, parent dispatch route, ownership and readiness.

## Operator decision log — 2026-11-11

The Operator approved all four recommendations presented in this chat. This
decision log resolves the initial discussion gate in the preserved `slice-doc.md`
and initial `cc-prompt.md`; it does not rewrite their issued historical content.

1. Authoring: `news/*.md` under the selected content root; demo posts live at
   `demo/news/*.md`, alongside `sources/` and `concepts/`.
2. URL: `/news/<slug>/`, with slug derived from the Markdown filename. Dates
   drive chronology without being embedded in the permalink.
3. Required frontmatter: `type: news`, nonempty `title`, strict valid calendar
   `date` as `YYYY-MM-DD`, one nonempty `category`, and a `tags` string list
   (which may be empty). Optional: `author` and `excerpt`. Derive category/tag
   URL slugs; omit the byline when author is absent. Body is authored Markdown.
4. Fixtures: add one small realistic demo post because no existing authored
   news post was found; use isolated additional fixtures for meaningful tests.
   With one demo post, `prev` and `next` are explicitly null and `related` empty.

Concrete approved example: `demo/news/search-lands-in-the-demo-wiki.md`, served
at `/news/search-lands-in-the-demo-wiki/`:

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

Search now supports highlighted results and category, tier,
content-type, and trust filters.
```

Authoring and frontmatter failure controls must distinguish missing, wrong-typed,
empty and invalid-date input. Keep concept/source behavior and shared sink routes
unchanged except for the additive news route. No full news index, archive,
taxonomy surface, RSS/feed or news-search implementation is assigned by Slice 01.

## Assignment history

| Assignment | Issued | Predecessor / reason | Disposition |
|---|---|---|---|
| `cc-prompt.md` | 2026-08-21 original record | Initial one-post assignment; initial discussion gate | Preserved; no CC implementation started; superseded by iteration 01 after approvals |
| `cc-prompt-iteration01.md` | 2026-11-11 | `cc-prompt.md`; approved model decisions, live source grounding, intake/ownership and discriminating tests | Current, issued to parent for explicit `gpt-6-luna / high` CC dispatch; implementation not yet started |

Current project-relative path:
`arc05-news-blog-content-type/slice01-news-model-and-post/cc-prompt-iteration01.md`.
Replacing the unstarted original prompt is not an executed correction round;
the five-refinement limit remains intact. Preserve every issued prompt.

CDC Locke (`01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`) owns planning/index work
and later independent source review. Requested/tool-selected: OpenAI
`gpt-6.1-sol / high`, per parent receipt; backend identity unreported. Parent
will dispatch source implementer CC with exact `gpt-6-luna / high`, then relay
its actual ID/settings and results to Locke. No CC context assigned yet.
CC exclusively owns source writes/staging/builds while assigned, plus this
slice's `closing-report.md` for intake and attested handoff. CDC does not
concurrently edit that report. CC cannot mutate/stage other planning files.
After return, CDC owns stable-source reproduction and planning commits.

## Author readiness and evidence boundary — 2026-11-11

The assignment's source spine, applied-guidance table, test matrix and intake
manifest contain the six prompt-authoring readiness checks. Author: Locke.
Baseline source remains clean at `75d2593d51c986346f0239bed36a248ac5b4e6f7`.
The full prompt-authoring guide (524 lines), collaboration 3.0.1, planning
layout/iteration/closing guides, Autonomous/model authority, agent packets,
testing/verification and retrospective contracts were loaded. Repo Rust
entrypoint and full anti-patterns chapter (2,188 lines) plus applied topic
sections were loaded before the source recipe; truncated reads were recovered.
Source/test inspection covers parser/model, shared routing, export/server,
Markdown/escaping, graph/wiki consumers, Pagefind coupling, Corpus fixture
construction sites and actual Make gate effects. No source was implemented.

Fresh full frontmatter scan at authoring: 752 recursively enumerated demo
Markdown files, 91 direct authored sources, 186 direct concepts, zero direct
news; content root zero Markdown; zero news/blog type matches, no read errors.
The Python query checked fenced frontmatter type/page_type across every file;
generated copies were not counted as new authored material. The CC manifest
requires a fresh population check and records drift rather than enforcing
these counts as a desired answer. All F rows remain open until independent
implementation acceptance. No make/test gate was run during preparation.

Author decisions within the approved single-post increment: Gregorian strict
date parsing and August 21, 2026 display; existing shared slug normalization;
explicit parse/route collision errors; metadata escaping with rendered body
safe only; null/null/[] post relationships for the sole demo post. Later
multi-post policy, archives/taxonomy/feed/search discussion gates remain open.
The uncompiled shapes are recommendations, with binding observable outcomes.

The prior live migration verifier remains frozen: it expects original live
ledgers/source, so it is not a new-development gate and must not be weakened.
Original prompt SHA-256: `206bdbd2b09551ca45f9c3a747f976cc5e40935a84398ed9dd9b4c9837b21e86`;
original slice-doc: `e17ea089e7ec680ddc6cfe5ebfd799ed16f93bd9341969ba99722f4b9ec6ca15`.
Retrospectives/verification records will be authored at actual closing, not
pre-filled at this phase. Autonomous authority continues through dispatch
and review; this phase return is not Slice 01 or Arc 05 closure.
