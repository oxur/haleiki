---
# === CORE IDENTIFICATION ===
concept: "Pattern Matching"
slug: "pattern-matching-rust"
page_type: "concept"
disambiguator: "Rust"

# === CLASSIFICATION ===
category: "programming-languages"
subcategory: "control-flow"
tier: "intermediate"
subject_headings:
  - "programming-paradigms"
keywords:
  - "match"
  - "if let"
  - "destructuring"
  - "enum"

# === PROVENANCE ===
derived_from:
  - slug: "rust-language"
    title: "Rust (programming language)"

# === CONFIDENCE ===
extraction_confidence: "medium"

# === VARIANTS ===
aliases:
  - "Rust pattern matching"
  - "match expressions"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "traits-rust"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is ownership in Rust?"

# === LIFECYCLE ===
status: "published"
stable_since: "1.0"
last_reviewed: "2026-05-03"
---

## Quick Definition

Pattern matching in Rust is a control flow mechanism using `match` and `if let` expressions that allows destructuring values and branching based on their structure. It is exhaustive: the compiler ensures all possible cases are handled.

## Core Definition

The `match` and `if let` expressions can be used for pattern matching in Rust. A `match` expression compares a value against a series of patterns and executes the code associated with the first matching pattern. Rust's pattern matching is exhaustive: the compiler requires that all possible values are accounted for ([rust-language](/source/rust-language/)). The `if let` form provides a more concise syntax when only one pattern is of interest. Pattern matching is widely used with Rust's `Option` and `Result` types, which represent optional values and error handling respectively ([rust-language](/source/rust-language/)).

## Prerequisites

This is a foundational concept for Rust's control flow with no prerequisites within the wiki.

## Key Properties

1. **Exhaustiveness** -- the compiler ensures all cases are handled in a `match` expression; unhandled patterns cause a compile error.
2. **Destructuring** -- patterns can destructure enums, tuples, structs, and references to bind their inner values to variables.
3. **Two forms** -- `match` for multi-arm matching, `if let` for single-pattern matching with an optional `else`.
4. **Expression-based** -- both `match` and `if let` are expressions that return values, enabling their use in assignments.
5. **Integration with `Option` and `Result`** -- pattern matching is the idiomatic way to handle `Option<T>` (Some/None) and `Result<T, E>` (Ok/Err) values ([rust-language](/source/rust-language/)).

## Context & Application

Pattern matching in Rust was influenced by functional programming languages such as OCaml, Haskell, and Standard ML. It replaces many uses of if-else chains and switch statements found in C-like languages. It is central to Rust's approach to error handling: rather than exceptions, Rust uses `Result` and `Option` types with pattern matching and the `?` operator. The `match` expression is also used with enums (algebraic data types) to implement state machines, parsers, and protocol handlers.

## Examples

**Example 1** ([rust-language](/source/rust-language/)):

```rust
fn double(x: Option<u64>) -> u64 {
    match x {
        Some(y) => y * 2,
        None => 0,
    }
}
```

**Example 2** ([rust-language](/source/rust-language/)): The `if let` form provides a more concise alternative:

```rust
fn double(x: Option<u64>) -> u64 {
    if let Some(y) = x {
        y * 2
    } else {
        0
    }
}
```

## Relationships

- **Related to traits-rust** -- pattern matching is often used with trait-implementing enum types for polymorphic behavior.

## Source Reference

Derived from:
- [rust-language](/source/rust-language/) -- provides the definition, `match` and `if let` syntax, `Option`/`Result` integration, and code examples in the "Pattern matching" and "Standard library" sections.

## Verification Notes

- Definition: synthesized from the "Pattern matching" section and "Standard library" discussion of [rust-language](/source/rust-language/)
- Key Properties: items 1, 3, 4, 5 explicit in source; item 2 is standard Rust knowledge consistent with source examples
- Code examples: both taken from the source page
- Confidence: MEDIUM -- the section presents examples but the definition of pattern matching as a concept is more implicit, requiring synthesis from the examples and surrounding context
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
