---
# === CORE IDENTIFICATION ===
concept: "Abelian Group"
slug: "abelian-group"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "algebra"
tier: "intermediate"
subject_headings:
  - "abstract-algebra"
keywords:
  - "commutative group"
  - "Abel"
  - "commutativity"

# === PROVENANCE ===
derived_from:
  - slug: "group-mathematics"
    title: "Group (mathematics)"
  - slug: "abstract-algebra"
    title: "Abstract algebra"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "commutative group"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "group-mathematics"
extends:
  - "group-mathematics"
related:
  - "ring-mathematics"
  - "group-theory-mathematics"
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

An abelian group is a group whose binary operation is commutative -- the order in which two elements are combined does not affect the result. Named after Niels Henrik Abel, abelian groups are the building blocks of ring and module theory.

## Core Definition

A group is called abelian (or commutative) if the group operation satisfies $a \cdot b = b \cdot a$ for all elements $a$ and $b$ ([group-mathematics](/source/group-mathematics/)). In an abelian group, either additive or multiplicative notation may be used ([group-mathematics](/source/group-mathematics/)). Gauss's study of multiplicative groups of residues mod $n$ and Kronecker's 1870 definition of abelian groups in the context of ideal class groups are early milestones ([abstract-algebra](/source/abstract-algebra/)).

## Prerequisites

- **Group (mathematics)** -- an abelian group is a group with an additional commutativity axiom.

## Key Properties

1. **Commutativity** -- $a \cdot b = b \cdot a$ for all elements ([group-mathematics](/source/group-mathematics/)).
2. **Additive notation** -- abelian groups are often written with $+$ as the operation, $0$ as identity, and $-a$ as inverse ([group-mathematics](/source/group-mathematics/)).
3. **Fundamental building block** -- the additive structure of every ring is an abelian group ([abstract-algebra](/source/abstract-algebra/)).
4. **Classification** -- finitely generated abelian groups are classified by the fundamental theorem: they decompose as direct sums of cyclic groups.

## Context & Application

Abelian group structures appeared implicitly in Gauss's Disquisitiones Arithmeticae (1798) and were made explicit by Kronecker (1870). Abelian groups are ubiquitous: the integers under addition, vector spaces under addition, and the additive groups of rings are all abelian. The theory of abelian groups is considerably simpler than that of non-abelian groups ([abstract-algebra](/source/abstract-algebra/)).

## Examples

**Example 1** ([group-mathematics](/source/group-mathematics/)): The integers $\mathbb{Z}$ with addition form an abelian group: $a + b = b + a$ for all integers.

**Example 2** ([abstract-algebra](/source/abstract-algebra/)): Cyclic groups $\mathbb{Z}_n$ (integers modulo $n$ under addition) are abelian groups that arise in number theory and cryptography.

## Relationships

- **Extends group-mathematics**: an abelian group is a group with the added commutativity axiom.
- **Related to ring-mathematics**: the additive structure of every ring is an abelian group.
- **Related to group-theory-mathematics**: abelian group theory is a major sub-discipline.

## Source Reference

Derived from:
- [group-mathematics](/source/group-mathematics/) -- defines abelian groups and distinguishes them from non-abelian groups
- [abstract-algebra](/source/abstract-algebra/) -- provides historical context (Gauss, Kronecker)

## Verification Notes

- Definition: from the Notation and terminology section of [group-mathematics](/source/group-mathematics/)
- Confidence: HIGH -- explicitly defined in the source
- Cross-references: verified
- Uncertainties: none
