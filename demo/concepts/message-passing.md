---
concept: "Message Passing"
slug: "message-passing"
page_type: "concept"
category: "programming-languages"
subcategory: "concurrency"
tier: "intermediate"
subject_headings:
  - "concurrency"
  - "distributed-systems"
keywords:
  - "asynchronous"
  - "mailbox"
  - "inter-process communication"
  - "shared-nothing"
derived_from:
  - slug: "erlang-language"
    title: "Erlang (programming language)"
  - slug: "lfe-language"
    title: "LFE (programming language)"
extraction_confidence: "high"
aliases:
  - "message-passing concurrency"
  - "asynchronous message passing"
prerequisites:
  - "actor-model"
extends: []
related:
  - "erlang"
  - "beam-vm"
contrasts_with: []
trust_level: "credible"
answers_questions:
  - "How do Erlang and LFE relate?"
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

Message passing is a concurrency mechanism where processes communicate by sending and receiving messages through mailboxes, rather than sharing memory. In Erlang/BEAM, it is asynchronous, shared-nothing, and the sole means of inter-process communication.

## Core Definition

Inter-process communication in Erlang works via a shared-nothing asynchronous message passing system: every process has a "mailbox," a queue of messages sent by other processes and not yet consumed. A process uses the `receive` primitive to retrieve messages that match desired patterns. A message may comprise any Erlang data structure, including primitives, tuples, lists, and functions. Communication with remote processes works identically to local communication, providing location transparency ([erlang-language](/source/erlang-language/)). LFE inherits this shared-nothing concurrent programming model ([lfe-language](/source/lfe-language/)).

## Prerequisites

- **Actor model** -- message passing is the communication mechanism of the actor model.

## Key Properties

1. **Asynchronous** -- messages are sent without blocking the sender.
2. **Shared-nothing** -- processes share no memory; all data is copied in messages.
3. **Pattern-matched receive** -- receivers selectively match messages against patterns.
4. **Location transparent** -- local and remote message passing use identical syntax.
5. **Any data type** -- messages can contain any Erlang term.

## Context & Application

Erlang's message passing eliminates the need for explicit locks, avoiding deadlock and race condition bugs common in shared-memory concurrency. The mailbox model allows processes to be selective about which messages they handle and in what order ([erlang-language](/source/erlang-language/)).

## Examples

**Example 1** ([erlang-language](/source/erlang-language/)): Sending: `ServerProcess ! {pause, 10}` sends a tuple asynchronously. Receiving:
```erlang
receive
    {data, DataContent} -> handle(DataContent);
    {hello, Text} -> io:format("Got hello: ~s", [Text])
end.
```

## Relationships

- **Prerequisite: actor-model** -- message passing implements actor communication.
- **Related to erlang** and **beam-vm** -- BEAM provides the message-passing infrastructure.

## Source Reference

Derived from:
- [erlang-language](/source/erlang-language/) -- provides the mechanism, syntax, and semantics
- [lfe-language](/source/lfe-language/) -- confirms the model on BEAM

## Verification Notes

- Definition: from "Concurrency and distribution orientation" section
- Confidence: HIGH -- explicitly defined with code examples
- Uncertainties: none
