---
# === CORE IDENTIFICATION ===
concept: "Functor"
slug: "functor"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "foundations"
tier: "intermediate"
subject_headings:
  - "category-theory"
keywords:
  - "structure-preserving map"
  - "covariant"
  - "contravariant"
  - "categories"

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
  - "category-mathematics"
extends: []
related:
  - "natural-transformation"
  - "category-theory-mathematics"
  - "morphism"
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

A functor is a structure-preserving map between categories. It maps objects to objects and morphisms to morphisms while preserving composition and identity, playing the role of a "morphism between categories."

## Core Definition

A covariant functor $F$ from a category $C$ to a category $D$ consists of: for each object $x$ in $C$, an object $F(x)$ in $D$; and for each morphism $f: x \to y$ in $C$, a morphism $F(f): F(x) \to F(y)$ in $D$, such that $F(1_x) = 1_{F(x)}$ (identity preservation) and $F(g \circ f) = F(g) \circ F(f)$ (composition preservation). A contravariant functor reverses the direction of morphisms: $f: x \to y$ maps to $F(f): F(y) \to F(x)$ ([category-theory](/source/category-theory/)).

## Prerequisites

- **Category** -- a functor maps between categories; understanding the definition of a category (objects, morphisms, composition) is essential.

## Key Properties

1. **Preserves structure** -- maps identities to identities and compositions to compositions ([category-theory](/source/category-theory/)).
2. **Covariant vs. contravariant** -- covariant functors preserve arrow direction; contravariant functors reverse it ([category-theory](/source/category-theory/)).
3. **Morphism of categories** -- functors are to categories what homomorphisms are to groups ([category-theory](/source/category-theory/)).

## Context & Application

Eilenberg and Mac Lane originally introduced categories precisely to formalize the notion of functor (and natural transformation). Functors appear throughout mathematics: the fundamental group is a functor from topological spaces to groups; homology is a functor from spaces to abelian groups. The forgetful functor from groups to sets "forgets" the group operation ([category-theory](/source/category-theory/)).

## Examples

**Example 1** ([category-theory](/source/category-theory/)): A forgetful functor from the category of groups to the category of sets maps each group to its underlying set and each group homomorphism to the corresponding function.

## Relationships

- **Related to natural-transformation**: natural transformations are morphisms between functors.
- **Related to category-theory-mathematics**: functors are the second fundamental concept of category theory.
- **Related to morphism**: functors map morphisms in one category to morphisms in another.

## Source Reference

Derived from:
- [category-theory](/source/category-theory/) -- provides the formal definition, covariant/contravariant distinction, and examples.

## Verification Notes

- Definition: from the Functors section of [category-theory](/source/category-theory/)
- Confidence: HIGH -- formally defined with axioms
- Cross-references: verified
- Uncertainties: none
