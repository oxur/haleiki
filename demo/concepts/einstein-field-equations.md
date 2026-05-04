---
# === CORE IDENTIFICATION ===
concept: "Einstein Field Equations"
slug: "einstein-field-equations"
page_type: "concept"

# === CLASSIFICATION ===
category: "theoretical-physics"
subcategory: "foundations"
tier: "intermediate"
subject_headings:
  - "gravitation"
  - "mathematical-physics"
keywords:
  - "Einstein equations"
  - "Ricci tensor"
  - "stress-energy tensor"
  - "spacetime curvature"

# === PROVENANCE ===
derived_from:
  - slug: "general-relativity"
    title: "General relativity"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "Einstein's equations"
  - "EFE"
  - "Einstein equations"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "general-relativity-theory"
  - "spacetime"
extends: []
related:
  - "black-hole"
  - "cosmological-model"
  - "gravitational-wave"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does general relativity differ from quantum mechanics?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

The Einstein field equations are a system of ten coupled nonlinear partial differential equations that form the core of general relativity, relating the geometry of spacetime (Einstein tensor) to the distribution of matter and energy (stress-energy tensor).

## Core Definition

The Einstein field equations specify how the geometry of space and time is influenced by whatever matter and radiation are present. They are written as $G_{\mu\nu} \equiv R_{\mu\nu} - \tfrac{1}{2}R\,g_{\mu\nu} = \kappa T_{\mu\nu}$, where $G_{\mu\nu}$ is the Einstein tensor (a specific divergence-free combination of the Ricci tensor $R_{\mu\nu}$ and the metric $g_{\mu\nu}$), $R$ is the curvature scalar, $T_{\mu\nu}$ is the stress-energy tensor, and $\kappa = 8\pi G/c^4$ ([general-relativity](/source/general-relativity/)). When no matter is present ($T_{\mu\nu} = 0$), the equations reduce to the vacuum Einstein equations $R_{\mu\nu} = 0$ ([general-relativity](/source/general-relativity/)).

## Prerequisites

- **General relativity** -- the Einstein field equations are the mathematical core of general relativity.
- **Spacetime** -- understanding spacetime as a pseudo-Riemannian manifold is essential to interpreting the equations.

## Key Properties

1. **Nonlinear** -- the equations are nonlinear partial differential equations, making exact solutions rare and difficult ([general-relativity](/source/general-relativity/)).
2. **Ten equations** -- there are ten independent equations (reflecting the symmetric nature of the rank-2 tensors) ([general-relativity](/source/general-relativity/)).
3. **Divergence-free** -- the stress-energy tensor must be divergence-free, encoding conservation of energy-momentum ([general-relativity](/source/general-relativity/)).
4. **Cosmological constant** -- the equations can include an additional term $\Lambda g_{\mu\nu}$ for the cosmological constant ([general-relativity](/source/general-relativity/)).

## Context & Application

Einstein presented these equations to the Prussian Academy of Science in November 1915. Karl Schwarzschild found the first non-trivial exact solution in 1916, laying the groundwork for black hole physics. The Friedmann solutions (1922) describe an expanding universe. Numerical methods and perturbation theory are used when exact solutions are unavailable ([general-relativity](/source/general-relativity/)).

## Examples

**Example 1** ([general-relativity](/source/general-relativity/)): The Schwarzschild solution describes the spacetime geometry around a non-rotating spherical mass and predicts the existence of event horizons.

**Example 2** ([general-relativity](/source/general-relativity/)): The Friedmann-Lemaitre-Robertson-Walker solutions model the large-scale expansion of the universe from the Big Bang.

## Relationships

- **Related to black-hole**: black holes arise as solutions where $R_{\mu\nu} = 0$ with particular boundary conditions.
- **Related to cosmological-model**: cosmological models are solutions of the field equations with the cosmological constant.
- **Related to gravitational-wave**: gravitational waves are wave-like solutions of the linearized field equations.

## Source Reference

Derived from:
- [general-relativity](/source/general-relativity/) -- provides the full mathematical formulation, exact solutions, and applications.

## Verification Notes

- Definition: directly from the "Einstein's equations" section
- Mathematical content: equations quoted verbatim from source
- Confidence: HIGH -- thoroughly and explicitly defined in the source
- Cross-references: black-hole, cosmological-model, gravitational-wave are planned concepts
- Uncertainties: none
