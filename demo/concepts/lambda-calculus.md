---
# === CORE IDENTIFICATION ===
concept: "Lambda Calculus"
slug: "lambda-calculus"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "foundations"
tier: "intermediate"
subject_headings:
  - "foundations-of-mathematics"
  - "computer-science"
keywords:
  - "Church"
  - "anonymous functions"
  - "beta reduction"
  - "computation"

# === PROVENANCE ===
derived_from:
  - slug: "type-theory"
    title: "Type theory"

# === CONFIDENCE ===
extraction_confidence: "medium"

# === VARIANTS ===
aliases:
  - "$\\lambda$-calculus"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "type-theory-foundations"
  - "curry-howard-correspondence"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What background is needed to understand homotopy type theory?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

The lambda calculus is a formal system for expressing computation based on function abstraction and application. Developed by Alonzo Church in the 1930s, it is the foundation of functional programming and the computational substrate on which type theory is built.

## Core Definition

Type theory is particularly popular in conjunction with Alonzo Church's lambda calculus. Church's simply typed lambda calculus was one of the earliest type theories and helped avoid paradoxes that afflicted the untyped lambda calculus. In the modern literature, "type theory" refers to a typed system based around lambda calculus ([type-theory](/source/type-theory/)). Lambda terms have the form $(\lambda v.t)$ where $v$ is a formal variable and $t$ is a term; the type of such a term is $\sigma \to \tau$ where $\sigma$ is the type of $v$ and $\tau$ is the type of $t$ ([type-theory](/source/type-theory/)). Computation proceeds by $\beta$-reduction: $(\lambda v.t)\,s \to t[v := s]$ ([type-theory](/source/type-theory/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki.

## Key Properties

1. **Function abstraction** -- $(\lambda v.t)$ creates an anonymous function from variable $v$ and body $t$ ([type-theory](/source/type-theory/)).
2. **Function application** -- applying a function to an argument produces a result ([type-theory](/source/type-theory/)).
3. **Beta-reduction** -- the computation rule: $(\lambda v.t)\,s \to t[v := s]$ ([type-theory](/source/type-theory/)).
4. **Eta-reduction** -- $(\lambda v.t\,v) \to t$ when $v$ is not free in $t$ ([type-theory](/source/type-theory/)).
5. **Foundation of type theory** -- typed lambda calculi are the computational substrate for all modern type theories ([type-theory](/source/type-theory/)).

## Context & Application

Church developed the lambda calculus in the 1930s as a formal system for studying functions and computability. It is equivalent in computational power to Turing machines. The simply typed lambda calculus is the basis for functional programming languages (ML, Haskell, etc.) and for proof assistants. Type theory adds a type discipline to lambda calculus to ensure safety and logical consistency ([type-theory](/source/type-theory/)).

## Examples

**Example 1** ([type-theory](/source/type-theory/)): The lambda term $(\lambda x.\mathrm{add}\,x\,x) : \mathsf{nat} \to \mathsf{nat}$ defines an anonymous function that doubles its natural number input.

**Example 2** ([type-theory](/source/type-theory/)): Beta-reduction in action: $(\lambda x.\mathrm{add}\,x\,x)\,2 \to \mathrm{add}\,2\,2$.

## Relationships

- **Related to type-theory-foundations**: type theory adds types to the lambda calculus.
- **Related to curry-howard-correspondence**: the lambda calculus is the computational side of the correspondence.

## Source Reference

Derived from:
- [type-theory](/source/type-theory/) -- provides lambda terms, reduction rules, and the role of lambda calculus in type theory.

## Verification Notes

- Definition: synthesized from the History and Definitions sections of [type-theory](/source/type-theory/)
- Confidence: MEDIUM -- the source discusses lambda calculus extensively but as part of the type theory article, not as a standalone definition
- Cross-references: verified
- Uncertainties: no dedicated source page for lambda calculus in this corpus
