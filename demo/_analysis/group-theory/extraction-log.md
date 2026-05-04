# Extraction Log: group-theory

**Source page:** `group-theory` (mathematics/algebra, foundational)
**Date:** 2026-05-03

## Concept Inventory

| # | Concept | Slug | Tier | Confidence | Notes |
|---|---------|------|------|------------|-------|
| 1 | Group theory | group-theory-mathematics | foundational | high | The main topic; explicitly defined in opening paragraph |
| 2 | Permutation group | permutation-group | intermediate | high | Dedicated section with clear definition |
| 3 | Lie group | lie-group | intermediate | high | Dedicated section, well-defined |
| 4 | Representation theory | representation-theory | intermediate | medium | Discussed extensively but definition synthesized from context |
| 5 | Symmetry group | symmetry-group | foundational | high | Clearly explained in "Connection of groups and symmetry" section |
| 6 | Finite group | finite-group | intermediate | medium | Discussed across multiple sections, no single definition |

## Disambiguation

- "Group" is in the disambiguation list: slug `group-mathematics`, disambiguator `mathematics`. However, the source `group-mathematics` already exists as a separate source page. The concept here is "group theory" (the study), not "group" (the algebraic structure). Using `group-theory-mathematics` to distinguish from the source page slug.

## Extraction Decisions

- **Extracted (6 concepts):** group-theory-mathematics, permutation-group, lie-group, representation-theory, symmetry-group, finite-group
- **Skipped:** Matrix groups (subsumed under permutation-group and transformation groups discussion, not enough standalone definition), transformation groups (too closely overlaps symmetry-group), abstract groups (the abstraction concept is covered within group-theory-mathematics), Galois theory (mentioned as application but the source page doesn't define it thoroughly enough for a standalone card), combinatorial/geometric group theory (mentioned but not defined in enough depth)
- **Cross-source notes:** None expected for this source page.

## Dependency Order

1. symmetry-group (foundational, no wiki prerequisites)
2. group-theory-mathematics (foundational)
3. permutation-group (requires group-theory-mathematics)
4. finite-group (requires group-theory-mathematics)
5. lie-group (requires group-theory-mathematics)
6. representation-theory (requires group-theory-mathematics, lie-group)
