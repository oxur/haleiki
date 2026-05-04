# Extraction Log: erlang-language

**Source page:** erlang-language
**Date:** 2026-05-03
**Extractor:** Claude Opus 4.6

## Concepts Extracted

| Slug | Concept | Tier | Confidence | New/Merged |
|------|---------|------|------------|------------|
| erlang | Erlang | intermediate | high | new |
| beam-vm | BEAM Virtual Machine | intermediate | high | new |
| let-it-crash | Let It Crash | intermediate | high | new |
| supervisor-tree | Supervisor Tree | intermediate | high | new |
| actor-model | Actor Model | intermediate | medium | new |
| message-passing | Message Passing | intermediate | high | new |
| hot-code-loading | Hot Code Loading | advanced | high | new |

## Concepts Contributed To (multi-source)

| Slug | Contribution |
|------|-------------|
| functional-programming | Erlang as concurrent functional language |
| garbage-collection | Per-process GC |
| lisp | Erlang influenced by Lisp |

## Concepts Skipped

| Concept | Reason |
|---------|--------|
| OTP | Mentioned substantially but interchangeable with Erlang/OTP; covered within erlang concept |
| Pattern matching (Erlang) | Covered within erlang concept; same concept as Rust pattern matching at abstract level |
| Erlang data types | Too granular (atoms, tuples, lists, maps, etc.) |
| Joe Armstrong | Biographical; could be extracted but biography cluster not in scope |
| Ericsson | Organization, not a concept |

## Notes

The erlang-language page (36KB) is rich in extractable concepts. I extracted 7 concepts covering the language itself, its runtime (BEAM), design philosophy (let-it-crash), architectural patterns (supervisor trees), concurrency model (actor model, message passing), and distinctive feature (hot code loading). Pattern matching was covered within the erlang card rather than as a separate card, since at the abstract level it overlaps with the existing pattern-matching-rust card.
