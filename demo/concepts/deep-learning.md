---
# === CORE IDENTIFICATION ===
concept: "Deep Learning"
slug: "deep-learning"
page_type: "concept"

# === CLASSIFICATION ===
category: "artificial-intelligence"
subcategory: "techniques"
tier: "intermediate"
subject_headings:
  - "computer-science"
  - "neural-networks"
keywords:
  - "neural networks"
  - "backpropagation"
  - "GPU"
  - "transformer"

# === PROVENANCE ===
derived_from:
  - slug: "artificial-intelligence"
    title: "Artificial intelligence"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "deep neural networks"
  - "DL"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "machine-learning"
extends:
  - "machine-learning"
related:
  - "large-language-model"
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

Deep learning is a type of machine learning that uses neural networks with multiple layers to progressively extract higher-level features from raw input, profoundly improving AI performance in computer vision, speech recognition, natural language processing, and other domains since 2012.

## Core Definition

Deep learning uses several layers of neurons between the network's inputs and outputs. The multiple layers can progressively extract higher-level features from the raw input ([artificial-intelligence](/source/artificial-intelligence/)). Deep learning has profoundly improved the performance of programs in many important subfields of AI, including computer vision, speech recognition, natural language processing, and image classification ([artificial-intelligence](/source/artificial-intelligence/)). The sudden success of deep learning in 2012-2015 was driven by the hundred-fold increase in speed from switching to GPUs and the availability of vast amounts of training data ([artificial-intelligence](/source/artificial-intelligence/)).

## Prerequisites

- **Machine learning** -- deep learning is a subset of machine learning.

## Key Properties

1. **Multiple layers** -- uses two or more hidden layers of neurons ([artificial-intelligence](/source/artificial-intelligence/)).
2. **Hierarchical feature extraction** -- lower layers identify simple patterns (edges), higher layers identify complex concepts (faces) ([artificial-intelligence](/source/artificial-intelligence/)).
3. **GPU acceleration** -- the 2012 breakthrough was enabled by GPU computing power ([artificial-intelligence](/source/artificial-intelligence/)).
4. **Key architectures** -- includes feedforward networks, convolutional neural networks (CNNs), recurrent neural networks (RNNs), LSTMs, and transformers ([artificial-intelligence](/source/artificial-intelligence/)).
5. **Backpropagation** -- the most common training technique adjusts weights through backpropagation ([artificial-intelligence](/source/artificial-intelligence/)).

## Relationships

- **Prerequisite: machine-learning** -- deep learning is a subset of ML.
- **Extends: machine-learning** -- deep learning extends ML with multi-layer neural networks.
- **Related to large-language-model** -- LLMs are built on deep learning architectures.

## Source Reference

Derived from:
- [artificial-intelligence](/source/artificial-intelligence/) -- provides definition, key architectures, historical context, and applications.

## Verification Notes

- Definition: directly adapted from [artificial-intelligence](/source/artificial-intelligence/)
- Key Properties: all items explicit in source
- Confidence: HIGH -- explicitly defined in source
- Cross-references: all slug references verified
- Uncertainties: none
