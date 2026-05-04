---
concept: "Supervisor Tree"
slug: "supervisor-tree"
page_type: "concept"
category: "programming-languages"
subcategory: "design-patterns"
tier: "intermediate"
subject_headings:
  - "fault-tolerance"
  - "concurrency"
keywords:
  - "OTP"
  - "process hierarchy"
  - "fault recovery"
derived_from:
  - slug: "erlang-language"
    title: "Erlang (programming language)"
extraction_confidence: "high"
aliases:
  - "supervision tree"
  - "OTP supervisor"
prerequisites:
  - "erlang"
  - "let-it-crash"
extends: []
related:
  - "beam-vm"
  - "otp"
contrasts_with: []
trust_level: "credible"
answers_questions:
  - "How do Erlang and LFE relate?"
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

A supervisor tree is a hierarchical architecture in Erlang/OTP where supervisor processes manage the lifecycle of child processes (workers or subordinate supervisors), restarting them when they crash according to the let-it-crash philosophy.

## Core Definition

A typical Erlang application is written in the form of a supervisor tree -- a hierarchy of processes where the top-level process (the supervisor) spawns multiple child processes that act either as workers or as lower-level supervisors. Within a supervisor tree, all supervisor processes are responsible for managing the lifecycle of their child processes, including handling crash situations. When a monitored child process crashes, the supervisor receives a message containing a tuple whose first member is the atom `'DOWN'` and takes appropriate corrective action ([erlang-language](/source/erlang-language/)).

## Prerequisites

- **Erlang** -- supervisor trees are an Erlang/OTP design pattern.
- **Let-it-crash** -- supervisor trees implement the let-it-crash recovery strategy.

## Key Properties

1. **Hierarchical** -- supervisors can spawn child supervisors, creating trees of arbitrary depth.
2. **Lifecycle management** -- supervisors manage child process creation, monitoring, and restart.
3. **Crash detection** -- uses `erlang:monitor/2` to receive `'DOWN'` messages from crashed children.
4. **Scalable fault tolerance** -- the hierarchical structure provides fault containment at each level.

## Context & Application

Supervisor trees are the standard architectural pattern in OTP (Open Telecom Platform). They enable Erlang applications to achieve high availability by isolating failures to individual processes and automatically recovering from them. The AXD301 switch achieved nine 9s of availability using this architecture ([erlang-language](/source/erlang-language/)).

## Examples

**Example 1** ([erlang-language](/source/erlang-language/)): A supervisor spawns a child process and monitors it with `erlang:monitor/2`. When the child crashes, the supervisor receives a `{' DOWN', ...}` message and decides whether to restart the child.

## Relationships

- **Prerequisites: erlang, let-it-crash** -- supervisor trees implement Erlang's crash recovery philosophy.
- **Related to beam-vm** -- BEAM provides the lightweight process infrastructure.
- **Related to otp** -- supervisor trees are a core OTP design principle.

## Source Reference

Derived from:
- [erlang-language](/source/erlang-language/) -- provides definition, mechanism, and architectural description

## Verification Notes

- Definition: from the "Supervisor trees" section of [erlang-language](/source/erlang-language/)
- Confidence: HIGH -- explicitly defined with code-level mechanism description
- Uncertainties: none
