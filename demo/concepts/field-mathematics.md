---
# === CORE IDENTIFICATION ===
concept: "Field"
slug: "field-mathematics"
page_type: "concept"
disambiguator: "mathematics"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "algebra"
tier: "intermediate"
subject_headings:
  - "abstract-algebra"
keywords:
  - "division"
  - "commutative ring"
  - "rational numbers"
  - "real numbers"
  - "Steinitz"

# === PROVENANCE ===
derived_from:
  - slug: "abstract-algebra"
    title: "Abstract algebra"
  - slug: "algebraic-structure"
    title: "Algebraic structure"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "field (algebra)"
  - "number field"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "ring-mathematics"
extends:
  - "ring-mathematics"
related:
  - "abstract-algebra-field"
  - "galois-theory"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does group theory relate to abstract algebra?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

A field is a commutative ring in which every nonzero element has a multiplicative inverse, making division (except by zero) always possible. The rational numbers, real numbers, and complex numbers are the most familiar examples.

## Core Definition

A field is a commutative division ring -- a set with two operations (addition and multiplication) where addition forms an abelian group, the nonzero elements form an abelian group under multiplication, and multiplication distributes over addition ([algebraic-structure](/source/algebraic-structure/)). The first abstract definition was given by Heinrich Martin Weber in 1893; Steinitz synthesized the theory in 1910, classifying fields by their characteristic ([abstract-algebra](/source/abstract-algebra/)).

## Prerequisites

- **Ring** -- a field is a ring with additional multiplicative structure (all nonzero elements are invertible).

## Key Properties

1. **Division always possible** -- every nonzero element has a multiplicative inverse ([algebraic-structure](/source/algebraic-structure/)).
2. **Characteristic** -- every field has a characteristic (either 0 or a prime $p$) that governs its arithmetic ([abstract-algebra](/source/abstract-algebra/)).
3. **Not a variety** -- fields cannot be axiomatized purely by identities (nonzero requirement is non-equational), so they don't form a variety in universal algebra ([algebraic-structure](/source/algebraic-structure/)).
4. **Finite fields** -- Galois showed that finite fields have $p^n$ elements for prime $p$ ([abstract-algebra](/source/abstract-algebra/)).

## Context & Application

Fields arose from Gauss's work on integers mod $p$, Galois's extension to finite fields, and Dedekind's introduction of the German term "Korper" (1871). Fields are fundamental to Galois theory (field extensions determine polynomial solvability), linear algebra (vector spaces are defined over fields), and algebraic geometry (function fields of varieties) ([abstract-algebra](/source/abstract-algebra/)).

## Examples

**Example 1** ([abstract-algebra](/source/abstract-algebra/)): The rational numbers $\mathbb{Q}$, real numbers $\mathbb{R}$, and complex numbers $\mathbb{C}$ are fields of characteristic 0.

**Example 2** ([abstract-algebra](/source/abstract-algebra/)): The integers modulo a prime $p$, denoted $\mathbb{F}_p = \mathbb{Z}/p\mathbb{Z}$, form a finite field of characteristic $p$.

## Relationships

- **Extends ring-mathematics**: a field is a commutative ring where every nonzero element is invertible.
- **Related to abstract-algebra-field**: field theory is a major branch of abstract algebra.
- **Related to galois-theory**: Galois theory studies field extensions and their automorphism groups.

## Source Reference

Derived from:
- [abstract-algebra](/source/abstract-algebra/) -- provides the historical development and classification
- [algebraic-structure](/source/algebraic-structure/) -- provides the formal definition within the hierarchy

## Verification Notes

- Definition: from both [abstract-algebra](/source/abstract-algebra/) (Early field theory) and [algebraic-structure](/source/algebraic-structure/) (Ring-like structures)
- Confidence: HIGH -- clearly defined in both sources
- Cross-references: verified
- Uncertainties: none
