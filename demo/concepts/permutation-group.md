---
# === CORE IDENTIFICATION ===
concept: "Permutation Group"
slug: "permutation-group"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "algebra"
tier: "intermediate"
subject_headings:
  - "abstract-algebra"
  - "combinatorics"
keywords:
  - "permutation"
  - "symmetric group"
  - "bijection"
  - "group action"

# === PROVENANCE ===
derived_from:
  - slug: "group-theory"
    title: "Group theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "permutation groups"
  - "group of permutations"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "group-theory-mathematics"
extends: []
related:
  - "symmetry-group"
  - "finite-group"
  - "representation-theory"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What distinguishes a permutation group from a matrix group?"
  - "What is a group in mathematics?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

A permutation group is a group consisting of bijections (permutations) of a set into itself, closed under composition and inverses. It was the first class of groups to be systematically studied.

## Core Definition

Given any set $X$ and a collection $G$ of bijections of $X$ into itself (known as permutations) that is closed under compositions and inverses, $G$ is a group acting on $X$. If $X$ consists of $n$ elements and $G$ consists of all permutations, $G$ is the symmetric group $S_n$; in general, any permutation group $G$ is a subgroup of the symmetric group of $X$. An early construction due to Cayley exhibited any group as a permutation group, acting on itself ($X = G$) by means of the left regular representation ([group-theory](/source/group-theory/)).

## Prerequisites

- **Group theory** -- permutation groups are a specific type of group; understanding the group axioms (closure, associativity, identity, inverses) is necessary to work with them.

## Key Properties

1. **Closure under composition** -- composing two permutations of a set yields another permutation.
2. **Subgroup of symmetric group** -- every permutation group on a set $X$ is a subgroup of the symmetric group $S_n$ where $n = |X|$.
3. **Cayley's theorem** -- every group is isomorphic to a permutation group, acting on itself via left multiplication.
4. **Simplicity results** -- for $n \geq 5$, the alternating group $A_n$ is simple, which plays a key role in the impossibility of solving general polynomial equations of degree $\geq 5$ in radicals ([group-theory](/source/group-theory/)).

## Context & Application

Permutation groups were the first class of groups to undergo systematic study, historically developed by Lagrange, Ruffini, Abel, Cayley, and Cauchy in pursuit of general solutions to polynomial equations. They remain important in combinatorics (via Burnside's lemma for counting), Galois theory (where permutation groups describe the symmetries of polynomial roots), and cryptography ([group-theory](/source/group-theory/)).

## Examples

**Example 1** ([group-theory](/source/group-theory/)): The Rubik's Cube group is a permutation group -- each sequence of moves permutes the positions and orientations of the cube's pieces.

**Example 2** ([group-theory](/source/group-theory/)): The symmetric group $S_5$ is not solvable, which implies that the general quintic equation cannot be solved by radicals.

## Relationships

- **Prerequisite: group-theory-mathematics** -- permutation groups are studied within the framework of group theory.
- **Related to symmetry-group** -- permutation groups are symmetry groups of sets with no additional structure.
- **Related to finite-group** -- many permutation groups are finite.
- **Related to representation-theory** -- representation theory generalizes the idea of acting on a set to acting on a vector space.
- **Contrasts with matrix groups** -- while permutation groups act on sets via bijections, matrix groups act on vector spaces via linear transformations. However, both are special cases of transformation groups ([group-theory](/source/group-theory/)).

## Source Reference

Derived from:
- [group-theory](/source/group-theory/) -- provides the definition, Cayley's theorem, the alternating group simplicity result, and historical context.

## Verification Notes

- Definition: directly quoted and adapted from the "Permutation groups" section of [group-theory](/source/group-theory/)
- Key Properties: items 1-3 explicit in source; item 4 explicitly stated
- Confidence: HIGH -- dedicated subsection with clear definitions and examples
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
