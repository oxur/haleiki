---
concept: "Erlang"
slug: "erlang"
page_type: "concept"
category: "programming-languages"
subcategory: "functional"
tier: "intermediate"
subject_headings:
  - "concurrency"
  - "telecommunications"
keywords:
  - "concurrent programming"
  - "fault tolerance"
  - "Ericsson"
  - "functional programming"
  - "OTP"
derived_from:
  - slug: "erlang-language"
    title: "Erlang (programming language)"
  - slug: "lfe-language"
    title: "LFE (programming language)"
extraction_confidence: "high"
aliases:
  - "Erlang/OTP"
  - "Erlang programming language"
prerequisites: []
extends: []
related:
  - "beam-vm"
  - "lisp"
  - "lfe"
  - "actor-model"
  - "let-it-crash"
  - "otp"
  - "functional-programming"
contrasts_with: []
trust_level: "credible"
answers_questions:
  - "How do Erlang and LFE relate?"
  - "What is the BEAM virtual machine?"
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Erlang is a general-purpose, concurrent, functional programming language designed for building distributed, fault-tolerant, soft real-time systems. It runs on the BEAM virtual machine and is used interchangeably with Erlang/OTP.

## Core Definition

Erlang is a general-purpose, concurrent, functional, high-level programming language with a garbage-collected runtime system. Developed by Joe Armstrong, Robert Virding, and Mike Williams at Ericsson in 1986, it was designed for systems requiring distribution, fault tolerance, soft real-time guarantees, high availability, and hot code swapping ([erlang-language](/source/erlang-language/)). The language features pattern matching, single assignment, dynamic typing, and eager evaluation. Applications are built from hundreds of lightweight processes that communicate via message passing. Erlang was influenced by Lisp, Prolog, PLEX, and Smalltalk ([erlang-language](/source/erlang-language/)).

## Prerequisites

This is an intermediate concept; familiarity with programming language concepts is helpful.

## Key Properties

1. **Concurrency-oriented** -- processes are the primary structuring mechanism; creation and destruction are lightweight operations.
2. **Fault-tolerant** -- error handling is non-local; processes either do what they're supposed to do or fail ("let it crash").
3. **Process isolation** -- processes are strongly isolated, share no resources, and communicate only via message passing.
4. **Hot code loading** -- code can be changed without stopping the system; two module versions can coexist in memory.
5. **Pattern matching** -- function clauses use pattern matching for dispatch and data destructuring.
6. **Functional** -- the sequential subset supports eager evaluation, single assignment, and dynamic typing.

## Context & Application

Erlang was originally proprietary within Ericsson, created to improve telephony application development. It was open-sourced in December 1998. The AXD301 switch, with over a million lines of Erlang, achieved nine 9s availability. Erlang is used by WhatsApp, RabbitMQ, and major telecom companies. LFE provides a Lisp syntax for writing Erlang programs, compiling to Core Erlang on the BEAM ([lfe-language](/source/lfe-language/)).

## Examples

**Example 1** ([erlang-language](/source/erlang-language/)): Joe Armstrong's process principles: "Everything is a process. Processes are strongly isolated. Message passing is the only way for processes to interact."

**Example 2** ([erlang-language](/source/erlang-language/)): A factorial implementation using pattern matching:
```erlang
fac(0) -> 1;
fac(N) when N > 0, is_integer(N) -> N * fac(N-1).
```

## Relationships

- **Related to beam-vm** -- Erlang runs on the BEAM virtual machine.
- **Related to lisp** -- Lisp influenced Erlang's design.
- **Related to lfe** -- LFE is a Lisp that compiles to Erlang.
- **Related to actor-model** -- Erlang's concurrency model is actor-based.
- **Related to let-it-crash** -- Erlang's distinctive error handling philosophy.

## Source Reference

Derived from:
- [erlang-language](/source/erlang-language/) -- provides definition, history, design philosophy, usage, concurrency model
- [lfe-language](/source/lfe-language/) -- provides context as an Erlang ecosystem language

## Verification Notes

- Definition: directly from the opening of [erlang-language](/source/erlang-language/)
- Key Properties: all items explicit in the source's process principles and language description
- Confidence: HIGH -- explicit definition and extensive examples
- Uncertainties: none
