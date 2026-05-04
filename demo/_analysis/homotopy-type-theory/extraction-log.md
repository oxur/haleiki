# Extraction Log: homotopy-type-theory

**Source page:** `homotopy-type-theory` (mathematics/foundations, advanced)
**Date:** 2026-05-03

## Concept Inventory

| # | Concept | Slug | Tier | Confidence | Notes |
|---|---------|------|------|------------|-------|
| 1 | Homotopy type theory | homotopy-type-theory-concept | advanced | high | Main topic, explicitly described |
| 2 | Univalence axiom | univalence-axiom | advanced | high | Dedicated subsection |
| 3 | Higher inductive type | higher-inductive-type | advanced | medium | Discussed in history, definition synthesized |
| 4 | Univalent foundations | univalent-foundations | advanced | medium | Discussed as related program |

## Extraction Decisions

- **Extracted (3):** homotopy-type-theory-concept, univalence-axiom, univalent-foundations
- **Skipped:** Higher inductive type (thin treatment, still active research), Cubical type theory (mentioned, specialist variant), Groupoid model (historical, specialist)

## Dependency Order

1. homotopy-type-theory-concept (advanced, requires type-theory-foundations, homotopy-concept)
2. univalence-axiom (advanced, requires homotopy-type-theory-concept)
3. univalent-foundations (advanced, requires univalence-axiom)
