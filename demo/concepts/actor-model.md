---
concept: "Actor Model"
slug: "actor-model"
page_type: "concept"
category: "programming-languages"
subcategory: "concurrency"
tier: "intermediate"
subject_headings:
  - "concurrency"
  - "distributed-systems"
keywords:
  - "message passing"
  - "concurrent computation"
  - "process isolation"
derived_from:
  - slug: "erlang-language"
    title: "Erlang (programming language)"
  - slug: "lfe-language"
    title: "LFE (programming language)"
extraction_confidence: "medium"
aliases:
  - "actor-based concurrency"
  - "actors"
prerequisites: []
extends: []
related:
  - "erlang"
  - "beam-vm"
  - "message-passing"
contrasts_with: []
trust_level: "credible"
answers_questions:
  - "How do Erlang and LFE relate?"
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

The actor model is a concurrency paradigm in which independent actors (processes) communicate exclusively through asynchronous message passing, sharing no state. Erlang and LFE implement this model through BEAM's lightweight process architecture.

## Core Definition

In the actor model as implemented by Erlang, every process is an independent actor that communicates with other actors solely through asynchronous message passing. Processes share no state ("shared-nothing architecture") and have strict isolation including separate data and garbage collection. Erlang recasts the ideas of communicating sequential processes (CSP) in a functional framework with asynchronous message passing ([erlang-language](/source/erlang-language/)). LFE inherits this model, providing shared-nothing concurrent programming via message passing ([lfe-language](/source/lfe-language/)).

## Prerequisites

This is an intermediate concept with no strict prerequisites within the wiki.

## Key Properties

1. **Process isolation** -- actors share no state; each has its own data and garbage collection.
2. **Asynchronous message passing** -- actors communicate by sending messages to each other's mailboxes.
3. **Mailbox-based** -- each actor has a queue of received messages; it selectively matches messages using pattern matching.
4. **Location transparency** -- communication with remote actors works identically to communication with local actors.
5. **Lightweight** -- actors have minimal overhead (~300 words), enabling millions of concurrent actors.

## Context & Application

Joe Armstrong summarized the Erlang process principles: "Everything is a process. Processes are strongly isolated. Message passing is the only way for processes to interact. Processes share no resources. Error handling is non-local." Erlang's actor model eliminates the need for explicit locks; a locking scheme is used only internally by the VM ([erlang-language](/source/erlang-language/)).

## Examples

**Example 1** ([erlang-language](/source/erlang-language/)): Sending a message to a process: `ServerProcess ! {pause, 10}` sends an asynchronous message consisting of a tuple.

**Example 2** ([erlang-language](/source/erlang-language/)): Receiving messages with pattern matching:
```erlang
receive
    a_message -> do_something;
    {data, DataContent} -> handle(DataContent)
end.
```

## Relationships

- **Related to erlang** -- Erlang implements the actor model.
- **Related to beam-vm** -- BEAM provides the runtime for actor-based processes.
- **Related to message-passing** -- message passing is the sole communication mechanism between actors.

## Source Reference

Derived from:
- [erlang-language](/source/erlang-language/) -- provides the process model, principles, and concurrency mechanisms
- [lfe-language](/source/lfe-language/) -- confirms actor model inheritance on BEAM

## Verification Notes

- Definition: synthesized from "Concurrency and distribution orientation" section and Armstrong's process principles
- Confidence: MEDIUM -- the term "actor model" is implied but the concept is thoroughly described
- Uncertainties: source uses "processes" terminology; the mapping to "actor model" is standard but not explicit in the source text
