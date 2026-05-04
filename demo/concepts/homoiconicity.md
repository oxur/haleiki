---
# === CORE IDENTIFICATION ===
concept: "Homoiconicity"
slug: "homoiconicity"
page_type: "concept"

# === CLASSIFICATION ===
category: "programming-languages"
subcategory: "language-design"
tier: "intermediate"
subject_headings:
  - "metaprogramming"
  - "functional-programming"
keywords:
  - "code as data"
  - "metaprogramming"
  - "macro systems"
  - "self-modifying"

# === PROVENANCE ===
derived_from:
  - slug: "lisp-language"
    title: "Lisp (programming language)"
  - slug: "lfe-language"
    title: "LFE (programming language)"

# === CONFIDENCE ===
extraction_confidence: "medium"

# === VARIANTS ===
aliases:
  - "code-data equivalence"
  - "code as data"
  - "homoiconic"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "s-expression"
extends: []
related:
  - "lisp"
  - "lfe"
  - "macro-system"
contrasts_with: []

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

Homoiconicity is a property of programming languages in which the structure of program code is represented faithfully and directly in a standard data structure, enabling programs to manipulate their own source code as data.

## Core Definition

Homoiconicity is the property whereby a language's program code is represented in the same data structures the language uses for data. Lisp was the first language where program code structure was represented faithfully and directly in a standard data structure -- a quality later dubbed "homoiconicity." Because Lisp functions are written as lists, they can be processed exactly like data, which allows easy writing of programs that manipulate other programs (metaprogramming) ([lisp-language](/source/lisp-language/)). LFE brought homoiconicity to the Erlang VM, enabling Lisp macros on the BEAM platform ([lfe-language](/source/lfe-language/)).

## Prerequisites

- **S-expression** -- homoiconicity in Lisp-family languages depends on the uniform representation of code and data as S-expressions.

## Key Properties

1. **Code-data equivalence** -- program source code is a valid data structure in the language itself.
2. **Metaprogramming enablement** -- programs can generate, analyze, and transform other programs at compile time or runtime.
3. **Macro foundation** -- homoiconicity enables syntactic macro systems that extend the language by transforming code as data before evaluation.
4. **Reflective capability** -- homoiconic languages support meta-circular evaluation, where the language's evaluator can be written in the language itself.

## Context & Application

Homoiconicity is most strongly associated with the Lisp family of languages, where all code is represented as S-expressions (nested lists). This property is what makes Lisp's macro systems possible -- macros receive unevaluated code as list data, transform it, and return new code. Many Lisp dialects exploit this feature to create domain-specific languages embedded within Lisp. Among the features LFE adds to the Erlang ecosystem, homoiconicity is primary: unlike Erlang's native syntax, LFE's Lisp syntax allows Lisp-style macros on the BEAM ([lfe-language](/source/lfe-language/)).

## Examples

**Example 1** ([lisp-language](/source/lisp-language/)): In Lisp, the expression `(+ 1 2)` is simultaneously a function call that adds 1 and 2, and a list of three elements (the symbol `+`, the number 1, and the number 2). Programs can manipulate this list as data.

**Example 2** ([lfe-language](/source/lfe-language/)): LFE's feature list explicitly includes "Metaprogramming via Lisp macros and the homoiconicity of a Lisp," enabling macro-based code generation on the Erlang VM.

## Relationships

- **Prerequisite: s-expression** -- homoiconicity in Lisp depends on the S-expression data structure.
- **Related to lisp** -- Lisp is the canonical homoiconic language family.
- **Related to lfe** -- LFE brings homoiconicity to the BEAM VM.
- **Related to macro-system** -- macros are the primary practical consequence of homoiconicity.

## Source Reference

Derived from:
- [lisp-language](/source/lisp-language/) -- provides the definition and historical context for homoiconicity
- [lfe-language](/source/lfe-language/) -- demonstrates homoiconicity's application on the BEAM platform

## Verification Notes

- Definition: synthesized from the "Language innovations" and "Syntax and semantics" sections of [lisp-language](/source/lisp-language/)
- Key Properties: items 1-3 explicit; item 4 inferred from the meta-circular evaluator discussion
- Confidence: MEDIUM -- the term "homoiconicity" appears explicitly but the definition is synthesized from surrounding context
- Uncertainties: none
