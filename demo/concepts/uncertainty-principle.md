---
# === CORE IDENTIFICATION ===
concept: "Uncertainty Principle"
slug: "uncertainty-principle"
page_type: "concept"

# === CLASSIFICATION ===
category: "theoretical-physics"
subcategory: "foundations"
tier: "foundational"
subject_headings:
  - "quantum-mechanics"
  - "mathematical-physics"
keywords:
  - "Heisenberg"
  - "position"
  - "momentum"
  - "commutation relation"
  - "standard deviation"

# === PROVENANCE ===
derived_from:
  - slug: "quantum-mechanics"
    title: "Quantum mechanics"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "Heisenberg uncertainty principle"
  - "indeterminacy principle"
  - "Heisenberg's uncertainty relation"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "wave-function"
extends: []
related:
  - "quantum-mechanics-theory"
  - "quantum-superposition"
  - "schrodinger-equation"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the uncertainty principle?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

The uncertainty principle states that for certain pairs of physical properties (such as position and momentum), the more precisely one property is known, the less precisely the other can be predicted. It is a fundamental consequence of the mathematical structure of quantum mechanics, not a limitation of measurement technology.

## Core Definition

One consequence of the basic quantum formalism is the uncertainty principle. In its most familiar form, this states that no preparation of a quantum particle can imply simultaneously precise predictions both for a measurement of its position and for a measurement of its momentum ([quantum-mechanics](/source/quantum-mechanics/)). The position operator $\hat{X}$ and momentum operator $\hat{P}$ do not commute but satisfy the canonical commutation relation $[\hat{X}, \hat{P}] = i\hbar$. Defining uncertainty as standard deviation, the principle is expressed mathematically as:

$$\sigma_X \sigma_P \geq \frac{\hbar}{2}$$

This generalizes to arbitrary pairs of self-adjoint operators $A$ and $B$: $\sigma_A \sigma_B \geq \frac{1}{2}|\langle [A,B] \rangle|$ ([quantum-mechanics](/source/quantum-mechanics/)).

## Prerequisites

- **Wave function** -- the uncertainty principle is derived from properties of quantum states (wave functions) in Hilbert space, including expectation values and standard deviations of observables.

## Key Properties

1. **Fundamental limit** -- it is not a limitation of measurement apparatus but an inherent property of the quantum formalism.
2. **Trade-off** -- either $\sigma_X$ or $\sigma_P$ can be made arbitrarily small, but not both simultaneously ([quantum-mechanics](/source/quantum-mechanics/)).
3. **Generalized form** -- the principle applies to any pair of non-commuting observables, with the lower bound determined by the expectation value of their commutator.
4. **Fourier transform connection** -- position and momentum descriptions are Fourier transforms of each other, which is the mathematical origin of the trade-off ([quantum-mechanics](/source/quantum-mechanics/)).

## Context & Application

The uncertainty principle was formulated by Werner Heisenberg in 1927 and is one of the most famous results of quantum mechanics. It explains why electrons in atoms are described by orbitals (probability distributions) rather than precise trajectories. In practice, the uncertainty principle limits the precision of simultaneous measurements of conjugate variables in spectroscopy, quantum optics, and quantum information science.

## Examples

**Example 1** ([quantum-mechanics](/source/quantum-mechanics/)): For a Gaussian wave packet $\psi(x,0) = \frac{1}{\sqrt[4]{\pi a}} e^{-x^2/(2a)}$, making the parameter $a$ smaller narrows the position spread but broadens the momentum distribution. Conversely, making $a$ larger narrows the momentum spread but broadens the position spread, directly illustrating the uncertainty principle.

**Example 2** ([quantum-mechanics](/source/quantum-mechanics/)): As a Gaussian wave packet evolves in time, its center moves at constant velocity but the position uncertainty grows (the wave packet spreads), while the momentum uncertainty remains constant.

## Relationships

- **Prerequisite: wave-function** -- the principle is derived from the mathematical properties of wave functions and their Fourier transforms.
- **Related to quantum-mechanics-theory** -- the uncertainty principle is a foundational consequence of quantum mechanics.
- **Related to quantum-superposition** -- a state localized in position is a superposition of many momentum eigenstates, and vice versa.
- **Related to schrodinger-equation** -- the time evolution described by the Schrodinger equation determines how uncertainties change over time.

## Common Confusions

- **Not about observer disturbance** -- the uncertainty principle is not about the act of measurement disturbing the system (though that also occurs). It is an intrinsic property of quantum states: a state cannot simultaneously be a precise eigenstate of both position and momentum.
- **Not about lack of knowledge** -- the uncertainty is not epistemic (we simply don't know the values) but ontological in most interpretations: the values are genuinely indeterminate.

## Source Reference

Derived from:
- [quantum-mechanics](/source/quantum-mechanics/) -- provides the formal statement, the commutation relation derivation, the generalized form, and the Gaussian wave packet illustration.

## Verification Notes

- Definition: directly adapted from the "Uncertainty principle" section of [quantum-mechanics](/source/quantum-mechanics/)
- Key Properties: all items explicitly stated in the source with mathematical formulation
- Examples: drawn from the "Free particle" section illustrating the Gaussian wave packet
- Confidence: HIGH -- dedicated section with complete mathematical derivation
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
