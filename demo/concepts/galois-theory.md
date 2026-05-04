---
# === CORE IDENTIFICATION ===
concept: "Galois Theory"
slug: "galois-theory"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "algebra"
tier: "intermediate"
subject_headings:
  - "abstract-algebra"
keywords:
  - "Galois group"
  - "polynomial equations"
  - "solvability by radicals"
  - "field extensions"

# === PROVENANCE ===
derived_from:
  - slug: "abstract-algebra"
    title: "Abstract algebra"
  - slug: "history-of-group-theory"
    title: "History of group theory"
  - slug: "group-mathematics"
    title: "Group (mathematics)"

# === CONFIDENCE ===
extraction_confidence: "medium"

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "group-theory-mathematics"
  - "field-mathematics"
extends: []
related:
  - "permutation-group"
  - "abstract-algebra-field"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does group theory relate to abstract algebra?"
  - "What foundational concepts does group theory build upon?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Galois theory is the branch of abstract algebra that connects field extensions to group theory by associating a group of symmetries (the Galois group) to the roots of a polynomial equation. It definitively explains which polynomial equations can be solved by radicals.

## Core Definition

Galois theory studies the symmetry group of the roots of a polynomial equation, now called the Galois group. Galois found that if $r_1, r_2, \ldots, r_n$ are the roots of an equation, there is always a group of permutations of the roots such that every function of the roots invariable by the substitutions of the group is rationally known, and conversely. The solvability of the Galois group determines the solvability of the equation with radicals ([history-of-group-theory](/source/history-of-group-theory/)). Galois's work linking group theory and field theory created the subject ([history-of-group-theory](/source/history-of-group-theory/)).

## Prerequisites

- **Group theory** -- the Galois group is a group of permutations; group-theoretic concepts (subgroups, normal subgroups, solvable groups) are essential.
- **Field** -- Galois theory operates on field extensions; understanding fields is required.

## Key Properties

1. **Group-field correspondence** -- associates a group (the Galois group) to each polynomial equation via its splitting field ([history-of-group-theory](/source/history-of-group-theory/)).
2. **Solvability criterion** -- a polynomial is solvable by radicals if and only if its Galois group is a solvable group ([history-of-group-theory](/source/history-of-group-theory/)).
3. **Quintic unsolvability** -- proves that there is no general formula for degree 5+ polynomials (Abel-Ruffini theorem, refined by Galois) ([abstract-algebra](/source/abstract-algebra/)).
4. **Historical catalyst** -- the quest to solve polynomial equations was the original motivation for group theory ([group-mathematics](/source/group-mathematics/)).

## Context & Application

Evariste Galois developed this theory in the early 1830s, published posthumously in 1846. It resolved a centuries-old question about the solvability of polynomial equations. Galois theory is fundamental to algebraic number theory, algebraic geometry, and coding theory. It also provided the first systematic use of the group concept ([history-of-group-theory](/source/history-of-group-theory/)).

## Examples

**Example 1** ([history-of-group-theory](/source/history-of-group-theory/)): The Galois group of a general quintic polynomial is the symmetric group $S_5$, which is not solvable, proving that no formula exists for solving quintics by radicals.

**Example 2** ([abstract-algebra](/source/abstract-algebra/)): Lagrange's 1770 study of solutions of the quintic led to the Galois group concept, as he studied permutations of roots.

## Relationships

- **Related to permutation-group**: Galois groups are specific permutation groups of polynomial roots.
- **Related to abstract-algebra-field**: Galois theory is a cornerstone of abstract algebra.

## Source Reference

Derived from:
- [abstract-algebra](/source/abstract-algebra/) -- provides the early group theory context
- [history-of-group-theory](/source/history-of-group-theory/) -- provides Galois's formulation and the solvability criterion
- [group-mathematics](/source/group-mathematics/) -- provides the historical motivation

## Verification Notes

- Definition: synthesized from the three sources; no single source provides a complete standalone definition
- Confidence: MEDIUM -- definition synthesized from historical discussions across three sources
- Cross-references: verified
- Uncertainties: no dedicated Galois theory source page in this corpus
