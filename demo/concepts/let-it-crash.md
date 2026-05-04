---
concept: "Let It Crash"
slug: "let-it-crash"
page_type: "concept"
category: "programming-languages"
subcategory: "design-philosophy"
tier: "intermediate"
subject_headings:
  - "fault-tolerance"
  - "concurrency"
keywords:
  - "crash recovery"
  - "supervisor trees"
  - "fault tolerance"
  - "Erlang philosophy"
derived_from:
  - slug: "erlang-language"
    title: "Erlang (programming language)"
extraction_confidence: "high"
aliases:
  - "let it crash philosophy"
  - "crash-oriented programming"
prerequisites:
  - "erlang"
extends: []
related:
  - "beam-vm"
  - "supervisor-tree"
  - "actor-model"
contrasts_with: []
trust_level: "credible"
answers_questions:
  - "How do Erlang and LFE relate?"
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

"Let it crash" is Erlang's design philosophy where processes are allowed to fail completely and be restarted by supervisors, rather than attempting in-process recovery from serious failures. This results in simpler, more robust code.

## Core Definition

Erlang is designed with a mechanism that makes it easy for external processes to monitor for crashes, rather than an in-process mechanism like exception handling. The "let it crash" philosophy prefers that a process be completely restarted rather than trying to recover from a serious failure. Though it still requires handling of errors, this philosophy results in less code devoted to defensive programming where error-handling code is highly contextual and specific ([erlang-language](/source/erlang-language/)).

## Prerequisites

- **Erlang** -- the let-it-crash philosophy is specific to Erlang's process model and requires understanding of Erlang's lightweight processes.

## Key Properties

1. **External monitoring** -- crashes are reported as messages to monitoring processes rather than caught in-process.
2. **Full restart** -- processes are completely restarted rather than attempting partial recovery.
3. **Reduced defensive code** -- less code devoted to contextual error handling.
4. **Supervisor hierarchy** -- supervisors manage child process lifecycles and handle crash recovery.
5. **Process do-or-fail** -- processes either do what they are supposed to do or fail cleanly.

## Context & Application

The let-it-crash philosophy is central to Erlang/OTP's approach to building fault-tolerant systems. It is realized through supervisor trees, a hierarchical architecture where supervisor processes spawn and monitor workers. When a monitored process crashes, the supervisor receives a `'DOWN'` message and takes corrective action, typically restarting the failed process. This architecture has proven to provide highly scalable and fault-tolerant environments ([erlang-language](/source/erlang-language/)).

## Examples

**Example 1** ([erlang-language](/source/erlang-language/)): The factorial function `fac(N) when N > 0, is_integer(N) -> N * fac(N-1)` will crash if given a non-integer or negative argument, illustrating the philosophy.

## Relationships

- **Prerequisite: erlang** -- the philosophy is specific to Erlang.
- **Related to beam-vm** -- BEAM provides the process isolation that makes let-it-crash viable.
- **Related to supervisor-tree** -- supervisor trees implement the let-it-crash recovery strategy.

## Source Reference

Derived from:
- [erlang-language](/source/erlang-language/) -- provides the definition, rationale, and supervisor tree implementation

## Verification Notes

- Definition: from the "Let it crash coding style" section
- Confidence: HIGH -- explicitly defined in a dedicated source section
- Uncertainties: none
