---
# === CORE IDENTIFICATION ===
concept: "Homotopy Equivalence"
slug: "homotopy-equivalence"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "topology"
tier: "intermediate"
subject_headings:
  - "algebraic-topology"
keywords:
  - "homotopy type"
  - "contractible"
  - "deformation retract"

# === PROVENANCE ===
derived_from:
  - slug: "homotopy"
    title: "Homotopy"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "homotopy type"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "homotopy-concept"
extends:
  - "homotopy-concept"
related:
  - "fundamental-group"
  - "homotopy-group"
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

Two topological spaces are homotopy equivalent if they can be continuously deformed into each other -- formally, if there exist maps between them whose compositions are homotopic to the respective identity maps. Homotopy equivalent spaces share all homotopy-invariant properties.

## Core Definition

A homotopy equivalence between topological spaces $X$ and $Y$ is a pair of continuous maps $f: X \to Y$ and $g: Y \to X$ such that $g \circ f$ is homotopic to $\mathrm{id}_X$ and $f \circ g$ is homotopic to $\mathrm{id}_Y$. If such a pair exists, $X$ and $Y$ are said to be homotopy equivalent or of the same homotopy type, denoted $X \simeq Y$. Spaces homotopy-equivalent to a point are called contractible ([homotopy](/source/homotopy/)).

## Prerequisites

- **Homotopy** -- the definition of homotopy equivalence depends on the concept of homotopy between continuous maps.

## Key Properties

1. **Equivalence relation** -- homotopy equivalence is reflexive, symmetric, and transitive ([homotopy](/source/homotopy/)).
2. **Weaker than homeomorphism** -- every homeomorphism is a homotopy equivalence, but not conversely ([homotopy](/source/homotopy/)).
3. **Invariance** -- homotopy equivalent spaces share path-connectedness, simple-connectedness, homology groups, cohomology groups, and homotopy groups ([homotopy](/source/homotopy/)).
4. **Contractibility** -- a space is contractible iff it is homotopy-equivalent to a point ([homotopy](/source/homotopy/)).

## Context & Application

Homotopy equivalence captures the essential topological "shape" of a space, ignoring details that can be continuously deformed away. It is the central equivalence relation in algebraic topology and provides the notion of "sameness" that homotopy type theory formalizes ([homotopy](/source/homotopy/)).

## Examples

**Example 1** ([homotopy](/source/homotopy/)): $\mathbb{R}^n$ is homotopy-equivalent to a point (contractible), since every point can be continuously shrunk to the origin via $H(t, x) = (1-t)x$.

**Example 2** ([homotopy](/source/homotopy/)): The Mobius strip and an untwisted strip are homotopy-equivalent (both deformable to a circle), but are not homeomorphic.

## Relationships

- **Extends homotopy-concept**: homotopy equivalence is built from the notion of homotopy between maps.
- **Related to fundamental-group**: homotopy-equivalent path-connected spaces have isomorphic fundamental groups.
- **Related to homotopy-group**: all homotopy groups are preserved under homotopy equivalence.

## Source Reference

Derived from:
- [homotopy](/source/homotopy/) -- provides the definition, comparison with homeomorphism, and examples.

## Verification Notes

- Definition: from the "Homotopy equivalence" section of [homotopy](/source/homotopy/)
- Confidence: HIGH -- formally defined with examples
- Cross-references: verified
- Uncertainties: none
