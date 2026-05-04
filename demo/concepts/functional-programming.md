---
concept: "Functional Programming"
slug: "functional-programming"
page_type: "concept"
category: "programming-languages"
subcategory: "paradigms"
tier: "foundational"
subject_headings:
  - "programming-paradigms"
keywords:
  - "immutability"
  - "higher-order functions"
  - "recursion"
  - "first-class functions"
derived_from:
  - slug: "lisp-language"
    title: "Lisp (programming language)"
  - slug: "erlang-language"
    title: "Erlang (programming language)"
  - slug: "lfe-language"
    title: "LFE (programming language)"
extraction_confidence: "medium"
aliases:
  - "FP"
  - "functional paradigm"
prerequisites: []
extends: []
related:
  - "lisp"
  - "erlang"
  - "lfe"
  - "s-expression"
contrasts_with: []
trust_level: "credible"
answers_questions:
  - "How do Erlang and LFE relate?"
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Functional programming is a programming paradigm that treats computation as the evaluation of mathematical functions, emphasizing immutability, first-class functions, recursion, and the avoidance of side effects. Lisp and Erlang are prominent functional languages.

## Core Definition

Functional programming is a paradigm in which programs are constructed by applying and composing functions. Lisp pioneered many functional programming concepts including higher-order functions, recursion, dynamic typing, and first-class functions ([lisp-language](/source/lisp-language/)). Erlang is a concurrent functional language supporting eager evaluation, single assignment, and recursion and higher-order functions instead of side-effect-based looping ([erlang-language](/source/erlang-language/)). LFE emphasizes recursion and higher-order functions instead of side-effect-based looping ([lfe-language](/source/lfe-language/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki.

## Key Properties

1. **First-class functions** -- functions are values that can be passed as arguments, returned from other functions, and stored in data structures.
2. **Higher-order functions** -- functions that take other functions as arguments or return functions as results.
3. **Recursion** -- the primary mechanism for iteration, replacing imperative loops.
4. **Immutability** -- data is not modified in place; new values are created from existing ones.
5. **Expression-oriented** -- programs are composed of expressions that evaluate to values, rather than statements that perform actions.

## Context & Application

Lisp, created in 1958, was the first programming language to make functional programming practical, introducing lambda expressions borrowed from Church's lambda calculus. McCarthy's 1960 paper showed that a Turing-complete language for algorithms could be built from a few simple operators and anonymous functions ([lisp-language](/source/lisp-language/)). Erlang applies functional programming principles to concurrent systems, where immutability and message passing eliminate shared-state bugs ([erlang-language](/source/erlang-language/)).

## Examples

**Example 1** ([lisp-language](/source/lisp-language/)): Lambda expression: `(lambda (arg) (+ arg 1))` creates an anonymous function that increments its argument.

**Example 2** ([erlang-language](/source/erlang-language/)): Higher-order function usage in quicksort: `qsort(Lists, fun(A,B) -> length(A) < length(B) end)` passes an anonymous comparison function.

## Relationships

- **Related to lisp** -- Lisp pioneered functional programming.
- **Related to erlang** -- Erlang applies functional programming to concurrent systems.
- **Related to lfe** -- LFE combines Lisp's functional syntax with Erlang's runtime.

## Source Reference

Derived from:
- [lisp-language](/source/lisp-language/) -- provides historical context and language innovations list
- [erlang-language](/source/erlang-language/) -- provides concurrent functional programming examples
- [lfe-language](/source/lfe-language/) -- confirms functional emphasis

## Verification Notes

- Definition: synthesized from multiple sources' descriptions of functional features
- Confidence: MEDIUM -- the concept is described through its features rather than explicitly defined
- Uncertainties: none
