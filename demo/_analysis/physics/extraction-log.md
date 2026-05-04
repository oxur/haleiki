# Extraction Log: physics

**Source page:** `physics` (theoretical-physics, foundational)
**Date:** 2026-05-03

## Concept Inventory

| # | Concept | Slug | Tier | Confidence | Notes |
|---|---------|------|------|------------|-------|
| 1 | Physics | physics-science | foundational | high | The main topic; explicitly defined in opening paragraph |
| 2 | Classical mechanics | classical-mechanics | foundational | high | Defined in Classical theory section |
| 3 | Modern physics | modern-physics | foundational | high | Distinguished from classical physics with clear criteria |
| 4 | Special relativity | special-relativity | intermediate | medium | Discussed in 20th century section, definition synthesized |
| 5 | Standard Model | standard-model | intermediate | medium | Referenced in multiple sections, definition available |
| 6 | Quantum field theory | quantum-field-theory | intermediate | medium | Referenced in Fields table, not deeply defined here |
| 7 | Classical physics | classical-physics | foundational | high | Clearly distinguished from modern physics |
| 8 | Electromagnetism | electromagnetism | foundational | medium | Discussed in Classical theory, definition synthesized |
| 9 | Thermodynamics | thermodynamics | foundational | medium | Discussed in Classical theory section |
| 10 | Particle physics | particle-physics | intermediate | high | Dedicated subsection with clear definition |

## Disambiguation

- "Physics" as a concept card needs disambiguation from the source page slug `physics`. Using `physics-science` (the discipline).
- "Field" could be `field-physics` if extracted, per disambiguation list.

## Extraction Decisions

- **Extracted (7 concepts):** physics-science, classical-mechanics, modern-physics, special-relativity, standard-model, classical-physics, particle-physics
- **Skipped:** Electromagnetism (mentioned throughout but better extracted from a dedicated EM source), Thermodynamics (same reasoning), Quantum field theory (passing mention; better from dedicated source), Optics (mentioned, not defined thoroughly), Astrophysics (subsection-level treatment, better from a dedicated source), Condensed matter physics (subsection but not enough for standalone card from this source alone)
- **Augment existing:** quantum-mechanics-theory (physics source adds historical context about Planck and Einstein's contributions)

## Dependency Order

1. physics-science (foundational, no wiki prerequisites)
2. classical-physics (foundational)
3. classical-mechanics (foundational)
4. modern-physics (requires classical-physics)
5. special-relativity (requires classical-mechanics)
6. particle-physics (requires modern-physics)
7. standard-model (requires particle-physics, quantum-field-theory)
