---
# === CORE IDENTIFICATION ===
concept: "Category Theory"
slug: "category-theory-mathematics"
page_type: "concept"
disambiguator: "mathematics"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "foundations"
tier: "advanced"
subject_headings:
  - "abstract-algebra"
  - "foundations-of-mathematics"
keywords:
  - "categories"
  - "functors"
  - "natural transformations"
  - "morphisms"
  - "Eilenberg"
  - "Mac Lane"

# === PROVENANCE ===
derived_from:
  - slug: "category-theory"
    title: "Category theory"
  - slug: "algebraic-structure"
    title: "Algebraic structure"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "abstract nonsense"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "abstract-algebra-field"
  - "algebraic-structure-concept"
extends: []
related:
  - "functor"
  - "natural-transformation"
  - "morphism"
  - "type-theory-foundations"
  - "homotopy-type-theory-concept"
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

Category theory is a general theory of mathematical structures and their structure-preserving maps (morphisms). It provides a unifying language for expressing constructions, theorems, and relationships that recur across diverse areas of mathematics.

## Core Definition

Category theory is a general theory of mathematical structures and their relations. It was introduced by Samuel Eilenberg and Saunders Mac Lane in the mid-20th century in their foundational work on algebraic topology. A category consists of objects and morphisms (arrows between objects), with a composition operation satisfying associativity and identity axioms. The second fundamental concept is the functor, which maps between categories while preserving structure. The third is the natural transformation, a morphism between functors ([category-theory](/source/category-theory/)). Category theory may be viewed as an extension of universal algebra, as universal algebra studies algebraic structures and category theory applies to any kind of mathematical structure ([category-theory](/source/category-theory/)).

## Prerequisites

- **Abstract algebra** -- category theory generalizes concepts from abstract algebra (groups, rings, homomorphisms).
- **Algebraic structure** -- categories formalize the notion of algebraic structures and their homomorphisms.

## Key Properties

1. **Three fundamental concepts** -- categories, functors, and natural transformations ([category-theory](/source/category-theory/)).
2. **Universal language** -- many constructions (products, quotients, limits, colimits) are expressed uniformly across different areas ([category-theory](/source/category-theory/)).
3. **Duality** -- every theorem has a dual obtained by "reversing all arrows" ([category-theory](/source/category-theory/)).
4. **Adjoint functors** -- a central concept: pairs of functors that are "inverse" in a weak sense, arising from universal properties ([category-theory](/source/category-theory/)).
5. **Foundation potential** -- topoi (certain categories) can serve as an alternative foundation for mathematics ([category-theory](/source/category-theory/)).
6. **Higher categories** -- the framework extends to 2-categories, n-categories, and infinity-categories ([category-theory](/source/category-theory/)).

## Context & Application

Eilenberg and Mac Lane introduced categories in 1945 to formalize relationships in algebraic topology. The original motivation was to understand natural transformations, which required functors, which required categories. Category theory has since become essential to algebraic geometry (scheme theory), logic (categorical logic, topos theory), computer science (functional programming, type theory), and theoretical physics (Feynman diagrams as monoidal categories) ([category-theory](/source/category-theory/)).

## Examples

**Example 1** ([category-theory](/source/category-theory/)): **Set**, the category of sets, has sets as objects and functions as morphisms. It is the prototypical example satisfying the category axioms.

**Example 2** ([category-theory](/source/category-theory/)): The category of groups has all groups as objects and group homomorphisms as morphisms. A forgetful functor from groups to sets "forgets" the group operation.

## Relationships

- **Related to functor**: functors are structure-preserving maps between categories.
- **Related to natural-transformation**: natural transformations relate functors.
- **Related to morphism**: morphisms are the arrows that define a category's structure.
- **Related to type-theory-foundations**: category theory and type theory have deep connections (Curry-Howard-Lambek).
- **Related to homotopy-type-theory-concept**: HoTT uses categories as models for type-theoretic structures.

## Source Reference

Derived from:
- [category-theory](/source/category-theory/) -- provides the definition, fundamental concepts, examples, and historical context
- [algebraic-structure](/source/algebraic-structure/) -- provides the categorical perspective on algebraic structures

## Verification Notes

- Definition: directly from the opening of [category-theory](/source/category-theory/)
- Key Properties: all drawn from explicit source content
- Confidence: HIGH -- the source page is entirely devoted to category theory
- Cross-references: verified
- Uncertainties: none
