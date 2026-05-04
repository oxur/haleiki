---
# === CORE IDENTIFICATION ===
concept: "Curry-Howard Correspondence"
slug: "curry-howard-correspondence"
page_type: "concept"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "foundations"
tier: "intermediate"
subject_headings:
  - "foundations-of-mathematics"
  - "computer-science"
keywords:
  - "proofs as programs"
  - "propositions as types"
  - "lambda calculus"
  - "logic"

# === PROVENANCE ===
derived_from:
  - slug: "type-theory"
    title: "Type theory"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "Curry-Howard isomorphism"
  - "proofs-as-programs"
  - "propositions-as-types"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "type-theory-foundations"
extends: []
related:
  - "lambda-calculus"
  - "intuitionistic-type-theory"
  - "category-theory-mathematics"
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

The Curry-Howard correspondence is the observed structural similarity between logical proofs and computer programs: propositions correspond to types, and proofs correspond to programs (terms) inhabiting those types. It is often summarized as "proofs as programs."

## Core Definition

The Curry-Howard correspondence is the observed similarity between logics and programming languages. The implication "$A \to B$" in logic resembles a function from type "$A$" to type "$B$" in a programming language. For a variety of logics, the rules are similar to expressions in a programming language's type system, and applications of the rules resemble programs. Thus the correspondence is summarized as "proofs as programs" ([type-theory](/source/type-theory/)).

## Prerequisites

- **Type theory** -- the correspondence connects type theory (as a formal system) to logic.

## Key Properties

1. **Propositions as types** -- logical propositions correspond to types ([type-theory](/source/type-theory/)).
2. **Proofs as programs** -- a proof of a proposition corresponds to a term inhabiting the corresponding type ([type-theory](/source/type-theory/)).
3. **Logic-computation bridge** -- connects formal logic, type theory, and programming languages ([type-theory](/source/type-theory/)).
4. **Extends to categories** -- the correspondence extends to category theory (Curry-Howard-Lambek), where types correspond to objects and proofs to morphisms ([type-theory](/source/type-theory/)).

## Context & Application

The correspondence was observed independently by Haskell Curry (1934) and William Alvin Howard (1969). It underlies modern proof assistants (Coq/Rocq, Lean, Agda), where proving a theorem is the same as constructing a term of a given type. The correspondence has been extended to include category theory (Lambek's observation) and is the theoretical foundation for the connection between logic and functional programming ([type-theory](/source/type-theory/)).

## Examples

**Example 1** ([type-theory](/source/type-theory/)): Under the correspondence, the logical connective "and" ($A \wedge B$) corresponds to the product type ($A \times B$), and "or" ($A \lor B$) corresponds to the sum type ($A + B$).

**Example 2** ([type-theory](/source/type-theory/)): The universal quantifier $\forall a \in A, P(a)$ corresponds to the dependent product type $\Pi a:A.P(a)$.

## Relationships

- **Related to lambda-calculus**: the typed lambda calculus is the programming-language side of the correspondence.
- **Related to intuitionistic-type-theory**: Martin-Lof type theory embodies the correspondence as its design principle.
- **Related to category-theory-mathematics**: the Curry-Howard-Lambek correspondence extends to categories.

## Source Reference

Derived from:
- [type-theory](/source/type-theory/) -- provides the definition, logic-type correspondence table, and extensions.

## Verification Notes

- Definition: from the "Curry-Howard correspondence" section of [type-theory](/source/type-theory/)
- Confidence: HIGH -- dedicated section with detailed treatment
- Cross-references: verified
- Uncertainties: none
