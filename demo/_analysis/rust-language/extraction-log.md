# Extraction Log: rust-language

**Source page:** `rust-language` (programming-languages/systems, intermediate)
**Date:** 2026-05-03

## Concept Inventory

| # | Concept | Slug | Tier | Confidence | Notes |
|---|---------|------|------|------------|-------|
| 1 | Ownership | ownership-rust | foundational | high | Dedicated section with clear rules |
| 2 | Borrowing | borrowing-rust | intermediate | high | Explained alongside ownership with code examples |
| 3 | Lifetimes | lifetimes-rust | intermediate | high | Dedicated subsection with diagrams and code |
| 4 | Borrow checker | borrow-checker | intermediate | high | Central mechanism, described in multiple sections |
| 5 | Traits | traits-rust | intermediate | high | Dedicated section with code examples |
| 6 | Pattern matching | pattern-matching-rust | intermediate | medium | Section present but definition synthesized from examples |

## Disambiguation

- "Ownership" is in the disambiguation list: slug `ownership-rust`, disambiguator `Rust`
- "Borrowing" also Rust-specific: slug `borrowing-rust`, disambiguator `Rust`
- "Traits" and "Pattern matching" are Rust-specific in this context: disambiguated with `-rust` suffix
- "Lifetimes" is Rust-specific: `lifetimes-rust`, disambiguator `Rust`

## Extraction Decisions

- **Extracted (6 concepts):** ownership-rust, borrowing-rust, lifetimes-rust, borrow-checker, traits-rust, pattern-matching-rust
- **Skipped:** Cargo (tooling, not a language concept), Rustfmt/Clippy (tooling), unsafe Rust (discussed but more of a feature escape hatch than a standalone concept at this level), macros (discussed but would need more depth for a full card), generics (covered via traits discussion), memory management (subsumed under ownership), type system (too broad, covered through ownership+traits)
- **Cross-source notes:** None expected for this source page.

## Dependency Order

1. ownership-rust (foundational for Rust)
2. borrowing-rust (requires ownership-rust)
3. lifetimes-rust (requires borrowing-rust)
4. borrow-checker (requires ownership-rust, borrowing-rust, lifetimes-rust)
5. traits-rust (independent of ownership chain)
6. pattern-matching-rust (independent)
