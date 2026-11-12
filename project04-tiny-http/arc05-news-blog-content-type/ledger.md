# Arc ledger

> Extracted without row changes on 2026-11-11 from the August plan. Existing dispositions/evidence are retained, not freshly reproduced by the migration.

Opens here; closes in `closing-report.md` at arc close. Class (a) = slices
closed; class (b) = composition reproduced at arc scale; class (c) = bubble-up
findings dispositioned.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| E-1 | Slice 01 (news model + post route) closed | ptr: `slice01-news-model-and-post/cdc-verification.md` | correctness | arc-plan | done | accepted child at source 5fe3cf46; CDC final row walk and retrospective complete; no composition inheritance |
| E-2 | Slice 02 (news index + archives + taxonomy views) closed | ptr: `slice02-news-index-archives-taxonomy/cdc-verification.md` (pending close) | correctness | arc-plan | open | |
| E-3 | Slice 03 (sink parity + feed/search disposition + arc close) closed | ptr: slice03 cdc-verification | correctness | arc-plan | open | |
| E-4 | The full news/blog route set renders from real/demo posts: `/news/`, at least one post route, `/news/archive/`, `/news/tags/`, and `/news/categories/` | reproduce at arc scale: build/export, inspect route files and rendered HTML | serious | project-plan P-10 | open | |
| E-5 | Both sinks serve identical news pages for the implemented route set | reproduce: compare static export files against live `haleiki serve` responses for news routes | serious | project-plan P-10 | open | |
| E-6 | News metadata composes correctly: date displays, category/tag links, prev/next, related posts or an explicit no-related policy | reproduce: inspect rendered post and list pages; run targeted tests for context assembly | correctness | templates/README.md | open | |
| E-7 | RSS/feed and search participation are dispositioned with operator-visible evidence | inspect Slice 03 decision log; verify implemented route(s) or documented deferral/re-entry | correctness | template contract / Arc 04 bubble-up | open | |
