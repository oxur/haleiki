---
# === CORE IDENTIFICATION ===
concept: "BEAM Virtual Machine"
slug: "beam-vm"
page_type: "concept"

# === CLASSIFICATION ===
category: "programming-languages"
subcategory: "runtime-systems"
tier: "intermediate"
subject_headings:
  - "concurrency"
  - "virtual-machines"
keywords:
  - "Erlang virtual machine"
  - "bytecode"
  - "lightweight processes"
  - "OTP"

# === PROVENANCE ===
derived_from:
  - slug: "erlang-language"
    title: "Erlang (programming language)"
  - slug: "lfe-language"
    title: "LFE (programming language)"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "BEAM"
  - "Erlang VM"
  - "Bogdan/Bjorn's Erlang Abstract Machine"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - "erlang"
  - "lfe"
  - "actor-model"
  - "message-passing"
  - "otp"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the BEAM virtual machine?"
  - "How do Erlang and LFE relate?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

The BEAM is the virtual machine that executes Erlang bytecode, providing lightweight process isolation, message-passing concurrency, and hot code loading. It is the shared runtime for Erlang, LFE, Elixir, and other BEAM languages.

## Core Definition

BEAM is the virtual machine at the core of the Erlang runtime system, included in the official Erlang/OTP distribution. It executes bytecode that is converted to threaded code at load time, and includes a native code compiler on most platforms. Work on BEAM began in 1992 to address the performance limitations of the earlier Prolog-based Erlang interpreter, compiling Erlang to C using a mix of natively compiled code and threaded code ([erlang-language](/source/erlang-language/)). LFE compiles to Core Erlang and runs on BEAM, providing Lisp syntax with full access to the BEAM runtime ([lfe-language](/source/lfe-language/)).

## Prerequisites

This is an intermediate concept; familiarity with virtual machines and programming languages is helpful but not strictly required within the wiki.

## Key Properties

1. **Lightweight processes** -- BEAM schedules its own processes (not OS threads), with minimal overhead (~300 words per process); benchmarks have run 20 million processes on a single machine.
2. **Process isolation** -- strict isolation between processes including separate data and garbage collection.
3. **Message passing** -- processes communicate via asynchronous shared-nothing message passing through mailboxes.
4. **Hot code loading** -- BEAM supports dynamic software updating, keeping two module versions in memory simultaneously.
5. **Distribution** -- transparent communication between processes on different BEAM nodes (different hosts).
6. **SMP support** -- native symmetric multiprocessing support since release R11B (May 2006).
7. **Multi-language** -- BEAM hosts multiple languages: Erlang, LFE, Elixir, and others.

## Context & Application

BEAM was developed to make Erlang suitable for production telephony systems at Ericsson. Its concurrency model, based on the actor model, enables the construction of distributed, fault-tolerant, soft real-time, highly available systems. The AXD301 switch, containing over a million lines of Erlang, achieved nine 9s of availability running on BEAM ([erlang-language](/source/erlang-language/)). Since Erlang was open-sourced in 1998, BEAM has become the foundation for a broader ecosystem including Elixir and LFE.

## Examples

**Example 1** ([erlang-language](/source/erlang-language/)): Creating a process and sending a message:
```erlang
ServerProcess = spawn(web, start_server, [Port, MaxConnections]),
ServerProcess ! {pause, 10}.
```

**Example 2** ([erlang-language](/source/erlang-language/)): Transparent distribution -- creating a process on a remote node uses the same `spawn` interface: `RemoteProcess = spawn(RemoteNode, web, start_server, [Port, MaxConnections])`.

## Relationships

- **Related to erlang** -- BEAM is the primary Erlang runtime.
- **Related to lfe** -- LFE targets BEAM, providing Lisp syntax for BEAM programs.
- **Related to actor-model** -- BEAM implements the actor model of concurrent computation.
- **Related to message-passing** -- BEAM processes communicate exclusively via message passing.
- **Related to otp** -- OTP (Open Telecom Platform) is a set of libraries and design principles for BEAM programs.

## Source Reference

Derived from:
- [erlang-language](/source/erlang-language/) -- provides BEAM's history, technical characteristics, process model, concurrency primitives, and hot code loading
- [lfe-language](/source/lfe-language/) -- confirms BEAM as a multi-language VM and LFE's compilation target

## Verification Notes

- Definition: synthesized from the "History," "Concurrency," and "Implementation" sections of [erlang-language](/source/erlang-language/)
- Key Properties: all items explicit in the source
- Confidence: HIGH -- BEAM is explicitly described with technical details
- Uncertainties: none
