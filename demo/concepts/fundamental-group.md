---
# === CORE IDENTIFICATION ===
concept: "Fundamental Group"
slug: "fundamental-group"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "topology"
tier: "intermediate"
subject_headings:
  - "algebraic-topology"
keywords:
  - "pi one"
  - "loops"
  - "path homotopy"
  - "simply connected"

# === PROVENANCE ===
derived_from:
  - slug: "homotopy"
    title: "Homotopy"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "first homotopy group"
  - "$\\pi_1$"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "homotopy-concept"
  - "group-mathematics"
extends: []
related:
  - "homotopy-group"
  - "homotopy-equivalence"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What background is needed to understand homotopy type theory?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

The fundamental group $\pi_1(X, x_0)$ of a topological space at a basepoint is the group of equivalence classes of loops based at that point, where two loops are equivalent if one can be continuously deformed into the other. It captures the "essential loops" in a space.

## Core Definition

The fundamental group is the case $n = 1$ of the homotopy groups. When one fixes $X = [0,1]^1 = [0,1]$ and takes its boundary $\{0, 1\}$ as a subspace (with both endpoints mapping to the basepoint $y_0$), the equivalence classes of maps under homotopy relative to the boundary form a group, denoted $\pi_1(Y, y_0)$ ([homotopy](/source/homotopy/)). Two loops are equivalent if one can be continuously deformed into the other while keeping the basepoint fixed (relative homotopy). The group operation is concatenation of loops ([homotopy](/source/homotopy/)).

## Prerequisites

- **Homotopy** -- the fundamental group is defined using homotopy classes of loops.
- **Group (mathematics)** -- the fundamental group is, as the name implies, a group.

## Key Properties

1. **Homotopy invariant** -- homotopy-equivalent path-connected spaces have isomorphic fundamental groups ([homotopy](/source/homotopy/)).
2. **Trivial iff simply connected** -- $\pi_1(X, x_0) = 0$ iff $X$ is simply connected ([homotopy](/source/homotopy/)).
3. **First in a tower** -- the fundamental group is $\pi_1$; higher homotopy groups $\pi_n$ generalize to higher-dimensional "loops" ([homotopy](/source/homotopy/)).

## Context & Application

The fundamental group was introduced by Poincare and is the most basic algebraic invariant of a topological space. It determines covering spaces, detects "holes" in spaces, and is the starting point for algebraic topology. The Poincare conjecture (proved 2003) asserts that a simply connected closed 3-manifold must be a 3-sphere ([abstract-algebra](/source/abstract-algebra/)).

## Examples

**Example 1** ([homotopy](/source/homotopy/)): The fundamental group of the circle $S^1$ is $\mathbb{Z}$ (the integers): each loop is classified by how many times it winds around the circle.

## Relationships

- **Related to homotopy-group**: the fundamental group is the first homotopy group $\pi_1$.
- **Related to homotopy-equivalence**: homotopy-equivalent spaces have isomorphic fundamental groups.

## Source Reference

Derived from:
- [homotopy](/source/homotopy/) -- provides the definition via homotopy groups and relative homotopy.

## Verification Notes

- Definition: from the "Groups" section of [homotopy](/source/homotopy/)
- Confidence: HIGH -- clearly defined as $n=1$ case of homotopy groups
- Cross-references: verified
- Uncertainties: none
