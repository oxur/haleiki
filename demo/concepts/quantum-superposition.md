---
# === CORE IDENTIFICATION ===
concept: "Quantum Superposition"
slug: "quantum-superposition"
page_type: "concept"

# === CLASSIFICATION ===
category: "theoretical-physics"
subcategory: "foundations"
tier: "foundational"
subject_headings:
  - "quantum-mechanics"
keywords:
  - "superposition"
  - "linear combination"
  - "eigenstate"
  - "interference"

# === PROVENANCE ===
derived_from:
  - slug: "quantum-mechanics"
    title: "Quantum mechanics"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "superposition"
  - "superposition principle"
  - "quantum state superposition"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "wave-function"
extends: []
related:
  - "quantum-mechanics-theory"
  - "quantum-entanglement"
  - "wave-particle-duality"
  - "uncertainty-principle"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is quantum superposition?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Quantum superposition is the principle that a quantum state can be a linear combination of eigenstates of an observable, existing in multiple states simultaneously until measured. Upon measurement, the system yields one eigenvalue with probability determined by the Born rule.

## Core Definition

More generally, a quantum state will be a linear combination of the eigenstates, known as a quantum superposition. When an observable is measured, the result will be one of its eigenvalues with probability given by the Born rule ([quantum-mechanics](/source/quantum-mechanics/)). The superposition principle implies that linear combinations of "separable" or "product states" are also valid joint states, enabling phenomena such as quantum entanglement and quantum interference ([quantum-mechanics](/source/quantum-mechanics/)).

## Prerequisites

- **Wave function** -- superposition is defined in terms of wave functions (state vectors in Hilbert space); understanding the wave function is necessary to grasp what is being superposed.

## Key Properties

1. **Linearity** -- if $\psi_1$ and $\psi_2$ are valid quantum states, then $\alpha \psi_1 + \beta \psi_2$ is also a valid state (with appropriate normalization).
2. **Measurement collapse** -- upon measurement, a superposition collapses to one of the eigenstates, yielding the corresponding eigenvalue.
3. **Probability via Born rule** -- the probability of obtaining eigenvalue $\lambda$ is $|\langle \vec{\lambda}, \psi \rangle|^2$ for a non-degenerate case ([quantum-mechanics](/source/quantum-mechanics/)).
4. **Enables interference** -- superposition of different paths leads to constructive and destructive interference, as in the double-slit experiment.

## Context & Application

Superposition is one of the most distinctive features of quantum mechanics, fundamentally distinguishing it from classical physics. It underpins the double-slit experiment (where a photon is in a genuine superposition of two paths), quantum computing (where qubits exist in superpositions of 0 and 1), and quantum communication protocols. The Mach-Zehnder interferometer demonstrates that a photon does not take one path or another but is in a genuine quantum superposition of the two paths ([quantum-mechanics](/source/quantum-mechanics/)).

## Examples

**Example 1** ([quantum-mechanics](/source/quantum-mechanics/)): In the Mach-Zehnder interferometer, a photon entering from the left is in a superposition of the "lower" and "upper" paths: $\psi = \alpha \psi_l + \beta \psi_u$. Blocking one path eliminates interference, confirming that the photon was in a genuine superposition.

**Example 2** ([quantum-mechanics](/source/quantum-mechanics/)): The double-slit experiment produces an interference pattern because each photon passes through both slits simultaneously as a superposition; detecting which slit destroys the interference.

## Relationships

- **Prerequisite: wave-function** -- superposition is a property of wave functions as vectors in Hilbert space.
- **Related to quantum-entanglement** -- entangled states are non-separable superpositions of product states.
- **Related to wave-particle-duality** -- superposition of paths explains the wave-like interference patterns observed for particles.
- **Related to uncertainty-principle** -- a state that is a superposition of position eigenstates has well-defined momentum properties, and vice versa.

## Common Confusions

- **Superposition does not mean the particle is "in two places at once"** -- the superposition describes probability amplitudes, not classical positions. The particle does not have a definite position until measured.
- **Superposition is not the same as classical probability mixing** -- a superposition allows interference between the components, while a classical probability mixture does not.

## Source Reference

Derived from:
- [quantum-mechanics](/source/quantum-mechanics/) -- provides the definition within the mathematical formulation section, the Mach-Zehnder interferometer example, and the double-slit experiment illustration.

## Verification Notes

- Definition: directly adapted from the "Mathematical formulation" section and "Mach-Zehnder interferometer" example
- Key Properties: all items explicit in source
- Common Confusions: synthesized from the source's discussion of the double-slit experiment and Mach-Zehnder interferometer
- Confidence: HIGH -- explicitly defined with mathematical formulation and multiple examples
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
