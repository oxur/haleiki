---
# === CORE IDENTIFICATION ===
concept: "Quantum Tunnelling"
slug: "quantum-tunnelling"
page_type: "concept"

# === CLASSIFICATION ===
category: "theoretical-physics"
subcategory: "foundations"
tier: "intermediate"
subject_headings:
  - "quantum-mechanics"
keywords:
  - "potential barrier"
  - "tunnelling"
  - "barrier penetration"
  - "radioactive decay"

# === PROVENANCE ===
derived_from:
  - slug: "quantum-mechanics"
    title: "Quantum mechanics"

# === CONFIDENCE ===
extraction_confidence: "medium"

# === VARIANTS ===
aliases:
  - "quantum tunneling"
  - "barrier penetration"
  - "tunnel effect"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "wave-function"
extends: []
related:
  - "quantum-mechanics-theory"
  - "schrodinger-equation"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is the wave function used to calculate measurement probabilities?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Quantum tunnelling is the phenomenon by which a quantum particle can pass through a potential energy barrier that it could not surmount according to classical mechanics. It occurs because the wave function has a non-zero amplitude on the far side of the barrier.

## Core Definition

A non-classical phenomenon predicted by quantum mechanics is quantum tunnelling: a particle that goes up against a potential barrier can cross it, even if its kinetic energy is smaller than the maximum of the potential. In classical mechanics, this particle would be trapped ([quantum-mechanics](/source/quantum-mechanics/)). The phenomenon arises because the Schrodinger equation permits the wave function to have a non-zero (though exponentially decaying) amplitude within and beyond a classically forbidden region, giving a finite probability of finding the particle on the other side.

## Prerequisites

- **Wave function** -- tunnelling is understood through the wave function's behavior in classically forbidden regions; the concept of probability amplitude extending through a barrier requires understanding wave functions.

## Key Properties

1. **Classically forbidden** -- in classical mechanics, a particle with insufficient energy to surmount a barrier is reflected; quantum mechanics allows a finite transmission probability.
2. **Exponential sensitivity** -- the tunnelling probability decreases exponentially with the barrier width and with the square root of the energy deficit.
3. **Non-zero wave function** -- the wave function does not go to zero at the barrier but decays exponentially within it, emerging with reduced amplitude on the other side.

## Context & Application

Quantum tunnelling has several important practical consequences, enabling radioactive decay (alpha particles tunnel out of the nucleus), nuclear fusion in stars (protons tunnel through the Coulomb barrier), and technologies such as scanning tunnelling microscopy, tunnel diodes, tunnel field-effect transistors, and flash memory ([quantum-mechanics](/source/quantum-mechanics/)).

## Examples

**Example 1** ([quantum-mechanics](/source/quantum-mechanics/)): The rectangular potential barrier furnishes a model for quantum tunnelling. A particle approaching a barrier of height $V_0$ with energy $E < V_0$ has a non-zero probability of being found on the far side.

**Example 2** ([quantum-mechanics](/source/quantum-mechanics/)): Nuclear fusion in stars relies on quantum tunnelling: protons do not have enough thermal energy to overcome the Coulomb barrier classically, but they can tunnel through it.

## Relationships

- **Prerequisite: wave-function** -- the wave function's exponential decay within a barrier is the mechanism underlying tunnelling.
- **Related to quantum-mechanics-theory** -- tunnelling is one of the signature non-classical phenomena of quantum mechanics.
- **Related to schrodinger-equation** -- solving the Schrodinger equation for a potential barrier yields the tunnelling probability.

## Source Reference

Derived from:
- [quantum-mechanics](/source/quantum-mechanics/) -- provides the conceptual description, the connection to the rectangular potential barrier, and practical applications including radioactive decay and scanning tunnelling microscopy.

## Verification Notes

- Definition: adapted from the "Overview and fundamental concepts" section of [quantum-mechanics](/source/quantum-mechanics/)
- Key Properties: items 1 and 3 explicit in source; item 2 is standard physics knowledge consistent with the source's description
- Confidence: MEDIUM -- the concept is clearly described but the source does not provide a full mathematical derivation of tunnelling; it describes the phenomenon and its consequences
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: the mathematical detail of tunnelling (transmission coefficients) is not provided in the source
