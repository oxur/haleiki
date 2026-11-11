---
project: project02-concept-cards
status: historical-reconstructed
depends-on: ["project01-demo-site"]
blocks: []
related: ["project03-cobalt-site-build", "project04-tiny-http"]
---

# Concept-card extraction

Historical period: 2026-05-03. Reconstruction date: 2026-11-11.

Reconstructed on 2026-11-11 for historical navigation only. Git subjects attest implementation; no historical tests were rerun and no new acceptance is claimed. Missing reports, prompts and independent review have not been invented. Original documents, including stale instructions and links, are preserved in `archive/`; use the root migration manifest to resolve their former paths.

## Definition of done and boundaries

Produce concept cards and bidirectional source provenance for the demo corpus. Git records 27 pilot cards plus 159 cards from the remaining corpus; these figures are historical attestations.

This is a finding aid for past work, not an approved new implementation plan.
Historical commands/specifications remain in the preserved sources. Closure,
publication and current runtime correctness are not established by this migration.

## Roadmap and recorded status

| Arc | Capability |
|-----|------------|
| [arc01-corpus-extraction](arc01-corpus-extraction/arc-plan.md) | Pilot, corpus extraction and source backlinks |

Numbers follow the first coherent delivery effort in the source ancestry, not the
filesystem modification time of ignored drafts. Early architecture, brand and
profile designs also informed later projects; links do not imply new chronology.

## Source records

- [docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md](../archive/docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md)
- [docs/dev/concept-cards/0002-a-guide-for-parallel-concept-card-extraction.md](../archive/docs/dev/concept-cards/0002-a-guide-for-parallel-concept-card-extraction.md)
- [workbench/0009-howto-concept-card-extraction-with-claude-code-v3.2.md](../archive/workbench/0009-howto-concept-card-extraction-with-claude-code-v3.2.md)
- [workbench/0010-a-guide-for-parallel-concept-card-re-extraction-v3.2.md](../archive/workbench/0010-a-guide-for-parallel-concept-card-re-extraction-v3.2.md)

The root [manifest](../migration-manifest.json) maps every former path
to its byte-preserved copy, including untracked workbench drafts and binary assets.
[History](../history-commits.txt) pins commit subjects to the source baseline.

## Unresolved scope and carryover

The upstream v3.2 howto/re-extraction guides are methodological inputs, not proof another Haleiki re-extraction occurred. No unrecorded run, automated extraction CLI, merge engine or quality acceptance is reconstructed. Guide v1.3 slug-namespace changes also cross-reference Project 03 M2 fixes.

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
