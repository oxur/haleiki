---
# === CORE IDENTIFICATION ===
concept: "Homotopy Type Theory"
slug: "homotopy-type-theory-concept"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "foundations"
tier: "advanced"
subject_headings:
  - "foundations-of-mathematics"
  - "algebraic-topology"
  - "computer-science"
keywords:
  - "HoTT"
  - "univalence"
  - "higher inductive types"
  - "Voevodsky"
  - "types as spaces"

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
  - "HoTT"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "type-theory-foundations"
  - "homotopy-concept"
extends:
  - "intuitionistic-type-theory"
related:
  - "univalence-axiom"
  - "univalent-foundations"
  - "category-theory-mathematics"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is homotopy type theory?"
  - "What background is needed to understand homotopy type theory?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Homotopy type theory (HoTT) is a research program that interprets types in intuitionistic type theory as spaces and identity types as paths, connecting type theory with homotopy theory. It includes the univalence axiom and provides a new foundation for mathematics.

## Core Definition

Homotopy type theory includes various lines of development of intuitionistic type theory, based on the interpretation of types as objects to which the intuition of abstract homotopy theory applies. This includes the construction of homotopical and higher-categorical models for type theories, the use of type theory as a logic for abstract homotopy theory and higher category theory, the development of mathematics within a type-theoretic foundation, and the formalization of these in computer proof assistants ([homotopy-type-theory](/source/homotopy-type-theory/)). Type theory is an active area of research, one direction being the development of homotopy type theory ([type-theory](/source/type-theory/)).

## Prerequisites

- **Type theory** -- HoTT builds on intuitionistic type theory, especially dependent types and identity types.
- **Homotopy** -- the interpretation of types as spaces and identity proofs as paths requires homotopy-theoretic intuition.

## Key Properties

1. **Types as spaces** -- types are interpreted as homotopy types (spaces up to homotopy equivalence) ([homotopy-type-theory](/source/homotopy-type-theory/)).
2. **Identity types as paths** -- the identity type $a =_A b$ is interpreted as the space of paths from $a$ to $b$ ([homotopy-type-theory](/source/homotopy-type-theory/)).
3. **Univalence axiom** -- equivalent types are equal ($(\text{A} \simeq \text{B}) \simeq (\text{A} = \text{B})$), proposed by Voevodsky ([homotopy-type-theory](/source/homotopy-type-theory/)).
4. **Higher inductive types** -- allow defining spaces by specifying not just points but also paths and higher paths ([homotopy-type-theory](/source/homotopy-type-theory/)).
5. **Constructive and formalizable** -- HoTT is constructive and has been partially formalized in proof assistants like Coq/Rocq and Agda ([homotopy-type-theory](/source/homotopy-type-theory/)).

## Context & Application

The groupoid model of Hofmann and Streicher (1994) first connected identity types to homotopy. Awodey and Warren (2005) constructed higher-dimensional models using model categories. Voevodsky independently developed the univalence axiom (2006-2009) and began formalizing mathematics in Coq. The 2012-13 IAS Special Year on Univalent Foundations brought researchers together and produced the HoTT Book. The term "homotopy type theory" was coined by Awodey in 2007 ([homotopy-type-theory](/source/homotopy-type-theory/)).

## Examples

**Example 1** ([homotopy-type-theory](/source/homotopy-type-theory/)): The groupoid model interprets types as groupoids and identity proofs as isomorphisms, the simplest "homotopical" model of type theory.

**Example 2** ([type-theory](/source/type-theory/)): The univalence axiom implies function extensionality, a result proved by Voevodsky -- showing that if two functions agree on all inputs, they are equal.

## Relationships

- **Extends intuitionistic-type-theory**: HoTT builds on Martin-Lof's type theory with the univalence axiom.
- **Related to univalence-axiom**: the central new axiom distinguishing HoTT from ordinary type theory.
- **Related to univalent-foundations**: Voevodsky's program for foundations of mathematics.
- **Related to category-theory-mathematics**: category theory provides semantic models for HoTT.

## Source Reference

Derived from:
- [homotopy-type-theory](/source/homotopy-type-theory/) -- provides the definition, history, key ideas, and program
- [type-theory](/source/type-theory/) -- contextualizes HoTT within the broader type theory landscape

## Verification Notes

- Definition: from the opening of [homotopy-type-theory](/source/homotopy-type-theory/)
- Confidence: HIGH -- the source page is entirely devoted to HoTT
- Multi-source: type-theory adds the broader context
- Cross-references: verified
- Uncertainties: none
