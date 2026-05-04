---
# === CORE IDENTIFICATION ===
concept: "Schrodinger Equation"
slug: "schrodinger-equation"
page_type: "concept"

# === CLASSIFICATION ===
category: "theoretical-physics"
subcategory: "foundations"
tier: "intermediate"
subject_headings:
  - "quantum-mechanics"
  - "mathematical-physics"
keywords:
  - "time evolution"
  - "Hamiltonian"
  - "wave equation"
  - "unitary operator"
  - "Erwin Schrodinger"

# === PROVENANCE ===
derived_from:
  - slug: "quantum-mechanics"
    title: "Quantum mechanics"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "Schroedinger equation"
  - "Schrodinger wave equation"
  - "time-dependent Schrodinger equation"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "wave-function"
extends: []
related:
  - "quantum-mechanics-theory"
  - "quantum-superposition"
  - "uncertainty-principle"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does the Schrödinger equation relate to quantum state evolution?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

The Schrodinger equation is the fundamental equation of quantum mechanics that describes how the quantum state of a system evolves over time. It relates the time derivative of the wave function to the Hamiltonian operator, which represents the total energy of the system.

## Core Definition

The time evolution of a quantum state is described by the Schrodinger equation:

$$i\hbar \frac{\partial}{\partial t} \psi(t) = H \psi(t)$$

Here $H$ denotes the Hamiltonian, the observable corresponding to the total energy of the system, and $\hbar$ is the reduced Planck constant. The solution is $\psi(t) = e^{-iHt/\hbar}\psi(0)$, where the operator $U(t) = e^{-iHt/\hbar}$ is the time-evolution operator, which has the crucial property of being unitary. This time evolution is deterministic: given an initial quantum state $\psi(0)$, it makes a definite prediction of what $\psi(t)$ will be at any later time ([quantum-mechanics](/source/quantum-mechanics/)).

## Prerequisites

- **Wave function** -- the Schrodinger equation operates on wave functions (state vectors in Hilbert space); understanding what a wave function is and how measurement probabilities are extracted from it is essential.

## Key Properties

1. **Linear** -- the equation is linear in $\psi$, meaning superpositions of solutions are also solutions.
2. **Deterministic evolution** -- between measurements, the Schrodinger equation determines the quantum state deterministically; indeterminism enters only upon measurement ([quantum-mechanics](/source/quantum-mechanics/)).
3. **Unitary time evolution** -- the time-evolution operator $U(t)$ is unitary, preserving the normalization of the state vector and thus ensuring probabilities sum to 1.
4. **Conservation laws** -- any observable $A$ that commutes with $H$ is conserved: its expectation value does not change over time. This is the quantum analog of Noether's theorem ([quantum-mechanics](/source/quantum-mechanics/)).
5. **Correspondence principle** -- the constant $i\hbar$ is introduced so that the Hamiltonian reduces to the classical Hamiltonian in the appropriate limit.

## Context & Application

The Schrodinger equation was developed by Erwin Schrodinger in 1925-1926 as part of wave mechanics. It is used to compute energy levels of atoms (e.g., the hydrogen atom), molecular orbitals, tunnelling probabilities, and the dynamics of quantum systems. Analytic solutions exist for a few model Hamiltonians (quantum harmonic oscillator, particle in a box, hydrogen atom), while more complex systems require approximation methods like perturbation theory ([quantum-mechanics](/source/quantum-mechanics/)).

## Examples

**Example 1** ([quantum-mechanics](/source/quantum-mechanics/)): For a particle in a one-dimensional box with infinite potential walls, the time-independent Schrodinger equation $-\frac{\hbar^2}{2m}\frac{d^2\psi}{dx^2} = E\psi$ yields quantized energy levels $E_n = \frac{n^2 h^2}{8mL^2}$.

**Example 2** ([quantum-mechanics](/source/quantum-mechanics/)): For a free particle, the general solution is a superposition of plane waves: $\psi(x,t) = \frac{1}{\sqrt{2\pi}}\int_{-\infty}^{\infty} \hat{\psi}(k,0) e^{i(kx - \frac{\hbar k^2}{2m}t)} dk$.

**Example 3** ([quantum-mechanics](/source/quantum-mechanics/)): The quantum harmonic oscillator has eigenstates given by Hermite polynomials with energy levels $E_n = \hbar\omega(n + \frac{1}{2})$.

## Relationships

- **Prerequisite: wave-function** -- the Schrodinger equation describes how the wave function evolves in time.
- **Related to quantum-mechanics-theory** -- the Schrodinger equation is the fundamental dynamical equation of quantum mechanics.
- **Related to quantum-superposition** -- the linearity of the equation means superpositions of solutions are also solutions.
- **Related to uncertainty-principle** -- the Fourier-transform relationship between position and momentum representations is connected to the structure of the Schrodinger equation.

## Source Reference

Derived from:
- [quantum-mechanics](/source/quantum-mechanics/) -- provides the equation, its solution via the time-evolution operator, the correspondence principle, conservation laws, and worked examples (free particle, particle in a box, harmonic oscillator).

## Verification Notes

- Definition: directly quoted from the "Time evolution of a quantum state" section of [quantum-mechanics](/source/quantum-mechanics/)
- Key Properties: all items explicitly stated with mathematical formulation
- Examples: three worked examples from the source's "Examples" section
- Confidence: HIGH -- the equation is presented with full derivation and multiple applications
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
