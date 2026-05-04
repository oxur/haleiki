---
# === CORE IDENTIFICATION ===
concept: "Borrowing"
slug: "borrowing-rust"
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
  - "references"
  - "shared reference"
  - "mutable reference"
  - "borrow"

# === PROVENANCE ===
derived_from:
  - slug: "rust-language"
    title: "Rust (programming language)"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "borrow"
  - "Rust borrowing"
  - "reference types"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "ownership-rust"
extends: []
related:
  - "lifetimes-rust"
  - "borrow-checker"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is borrowing in Rust?"
  - "What distinguishes ownership from borrowing in Rust?"
  - "How does the borrow checker enforce memory safety in Rust?"

# === LIFECYCLE ===
status: "published"
stable_since: "1.0"
last_reviewed: "2026-05-03"
---

## Quick Definition

Borrowing in Rust allows temporary access to a value through references without transferring ownership. Shared references (`&T`) allow read-only access, while mutable references (`&mut T`) allow modification, but Rust enforces that at most one mutable reference or any number of shared references exist at a time.

## Core Definition

Values can be borrowed, meaning they are temporarily passed to a different function before being returned to the owner. The `&` operator creates a reference (shared borrow), and `&mut` creates a mutable reference. Rust's type system separates shared, immutable references of the form `&T` from unique, mutable references of the form `&mut T`. A mutable reference can be coerced to an immutable reference, but not vice versa ([rust-language](/source/rust-language/)). The built-in reference types using the `&` symbol do not involve run-time reference counting; the safety and validity of the underlying pointers is verified at compile time, preventing dangling pointers and other forms of undefined behavior ([rust-language](/source/rust-language/)).

## Prerequisites

- **Ownership** -- borrowing is defined in terms of the ownership system; a borrow temporarily provides access to a value without transferring its ownership.

## Key Properties

1. **Shared references (`&T`)** -- allow read-only access; multiple shared references can coexist.
2. **Mutable references (`&mut T`)** -- allow read-write access; at most one mutable reference can exist at a time, and no shared references can coexist with it.
3. **No runtime cost** -- references do not involve reference counting; all checking is done at compile time ([rust-language](/source/rust-language/)).
4. **Non-null guarantee** -- the `&` and `&mut` reference types are guaranteed to not be null and to point to valid memory ([rust-language](/source/rust-language/)).
5. **Temporary access** -- a borrow does not transfer ownership; the original owner retains responsibility for the value.

## Context & Application

Borrowing works together with ownership and lifetimes to form Rust's memory safety model. It allows functions to access data without taking ownership, enabling patterns like passing a large data structure to a function without copying it. The distinction between shared and mutable references prevents data races at compile time, making concurrent programming safer ([rust-language](/source/rust-language/)).

## Examples

**Example 1** ([rust-language](/source/rust-language/)):

```rust
fn print_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello, World");
    print_string(&s); // s borrowed by print_string
    print_string(&s); // s has not been consumed; we can call the function many times
}
```

## Relationships

- **Prerequisite: ownership-rust** -- borrowing is an alternative to moving ownership, so understanding ownership is essential.
- **Related to lifetimes-rust** -- lifetimes specify how long a reference (borrow) remains valid.
- **Related to borrow-checker** -- the borrow checker enforces the rules of borrowing at compile time.

## Common Errors

- **Attempting to mutate through a shared reference** -- `&T` references are read-only; attempting to modify through one is a compile error.
- **Holding a mutable and shared reference simultaneously** -- the compiler rejects code that creates both `&T` and `&mut T` references to the same value at the same time.

## Common Confusions

- **Borrowing is not the same as copying** -- a borrow creates a reference to the original data, not a duplicate. The original data remains in place with its original owner.
- **References are not pointers in the C sense** -- Rust references are guaranteed safe (non-null, pointing to valid memory) and are checked at compile time.

## Source Reference

Derived from:
- [rust-language](/source/rust-language/) -- provides the borrowing rules, the distinction between `&T` and `&mut T`, compile-time checking, and code examples in the "Ownership and references" and "Memory management" sections.

## Verification Notes

- Definition: directly adapted from the "Ownership and references" section of [rust-language](/source/rust-language/)
- Key Properties: all items explicit in source
- Code example: taken directly from the source page
- Confidence: HIGH -- explicit definitions with code examples
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
