---
# === CORE IDENTIFICATION ===
concept: "Topological Quantum Field Theory"
slug: "topological-qft-concept"
page_type: "concept"

# === CLASSIFICATION ===
category: "theoretical-physics"
subcategory: "unification"
tier: "advanced"
subject_headings:
  - "mathematical-physics"
  - "algebraic-topology"
keywords:
  - "TQFT"
  - "topological invariants"
  - "Witten"
  - "Chern-Simons"
  - "cobordism"

# === PROVENANCE ===
derived_from:
  - slug: "topological-qft"
    title: "Topological quantum field theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "TQFT"
  - "topological field theory"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "quantum-mechanics-theory"
extends: []
related:
  - "edward-witten-physicist"
  - "category-theory-mathematics"
  - "homotopy-concept"
  - "string-theory-physics"
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

A topological quantum field theory (TQFT) is a quantum field theory whose correlation functions are topological invariants -- independent of the spacetime metric. TQFTs connect physics to knot theory, four-manifold invariants, and algebraic topology.

## Core Definition

A topological quantum field theory is a quantum field theory that computes topological invariants. In a TQFT, correlation functions are metric-independent and therefore unchanged under any deformation of spacetime. TQFTs fall into two classes: Schwarz-type (where the action functional is explicitly metric-independent, such as Chern-Simons theory) and Witten-type (where metric independence emerges after a topological twist) ([topological-qft](/source/topological-qft/)). Mathematically, a TQFT can be formalized as a functor from a category of cobordisms to the category of vector spaces (Atiyah axioms) ([topological-qft](/source/topological-qft/)).

## Prerequisites

- **Quantum mechanics** -- TQFTs are quantum field theories and require quantum mechanical concepts.

## Key Properties

1. **Metric independence** -- observables do not depend on the spacetime metric ([topological-qft](/source/topological-qft/)).
2. **Two types** -- Schwarz-type (explicitly metric-free action) and Witten-type (metric independence via topological twist) ([topological-qft](/source/topological-qft/)).
3. **Background independence** -- TQFTs exemplify background-independent quantum field theories, relevant to quantum gravity ([topological-qft](/source/topological-qft/)).
4. **Functorial axiomatization** -- Atiyah's axioms define a TQFT as a functor from cobordisms to vector spaces ([topological-qft](/source/topological-qft/)).
5. **Fields Medal connections** -- Donaldson, Jones, Witten, and Kontsevich all won Fields Medals for TQFT-related work ([topological-qft](/source/topological-qft/)).

## Context & Application

Witten's 1988 paper on topological Yang-Mills theory introduced Witten-type TQFTs, connecting gauge theory to Donaldson invariants of four-manifolds. Chern-Simons theory, a Schwarz-type TQFT, computes knot invariants (Jones polynomial). TQFTs also appear in condensed matter physics as low-energy effective theories of topologically ordered states (fractional quantum Hall effect) ([topological-qft](/source/topological-qft/)).

## Examples

**Example 1** ([topological-qft](/source/topological-qft/)): Chern-Simons theory with gauge group SU(2) computes the Jones polynomial of knots, connecting quantum field theory to knot invariants.

**Example 2** ([topological-qft](/source/topological-qft/)): Witten's topological Yang-Mills theory in four dimensions computes Donaldson invariants of four-manifolds.

## Relationships

- **Related to edward-witten-physicist**: Witten introduced Witten-type TQFTs and received the Fields Medal for this work.
- **Related to category-theory-mathematics**: the Atiyah axioms define a TQFT as a functor from cobordisms to vector spaces.
- **Related to homotopy-concept**: topological invariance is closely tied to homotopy invariance.
- **Related to string-theory-physics**: topological string theory is a TQFT variant within string theory.

## Source Reference

Derived from:
- [topological-qft](/source/topological-qft/) -- provides the definition, classification, mathematical formulation, and examples.

## Verification Notes

- Definition: from the opening and Overview sections of [topological-qft](/source/topological-qft/)
- Confidence: HIGH -- the source page is devoted to TQFTs
- Cross-references: verified
- Uncertainties: none
