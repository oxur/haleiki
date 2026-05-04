---
# === CORE IDENTIFICATION ===
concept: "Lisp"
slug: "lisp"
page_type: "concept"

# === CLASSIFICATION ===
category: "programming-languages"
subcategory: "language-families"
tier: "foundational"
subject_headings:
  - "functional-programming"
  - "artificial-intelligence"
keywords:
  - "list processing"
  - "lambda calculus"
  - "functional programming"
  - "AI programming"

# === PROVENANCE ===
derived_from:
  - slug: "lisp-language"
    title: "Lisp (programming language)"
  - slug: "john-mccarthy"
    title: "John McCarthy (computer scientist)"
  - slug: "lfe-language"
    title: "LFE (programming language)"
  - slug: "erlang-language"
    title: "Erlang (programming language)"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "LISP"
  - "list processing"
  - "Lisp programming language"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "s-expression"
  - "homoiconicity"
  - "functional-programming"
  - "garbage-collection"
  - "john-mccarthy-computer-scientist"
  - "lfe"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is artificial intelligence?"
  - "How do Erlang and LFE relate?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Lisp is a family of programming languages with a distinctive fully parenthesized prefix notation (S-expressions), originally designed in the late 1950s by John McCarthy. It is the second-oldest high-level programming language still in common use, after Fortran.

## Core Definition

Lisp (historically LISP, an abbreviation of "list processing") is a family of programming languages with a long history and a distinctive, fully parenthesized prefix notation. Created by John McCarthy beginning in 1958 at MIT, it was originally a practical mathematical notation for computer programs influenced by Alonzo Church's lambda calculus. It quickly became the favored programming language for artificial intelligence research ([lisp-language](/source/lisp-language/)). Lisp pioneered many ideas in computer science: tree data structures, automatic storage management (garbage collection), dynamic typing, conditionals, higher-order functions, recursion, the self-hosting compiler, and the read-eval-print loop ([lisp-language](/source/lisp-language/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki.

## Key Properties

1. **S-expression syntax** -- all code and data are written as parenthesized lists using prefix notation, making Lisp homoiconic.
2. **List-based data structures** -- linked lists are a major data structure; Lisp source code is itself made of lists.
3. **Macro systems** -- the interchangeability of code and data enables powerful macro systems for creating new syntax and domain-specific languages.
4. **Garbage collection** -- Lisp introduced automatic garbage collection, invented by McCarthy around 1959.
5. **Dynamic typing** -- values carry type information at runtime; variables are not statically typed.
6. **Lambda expressions** -- functions are first-class values, created with lambda, following Church's notation.
7. **Dialect diversity** -- major modern dialects include Common Lisp, Scheme, Racket, Clojure, and LFE.

## Context & Application

McCarthy began developing Lisp in 1958 at MIT, motivated by a desire for an AI programming language that would work on the IBM 704 ([john-mccarthy](/source/john-mccarthy/)). Steve Russell implemented the first Lisp interpreter by compiling McCarthy's eval function into IBM 704 machine code. Lisp was the first language to implement the eval function, which blurred the line between language and interpreter. The language influenced many subsequent languages including Python, Ruby, JavaScript, Haskell, and Erlang ([lisp-language](/source/lisp-language/)). Erlang was directly influenced by Lisp, and LFE (Lisp Flavoured Erlang) is a Lisp dialect that runs on the Erlang BEAM virtual machine ([erlang-language](/source/erlang-language/), [lfe-language](/source/lfe-language/)).

## Examples

**Example 1** ([lisp-language](/source/lisp-language/)): The expression `(+ 1 2 3 4)` evaluates to 10, demonstrating prefix notation with variadic arithmetic operators.

**Example 2** ([lisp-language](/source/lisp-language/)): `(defun foo (a b c d) (+ a b c d))` defines a named function, showing how lambda expressions are stored in symbols.

**Example 3** ([john-mccarthy](/source/john-mccarthy/)): McCarthy published Lisp's design in "Recursive Functions of Symbolic Expressions and Their Computation by Machine, Part I" (Communications of the ACM, 1960), showing that with a few simple operators and anonymous functions, one can build a Turing-complete language.

## Relationships

- **Related to s-expression** -- S-expressions are Lisp's defining notation for code and data.
- **Related to homoiconicity** -- Lisp's code-as-data property, enabled by S-expressions.
- **Related to garbage-collection** -- Lisp introduced automatic garbage collection.
- **Related to john-mccarthy-computer-scientist** -- McCarthy invented Lisp.
- **Related to lfe** -- LFE is a modern Lisp dialect on the Erlang VM.

## Source Reference

Derived from:
- [lisp-language](/source/lisp-language/) -- provides the comprehensive definition, history, language innovations, dialect taxonomy, and syntax
- [john-mccarthy](/source/john-mccarthy/) -- provides biographical context for the invention of Lisp and its connection to AI research
- [lfe-language](/source/lfe-language/) -- confirms Lisp's influence as a language family via a modern BEAM-targeting dialect
- [erlang-language](/source/erlang-language/) -- documents Lisp as an influence on Erlang's design

## Verification Notes

- Definition: directly from the opening of [lisp-language](/source/lisp-language/)
- Key Properties: all items explicit in the source's "Language innovations" section (Paul Graham's nine aspects)
- Multi-source merge: john-mccarthy adds inventor context; lfe-language and erlang-language add dialect and influence context
- Confidence: HIGH -- Lisp is explicitly and extensively defined in the source
- Uncertainties: none
