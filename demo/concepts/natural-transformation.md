---
# === CORE IDENTIFICATION ===
concept: "Natural Transformation"
slug: "natural-transformation"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "foundations"
tier: "advanced"
subject_headings:
  - "category-theory"
keywords:
  - "morphism of functors"
  - "commutative diagram"
  - "natural isomorphism"

# === PROVENANCE ===
derived_from:
  - slug: "category-theory"
    title: "Category theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "functor"
extends: []
related:
  - "category-theory-mathematics"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does category theory relate to other areas of mathematics?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

A natural transformation is a way of transforming one functor into another while respecting the structure of the categories involved. It associates to every object a morphism between the functor values, such that a naturality condition (a commutative diagram) is satisfied.

## Core Definition

If $F$ and $G$ are covariant functors between categories $C$ and $D$, a natural transformation $\eta$ from $F$ to $G$ associates to every object $X$ in $C$ a morphism $\eta_X: F(X) \to G(X)$ in $D$ such that for every morphism $f: X \to Y$ in $C$, the naturality condition holds: $\eta_Y \circ F(f) = G(f) \circ \eta_X$ ([category-theory](/source/category-theory/)). Two functors are naturally isomorphic if there exists a natural transformation where every $\eta_X$ is an isomorphism ([category-theory](/source/category-theory/)).

## Prerequisites

- **Functor** -- natural transformations are defined between functors; the concept of functor is essential.

## Key Properties

1. **Naturality condition** -- the defining commutative diagram ensures coherence across all objects ([category-theory](/source/category-theory/)).
2. **Third fundamental concept** -- category theory's three pillars are categories, functors, and natural transformations ([category-theory](/source/category-theory/)).
3. **Historical motivation** -- Eilenberg and Mac Lane originally introduced categories and functors specifically to formalize natural transformations ([category-theory](/source/category-theory/)).

## Context & Application

Natural transformations formalize the intuitive notion that two mathematical constructions are "naturally" related. For example, the determinant is a natural transformation from the matrix functor to the multiplicative group functor. The concept motivated the invention of all of category theory ([category-theory](/source/category-theory/)).

## Examples

**Example 1** ([category-theory](/source/category-theory/)): The commutative diagram defining a natural transformation: for $f: X \to Y$, the square with $F(f)$, $G(f)$, $\eta_X$, and $\eta_Y$ must commute.

## Relationships

- **Related to category-theory-mathematics**: natural transformations are the third fundamental concept.

## Source Reference

Derived from:
- [category-theory](/source/category-theory/) -- provides the formal definition and commutative diagram.

## Verification Notes

- Definition: from the Natural transformations section of [category-theory](/source/category-theory/)
- Confidence: HIGH -- formally defined
- Cross-references: verified
- Uncertainties: none
