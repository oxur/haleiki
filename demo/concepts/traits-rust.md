---
# === CORE IDENTIFICATION ===
concept: "Traits"
slug: "traits-rust"
page_type: "concept"
disambiguator: "Rust"

# === CLASSIFICATION ===
category: "programming-languages"
subcategory: "type-system"
tier: "intermediate"
subject_headings:
  - "type-systems"
  - "polymorphism"
keywords:
  - "trait"
  - "polymorphism"
  - "static dispatch"
  - "dynamic dispatch"
  - "trait objects"

# === PROVENANCE ===
derived_from:
  - slug: "rust-language"
    title: "Rust (programming language)"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "Rust traits"
  - "trait system"
  - "trait bounds"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "pattern-matching-rust"
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

Traits in Rust define shared behavior across types by declaring method signatures and optional default implementations. They enable polymorphism through both static dispatch (generics with trait bounds) and dynamic dispatch (trait objects).

## Core Definition

Rust supports polymorphism through traits, generic functions, and trait objects. Common behavior between types is declared using traits and `impl` blocks ([rust-language](/source/rust-language/)). Traits declare method signatures that types can implement, optionally providing default implementations that may be overridden. Generics use trait bounds (`T: TraitName`) to constrain type parameters. By default, traits use static dispatch: the compiler monomorphizes the function for each concrete type, yielding performance equivalent to type-specific code. When the exact type is not known at compile time, trait objects (`&dyn Trait` and `Box<dyn Trait>`) provide dynamic dispatch via a lookup table ([rust-language](/source/rust-language/)).

## Prerequisites

This is a foundational concept for Rust's type system with no prerequisites within the wiki.

## Key Properties

1. **Method declaration** -- traits declare method signatures; types implement them via `impl TraitName for TypeName` blocks.
2. **Default implementations** -- traits can provide default method bodies that implementors may override.
3. **Static dispatch** -- by default, generic functions with trait bounds are monomorphized at compile time, resulting in zero runtime cost ([rust-language](/source/rust-language/)).
4. **Dynamic dispatch** -- trait objects (`&dyn Trait`, `Box<dyn Trait>`) use vtable-based dynamic dispatch when the concrete type is not known at compile time. This adds a small runtime cost but keeps binary size smaller.
5. **Trait bounds** -- constrain generic type parameters to types implementing specific traits, e.g., `fn zero<Num: Zero>() -> Num`.
6. **Object safety** -- only "object-safe" traits can be used as trait objects.

## Context & Application

Traits are Rust's primary mechanism for abstraction and code reuse, serving a role similar to interfaces in other languages but with additional capabilities (default methods, associated types, operator overloading). The standard library makes extensive use of traits: `Iterator`, `Display`, `Clone`, `Drop`, `From`, and many more. Traits also enable operator overloading and the zero-cost abstraction philosophy central to Rust's design ([rust-language](/source/rust-language/)).

## Examples

**Example 1** ([rust-language](/source/rust-language/)):

```rust
trait Zero: Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool
    where
        Self: PartialEq,
    {
        self == &Zero::zero()
    }
}

impl Zero for u32 {
    fn zero() -> u32 { 0 }
}

impl Zero for f32 {
    fn zero() -> Self { 0.0 }
}
```

**Example 2** ([rust-language](/source/rust-language/)): Trait objects allow heterogeneous collections:

```rust
use std::fmt::Display;

let v: Vec<Box<dyn Display>> = vec![
    Box::new(3),
    Box::new(5.0),
    Box::new("hi"),
];

for x in v {
    println!("{x}");
}
```

## Relationships

- **Related to pattern-matching-rust** -- traits and pattern matching work together in Rust; for example, trait-implementing enums are commonly matched.

## Common Confusions

- **Traits are not classes** -- traits define behavior but do not contain data fields. Data is defined in `struct`s and `enum`s; traits are implemented for those types separately.
- **Static vs. dynamic dispatch** -- by default, Rust uses static dispatch (monomorphization) for generics, not dynamic dispatch. Trait objects opt into dynamic dispatch explicitly.

## Source Reference

Derived from:
- [rust-language](/source/rust-language/) -- provides the definition, trait bounds, static/dynamic dispatch, trait objects, and code examples in the "Traits," "Generic functions," and "Trait objects" subsections.

## Verification Notes

- Definition: directly adapted from the "Polymorphism" section of [rust-language](/source/rust-language/)
- Key Properties: all items explicit in source
- Code examples: both taken from the source page
- Confidence: HIGH -- dedicated subsections with definitions and multiple code examples
- Cross-references: all slug references verified against current concept inventory
- Uncertainties: none
