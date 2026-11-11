---
project: project01-demo-site
status: historical-reconstructed
depends-on: []
blocks: []
related: ["project02-concept-cards", "project03-cobalt-site-build"]
---

# Demo site and source acquisition

Historical period: 2026-03-17–2026-05-03. Reconstruction date: 2026-11-11.

Reconstructed on 2026-11-11 for historical navigation only. Git subjects attest implementation; no historical tests were rerun and no new acceptance is claimed. Missing reports, prompts and independent review have not been invented. Original documents, including stale instructions and links, are preserved in `archive/`; use the root migration manifest to resolve their former paths.

## Definition of done and boundaries

Acquire and transform a licensed Wikimedia/MediaWiki corpus into local source pages and media. The original ten-phase demo plan is preserved; only phases 1–5 have detailed milestone files here.

This is a finding aid for past work, not an approved new implementation plan.
Historical commands/specifications remain in the preserved sources. Closure,
publication and current runtime correctness are not established by this migration.

## Roadmap and recorded status

| Arc | Capability |
|-----|------------|
| [arc01-scaffolding-and-manifest](arc01-scaffolding-and-manifest/arc-plan.md) | Scaffolding and manifest |
| [arc02-fetch-pipeline](arc02-fetch-pipeline/arc-plan.md) | Fetch pipeline |
| [arc03-clean-and-rewrite](arc03-clean-and-rewrite/arc-plan.md) | HTML cleaning and links |
| [arc04-media-pipeline](arc04-media-pipeline/arc-plan.md) | Media acquisition and attribution data |
| [arc05-markdown-and-frontmatter](arc05-markdown-and-frontmatter/arc-plan.md) | Markdown source corpus |

Numbers follow the first coherent delivery effort in the source ancestry, not the
filesystem modification time of ignored drafts. Early architecture, brand and
profile designs also informed later projects; links do not imply new chronology.

## Source records

- [docs/design/01-draft/0001-haleiki-architecture-document.md](../archive/docs/design/01-draft/0001-haleiki-architecture-document.md)
- [docs/design/05-active/0002-haleiki-demo-site-design-document.md](../archive/docs/design/05-active/0002-haleiki-demo-site-design-document.md)
- [docs/design/05-active/0003-haleiki-demo-site-project-plan.md](../archive/docs/design/05-active/0003-haleiki-demo-site-project-plan.md)

The root [manifest](../migration-manifest.json) maps every former path
to its byte-preserved copy, including untracked workbench drafts and binary assets.
[History](../history-commits.txt) pins commit subjects to the source baseline.

## Unresolved scope and carryover

Original phases 6–10 remain historical planned scope, not declared done. Build integration, attribution and development/deployment work appear later in Project 03; concept extraction appears in Project 02. Original CI/public deployment and other residual criteria are not accepted by inference. April–May pipeline fixes are recorded by commits 3c23242, d805985, 3242d91, 43b36ae, 3285cfb, 287feb8, 06f534c and 2c99f24. Commit 8b18590 records 91 articles, 10 categories and 1952 images. These are Git-subject attestations; no extra repair slices or missing reports were invented.

See [project ledger](ledger.md). Rows are deliberately
open where historical acceptance is missing. No modern retrospective has been
backdated or substituted for an original acceptance record.

## Workflow and authority

Reconstruction only under the Operator's 2026-11-11 migration request; no code
execution assignment or new contributor arrangement. Future implementation uses
the default CDC + CC workflow unless the Operator selects otherwise. Existing
independent/Operator gates remain binding. Plans, ledgers and supporting records
live on orphan branch `planning` in `.worktrees/planning`.

## Version history

- 2026-11-11: v1.0 finding aid reconstructed from existing files and Git messages;
  missing content intentionally not supplied, no historical acceptance upgraded.
