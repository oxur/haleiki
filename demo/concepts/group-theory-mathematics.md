---
# === CORE IDENTIFICATION ===
concept: "Group Theory"
slug: "group-theory-mathematics"
page_type: "concept"
disambiguator: "mathematics"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "algebra"
tier: "foundational"
subject_headings:
  - "abstract-algebra"
  - "symmetry"
keywords:
  - "groups"
  - "algebraic structures"
  - "symmetry"
  - "abstract algebra"

# === PROVENANCE ===
derived_from:
  - slug: "group-theory"
    title: "Group theory"
  - slug: "abstract-algebra"
    title: "Abstract algebra"
  - slug: "group-mathematics"
    title: "Group (mathematics)"
  - slug: "history-of-group-theory"
    title: "History of group theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "theory of groups"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "symmetry-group"
  - "permutation-group"
  - "finite-group"
  - "lie-group"
  - "representation-theory"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is group theory?"
  - "How does group theory relate to abstract algebra?"
  - "What foundational concepts does group theory build upon?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Group theory is the branch of abstract algebra that studies algebraic structures known as groups. It provides the mathematical framework for analysing symmetry across mathematics, physics, chemistry, and cryptography.

## Core Definition

In abstract algebra, group theory studies the algebraic structures known as groups. The concept of a group is central to abstract algebra: other well-known algebraic structures, such as rings, fields, and vector spaces, can all be seen as groups endowed with additional operations and axioms ([group-theory](/source/group-theory/)). Groups recur throughout mathematics, and the methods of group theory have influenced many parts of algebra. Linear algebraic groups and Lie groups are two branches of group theory that have experienced significant advances and have become subject areas in their own right ([group-theory](/source/group-theory/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki.

## Key Properties

1. **Central to abstract algebra** -- groups are the simplest algebraic structures, and other structures (rings, fields, vector spaces) are groups with additional operations.
2. **Three historical sources** -- group theory arose from number theory, the theory of algebraic equations, and geometry ([group-theory](/source/group-theory/)).
3. **Broad applicability** -- group-theoretic methods apply to physics (Standard Model, gauge theory), chemistry (molecular symmetry), cryptography (elliptic curve cryptography), and more.
4. **Classification achievement** -- the classification of finite simple groups, completed in the 20th century across more than 10,000 journal pages, is a landmark result ([group-theory](/source/group-theory/)).
5. **Multiple branches** -- major branches include finite group theory, representation theory, Lie theory, and combinatorial/geometric group theory.

## Context & Application

Group theory emerged in the 19th century from the work of Euler, Gauss, Lagrange, Abel, and Galois. Galois first used groups to determine the solvability of polynomial equations, establishing what is now called Galois theory. Klein's Erlangen programme used group theory as the organizing principle of geometry. Lie groups, developed by Sophus Lie, provide the framework for continuous symmetry in differential equations and theoretical physics. In modern applications, group theory underpins public-key cryptography (via elliptic curve groups), molecular symmetry classification in chemistry, and the symmetry groups of the Standard Model in particle physics ([group-theory](/source/group-theory/)).

## Examples

**Example 1** ([group-theory](/source/group-theory/)): The Rubik's Cube puzzle illustrates permutation groups -- each sequence of moves is a permutation of the cube's state, and the set of all achievable states forms a group.

**Example 2** ([group-theory](/source/group-theory/)): The circle of fifths in music can be endowed with a cyclic group structure due to its 12-periodicity, enabling applications of group theory to musical set theory.

**Example 3** ([group-theory](/source/group-theory/)): Caesar's cipher can be interpreted as a group operation on the cyclic group $\mathbb{Z}_{26}$.

## Relationships

- **Related to symmetry-group**: group theory provides the formal study of symmetry groups.
- **Related to permutation-group**: permutation groups were the first class of groups to be systematically studied.
- **Related to finite-group**: finite group theory is a major branch, culminating in the classification of finite simple groups.
- **Related to lie-group**: Lie groups represent continuous symmetry, a major branch of group theory.
- **Related to representation-theory**: representation theory studies how groups act on vector spaces.

## Common Confusions

- **Group theory vs. group** -- group theory is the study of groups as a discipline; a group (mathematics) is the algebraic structure itself. One is a field of study, the other is its object.
- **Group theory vs. abstract algebra** -- group theory is a subfield of abstract algebra. Abstract algebra also encompasses rings, fields, modules, and other structures.

## Source Reference

Derived from:
- [group-theory](/source/group-theory/) -- provides the definition, historical context, branches, applications, and examples.

## Verification Notes

- Definition: directly adapted from the opening paragraph of [group-theory](/source/group-theory/)
- Key Properties: all items explicit or directly synthesized from the source
- Historical context: drawn from the History section
- Confidence: HIGH -- the source page is entirely devoted to this concept
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
