---
# === CORE IDENTIFICATION ===
concept: "Ownership"
slug: "ownership-rust"
page_type: "concept"
disambiguator: "Rust"

# === CLASSIFICATION ===
category: "programming-languages"
subcategory: "memory-model"
tier: "foundational"
subject_headings:
  - "memory-management"
  - "type-systems"
keywords:
  - "move semantics"
  - "RAII"
  - "resource management"
  - "affine types"

# === PROVENANCE ===
derived_from:
  - slug: "rust-language"
    title: "Rust (programming language)"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "ownership system"
  - "ownership model"
  - "Rust ownership"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "borrowing-rust"
  - "lifetimes-rust"
  - "borrow-checker"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is ownership in Rust?"
  - "How does Rust's ownership system relate to memory safety?"
  - "What distinguishes ownership from borrowing in Rust?"
  - "How does Rust's memory model compare to garbage collection?"

# === LIFECYCLE ===
status: "published"
stable_since: "1.0"
last_reviewed: "2026-05-03"
---

## Quick Definition

Ownership is Rust's compile-time system for managing memory and resources without a garbage collector. Each value has exactly one owner at a time, and the value is dropped (its destructor is run) when the owner goes out of scope.

## Core Definition

Rust's ownership system consists of rules that ensure memory safety without using a garbage collector. At compile time, each value must be attached to a variable called the owner of that value, and every value must have exactly one owner. Values are moved between different owners through assignment or passing a value as a function parameter ([rust-language](/source/rust-language/)). Because of these ownership rules, Rust types are known as affine types, meaning each value may be used at most once. This enforces a form of software fault isolation as the owner of a value is solely responsible for its correctness and deallocation ([rust-language](/source/rust-language/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki.

## Key Properties

1. **Single owner** -- every value has exactly one owner at any given time.
2. **Move semantics** -- assigning a value to another variable or passing it to a function moves ownership; the original variable can no longer be used.
3. **Automatic deallocation** -- when a value goes out of scope, it is dropped by running its destructor (the `Drop` trait) ([rust-language](/source/rust-language/)).
4. **Compile-time enforcement** -- ownership rules are checked at compile time, not at runtime, resulting in zero runtime overhead.
5. **Affine types** -- Rust types are affine, meaning each value may be used at most once (unless it implements the `Copy` trait).
6. **No garbage collector** -- ownership replaces garbage collection, providing deterministic resource management with very low overhead ([rust-language](/source/rust-language/)).

## Construction / Recognition

To apply ownership in practice:

1. Declare a variable with `let` -- this variable becomes the owner of its value.
2. Moving: assign the variable to another variable or pass it to a function -- ownership transfers, and the original binding becomes invalid.
3. If you need to use the value after passing it, use borrowing (`&`) instead of moving.
4. When the owner goes out of scope, the value's destructor runs automatically.

## Context & Application

The ownership system was in place by 2010 and was gradually consolidated, with the garbage collector removed by 2013 in favor of the ownership system ([rust-language](/source/rust-language/)). Ownership is the foundation of Rust's memory safety guarantees, which include the absence of null pointer dereferences, dangling pointers, buffer overflows, double frees, and data races. It also manages non-memory resources such as file handles, network sockets, and locks ([rust-language](/source/rust-language/)).

## Examples

**Example 1** ([rust-language](/source/rust-language/)):

```rust
fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello, World");
    print_string(s); // s consumed by print_string
    // s has been moved, so cannot be used any more
    // another print_string(s); would result in a compile error
}
```

**Example 2** ([rust-language](/source/rust-language/)): The `Drop` trait allows programmatic definition of destructors, so that when objects are dropped, associated resources (file handles, sockets, locks) are released automatically.

## Relationships

- **Related to borrowing-rust** -- borrowing allows temporary access to a value without transferring ownership.
- **Related to lifetimes-rust** -- lifetimes track how long references (borrows) are valid, working together with ownership to prevent dangling pointers.
- **Related to borrow-checker** -- the borrow checker enforces both ownership and borrowing rules at compile time.

## Common Errors

- **Using a value after it has been moved** -- attempting to use a variable after its value has been moved to another owner causes a compile error.
- **Forgetting to return ownership** -- when a function takes ownership of a value and the caller needs it back, the function must return the value (or use borrowing instead).

## Common Confusions

- **Ownership is not garbage collection** -- ownership provides deterministic, compile-time resource management, while garbage collection is a runtime mechanism. Rust does not have a garbage collector.
- **Moving is not copying** -- unless a type implements `Copy`, assignment and function calls move the value, not copy it.

## Source Reference

Derived from:
- [rust-language](/source/rust-language/) -- provides the ownership rules, move semantics, the `Drop` trait, affine types, and code examples in the "Ownership and references" and "Memory management" sections.

## Verification Notes

- Definition: directly adapted from the "Ownership and references" section of [rust-language](/source/rust-language/)
- Key Properties: all items explicit in source
- Code examples: taken directly from the source page
- Confidence: HIGH -- dedicated section with clear rules and extensive code examples
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
