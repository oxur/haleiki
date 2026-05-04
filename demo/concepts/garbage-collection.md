---
concept: "Garbage Collection"
slug: "garbage-collection"
page_type: "concept"
category: "programming-languages"
subcategory: "memory-management"
tier: "foundational"
subject_headings:
  - "memory-management"
keywords:
  - "automatic memory management"
  - "heap"
  - "memory reclamation"
derived_from:
  - slug: "lisp-language"
    title: "Lisp (programming language)"
  - slug: "john-mccarthy"
    title: "John McCarthy (computer scientist)"
  - slug: "erlang-language"
    title: "Erlang (programming language)"
extraction_confidence: "high"
aliases:
  - "GC"
  - "automatic memory management"
prerequisites: []
extends: []
related:
  - "lisp"
  - "john-mccarthy-computer-scientist"
  - "erlang"
contrasts_with:
  - "ownership-rust"
trust_level: "credible"
answers_questions:
  - "How does Rust's memory model compare to garbage collection?"
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Garbage collection is a form of automatic memory management in which the runtime system automatically reclaims memory occupied by objects no longer in use. It was invented by John McCarthy around 1959 for the Lisp programming language.

## Core Definition

Garbage collection is a system in which the runtime walks the heap looking for unused memory and automatically reclaims it. Lisp introduced the concept of automatic garbage collection, and progress in modern sophisticated garbage collection algorithms such as generational garbage collection was stimulated by its use in Lisp ([lisp-language](/source/lisp-language/)). McCarthy invented garbage collection around 1959 to solve memory management problems in Lisp ([john-mccarthy](/source/john-mccarthy/)). Erlang uses garbage collection with per-process isolation: each Erlang process has its own garbage collector, preventing stop-the-world pauses from affecting the entire system ([erlang-language](/source/erlang-language/)).

## Prerequisites

This is a foundational concept with no prerequisites within the wiki.

## Key Properties

1. **Automatic reclamation** -- the runtime identifies and frees unused memory without programmer intervention.
2. **Heap walking** -- the garbage collector traverses the heap to find unreachable objects.
3. **Per-process isolation (Erlang)** -- Erlang's BEAM runs garbage collection separately for each process.
4. **Generational algorithms** -- modern GC uses generational approaches, dividing objects by age.

## Context & Application

Garbage collection was developed by Daniel Edwards at MIT prior to 1962 for Lisp, based on McCarthy's invention ([lisp-language](/source/lisp-language/)). It eliminates common memory errors (use-after-free, double-free, memory leaks) at the cost of runtime overhead and occasional pauses. Erlang's per-process GC is a key enabler of its soft real-time guarantees. Rust takes a contrasting approach with its ownership system, achieving memory safety without garbage collection.

## Examples

**Example 1** ([john-mccarthy](/source/john-mccarthy/)): McCarthy "invented so-called 'garbage collection' methods, a kind of automatic memory management, to solve problems in Lisp" around 1959.

**Example 2** ([erlang-language](/source/erlang-language/)): BEAM provides strict process isolation including "data and garbage collection, separated individually by each Erlang process."

## Relationships

- **Related to lisp** -- Lisp introduced garbage collection.
- **Related to john-mccarthy-computer-scientist** -- McCarthy invented garbage collection.
- **Related to erlang** -- Erlang uses per-process garbage collection.
- **Contrasts with ownership-rust** -- Rust's ownership system is an alternative to garbage collection.

## Source Reference

Derived from:
- [lisp-language](/source/lisp-language/) -- provides historical context and GC algorithm evolution
- [john-mccarthy](/source/john-mccarthy/) -- documents the invention
- [erlang-language](/source/erlang-language/) -- describes per-process GC

## Verification Notes

- Definition: synthesized from multiple sources
- Confidence: HIGH -- explicitly described with clear attribution
- Uncertainties: none
