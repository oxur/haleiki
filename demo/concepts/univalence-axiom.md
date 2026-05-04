---
# === CORE IDENTIFICATION ===
concept: "Univalence Axiom"
slug: "univalence-axiom"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "foundations"
tier: "advanced"
subject_headings:
  - "foundations-of-mathematics"
  - "type-theory"
keywords:
  - "Voevodsky"
  - "equivalence of types"
  - "identity type"
  - "HoTT"

# === PROVENANCE ===
derived_from:
  - slug: "homotopy-type-theory"
    title: "Homotopy type theory"
  - slug: "type-theory"
    title: "Type theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "univalence"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "homotopy-type-theory-concept"
extends: []
related:
  - "univalent-foundations"
  - "type-theory-foundations"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is homotopy type theory?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

The univalence axiom, introduced by Vladimir Voevodsky, states that equivalence of types is equivalent to equality of types. It is the defining axiom of homotopy type theory, making the identity type of type universes match the homotopy-theoretic notion of equivalence.

## Core Definition

The univalence axiom holds that equivalence of types is equality of types. Formally, for types $A$ and $B$ in a universe, the canonical map from the identity type $(A =_U B)$ to the type of equivalences $(A \simeq B)$ is itself an equivalence ([type-theory](/source/type-theory/)). Voevodsky introduced the concept of univalent fibration in 2006 and formulated the axiom syntactically in 2009, after recognizing that identity types could be used in combination with univalent universes ([homotopy-type-theory](/source/homotopy-type-theory/)). Voevodsky also proved that the univalence axiom implies function extensionality ([homotopy-type-theory](/source/homotopy-type-theory/)).

## Prerequisites

- **Homotopy type theory** -- the univalence axiom is the central axiom of HoTT; understanding identity types and the type-theoretic universe is essential.

## Key Properties

1. **Equivalence = equality** -- types that are equivalent (have a structure-preserving bijection) are equal in the type-theoretic sense ([type-theory](/source/type-theory/)).
2. **Implies function extensionality** -- functions that agree on all inputs are provably equal ([homotopy-type-theory](/source/homotopy-type-theory/)).
3. **Normalizes in cubical type theory** -- the axiom can be given computational content in cubical type theory (2016) ([type-theory](/source/type-theory/)).
4. **Generalizes Hofmann-Streicher** -- the axiom generalizes the "universe extensionality" principle for 1-types to all homotopy levels ([homotopy-type-theory](/source/homotopy-type-theory/)).

## Context & Application

Voevodsky's univalence axiom is the most distinctive feature of homotopy type theory. It was formulated between 2006-2009 and formalized in Coq/Rocq as the "Foundations" (later "UniMath") library. The axiom enables "synthetic homotopy theory" -- developing homotopy-theoretic results purely within type theory. Cubical type theory (2016) achieves univalence computationally, without needing to add it as an axiom ([homotopy-type-theory](/source/homotopy-type-theory/)).

## Examples

**Example 1** ([homotopy-type-theory](/source/homotopy-type-theory/)): The Hofmann-Streicher groupoid model satisfies a restricted form of univalence ("universe extensionality") for 1-types, foreshadowing the full axiom.

## Relationships

- **Related to univalent-foundations**: the univalence axiom is the core of Voevodsky's foundational program.
- **Related to type-theory-foundations**: the axiom is added to Martin-Lof type theory.

## Source Reference

Derived from:
- [homotopy-type-theory](/source/homotopy-type-theory/) -- provides the history, formulation, and implications
- [type-theory](/source/type-theory/) -- mentions univalence among important axioms

## Verification Notes

- Definition: from the univalence axiom subsections of both sources
- Confidence: HIGH -- dedicated subsections in both sources
- Cross-references: verified
- Uncertainties: none
