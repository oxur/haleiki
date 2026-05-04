# Corpus-Wide Disambiguation List

Pre-flight audit for the full 91-article Haleiki demo corpus.

## Cross-Domain Collisions

| Bare name | Disambiguated slugs | Disambiguator | Rationale |
|-----------|-------------------|---------------|-----------|
| Group | `group-mathematics` | "mathematics" | Exists from pilot. Bare `group` reserved for music-theory/sociology. |
| Composition | `composition-music` | "music" | If extracted. Functional programming sense unlikely in this corpus. |
| Field | `field-mathematics`, `field-physics` | "mathematics", "physics" | Both senses likely in math + physics articles. |
| Theory | No disambiguation needed | — | Context always clear from category. |
| Scale | `scale-music` | "music" | If extracted. Geology/math senses unlikely to produce cards. |
| Phase | No disambiguation needed | — | Only physics sense likely in this corpus. |
| Temperament | `temperament-music` | "music" | If extracted. Psychology sense unlikely. |
| Mode | `mode-music` | "music" | If extracted. CS/philosophy senses unlikely. |
| Canon | `canon-music` | "music" | If extracted. Literary/religious senses exist but unlikely as cards. |

## Source-Page-vs-Concept Collisions (Biographical/Entity Articles)

| Source page | Concept card slug | Rationale |
|------------|-------------------|-----------|
| `padmasambhava` | `padmasambhava-teacher` | Pilot pattern. Person → role suffix. |
| `longchenpa` | `longchenpa-scholar` | Pilot pattern. |
| `edward-witten` | `edward-witten-physicist` | Biography → role suffix. |
| `amanda-askell` | `amanda-askell-researcher` | Biography → role suffix. |
| `john-mccarthy` | `john-mccarthy-computer-scientist` | Biography → role suffix. |
| `johann-sebastian-bach` | `johann-sebastian-bach-composer` | Biography → role suffix. |
| `olivier-messiaen` | `olivier-messiaen-composer` | Biography → role suffix. |
| `claude-debussy` | `claude-debussy-composer` | Biography → role suffix. |
| `arvo-part` | `arvo-part-composer` | Biography → role suffix. |
| `philip-glass` | `philip-glass-composer` | Biography → role suffix. |
| `max-richter` | `max-richter-composer` | Biography → role suffix. |
| `olafur-arnalds` | `olafur-arnalds-composer` | Biography → role suffix. |
| `david-lewin` | `david-lewin-theorist` | Biography → role suffix. |
| `miles-davis` | `miles-davis-musician` | Biography → role suffix. |
| `john-coltrane` | `john-coltrane-musician` | Biography → role suffix. |
| `palestrina` | `palestrina-composer` | Biography → role suffix. |
| Teachers/scholars (Tibetan Buddhism) | `<name>-teacher` or `<name>-scholar` | Follow pilot pattern. |

## Rust-Specific Disambiguation (from pilot)

| Bare name | Disambiguated slug | Disambiguator |
|-----------|-------------------|---------------|
| Ownership | `ownership-rust` | "Rust" |
| Borrowing | `borrowing-rust` | "Rust" |
| Lifetimes | `lifetimes-rust` | "Rust" |
| Traits | `traits-rust` | "Rust" |
| Pattern matching | `pattern-matching-rust` | "Rust" |

## Anticipated Future Collisions

| Bare name | Planned suffix | Rationale |
|-----------|---------------|-----------|
| Consciousness | `consciousness-philosophy` | "philosophy of mind" — if extracted from philosophy-of-mind. AI consciousness sense exists. |
| Perception | `perception-philosophy` | "philosophy of mind" — if extracted. |
| Language | No suffix needed | Only linguistics sense in this corpus. |
| Earth | No suffix needed | Only geology sense. |
| Mantle | `mantle-geology` | "geology" — avoids collision with clothing/figurative sense. |
| Crust | `earths-crust-geology` | "geology" — follows source slug pattern. |
