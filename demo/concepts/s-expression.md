---
# === CORE IDENTIFICATION ===
concept: "S-expression"
slug: "s-expression"
page_type: "concept"

# === CLASSIFICATION ===
category: "programming-languages"
subcategory: "syntax"
tier: "foundational"
subject_headings:
  - "functional-programming"
  - "data-structures"
keywords:
  - "symbolic expression"
  - "parenthesized notation"
  - "prefix notation"
  - "homoiconicity"
  - "list notation"

# === PROVENANCE ===
derived_from:
  - slug: "lisp-language"
    title: "Lisp (programming language)"
  - slug: "lfe-language"
    title: "LFE (programming language)"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "symbolic expression"
  - "sexp"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "lisp"
  - "homoiconicity"
  - "cons-cell"
contrasts_with:
  - "m-expression"

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is artificial intelligence?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

An S-expression (symbolic expression) is Lisp's notation for representing both code and data as parenthesized lists using prefix notation. The interchangeability of code and data via S-expressions is a defining characteristic of the Lisp language family.

## Core Definition

S-expressions are the fundamental notation in Lisp for representing both program code and data structures as parenthesized lists. A function call or syntactic form is written as a list with the function or operator name first, followed by arguments -- for instance, `(f arg1 arg2 arg3)`. McCarthy's 1958 paper introduced S-expressions to mirror the internal representation of code and data, alongside M-expressions (meta expressions) for expressing functions; programmers rapidly chose S-expressions and M-expressions were abandoned ([lisp-language](/source/lisp-language/)). LFE brought S-expression syntax and homoiconicity to the Erlang VM ([lfe-language](/source/lfe-language/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki.

## Key Properties

1. **Parenthesized prefix notation** -- all expressions are written as lists with the operator first: `(+ 1 2 3)` evaluates to 6.
2. **Code-data equivalence** -- Lisp programs are themselves S-expressions, allowing programs to manipulate source code as data (homoiconicity).
3. **Recursive structure** -- S-expressions can be nested: `(list 1 2 (list 3 4))` evaluates to `(1 2 (3 4))`.
4. **Atom and list duality** -- an S-expression is either an atom (number, symbol) or a list of S-expressions.
5. **Macro enablement** -- the structural regularity of S-expressions enables Lisp's macro systems, allowing programmers to extend the language.

## Context & Application

S-expressions were introduced by John McCarthy in 1958 as part of Lisp's design. They provide a uniform notation that eliminates the distinction between expressions and statements found in most other languages. The simplicity and consistency of S-expression syntax facilitates metaprogramming: programs that write or transform other programs. This capability has made S-expressions influential far beyond Lisp, appearing in configuration formats, data serialization, and other domains ([lisp-language](/source/lisp-language/)).

## Examples

**Example 1** ([lisp-language](/source/lisp-language/)): `(list 1 2 (quote foo))` evaluates to the list `(1 2 foo)`, demonstrating prefix notation and quoting.

**Example 2** ([lisp-language](/source/lisp-language/)): `((lambda (arg) (+ arg 1)) 5)` evaluates to 6, showing that anonymous functions are also S-expressions.

**Example 3** ([lfe-language](/source/lfe-language/)): LFE uses S-expression syntax for Erlang VM code, making no syntactic distinction between expressions and statements.

## Relationships

- **Related to lisp** -- S-expressions are Lisp's defining syntactic form.
- **Related to homoiconicity** -- S-expressions enable homoiconicity by making code and data share the same representation.
- **Contrasts with m-expression** -- M-expressions were McCarthy's alternative notation that was ultimately abandoned.

## Source Reference

Derived from:
- [lisp-language](/source/lisp-language/) -- provides the definition, history (McCarthy 1958), examples, and explanation of code-data equivalence
- [lfe-language](/source/lfe-language/) -- confirms S-expression use on the BEAM VM and its role in enabling homoiconicity for Erlang

## Verification Notes

- Definition: directly from the "Symbolic expressions" section of [lisp-language](/source/lisp-language/)
- Key Properties: items 1-4 explicit in source; item 5 synthesized from the macro discussion
- Multi-source merge: LFE source adds the cross-platform angle (S-expressions on BEAM) without changing the concept definition
- Confidence: HIGH -- S-expressions are explicitly defined with numerous examples
- Uncertainties: none
