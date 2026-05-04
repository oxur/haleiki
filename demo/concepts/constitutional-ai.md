---
# === CORE IDENTIFICATION ===
concept: "Constitutional AI"
slug: "constitutional-ai"
page_type: "concept"

# === CLASSIFICATION ===
category: "artificial-intelligence"
subcategory: "alignment"
tier: "intermediate"
subject_headings:
  - "computer-science"
  - "ethics"
keywords:
  - "RLHF"
  - "alignment"
  - "Anthropic"
  - "AI safety"

# === PROVENANCE ===
derived_from:
  - slug: "claude-language-model"
    title: "Claude (language model)"

# === CONFIDENCE ===
extraction_confidence: "medium"

# === VARIANTS ===
aliases:
  - "CAI"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "artificial-intelligence-field"
extends: []
related:
  - "claude-ai"
  - "amanda-askell-researcher"
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

Constitutional AI is a technique developed by Anthropic to improve the ethical and legal compliance of AI systems by training models to evaluate their own outputs against a set of principles ("constitution"), used alongside reinforcement learning from human feedback (RLHF) in training Claude.

## Core Definition

Constitutional AI is a technique developed by Anthropic to improve ethical and legal compliance (AI alignment) of language models ([claude-language-model](/source/claude-language-model/)). Claude models are fine-tuned using both reinforcement learning from human feedback (RLHF) and constitutional AI in an attempt to enforce ethical guidelines ([claude-language-model](/source/claude-language-model/)).

## Prerequisites

- **Artificial intelligence** -- constitutional AI is an AI alignment technique.

## Key Properties

1. **Principle-based training** -- the model is trained to evaluate its outputs against a set of principles.
2. **Complement to RLHF** -- used alongside RLHF to align model behavior ([claude-language-model](/source/claude-language-model/)).
3. **Developed by Anthropic** -- created by the same organization that develops Claude ([claude-language-model](/source/claude-language-model/)).

## Context & Application

Constitutional AI represents one approach to the AI alignment problem -- ensuring AI systems behave in accordance with human values and intentions. It has been central to Anthropic's approach to developing Claude, which also refused to allow its use for mass surveillance and autonomous weapons ([claude-language-model](/source/claude-language-model/)).

## Relationships

- **Prerequisite: artificial-intelligence-field** -- this is an AI alignment technique.
- **Related to claude-ai** -- used in Claude's training.
- **Related to amanda-askell-researcher** -- Askell's work on character training connects to constitutional AI.

## Source Reference

Derived from:
- [claude-language-model](/source/claude-language-model/) -- provides the definition and role in Claude's training.

## Verification Notes

- Definition: synthesized from references in [claude-language-model](/source/claude-language-model/)
- Confidence: MEDIUM -- the source references constitutional AI but does not provide a deep standalone definition
- Cross-references: all slug references verified
- Uncertainties: detailed mechanism not fully described in the source
