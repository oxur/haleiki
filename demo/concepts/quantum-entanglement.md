---
# === CORE IDENTIFICATION ===
concept: "Quantum Entanglement"
slug: "quantum-entanglement"
page_type: "concept"

# === CLASSIFICATION ===
category: "theoretical-physics"
subcategory: "foundations"
tier: "intermediate"
subject_headings:
  - "quantum-mechanics"
  - "quantum-information"
keywords:
  - "entanglement"
  - "Bell's theorem"
  - "non-separable"
  - "EPR paradox"
  - "quantum computing"

# === PROVENANCE ===
derived_from:
  - slug: "quantum-mechanics"
    title: "Quantum mechanics"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "entanglement"
  - "quantum correlations"
  - "EPR correlations"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "quantum-superposition"
extends: []
related:
  - "quantum-mechanics-theory"
  - "wave-function"
  - "schrodinger-equation"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What must I understand before learning about quantum entanglement?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Quantum entanglement is a phenomenon in which the properties of two or more quantum systems become correlated in such a way that a description of the whole cannot be expressed as a product of descriptions of the individual parts. Schrodinger called it "the characteristic trait of quantum mechanics."

## Core Definition

When quantum systems interact, the result can be the creation of quantum entanglement: their properties become so intertwined that a description of the whole solely in terms of the individual parts is no longer possible ([quantum-mechanics](/source/quantum-mechanics/)). Mathematically, the Hilbert space of a composite system is the tensor product $\mathcal{H}_{AB} = \mathcal{H}_A \otimes \mathcal{H}_B$. States that cannot be written as $\psi_A \otimes \psi_B$ are called entangled. For example, the state $\frac{1}{\sqrt{2}}(\psi_A \otimes \psi_B + \phi_A \otimes \phi_B)$ is a valid joint state that is not separable ([quantum-mechanics](/source/quantum-mechanics/)).

## Prerequisites

- **Quantum superposition** -- entangled states are non-separable superpositions of product states; understanding superposition is necessary to grasp why entangled states differ from simple product states.

## Key Properties

1. **Non-separability** -- an entangled state cannot be written as a tensor product of individual states.
2. **Reduced density matrices** -- for entangled states, individual subsystems can only be described by reduced density matrices, with a necessary loss of information compared to the full composite state ([quantum-mechanics](/source/quantum-mechanics/)).
3. **Bell's theorem** -- local hidden variable theories are incompatible with quantum mechanics; Bell tests show that entangled particles violate Bell inequalities ([quantum-mechanics](/source/quantum-mechanics/)).
4. **No faster-than-light communication** -- despite correlations, entanglement does not allow sending signals faster than light, as demonstrated by the no-communication theorem ([quantum-mechanics](/source/quantum-mechanics/)).
5. **Decoherence** -- systems interacting with their environment generally become entangled with it, a process called quantum decoherence, which explains why quantum effects are hard to observe macroscopically.

## Context & Application

Quantum entanglement enables quantum computing and is part of quantum communication protocols such as quantum key distribution and superdense coding. The EPR paradox (Einstein, Podolsky, Rosen, 1935) argued that entanglement implies quantum mechanics is incomplete. Bell's theorem (1964) showed that local hidden variable theories are incompatible with quantum predictions, and subsequent Bell test experiments have confirmed the quantum predictions ([quantum-mechanics](/source/quantum-mechanics/)).

## Examples

**Example 1** ([quantum-mechanics](/source/quantum-mechanics/)): If system $A$ can be in states $\psi_A$ or $\phi_A$, and system $B$ in states $\psi_B$ or $\phi_B$, then $\frac{1}{\sqrt{2}}(\psi_A \otimes \psi_B + \phi_A \otimes \phi_B)$ is an entangled state that cannot be factored into a product of individual states.

**Example 2** ([quantum-mechanics](/source/quantum-mechanics/)): In the Mach-Zehnder interferometer, an apparatus can become entangled with the system being measured, and such entanglement is a key feature of models of measurement processes.

## Relationships

- **Prerequisite: quantum-superposition** -- entangled states are superpositions of product states.
- **Related to quantum-mechanics-theory** -- entanglement is a defining feature of quantum mechanics.
- **Related to wave-function** -- entangled states are described by joint wave functions in a tensor product Hilbert space.
- **Related to schrodinger-equation** -- entanglement arises dynamically through interaction, governed by the Schrodinger equation.

## Common Confusions

- **Entanglement does not allow faster-than-light communication** -- the correlations are only apparent when comparing results from both subsystems, which requires classical communication.
- **Entanglement is not "spooky action at a distance" in the sense Einstein feared** -- Bell test experiments show that quantum mechanics is correct, but the correlations do not violate relativity.

## Source Reference

Derived from:
- [quantum-mechanics](/source/quantum-mechanics/) -- provides the formal definition using tensor products, the Bell's theorem discussion, the no-communication theorem, and the decoherence connection.

## Verification Notes

- Definition: directly adapted from the "Composite systems and entanglement" section of [quantum-mechanics](/source/quantum-mechanics/)
- Key Properties: all items explicitly stated in the source
- Common Confusions: synthesized from the source's discussion of hidden variables and the no-communication theorem
- Confidence: HIGH -- dedicated subsection with mathematical formulation and extensive discussion
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
