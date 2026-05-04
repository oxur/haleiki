---
concept: "Hot Code Loading"
slug: "hot-code-loading"
page_type: "concept"
category: "programming-languages"
subcategory: "runtime-systems"
tier: "advanced"
subject_headings:
  - "fault-tolerance"
  - "deployment"
keywords:
  - "dynamic software updating"
  - "live upgrade"
  - "module versioning"
derived_from:
  - slug: "erlang-language"
    title: "Erlang (programming language)"
extraction_confidence: "high"
aliases:
  - "hot code swapping"
  - "live code updating"
  - "dynamic software updating"
prerequisites:
  - "erlang"
  - "beam-vm"
extends: []
related:
  - "let-it-crash"
contrasts_with: []
trust_level: "credible"
answers_questions:
  - "What is the BEAM virtual machine?"
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Hot code loading is Erlang/BEAM's ability to update running code without stopping the system, keeping two versions of a module in memory simultaneously so processes can transition to new code during normal operation.

## Core Definition

Erlang supports language-level dynamic software updating. Code is loaded and managed as "module" units. The system can keep two versions of a module in memory at the same time (the "new" and the "old" version), and processes can concurrently run code from each. A process will not move into the new version until it makes an external call to its module ([erlang-language](/source/erlang-language/)). This capability enables non-stop applications where code changes can be deployed without service interruption.

## Prerequisites

- **Erlang** -- hot code loading is an Erlang/OTP feature.
- **BEAM VM** -- BEAM provides the runtime mechanism for dual-version module management.

## Key Properties

1. **Dual version coexistence** -- two versions of a module can exist simultaneously in memory.
2. **Module-level granularity** -- the module is the unit of code loading and versioning.
3. **Explicit migration** -- processes move to the new version only on external module calls.
4. **Zero downtime** -- enables code updates without stopping the running system.

## Context & Application

Hot code loading is essential for Erlang's target domain of non-stop telecommunications systems. The AXD301 switch relied on this capability to achieve nine 9s of availability. The mechanism works by intercepting module-qualified function calls (`?MODULE:function(Args)`) to trigger version migration ([erlang-language](/source/erlang-language/)).

## Examples

**Example 1** ([erlang-language](/source/erlang-language/)): A counter module can be upgraded by sending a `code_switch` message, causing the process to call `?MODULE:codeswitch(Sum)`, which forces the use of the latest module version.

## Relationships

- **Prerequisites: erlang, beam-vm** -- hot code loading is a BEAM feature.
- **Related to let-it-crash** -- both support high-availability systems.

## Source Reference

Derived from:
- [erlang-language](/source/erlang-language/) -- provides mechanism, code example, and rationale

## Verification Notes

- Definition: from "Hot code loading and modules" section
- Confidence: HIGH -- explicitly defined with code example
- Uncertainties: none
