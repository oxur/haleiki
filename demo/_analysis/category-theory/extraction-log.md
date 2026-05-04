# Extraction Log: category-theory

**Source page:** `category-theory` (mathematics/foundations, advanced)
**Date:** 2026-05-03

## Concept Inventory

| # | Concept | Slug | Tier | Confidence | Notes |
|---|---------|------|------|------------|-------|
| 1 | Category theory | category-theory-mathematics | advanced | high | Main topic, explicitly defined |
| 2 | Category | category-mathematics | intermediate | high | Formal definition with axioms |
| 3 | Functor | functor | intermediate | high | Defined with covariant/contravariant distinction |
| 4 | Natural transformation | natural-transformation | advanced | high | Defined with commutative diagram |
| 5 | Morphism | morphism | intermediate | high | Defined with many properties listed |
| 6 | Universal property | universal-property | advanced | medium | Discussed but definition synthesized |

## Extraction Decisions

- **Extracted (5):** category-theory-mathematics, category-mathematics, functor, natural-transformation, morphism
- **Skipped:** Universal property (discussed, but definition is brief and abstract), Adjoint functor (mentioned, not defined), Yoneda lemma (mentioned, not defined), Topos (mentioned in history, not defined), n-category (mentioned, specialist)

## Dependency Order

1. morphism (intermediate)
2. category-mathematics (intermediate, requires morphism)
3. functor (intermediate, requires category-mathematics)
4. natural-transformation (advanced, requires functor)
5. category-theory-mathematics (advanced, requires all above)
