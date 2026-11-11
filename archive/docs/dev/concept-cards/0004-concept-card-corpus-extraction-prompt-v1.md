# Concept Card Corpus Extraction Prompt, v1

Full-corpus concept-card extraction across the remaining Haleiki demo source pages, building on the 27-card pilot and using the v1.1 schema and process docs. Five-agent parallel extraction per `0002-a-guide-for-parallel-concept-card-extraction.md`.

---

## Reference docs (read these first)

- `docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md` (v1.1) — schema, body sections, file organization, three disambiguation cases, substantive coverage threshold
- `docs/dev/concept-cards/0002-a-guide-for-parallel-concept-card-extraction.md` (v1.1) — corpus-scale process, five-agent parallel pattern, pre-flight disambiguation audit, source-page processing order
- `docs/design/01-draft/0001-haleiki-architecture-document.md` — architecture context (especially §3 content model, §3.4 concept merging)
- `docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md` — trust/canon design and inheritance rules

---

## Corpus state and scope

**Source pages**: `demo/sources/` contains 91 source pages across 10 categories.

**Existing concept cards from pilot**: 27 cards in `demo/concepts/`, derived from these 5 source pages: `group-theory`, `quantum-mechanics`, `rust-language`, `padmasambhava`, `longchenpa`.

**Scope of this run**: process the **86 remaining source pages** (everything in `demo/sources/` except the 5 pilot source pages).

**Merge-aware behavior**: existing pilot cards may be augmented during this run when the corpus's other source pages substantively cover concepts already represented (e.g., the `nyingma` source page will augment `demo/concepts/nyingma.md` with a third `derived_from` entry). Apply the merge mechanics from 0001 § "Merge Mechanics" — augment, don't overwrite.

**Expected output**: ~450-650 new concept cards plus augmentations to some of the 27 existing cards. Total final corpus: ~480-680 concept cards.

---

## Paths and profile assumptions

- **Source pages**: `demo/sources/` (input)
- **Concept cards**: `demo/concepts/` (output, augment-aware — 27 cards already exist)
- **Analysis records**: `demo/_analysis/<source-slug>/` (per-source) and `demo/_analysis/_corpus/` (corpus-level)
- **Profile**: `encyclopedia`. Trust enabled, default `credible`. Canon disabled.
- **Source-page genre**: all `encyclopedia` (Wikipedia + Rigpa Wiki origin); per-genre trust default for encyclopedia is `credible`.

---

## Process

Follow the four-phase process in 0002 (v1.1) directly. Notes specific to this run:

### Phase 0 — Requirements & Scope

1. Audit corpus state per 0002 § 0.0:
   - 91 source pages total in `demo/sources/`
   - 27 existing concept cards in `demo/concepts/` from the pilot
   - Categorize source pages by category, tier, genre
   - Note the 5 source pages already processed (skip them in Phase 2)
2. Generate **30-50 competency questions** spanning all 10 categories per 0002 § 0.1. Use the 25-CQ pilot set as a starting point and expand. Save updated list to `demo/_analysis/_corpus/competency-questions.md`.
3. Confirm the deployment profile defaults (encyclopedia, trust enabled, default credible).

### Phase 1 — Analysis & Planning

1. Run corpus analysis per 0002 § 1.1.
2. Map CQs to source pages and candidate concepts per 0002 § 1.2.
3. **Pre-flight disambiguation audit per 0002 § 1.2.5** — this is new in v1.1 and important. Build the corpus-wide disambiguation list covering:
   - **Cross-domain collisions**: scan for concept names likely to mean different things across the 10 categories. Watch for `composition` (music vs. functional programming), `field` (math vs. physics vs. linguistics), `theory` (physics theories vs. math theories vs. music theory), `phase` (physics vs. moon-phases vs. life-phases), `state` (quantum state vs. political state vs. mental state), etc.
   - **Source-page-vs-concept-card collisions**: for each source page about a single entity (especially biographies and "X (programming language)"-style articles), anticipate whether the extracted concept will need a role/category suffix. The pilot pattern: source page `padmasambhava` → concept card `padmasambhava-teacher`; source page `rust-language` → atomic concepts derived (no entity-card needed).
   - **Anticipated future collisions**: names with broad cross-domain use (`traits`, `borrowing`, `lifetimes`, `pattern matching`, `class`, etc.) that should get suffixes proactively.
   - Save list to `demo/_analysis/_corpus/disambiguation-list.md`.
4. Choose cluster strategy per 0002 § 1.3. Recommendation for this corpus: **by-category, with two categories per agent**, balanced for source-page count. Approximate per-category source-page counts: tibetan-buddhism ≈ 16, music ≈ 17, theoretical-physics ≈ 9, mathematics ≈ 9, linguistics ≈ 10, programming-languages ≈ 5, philosophy-of-mind ≈ 5, AI ≈ 4, botany ≈ 7, geology ≈ 8. Aim for ±20% balance across 5 agents. Subtract the 5 pilot source pages from the count when balancing.
5. Apply Step 1.6 — within each cluster, agents process foundational-tier and concept-dense source pages first.
6. Compile agent prompts per 0002 § 1.5, embedding the full corpus disambiguation list and the existing-pilot-cards list.

### Phase 2 — Parallel Extraction

Launch all 5 Opus sub-agents in parallel using the agent prompt template in 0002 (v1.1). Each agent must:

- Read its assigned cluster's source pages (skipping any of the 5 pilot source pages)
- Apply the corpus-wide disambiguation list as written
- For each candidate concept slug, check `demo/concepts/<slug>.md` first; if exists, MERGE (augment `derived_from`, augment body sections, reconcile `trust_level` to lowest, update `last_reviewed`, document the merge in Verification Notes); if not, extract fresh
- Track per-source-page extraction notes in `demo/_analysis/<source-slug>/extraction-log.md`
- Apply substantive coverage threshold from 0001 — skip mere mentions, extract substantive treatments

### Phase 3 — Validation & Cross-Source Merge

Run all six steps from 0002 § Phase 3:

1. Structural validation (3.1)
2. Slug-uniqueness and cross-reference resolution (3.2)
3. Cross-source merge resolution (3.3) — find any same-`concept` cards with different slugs where disambiguator isn't set, and merge or disambiguate
4. Trust inheritance audit (3.4) — confirm multi-source cards default to lowest trust
5. CQ coverage check (3.5) — every CQ should be answered
6. LLM artifact check (3.6)

---

## Reporting

Branch this work; don't commit to main. Have changes staged but uncommitted.

Report when done:

1. **Branch name and diff stat**.
2. **Counts**:
   - Total source pages processed (should be 86, the non-pilot pages)
   - Total concept cards in `demo/concepts/` after this run (should be ~480-680, including the 27 from pilot)
   - New concept cards created vs. existing pilot cards augmented
   - Cross-source merges performed (where multiple source pages contributed to a single card)
   - Per-agent breakdown
3. **Augmented pilot cards**: list every pilot card that gained a new `derived_from` entry from this run, with the new source-page slugs.
4. **New cross-source merges**: list every newly-multi-source card with its `derived_from` slugs (excluding the pilot's three).
5. **Disambiguators applied**: count and breakdown by case (cross-domain / source-page-collision / anticipated-future).
6. **Validation report**: output of each Phase 3 step. Should all pass. If anything fails, report and fix before reporting back.
7. **Phase 1 disambiguation list**: paste the full content of `demo/_analysis/_corpus/disambiguation-list.md`. We want to see what your audit produced.
8. **Cluster assignments**: paste the agent-cluster mapping from Phase 1.3.
9. **Schema rough edges**: anything in v1.1 that surfaced new awkwardness at corpus scale. Examples to think about:
   - Did any source page produce concepts that didn't fit cleanly into the schema?
   - Did the substantive-coverage threshold work consistently across genres (encyclopedia articles vs. biographical articles vs. Rigpa Wiki entries)?
   - Did the trust inheritance produce strange outcomes for any multi-source cards?
   - Were there cases where a concept legitimately belongs to multiple primary categories (where `subject_headings` wasn't enough)?
10. **Process rough edges**: anything in 0002 (v1.1) that surfaced friction at corpus scale. Specifically:
    - Did the pre-flight disambiguation audit catch the right things, or did surprises emerge during extraction?
    - How well did the by-category cluster strategy work? Were any cross-cluster concepts annoying to resolve?
    - Did the merge-aware behavior handle existing pilot cards cleanly?
    - Was the substantive-coverage threshold helpful, or did agents end up making different judgment calls?
11. **Sample output**: paste the full content of five representative cards spanning the diversity of the corpus:
    - One from a math-heavy article (any of theoretical-physics or mathematics)
    - One from a biographical article (Tibetan Buddhism or music)
    - One that augmented a pilot card (show the multi-source `derived_from`)
    - One from a Rigpa Wiki source
    - One short concept card from a less-densely-covered category (botany, geology, AI)
12. **Subjective verdict**: how does the corpus-scale output compare to the pilot? Are these shippable? What's your honest read of quality across the diversity?

---

## Operational notes

- **Wall time estimate**: ~90-150 minutes for Phase 2 (5 agents × ~17 source pages each, ~5-7 minutes per source page including merge-aware checks). Plus ~30-45 minutes for Phase 0/1/3.
- **No network operations needed**: all source pages are already on disk from the demo fetch pipeline. This is purely a content-transformation pass.
- **Token cost**: substantial. Five sub-agents reading source pages and writing concept cards across a 91-article corpus is the most expensive operation we've run. Plan accordingly.
- **Filename collision races**: the 5-agent flat-directory write pattern can collide when two agents extract the same cross-cluster concept. Use Strategy B from 0002 § "Coordination" — first agent creates, second agent on encountering the file switches to merge mode. The merge-aware reads handle this naturally.

---

If anything substantive comes up that requires a design call before extraction can proceed cleanly (e.g., the substantive-coverage threshold doesn't actually distinguish well in some genre, or the disambiguation list has gaps that bite during extraction), stop and report rather than guessing. We'd rather you flag than work around. Same convention as the pilot run.
