---
# === CORE IDENTIFICATION ===
concept: "Large Language Model"
slug: "large-language-model"
page_type: "concept"

# === CLASSIFICATION ===
category: "artificial-intelligence"
subcategory: "techniques"
tier: "intermediate"
subject_headings:
  - "computer-science"
  - "natural-language-processing"
keywords:
  - "LLM"
  - "GPT"
  - "transformer"
  - "generative AI"
  - "pre-training"

# === PROVENANCE ===
derived_from:
  - slug: "artificial-intelligence"
    title: "Artificial intelligence"
  - slug: "claude-language-model"
    title: "Claude (language model)"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "LLM"
  - "generative pre-trained transformer"
  - "GPT"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "artificial-intelligence-field"
  - "machine-learning"
extends:
  - "deep-learning"
related:
  - "claude-ai"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is artificial intelligence?"

# === LIFECYCLE ===
status: "published"
last_reviewed: "2026-05-03"
---

## Quick Definition

A large language model (LLM) is a type of generative AI system, typically based on the transformer architecture, that is pre-trained on vast text corpora to predict the next token and can then generate coherent, human-like text across a wide range of tasks.

## Core Definition

Generative pre-trained transformers (GPT) are large language models that generate text based on the semantic relationships between words in sentences ([artificial-intelligence](/source/artificial-intelligence/)). Text-based GPT models are pre-trained on a large corpus of text; the pretraining consists of predicting the next token. Throughout this pretraining, GPT models accumulate knowledge about the world and can then generate human-like text by repeatedly predicting the next token ([artificial-intelligence](/source/artificial-intelligence/)). Typically, a subsequent training phase makes the model more truthful, useful, and harmless, usually with a technique called reinforcement learning from human feedback (RLHF) ([artificial-intelligence](/source/artificial-intelligence/)). Claude is a series of large language models developed by Anthropic ([claude-language-model](/source/claude-language-model/)).

## Prerequisites

- **Artificial intelligence** -- LLMs are AI systems.
- **Machine learning** -- LLMs use machine learning techniques for training.

## Key Properties

1. **Transformer architecture** -- based on the transformer architecture that uses attention mechanisms ([artificial-intelligence](/source/artificial-intelligence/)).
2. **Pre-training on text** -- trained on large text corpora by predicting the next token ([artificial-intelligence](/source/artificial-intelligence/)).
3. **Fine-tuning** -- subsequent training using RLHF or other techniques to improve behavior ([artificial-intelligence](/source/artificial-intelligence/)).
4. **Multi-modal variants** -- some models can process different data types: images, videos, sound, and text ([claude-language-model](/source/claude-language-model/)).
5. **Hallucination risk** -- prone to generating falsehoods called "hallucinations" ([artificial-intelligence](/source/artificial-intelligence/)).
6. **Current systems** -- include ChatGPT, Claude, Gemini, Copilot, and Meta AI ([artificial-intelligence](/source/artificial-intelligence/)).

## Context & Application

In 2019, GPT language models began to generate coherent text, and by 2023 these models were able to achieve human-level scores on professional examinations ([artificial-intelligence](/source/artificial-intelligence/)). Growth in this area accelerated after 2017 with the transformer architecture ([artificial-intelligence](/source/artificial-intelligence/)).

## Relationships

- **Prerequisite: artificial-intelligence-field** -- LLMs are AI systems.
- **Prerequisite: machine-learning** -- LLMs use machine learning.
- **Extends: deep-learning** -- LLMs are a specific application of deep learning.
- **Related to claude-ai** -- Claude is a specific LLM family.

## Source Reference

Derived from:
- [artificial-intelligence](/source/artificial-intelligence/) -- provides definition, training process, capabilities, and current systems.
- [claude-language-model](/source/claude-language-model/) -- provides Claude-specific details as an LLM example.

## Verification Notes

- Definition: synthesized from GPT section of [artificial-intelligence](/source/artificial-intelligence/)
- Key Properties: all items explicit in sources
- Confidence: HIGH -- substantial treatment in both sources
- Cross-references: all slug references verified
- Uncertainties: none
