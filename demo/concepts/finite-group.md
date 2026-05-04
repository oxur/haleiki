---
# === CORE IDENTIFICATION ===
concept: "Finite Group"
slug: "finite-group"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "algebra"
tier: "intermediate"
subject_headings:
  - "abstract-algebra"
keywords:
  - "finite"
  - "simple group"
  - "classification"
  - "solvable group"

# === PROVENANCE ===
derived_from:
  - slug: "group-theory"
    title: "Group theory"

# === CONFIDENCE ===
extraction_confidence: "medium"

# === VARIANTS ===
aliases:
  - "finite groups"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "group-theory-mathematics"
extends: []
related:
  - "permutation-group"
  - "symmetry-group"
  - "lie-group"
contrasts_with:
  - "lie-group"

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do finite groups differ from Lie groups?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

A finite group is a group with a finite number of elements. The theory of finite groups is a major branch of group theory, culminating in the complete classification of finite simple groups.

## Core Definition

Finite groups are groups whose underlying set contains finitely many elements. During the twentieth century, mathematicians investigated some aspects of the theory of finite groups in great depth, especially the local theory of finite groups and the theory of solvable and nilpotent groups. As a consequence, the complete classification of finite simple groups was achieved, meaning that all those simple groups from which all finite groups can be built are now known ([group-theory](/source/group-theory/)). Finite groups often arise when considering symmetry of mathematical or physical objects that admit just a finite number of structure-preserving transformations.

## Prerequisites

- **Group theory** -- understanding the axioms and basic vocabulary of group theory is needed to study the finite case.

## Key Properties

1. **Finite cardinality** -- the group has a finite number of elements (its order).
2. **Classification of finite simple groups** -- all finite simple groups have been classified, a collaborative effort spanning more than 10,000 journal pages published mostly between 1960 and 2004 ([group-theory](/source/group-theory/)).
3. **Building blocks** -- every finite group can be constructed from finite simple groups (via composition series), making simple groups the fundamental building blocks.
4. **Symmetry in physics and chemistry** -- finite groups model the discrete symmetries of molecules and crystals.

## Context & Application

Finite group theory has applications in physics (where finite symmetry groups describe crystal structures and molecular symmetries), chemistry (point groups and space groups), combinatorics (counting via Burnside's lemma), and cryptography (Diffie-Hellman key exchange uses finite cyclic groups). The classification of finite simple groups is considered one of the most important mathematical achievements of the 20th century ([group-theory](/source/group-theory/)).

## Examples

**Example 1** ([group-theory](/source/group-theory/)): The symmetric group $S_n$ (the group of all permutations of $n$ elements) is a finite group of order $n!$.

**Example 2** ([group-theory](/source/group-theory/)): In chemistry, the symmetry operations of a water molecule (identity and 180-degree rotation) form a finite group isomorphic to $\mathbb{Z}_2$.

## Relationships

- **Prerequisite: group-theory-mathematics** -- finite groups are a central object of study in group theory.
- **Related to permutation-group** -- many finite groups can be realized as permutation groups via Cayley's theorem.
- **Related to symmetry-group** -- finite groups model discrete symmetries.
- **Contrasts with lie-group** -- Lie groups model continuous symmetry, while finite groups model discrete symmetry. The two are connected: Lie groups have associated finite Weyl groups that influence their structure ([group-theory](/source/group-theory/)).

## Source Reference

Derived from:
- [group-theory](/source/group-theory/) -- provides the definition via the "Finite group theory" section, the classification result, and applications in physics and chemistry.

## Verification Notes

- Definition: synthesized from the "Finite group theory" subsection and the broader discussion of finite groups throughout [group-theory](/source/group-theory/)
- Key Properties: items 1-3 synthesized from context; item 4 from the chemistry/physics applications sections
- Confidence: MEDIUM -- no single explicit definition paragraph; synthesized from discussion across multiple sections
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: the source page discusses finite groups across multiple sections without a single formal definition
