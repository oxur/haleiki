---
# === CORE IDENTIFICATION ===
concept: "Lifetimes"
slug: "lifetimes-rust"
page_type: "concept"
disambiguator: "Rust"

# === CLASSIFICATION ===
category: "programming-languages"
subcategory: "memory-model"
tier: "intermediate"
subject_headings:
  - "memory-management"
  - "type-systems"
keywords:
  - "lifetime parameters"
  - "reference validity"
  - "dangling pointer"
  - "scope"

# === PROVENANCE ===
derived_from:
  - slug: "rust-language"
    title: "Rust (programming language)"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "Rust lifetimes"
  - "lifetime annotations"
  - "lifetime parameters"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "borrowing-rust"
extends: []
related:
  - "ownership-rust"
  - "borrow-checker"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What concepts are needed to understand Rust's lifetime system?"

# === LIFECYCLE ===
status: "published"
stable_since: "1.0"
last_reviewed: "2026-05-03"
---

## Quick Definition

Lifetimes in Rust are compile-time annotations that specify how long a reference remains valid. They ensure that references never outlive the data they point to, preventing dangling pointer errors without runtime checks.

## Core Definition

Object lifetime refers to the period of time during which a reference is valid; that is, the time between the object creation and destruction. These lifetimes are implicitly associated with all Rust reference types. While often inferred, they can also be indicated explicitly with named lifetime parameters (often denoted `'a`, `'b`, and so on) ([rust-language](/source/rust-language/)). A value's lifetime in Rust is inferred from the set of locations in the source code for which a variable is valid. The borrow checker then enforces that references are only used in locations where the associated lifetime is valid ([rust-language](/source/rust-language/)).

## Prerequisites

- **Borrowing** -- lifetimes annotate references (borrows); understanding what a reference is and how borrowing works is essential to understanding lifetimes.

## Key Properties

1. **Compile-time** -- lifetimes are checked entirely at compile time with no runtime overhead.
2. **Often inferred** -- in many cases, the compiler infers lifetimes automatically (lifetime elision), so explicit annotations are not always needed.
3. **Explicit parameters** -- when the compiler cannot infer lifetimes (e.g., for functions returning references), explicit lifetime parameters like `'a` are used.
4. **Prevent dangling references** -- the borrow checker rejects programs where a reference outlives the data it points to.
5. **Work with ownership** -- ownership and lifetimes work together to prevent memory safety issues such as dangling pointers ([rust-language](/source/rust-language/)).

## Context & Application

Lifetimes are central to Rust's approach to memory safety. They allow the compiler to verify that all references are valid without needing a garbage collector or runtime checks. Lifetime annotations are most commonly needed in function signatures that return references, struct definitions containing references, and complex borrowing patterns.

## Examples

**Example 1** ([rust-language](/source/rust-language/)):

```rust
fn main() {
    let mut x = 5;            // --+- Lifetime 'a
    let r = &x;               // -+-- Lifetime 'b
    println!("r: {}", r);     // -+
    let r2 = &mut x;          // -+-- Lifetime 'c
}                             // --+
```

Variable `x` has a longer lifetime (`'a`) than reference `r` (`'b`), so the borrow is valid.

**Example 2** ([rust-language](/source/rust-language/)):

```rust
fn remove_prefix<'a>(mut original: &'a str, prefix: &str) -> &'a str {
    if original.starts_with(prefix) {
        original = original[prefix.len()..];
    }
    original
}
```

The explicit lifetime `'a` specifies that the returned reference has the same lifetime as `original`, not `prefix`.

## Relationships

- **Prerequisite: borrowing-rust** -- lifetimes annotate references, which are the mechanism of borrowing.
- **Related to ownership-rust** -- ownership determines when values are dropped; lifetimes ensure references don't outlive the owned data.
- **Related to borrow-checker** -- the borrow checker enforces lifetime validity.

## Common Errors

- **Returning a reference to a local variable** -- a function cannot return a reference to data created within the function, because the data is dropped when the function returns.
- **Lifetime mismatch** -- storing a reference with a shorter lifetime in a variable with a longer lifetime causes a compile error.

## Common Confusions

- **Lifetimes are not dynamic durations** -- lifetimes do not describe how long a value exists at runtime. They are static annotations describing the scope in the source code where a reference is valid.
- **Lifetime annotations do not change behavior** -- they do not allocate or deallocate memory. They only constrain which programs the compiler accepts.

## Source Reference

Derived from:
- [rust-language](/source/rust-language/) -- provides the definition, lifetime diagrams, explicit lifetime parameters, and the connection to the borrow checker in the "Lifetimes" subsection.

## Verification Notes

- Definition: directly adapted from the "Lifetimes" subsection of [rust-language](/source/rust-language/)
- Key Properties: all items explicit in source
- Code examples: both taken from the source page
- Confidence: HIGH -- dedicated subsection with ASCII lifetime diagrams and code examples
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
