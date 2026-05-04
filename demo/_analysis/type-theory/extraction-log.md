# Extraction Log: type-theory

**Source page:** `type-theory` (mathematics/foundations, intermediate)
**Date:** 2026-05-03

## Concept Inventory

| # | Concept | Slug | Tier | Confidence | Notes |
|---|---------|------|------|------------|-------|
| 1 | Type theory | type-theory-foundations | intermediate | high | Main topic, explicitly defined |
| 2 | Lambda calculus | lambda-calculus | intermediate | high | Central to type theory, well-described |
| 3 | Curry-Howard correspondence | curry-howard-correspondence | intermediate | high | Dedicated section |
| 4 | Dependent type | dependent-type | advanced | high | Dedicated section |
| 5 | Intuitionistic type theory | intuitionistic-type-theory | advanced | medium | Discussed, definition synthesized |
| 6 | Identity type | identity-type | advanced | medium | Dedicated subsection |

## Extraction Decisions

- **Extracted (5):** type-theory-foundations, lambda-calculus, curry-howard-correspondence, dependent-type, intuitionistic-type-theory
- **Skipped:** Identity type (specialist, better from HoTT source), Product/Sum types (internal detail), Proof assistants (applications list, not concepts), Simply typed lambda calculus (variant listed, not deeply defined)

## Dependency Order

1. lambda-calculus (intermediate)
2. type-theory-foundations (intermediate, requires lambda-calculus)
3. curry-howard-correspondence (intermediate, requires type-theory-foundations)
4. dependent-type (advanced, requires type-theory-foundations)
5. intuitionistic-type-theory (advanced, requires dependent-type)
