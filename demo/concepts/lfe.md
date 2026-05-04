---
concept: "LFE"
slug: "lfe"
page_type: "concept"
category: "programming-languages"
subcategory: "functional"
tier: "advanced"
subject_headings:
  - "concurrency"
  - "functional-programming"
keywords:
  - "Lisp Flavoured Erlang"
  - "BEAM"
  - "Robert Virding"
  - "Lisp dialect"
derived_from:
  - slug: "lfe-language"
    title: "LFE (programming language)"
extraction_confidence: "high"
aliases:
  - "Lisp Flavoured Erlang"
  - "Lisp Flavored Erlang"
prerequisites:
  - "erlang"
  - "lisp"
extends:
  - "erlang"
  - "lisp"
related:
  - "beam-vm"
  - "homoiconicity"
  - "s-expression"
  - "actor-model"
contrasts_with: []
trust_level: "credible"
answers_questions:
  - "How do Erlang and LFE relate?"
  - "What is the BEAM virtual machine?"
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

LFE (Lisp Flavoured Erlang) is a functional, concurrent programming language and Lisp dialect built on Core Erlang and the BEAM virtual machine, combining Lisp's macro system and S-expression syntax with Erlang's concurrency and fault-tolerance.

## Core Definition

LFE is a Lisp dialect that compiles to Core Erlang and runs on the BEAM virtual machine. Designed by Robert Virding (co-creator of Erlang) and first released in 2008, LFE provides Lisp syntax for writing distributed, fault-tolerant, soft real-time applications. It extends Erlang with metaprogramming via Lisp macros and homoiconicity, and provides a feature-rich REPL that supports function and macro definitions ([lfe-language](/source/lfe-language/)).

## Prerequisites

- **Erlang** -- LFE compiles to Core Erlang and runs on the BEAM; understanding Erlang's runtime model is essential.
- **Lisp** -- LFE uses Lisp syntax (S-expressions), macros, and homoiconicity; understanding the Lisp language family provides context.

## Key Properties

1. **Zero-penalty Erlang interop** -- seamless, zero-cost function calls between LFE and Erlang code in both directions.
2. **Lisp macros** -- full metaprogramming via Lisp-style macros, enabled by homoiconicity.
3. **BEAM concurrency** -- inherits Erlang's lightweight processes, message passing, and fault tolerance.
4. **Pattern matching** -- supports Erlang-style pattern matching within Lisp syntax.
5. **Lisp-2 namespace** -- separate namespaces for variables and functions, following Common Lisp convention.
6. **Feature-rich REPL** -- unlike Erlang's shell, the LFE REPL supports function and macro definitions.

## Context & Application

Robert Virding began LFE in 2007, motivated by prior Lisp experience, curiosity about compiling alternative languages to Core Erlang, and the goal of creating a Lisp specifically designed for the BEAM. LFE influenced other BEAM languages (Joxa, Concurrent Schemer) and was itself influenced by Common Lisp, Maclisp, Scheme, Elixir, and Clojure ([lfe-language](/source/lfe-language/)).

## Examples

**Example 1** ([lfe-language](/source/lfe-language/)): LFE appears in the timeline of Lisp dialects alongside Common Lisp, Scheme, Clojure, and Racket, positioning it within the broader Lisp family tree.

## Relationships

- **Extends erlang** -- LFE adds Lisp syntax and macros to Erlang's capabilities.
- **Extends lisp** -- LFE is a Lisp dialect targeting a novel platform (BEAM).
- **Related to beam-vm** -- LFE compiles to BEAM bytecode.
- **Related to homoiconicity** -- LFE brings homoiconicity to the Erlang ecosystem.

## Source Reference

Derived from:
- [lfe-language](/source/lfe-language/) -- provides definition, history, features, motivations, and design rationale

## Verification Notes

- Definition: directly from the opening of [lfe-language](/source/lfe-language/)
- Key Properties: all items from the "Features" section
- Confidence: HIGH -- explicit definition, feature list, and history
- Uncertainties: none
