---
# === CORE IDENTIFICATION ===
concept: "Homotopy"
slug: "homotopy-concept"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "topology"
tier: "intermediate"
subject_headings:
  - "algebraic-topology"
  - "topology"
keywords:
  - "continuous deformation"
  - "homotopy equivalence"
  - "fundamental group"
  - "path"

# === PROVENANCE ===
derived_from:
  - slug: "homotopy"
    title: "Homotopy"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "homotopic"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "homotopy-equivalence"
  - "fundamental-group"
  - "homotopy-group"
  - "homotopy-type-theory-concept"
  - "category-theory-mathematics"
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

A homotopy is a continuous deformation of one continuous function into another. Two functions are homotopic if one can be "smoothly morphed" into the other. This notion is central to algebraic topology, where it defines equivalence relations on spaces.

## Core Definition

In topology, two continuous functions from one topological space to another are called homotopic if one can be continuously deformed into the other. Formally, a homotopy between continuous functions $f$ and $g$ from $X$ to $Y$ is a continuous function $H: X \times [0,1] \to Y$ such that $H(x,0) = f(x)$ and $H(x,1) = g(x)$ for all $x \in X$ ([homotopy](/source/homotopy/)). Being homotopic is an equivalence relation on the set of all continuous functions from $X$ to $Y$, and this relation is compatible with function composition ([homotopy](/source/homotopy/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki (topology prerequisites are external to this corpus).

## Key Properties

1. **Equivalence relation** -- homotopy defines an equivalence relation on continuous functions ([homotopy](/source/homotopy/)).
2. **Compatible with composition** -- if $f_1 \sim g_1$ and $f_2 \sim g_2$, then $f_2 \circ f_1 \sim g_2 \circ g_1$ ([homotopy](/source/homotopy/)).
3. **Homotopy invariance** -- many algebraic topological invariants (homology, cohomology, fundamental group) respect homotopy equivalence ([homotopy](/source/homotopy/)).
4. **Parameterized deformation** -- the unit interval $[0,1]$ parameterizes the continuous "interpolation" between two functions ([homotopy](/source/homotopy/)).

## Context & Application

Homotopy is foundational to algebraic topology, where it defines when two spaces are "the same" up to continuous deformation. The homotopy groups $\pi_n(Y, y_0)$ are fundamental invariants. Homotopy theory provides the mathematical underpinning for homotopy type theory, which reinterprets types as homotopy types. Applications extend to differential equations (homotopy continuation methods) and deep learning (flow-based generative models) ([homotopy](/source/homotopy/)).

## Examples

**Example 1** ([homotopy](/source/homotopy/)): The classic "mug and doughnut" illustration: a torus (doughnut surface) can be continuously deformed into a coffee mug surface, demonstrating homotopy equivalence between the two shapes.

**Example 2** ([homotopy](/source/homotopy/)): A linear homotopy between two paths $f, g : [0,1] \to C$ in a convex set $C$ is given by $H(s,t) = (1-t)f(s) + tg(s)$.

## Relationships

- **Related to homotopy-equivalence**: homotopy equivalence is the equivalence relation on topological spaces induced by homotopy.
- **Related to fundamental-group**: the fundamental group is the first homotopy group $\pi_1$.
- **Related to homotopy-group**: homotopy groups generalize the fundamental group to higher dimensions.
- **Related to homotopy-type-theory-concept**: HoTT reinterprets type-theoretic identity types through the lens of homotopy.
- **Related to category-theory-mathematics**: the homotopy category formalizes homotopy equivalence classes as morphisms.

## Source Reference

Derived from:
- [homotopy](/source/homotopy/) -- provides the formal definition, properties, examples, variants, and applications.

## Verification Notes

- Definition: directly from the formal definition section of [homotopy](/source/homotopy/)
- Confidence: HIGH -- the source page is entirely devoted to homotopy
- Cross-references: verified
- Uncertainties: none
