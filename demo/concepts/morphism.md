---
# === CORE IDENTIFICATION ===
concept: "Morphism"
slug: "morphism"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "foundations"
tier: "intermediate"
subject_headings:
  - "category-theory"
  - "abstract-algebra"
keywords:
  - "arrow"
  - "structure-preserving map"
  - "homomorphism"
  - "isomorphism"

# === PROVENANCE ===
derived_from:
  - slug: "category-theory"
    title: "Category theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "arrow"
  - "map"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "category-mathematics"
  - "homomorphism"
  - "functor"
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

A morphism is an arrow between two objects in a category, generalizing the notion of a structure-preserving map (such as functions between sets or homomorphisms between groups). Morphisms compose associatively and each object has an identity morphism.

## Core Definition

In category theory, a morphism (also called a map or arrow) relates a source object to a target object. Morphisms compose associatively: given $f: a \to b$ and $g: b \to c$, their composition $g \circ f: a \to c$ is defined. Every object $x$ has an identity morphism $1_x$ such that $1_b \circ f = f = f \circ 1_a$ ([category-theory](/source/category-theory/)). Morphisms can be monomorphisms (injective-like), epimorphisms (surjective-like), isomorphisms (invertible), endomorphisms ($a = b$), or automorphisms (both endomorphism and isomorphism) ([category-theory](/source/category-theory/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki.

## Key Properties

1. **Source and target** -- every morphism goes from one object (source) to another (target) ([category-theory](/source/category-theory/)).
2. **Composition** -- morphisms compose associatively ([category-theory](/source/category-theory/)).
3. **Identity** -- every object has an identity morphism ([category-theory](/source/category-theory/)).
4. **Classification** -- monomorphisms, epimorphisms, isomorphisms, endomorphisms, automorphisms ([category-theory](/source/category-theory/)).

## Context & Application

Morphisms generalize the notion of "structure-preserving map" across all of mathematics. In the category of sets, morphisms are functions. In the category of groups, morphisms are group homomorphisms. In the category of topological spaces, morphisms are continuous maps. The abstraction allows theorems proved about morphisms in general to apply in all these contexts simultaneously ([category-theory](/source/category-theory/)).

## Examples

**Example 1** ([category-theory](/source/category-theory/)): In **Set** (the category of sets), morphisms are ordinary functions between sets.

**Example 2** ([category-theory](/source/category-theory/)): A retraction is a morphism $f: a \to b$ that has a right inverse $g: b \to a$ with $f \circ g = 1_b$; a section has a left inverse.

## Relationships

- **Related to category-mathematics**: morphisms are one of the two defining components of a category.
- **Related to homomorphism**: homomorphisms are morphisms in categories of algebraic structures.
- **Related to functor**: functors map morphisms in one category to morphisms in another.

## Source Reference

Derived from:
- [category-theory](/source/category-theory/) -- provides the definition, composition rules, and classification.

## Verification Notes

- Definition: from the Categories and Morphisms sections of [category-theory](/source/category-theory/)
- Confidence: HIGH -- formally defined with clear properties
- Cross-references: verified
- Uncertainties: none
