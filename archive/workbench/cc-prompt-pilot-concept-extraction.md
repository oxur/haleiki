# Concept Card Generate Prompt, v1

Five-article concept-card extraction pilot using the just-finalized v1 schema. Goal: surface rough edges before we scale to the full corpus.

---

## Reference docs (read these first)

- `docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md` — schema, body sections, file organization, cross-source merging, quality bar
- `docs/dev/concept-cards/0002-a-guide-for-parallel-concept-card-extraction.md` — corpus-scale process; we're scoping it down for this pilot but the validation/merge logic in Phase 3 still applies
- `docs/design/01-draft/0001-haleiki-architecture-document.md` — architecture context (especially §3 content model)
- `docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md` — trust/canon inheritance design

---

## Paths and profile assumptions

- **Source pages live at** `demo/sources/` (the demo content corpus is self-contained — do not write to `content/`).
- **Concept cards write to** `demo/concepts/`.
- **Analysis records write to** `demo/_analysis/<source-slug>/` (per-source) and `demo/_analysis/_corpus/` (corpus-level CQ list).
- **Profile**: `encyclopedia`. Trust enabled, default `credible`. Canon disabled.
- **Source-page genre**: treat all as `encyclopedia` (Wikipedia + Rigpa Wiki origin). Per-genre trust default for encyclopedia is `credible`. The demo source pages don't currently carry an explicit `genre` field — that's a separate retrofit, not in scope here.

---

## The five source pages

1. `group-theory` — mathematics/algebra, foundational. Math notation. Expect ~5-8 concepts (group, group operation, group axioms, subgroup, etc.).
2. `quantum-mechanics` — theoretical-physics/foundations, foundational. Heavy math notation, LaTeX preserved by demo pipeline. Expect ~7-10 concepts.
3. `rust-language` — programming-languages/systems, intermediate. Code blocks. Expect ~5-7 concepts. Apply disambiguator suffix `-rust` for concept names that legitimately collide with other domains' senses (definitely "ownership"; surface others as you find them).
4. `padmasambhava` — tibetan-buddhism/teachers, foundational. Biographical, Wikipedia source. Expect ~3-5 concepts.
5. `longchenpa` — tibetan-buddhism/scholars, advanced. Biographical, Rigpa Wiki source. Expect ~3-5 concepts.

Total expected concept-card count: ~25-35.

---

## Cross-page overlap to watch for

The two Tibetan Buddhism articles (`padmasambhava`, `longchenpa`) will likely both substantively discuss:

- `nyingma`
- `dzogchen`
- `vajrayana`

If both source pages cover these as more than passing mentions, produce single concept cards with multi-source `derived_from`. Apply the merge-aware extraction from 0001 § "Cross-Source Concept Merging." If they're only mentioned (not substantively discussed), skip the concept rather than producing a thin card.

The other three articles (group-theory, quantum-mechanics, rust-language) probably don't overlap with each other or with the Buddhism articles.

---

## Disambiguation list (apply proactively)

- "Ownership" → slug `ownership-rust`, disambiguator `Rust`. Bare `ownership` reserved for future property-law/general senses.
- "Group" → slug `group-mathematics`, disambiguator `mathematics`. Bare `group` reserved for music-theory/sociology/etc.

If you encounter other names that might collide with concepts the wiki could plausibly add later, surface them in your report and propose disambiguators.

---

## Process (scoped-down 4-phase)

Don't spawn sub-agents — five articles is small enough to run sequentially yourself.

### Phase 0 — Setup

1. Confirm `demo/sources/` exists and contains all five source pages. If any missing, stop and report.
2. Create `demo/concepts/` if needed. Confirm empty (only `.gitkeep`).
3. Generate 10-15 competency questions across the five articles (mix of definitional, relational, procedural, diagnostic). Save to `demo/_analysis/_corpus/competency-questions.md`.

### Phase 1 — Analysis

1. For each source page, read in full and produce a concept inventory (candidate slugs, prerequisite analysis, tier assignment).
2. Identify cross-page concepts (the Tibetan Buddhism case especially).
3. Map each CQ to source pages and candidate concepts.

### Phase 2 — Extraction (sequential)

For each source page, in any order:

1. List candidate concepts with proposed slugs (apply disambiguator suffixes per the list above and your own judgment).
2. For each candidate slug, check `demo/concepts/<slug>.md`:
   - **If exists** (multi-source): merge per 0001 § "Merge Mechanics" — augment `derived_from`, augment body sections (especially Examples and Context & Application), reconcile trust_level (lowest among combined sources), update `last_reviewed`, document the merge in Verification Notes.
   - **If not**: extract fresh per the v1 template in 0001.
3. Write to `demo/concepts/<slug>.md`.
4. Track per-source-page in `demo/_analysis/<source-slug>/extraction-log.md`: which concepts were extracted, which were merges, which were skipped (and why).

### Phase 3 — Validation

1. Structural validation per 0002 § 3.1 (frontmatter completeness, slug-filename consistency, `page_type: concept`).
2. Slug-uniqueness and cross-reference resolution per 0002 § 3.2.
3. Cross-source merge resolution per 0002 § 3.3 — find any concept-name collisions where disambiguator isn't set.
4. Trust inheritance audit per 0002 § 3.4 — multi-source cards should default to lowest trust among sources unless explicitly overridden with rationale.
5. CQ coverage check per 0002 § 3.5 — every CQ should be answered by at least one card.
6. LLM artifact check per 0002 § 3.6.

---

## Reporting

Branch this work; don't commit to main. Have the changes staged but uncommitted.

Report when done:

1. **Branch name and diff stat** — `git diff main --stat`.
2. **Counts** — total source pages processed, total concept cards written, total merges performed, breakdown per source page.
3. **Cross-source merges** — list every multi-source card with its `derived_from` slugs (e.g., "nyingma — derived from padmasambhava, longchenpa").
4. **Disambiguators applied** — list every slug with a disambiguator suffix and its `disambiguator` value.
5. **Validation report** — output of each Phase 3 step. Should all pass.
6. **Schema rough edges** — anything in the v1 schema (0001) you found awkward, under-specified, or that didn't quite fit. Examples to think about:
   - Was `derived_from` clear on what counts as derivation vs. mere mention?
   - Did `disambiguator` do what we wanted in practice?
   - Were there fields you wanted that don't exist?
   - Any inheritance rule that produced a strange outcome?
   - Any body section that felt forced or empty for a particular concept?
7. **Process rough edges** — anything in 0002 (or the scoped-down version of it) that felt off. Was Phase 0 too heavy for 5 articles? Was the disambiguation list approach workable? Did the merge-aware check feel natural?
8. **Sample output** — paste the full content of three representative cards:
   - One from a math-heavy source (a `quantum-mechanics` or `group-theory` derived card)
   - One that's a multi-source merge (a `nyingma` or `dzogchen` card if either ended up multi-source)
   - One with a code block (a `rust-language` derived card)
9. **Subjective verdict** — would you ship these as production concept cards as-is, or do they need human review and editing? What's your honest read of the quality bar?

---

If anything substantive comes up that requires a design call before you can complete a card cleanly (e.g., the schema doesn't actually accommodate something you're trying to do, or the process has a step that doesn't make sense in practice), stop and report rather than guessing. We'd rather you flag than work around.
