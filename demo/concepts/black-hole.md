---
# === CORE IDENTIFICATION ===
concept: "Black Hole"
slug: "black-hole"
page_type: "concept"

# === CLASSIFICATION ===
category: "theoretical-physics"
subcategory: "gravitation"
tier: "intermediate"
subject_headings:
  - "gravitation"
  - "astrophysics"
keywords:
  - "event horizon"
  - "singularity"
  - "Schwarzschild"
  - "Kerr"
  - "Hawking radiation"

# === PROVENANCE ===
derived_from:
  - slug: "general-relativity"
    title: "General relativity"
  - slug: "string-theory"
    title: "String theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "general-relativity-theory"
  - "einstein-field-equations"
extends: []
related:
  - "spacetime"
  - "gravitational-wave"
  - "hawking-radiation"
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

A black hole is a region of spacetime where gravity is so extreme that nothing, not even light, can escape from within its event horizon. Black holes are predicted by general relativity and confirmed by astronomical observations including gravitational wave detections.

## Core Definition

In general relativity, a black hole is defined as a region of spacetime in which the gravitational field is so strong that no particle or radiation can escape. The boundary of this region is called the event horizon. Black holes are the end-state for massive stars undergoing gravitational collapse ([general-relativity](/source/general-relativity/)). They are characterized by only eleven parameters: electric charge, mass-energy, linear momentum, angular momentum, and location at a specified time -- the "no-hair theorem" states that "black holes have no hair" ([general-relativity](/source/general-relativity/)).

String theory has provided tools for understanding the thermodynamic properties of black holes, including a derivation by Strominger and Vafa of the Bekenstein-Hawking entropy formula $S = \frac{c^3 k A}{4\hbar G}$ from counting microstates of D-brane systems ([string-theory](/source/string-theory/)).

## Prerequisites

- **General relativity** -- black holes are solutions of the Einstein field equations.
- **Einstein field equations** -- understanding the mathematical framework is needed to define black holes precisely.

## Key Properties

1. **Event horizon** -- the boundary beyond which nothing can escape ([general-relativity](/source/general-relativity/)).
2. **No-hair theorem** -- black holes are characterized by mass, charge, and angular momentum only ([general-relativity](/source/general-relativity/)).
3. **Black hole mechanics** -- laws analogous to thermodynamics govern black holes; entropy is proportional to surface area ([general-relativity](/source/general-relativity/)).
4. **Hawking radiation** -- semiclassical calculations predict thermal radiation from the event horizon ([general-relativity](/source/general-relativity/)).
5. **Singularities** -- generic feature predicted by the singularity theorems ([general-relativity](/source/general-relativity/)).

## Context & Application

Black holes were first predicted by the Schwarzschild solution (1916). The concept became central during the "golden age of general relativity" (1960-1975). LIGO's 2016 detection of gravitational waves from merging black holes provided direct observational confirmation. Supermassive black holes at galactic centers play essential roles in galaxy formation ([general-relativity](/source/general-relativity/)).

## Examples

**Example 1** ([general-relativity](/source/general-relativity/)): The Schwarzschild metric describes a static, non-rotating black hole; the Kerr metric describes a rotating black hole with an ergosphere.

**Example 2** ([string-theory](/source/string-theory/)): Strominger and Vafa showed that D-brane systems reproduce the Bekenstein-Hawking entropy $S = c^3 kA / 4\hbar G$ exactly for certain extremal black holes.

## Relationships

- **Related to spacetime**: black holes are extreme geometries of spacetime.
- **Related to gravitational-wave**: merging black holes produce the strongest gravitational wave signals.
- **Related to hawking-radiation**: predicted thermal emission from black hole horizons.

## Source Reference

Derived from:
- [general-relativity](/source/general-relativity/) -- provides the definition, properties, classification, and observational context
- [string-theory](/source/string-theory/) -- provides the entropy microstate counting

## Verification Notes

- Definition: from the "Black holes and other compact objects" section of [general-relativity](/source/general-relativity/)
- Bekenstein-Hawking formula: quoted from [string-theory](/source/string-theory/)
- Confidence: HIGH -- thoroughly defined across multiple sections of the source
- Multi-source merge: GR provides the classical definition; string theory adds the entropy derivation
- Cross-references: hawking-radiation is a planned concept
- Uncertainties: none
