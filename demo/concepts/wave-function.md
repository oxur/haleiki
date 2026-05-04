---
# === CORE IDENTIFICATION ===
concept: "Wave Function"
slug: "wave-function"
page_type: "concept"

# === CLASSIFICATION ===
category: "theoretical-physics"
subcategory: "foundations"
tier: "foundational"
subject_headings:
  - "quantum-mechanics"
  - "mathematical-physics"
keywords:
  - "probability amplitude"
  - "Hilbert space"
  - "state vector"
  - "Born rule"

# === PROVENANCE ===
derived_from:
  - slug: "quantum-mechanics"
    title: "Quantum mechanics"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "wavefunction"
  - "quantum state"
  - "state vector"
  - "psi"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "quantum-superposition"
  - "schrodinger-equation"
  - "uncertainty-principle"
  - "quantum-mechanics-theory"
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

A wave function is a mathematical entity that provides information, in the form of probability amplitudes, about what measurements of a quantum particle's energy, momentum, and other physical properties may yield. It is a vector in a complex Hilbert space representing the state of a quantum system.

## Core Definition

In the mathematically rigorous formulation of quantum mechanics, the state of a quantum mechanical system is a vector $\psi$ belonging to a separable complex Hilbert space $\mathcal{H}$. This vector is postulated to be normalized under the Hilbert space inner product, that is, $\langle \psi, \psi \rangle = 1$, and it is well-defined up to a complex number of modulus 1 (the global phase) ([quantum-mechanics](/source/quantum-mechanics/)). A quantum particle like an electron can be described by a wave function which associates to each point in space a probability amplitude. Applying the Born rule to these amplitudes gives a probability density function for the position that the particle will be found to have when measured ([quantum-mechanics](/source/quantum-mechanics/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki.

## Key Properties

1. **Lives in Hilbert space** -- the wave function is a vector in a complex Hilbert space whose exact nature depends on the system (e.g., $L^2(\mathbb{C})$ for position/momentum, $\mathbb{C}^2$ for spin-1/2).
2. **Normalization** -- the wave function must satisfy $\langle \psi, \psi \rangle = 1$.
3. **Global phase invariance** -- $\psi$ and $e^{i\alpha}\psi$ represent the same physical system.
4. **Born rule** -- the probability of a measurement outcome is given by the squared modulus of the relevant probability amplitude: $|\langle \vec{\lambda}, \psi \rangle|^2$ for a non-degenerate eigenvalue $\lambda$ ([quantum-mechanics](/source/quantum-mechanics/)).
5. **Collapse postulate** -- after measurement yielding result $\lambda$, the wave function collapses to the corresponding eigenstate.

## Context & Application

The wave function is the central mathematical object of quantum mechanics. It was introduced through the work of Schrodinger (wave mechanics) and Born (probabilistic interpretation, 1926). The wave function formalism unifies with Heisenberg's matrix mechanics through Dirac's transformation theory. In practice, wave functions are used to compute atomic orbitals, molecular energy levels, scattering amplitudes, and tunnelling probabilities ([quantum-mechanics](/source/quantum-mechanics/)).

## Examples

**Example 1** ([quantum-mechanics](/source/quantum-mechanics/)): The electron wave function for an unexcited hydrogen atom is a spherically symmetric function known as an $s$ orbital.

**Example 2** ([quantum-mechanics](/source/quantum-mechanics/)): A Gaussian wave packet $\psi(x,0) = \frac{1}{\sqrt[4]{\pi a}} e^{-x^2/(2a)}$ illustrates the uncertainty principle: making $a$ smaller narrows the position spread but broadens the momentum spread.

## Relationships

- **Related to quantum-superposition** -- wave functions can be superpositions of eigenstates.
- **Related to schrodinger-equation** -- the Schrodinger equation governs the time evolution of the wave function.
- **Related to uncertainty-principle** -- the uncertainty principle constrains the spreads of conjugate observables in any wave function.
- **Related to quantum-mechanics-theory** -- the wave function is the central mathematical object of quantum mechanics.

## Source Reference

Derived from:
- [quantum-mechanics](/source/quantum-mechanics/) -- provides the formal definition, the Born rule, normalization conditions, and examples including the hydrogen atom and Gaussian wave packets.

## Verification Notes

- Definition: directly adapted from the "Mathematical formulation" section of [quantum-mechanics](/source/quantum-mechanics/)
- Key Properties: all items explicitly stated in the source
- Examples: drawn from the "Free particle" and "Time evolution" subsections
- Confidence: HIGH -- the source provides a rigorous mathematical formulation
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
