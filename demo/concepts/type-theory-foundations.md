---
# === CORE IDENTIFICATION ===
concept: "Type Theory"
slug: "type-theory-foundations"
page_type: "concept"
disambiguator: "foundations"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "foundations"
tier: "intermediate"
subject_headings:
  - "foundations-of-mathematics"
  - "computer-science"
keywords:
  - "types"
  - "lambda calculus"
  - "proof assistants"
  - "Martin-Lof"
  - "propositions as types"

# === PROVENANCE ===
derived_from:
  - slug: "type-theory"
    title: "Type theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "theory of types"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "lambda-calculus"
extends: []
related:
  - "curry-howard-correspondence"
  - "dependent-type"
  - "intuitionistic-type-theory"
  - "homotopy-type-theory-concept"
  - "category-theory-mathematics"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is homotopy type theory?"
  - "What background is needed to understand homotopy type theory?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Type theory is the study of formal systems that classify expressions by their types, playing a role similar to data types in programming. It serves as a foundation for constructive mathematics, a basis for proof assistants, and a bridge between logic and computation.

## Core Definition

In mathematics and theoretical computer science, type theory is the study of formal systems that classify expressions or mathematical objects by their types. A type plays a role similar to a data type in programming: it specifies what kind of thing an expression is and how it may be used. Type theories are used in programming languages (type systems), formal logic, and the formalization of mathematics. Some type theories, such as Martin-Lof's intuitionistic type theory, have been proposed as alternatives to set theory as a foundation of mathematics ([type-theory](/source/type-theory/)).

## Prerequisites

- **Lambda calculus** -- type theory is built on lambda calculus; the typed lambda calculus is the simplest type theory.

## Key Properties

1. **Types classify terms** -- every term belongs to a type, restricting how it may be used ([type-theory](/source/type-theory/)).
2. **Rules-based** -- type theories are defined by inference rules, not axioms (unlike set theory) ([type-theory](/source/type-theory/)).
3. **Constructive** -- most type theories are constructive (intuitionistic), requiring witnesses for existence proofs ([type-theory](/source/type-theory/)).
4. **Curry-Howard correspondence** -- types correspond to propositions, terms to proofs, providing "proofs as programs" ([type-theory](/source/type-theory/)).
5. **Foundation potential** -- type theory can serve as a foundation for mathematics, alternative to ZFC set theory ([type-theory](/source/type-theory/)).
6. **Dependent types** -- types that depend on terms enable encoding of quantifiers and sophisticated specifications ([type-theory](/source/type-theory/)).

## Context & Application

Type theory was created to avoid paradoxes in naive set theory and formal logic, starting with Russell's ramified theory of types (1908). Church's simply typed lambda calculus (1940) connected types to lambda calculus. Martin-Lof developed intuitionistic type theory in the 1970s as a constructive foundation for mathematics. Coquand's calculus of constructions underlies proof assistants like Coq/Rocq and Lean. Homotopy type theory (2010s) is an active research frontier connecting type theory to algebraic topology ([type-theory](/source/type-theory/)).

## Examples

**Example 1** ([type-theory](/source/type-theory/)): In a type theory with natural numbers, $42 : \mathsf{nat}$ asserts that 42 is a term of type nat, and $\mathrm{add} : \mathsf{nat} \to (\mathsf{nat} \to \mathsf{nat})$ asserts that add takes two natural numbers and returns a natural number.

**Example 2** ([type-theory](/source/type-theory/)): The lambda term $(\lambda x.\mathrm{add}\,x\,x) : \mathsf{nat} \to \mathsf{nat}$ defines an anonymous function that doubles its input.

## Relationships

- **Related to curry-howard-correspondence**: the fundamental bridge between logic and type theory.
- **Related to dependent-type**: dependent types extend type theory with types parameterized by terms.
- **Related to intuitionistic-type-theory**: Martin-Lof's type theory is the foundation for constructive mathematics.
- **Related to homotopy-type-theory-concept**: HoTT interprets types as homotopy types.
- **Related to category-theory-mathematics**: categories and type theories are deeply connected (Lambek correspondence).

## Source Reference

Derived from:
- [type-theory](/source/type-theory/) -- provides the definition, history, logic, definitions, and connections to foundations.

## Verification Notes

- Definition: directly from the opening of [type-theory](/source/type-theory/)
- Key Properties: all drawn from explicit source content
- Confidence: HIGH -- the source page is entirely devoted to type theory
- Cross-references: verified
- Uncertainties: none
