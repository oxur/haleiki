---
# === CORE IDENTIFICATION ===
concept: "Lie Group"
slug: "lie-group"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "algebra"
tier: "intermediate"
subject_headings:
  - "abstract-algebra"
  - "differential-geometry"
  - "theoretical-physics"
keywords:
  - "continuous symmetry"
  - "differentiable manifold"
  - "Sophus Lie"
  - "transformation group"

# === PROVENANCE ===
derived_from:
  - slug: "group-theory"
    title: "Group theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "Lie groups"
  - "continuous group"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "group-theory-mathematics"
extends: []
related:
  - "symmetry-group"
  - "representation-theory"
  - "finite-group"
contrasts_with:
  - "finite-group"

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do finite groups differ from Lie groups?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

A Lie group is a group that is also a differentiable manifold, where the group operations (multiplication and inversion) are smooth maps. Lie groups are the mathematical framework for continuous symmetry.

## Core Definition

A Lie group is a group that is also a differentiable manifold, with the property that the group operations are compatible with the smooth structure. Lie groups are named after Sophus Lie, who laid the foundations of the theory of continuous transformation groups. They represent the best-developed theory of continuous symmetry of mathematical objects and structures, making them indispensable tools for many parts of contemporary mathematics and modern theoretical physics ([group-theory](/source/group-theory/)).

## Prerequisites

- **Group theory** -- Lie groups are groups with additional structure; understanding the basic group axioms is essential.

## Key Properties

1. **Dual structure** -- a Lie group carries both an algebraic structure (group operations) and a geometric structure (differentiable manifold).
2. **Smooth operations** -- the group multiplication and inversion maps are smooth (infinitely differentiable).
3. **Continuous symmetry** -- Lie groups provide the natural framework for analysing continuous symmetries of differential equations, analogous to how permutation groups analyse discrete symmetries of algebraic equations ([group-theory](/source/group-theory/)).
4. **Representation theory** -- the study of Lie group representations is a major area, connecting algebra to physics and harmonic analysis.
5. **Classification** -- compact connected Lie groups have been completely classified.

## Context & Application

Lie introduced these groups in 1884 for studying analytic problems. The term "groupes de Lie" first appeared in French in 1893 in the thesis of Lie's student Arthur Tresse. Lie groups are essential in modern theoretical physics: the Standard Model of particle physics is formulated using Lie groups (the Lorentz group, Poincare group, and gauge groups). They also provide the framework for differential Galois theory, extending Galois theory from discrete to continuous symmetries ([group-theory](/source/group-theory/)).

## Examples

**Example 1** ([group-theory](/source/group-theory/)): The group U(1) of complex numbers of absolute value 1 is a Lie group. Fourier polynomials can be interpreted as the characters of U(1) acting on the $L^2$-space of periodic functions.

**Example 2** ([group-theory](/source/group-theory/)): The general linear groups over finite fields are finite analogs of classical Lie groups, studied by Chevalley and Steinberg.

## Relationships

- **Prerequisite: group-theory-mathematics** -- Lie groups are a specific class of groups studied within group theory.
- **Related to symmetry-group** -- Lie groups model continuous symmetry groups.
- **Related to representation-theory** -- unitary representation theory of Lie groups is a major subdomain.
- **Contrasts with finite-group** -- finite groups model discrete symmetry with finitely many elements; Lie groups model continuous symmetry and have uncountably many elements. However, the two are connected: finite Weyl groups are associated with Lie groups ([group-theory](/source/group-theory/)).

## Source Reference

Derived from:
- [group-theory](/source/group-theory/) -- provides the definition in the "Lie theory" subsection, historical context, and connections to physics and representation theory.

## Verification Notes

- Definition: directly adapted from the "Lie theory" subsection of [group-theory](/source/group-theory/)
- Key Properties: items 1-3 explicit in source; items 4-5 synthesized from the "Groups with additional structure" and "Representation of groups" sections
- Confidence: HIGH -- dedicated subsection with clear definition and extensive discussion
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
