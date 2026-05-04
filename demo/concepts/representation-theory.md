---
# === CORE IDENTIFICATION ===
concept: "Representation Theory"
slug: "representation-theory"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "algebra"
tier: "intermediate"
subject_headings:
  - "abstract-algebra"
  - "linear-algebra"
  - "theoretical-physics"
keywords:
  - "group homomorphism"
  - "linear transformation"
  - "irreducible representation"
  - "character theory"

# === PROVENANCE ===
derived_from:
  - slug: "group-theory"
    title: "Group theory"

# === CONFIDENCE ===
extraction_confidence: "medium"

# === VARIANTS ===
aliases:
  - "group representation theory"
  - "representation of groups"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "group-theory-mathematics"
  - "lie-group"
extends: []
related:
  - "symmetry-group"
  - "permutation-group"
  - "finite-group"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a group in mathematics?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Representation theory studies how groups act on vector spaces via linear transformations. A representation is a group homomorphism from a group to the general linear group of a vector space, making abstract group operations concrete through matrix multiplication.

## Core Definition

A representation of a group $G$ on a vector space $V$ is a group homomorphism $\rho: G \to \operatorname{GL}(V)$, where $\operatorname{GL}(V)$ consists of the invertible linear transformations of $V$. In other words, to every group element $g$ is assigned an automorphism $\rho(g)$ such that $\rho(g) \circ \rho(h) = \rho(gh)$ for any $h$ in $G$ ([group-theory](/source/group-theory/)). Given a group $G$, representation theory then asks what representations of $G$ exist, with the employed methods and results differing between representation theory of finite groups and representations of Lie groups.

## Prerequisites

- **Group theory** -- representation theory is defined in terms of groups and group homomorphisms.
- **Lie group** -- the representation theory of Lie groups is one of the two main subdomains, requiring familiarity with Lie groups.

## Key Properties

1. **Concretization** -- representations make abstract group operations explicit through matrix multiplication ([group-theory](/source/group-theory/)).
2. **Decomposition** -- for finite groups, every representation decomposes into irreducible parts (Maschke's theorem), which are more manageable via Schur's lemma ([group-theory](/source/group-theory/)).
3. **Character theory** -- the totality of representations is governed by the group's characters; Fourier polynomials can be interpreted as characters of U(1).
4. **Two main branches** -- representation theory of finite groups and representations of Lie groups are the principal subdomains.

## Context & Application

Representation theory is important in physics because group representations point the way to possible physical theories. In the Standard Model, particles are classified by their transformation properties under the relevant symmetry groups. In harmonic analysis, Haar measures on Lie groups are used for pattern recognition and image processing. The theory provides tools to simplify the study of complex objects by leveraging a well-understood group acting on them ([group-theory](/source/group-theory/)).

## Examples

**Example 1** ([group-theory](/source/group-theory/)): Fourier polynomials can be interpreted as the characters of $U(1)$, the group of complex numbers of absolute value 1, acting on the $L^2$-space of periodic functions.

**Example 2** ([group-theory](/source/group-theory/)): For a finite group $G$ acting on a vector space $V$, Maschke's theorem guarantees that $V$ decomposes into irreducible subrepresentations.

## Relationships

- **Prerequisite: group-theory-mathematics** -- representation theory is a branch of group theory.
- **Prerequisite: lie-group** -- representations of Lie groups form a major subdomain.
- **Related to symmetry-group** -- representations formalize how symmetry groups act on vector spaces.
- **Related to finite-group** -- representation theory of finite groups uses techniques like Maschke's theorem and character tables.

## Source Reference

Derived from:
- [group-theory](/source/group-theory/) -- provides the definition in the "Representation of groups" section, Maschke's theorem, Schur's lemma, character theory, and physics applications.

## Verification Notes

- Definition: adapted from the "Representation of groups" section of [group-theory](/source/group-theory/)
- Key Properties: items 1-2 explicit in source; items 3-4 synthesized from the discussion
- Confidence: MEDIUM -- the concept is discussed extensively but the definition requires synthesis from the formal homomorphism definition and the surrounding context
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: the source page discusses representation theory as a topic within group theory rather than as a fully self-contained concept
