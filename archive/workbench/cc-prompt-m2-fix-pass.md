# M2 Fix-Pass: Slug Namespace, Data Shapes, Validator Gaps

Five issues from the M2 review, each with a clear resolution. Two of them require small doc revisions (v1.3 of the concept-card howto + parallel guide) before the code changes; the rest are surgical code edits.

---

## Reference docs

- `docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md` (v1.2 → v1.3 in this pass)
- `docs/dev/concept-cards/0002-a-guide-for-parallel-concept-card-extraction.md` (v1.2 → v1.3 in this pass)
- `docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md` §8.1 — the canonical `profile.json` shape
- `tools/src/{parser,graph,validator,generator}.rs` — code being modified

---

## Fix 1: Slug namespace — codify Path B (type-namespaced slugs)

**Decision**: source pages and concept cards live in *separate* slug namespaces. They may share names. URLs already disambiguate (`/source/<slug>/` vs `/<slug>/`). Cross-references resolve via an explicit rule (below).

**v1.3 doc updates** (both `0001` and `0002`):

In `0001-howto-wiki-concept-card-extraction.md`:

- **Golden Rule 6** — drop case (b) (source-page-vs-concept-card collision) entirely. Cases (a) cross-domain collision and (c) anticipated future collision stay. Replace case (b) with a paragraph stating: *"A source page and a concept card may share a slug. URLs are namespaced (`/source/<slug>/` for sources, `/<slug>/` for concepts), and cross-references resolve via the rule in § 'Cross-Reference Resolution' below."*
- **Disambiguation Strategy** section — drop the "Source-page-vs-concept-card collisions" table. Cases (a) and (c) tables stay. The 17 existing concept cards that *do* share slugs with their source pages (`nyingma`, `kagyu`, `dzogchen`, `vajrayana`, `madhyamaka`, `longchen-nyingtik`, `mycorrhiza`, `cool-jazz`, `jazz`, `language`, `sanskrit`, `proto-indo-european`, `proto-sinaitic-script`, `kharosthi`, `classical-tibetan`, `avestan`, `writing-system`) are *not* incorrect under v1.3 — they're the documented pattern.
- **Add a new section, "Cross-Reference Resolution"**, after the Disambiguation Strategy section:

  ```
  When a relationship field references a slug, resolution depends on the field:

  - `derived_from` (concept → source) — always resolves to the source page
  - `concepts_generated` (source → concept) — always resolves to the concept card
  - `prerequisites`, `extends`, `related`, `contrasts_with` — concept-first: if a concept card with that slug exists, resolve to it; otherwise resolve to the source page if one exists with that slug; otherwise unresolved (forward reference, see below).

  Forward references (slugs that don't resolve to any existing page) are normal in a wiki — they may be planned future content or simply gaps. The validator does not flag them.
  ```

- **Version history** — add v1.3 entry: *"Path B for slug namespace: source pages and concept cards live in separate slug namespaces; URL prefix `/source/` disambiguates. Cross-reference resolution rule added. Disambiguation case (b) removed (no longer needed). The 17 corpus collisions are now correct-by-design."*

In `0002-a-guide-for-parallel-concept-card-extraction.md`:

- **Phase 1.2.5 (pre-flight disambiguation audit)** — drop case (b) from the audit. Cases (a) and (c) stay.
- **Agent prompt template § "Disambiguation List (corpus-wide)"** — drop the "Source-page collisions" subsection.
- **Step 3.3 (Cross-Source Merge Resolution)** — no change in semantics, but the wording around "concept-name collisions" should be updated: now collisions only matter for *within-type* duplication (two concept cards with the same slug, two source pages with the same slug), not cross-type.
- **Version history** — add v1.3 entry mirroring 0001's.

---

## Fix 2: `validator.rs` changes

**`check_duplicate_slugs` — implement it for real.**

Currently a no-op. Replace with: detect duplicate slugs *within sources* and *within concepts* separately. Since the parser uses a HashMap keyed by slug, parser-time collision silently overwrites. The validator should track filename-vs-slug-field divergence to catch these:

```rust
fn check_duplicate_slugs(corpus: &Corpus, report: &mut ValidationReport) {
    // Compare on-disk filenames to slug fields in frontmatter.
    // If a directory has two files whose slug-field values match, that's a duplicate.
    // (The parser HashMap silently overwrites; we need to detect this from filesystem.)
}
```

Approach: re-walk `<content_dir>/sources/*.md` and `<content_dir>/concepts/*.md`, parse each just for the `slug` field, group by slug value, flag any group with size > 1 as a `DuplicateSlug` error. Cross-type duplicates (a source and a concept with the same slug) are *not* errors per Path B.

**Remove the broken-cross-reference check entirely.**

The 73 warnings about "slug not found in corpus" should not fire. Per § "Cross-Reference Resolution" in v1.3, forward references are normal and not flagged. Delete the check function (or no-op it with a comment explaining why).

If you want a low-key informational signal, you can have `haleiki stats` print a line like *"47 typed-relationship slugs reference content not yet in the corpus (forward references — normal in wiki content)."* That's count-only, no per-link noise.

**Keep all other checks** as-is (unknown category, unknown tier, dangling provenance, missing concept card from `concepts_generated`, bidirectional mismatch, orphan source/concept, slug-filename mismatch).

---

## Fix 3: `generator.rs` — `categories.json` shape

Replace the current shape (subcategories at top level, no source/concept distinction, with duplicates) with the spec'd shape:

```json
{
  "tibetan-buddhism": {
    "name": "tibetan-buddhism",
    "description": "Tibetan Buddhist philosophy, practices, teachers, and lineages",
    "sources": ["dzogchen", "padmasambhava", "kagyu", "nyingma", ...],
    "concepts": ["dzogchen", "kagyu", "nyingma", "rigpa", "sunyata", ...],
    "subcategories": {
      "teachers": {
        "sources": ["padmasambhava", "dilgo-khyentse", ...],
        "concepts": ["padmasambhava-teacher", "vimalamitra-teacher", ...]
      },
      "schools": {
        "sources": ["nyingma", "kagyu"],
        "concepts": ["nyingma", "kagyu"]
      },
      ...
    }
  },
  "theoretical-physics": { ... },
  ...
}
```

Notes:

- `name` and `description` come from `taxonomy.yaml`.
- `sources` and `concepts` are the slug arrays for that category, sorted alphabetically (use `BTreeMap` or sort before serializing for diff stability).
- Slugs that exist in *both* sources and concepts under the same category appear in both arrays. Templates can show them as separate items or merge in display layer.
- Subcategories nest the same `{sources, concepts}` shape.
- A page (source or concept) appears under exactly one `category`/`subcategory` path. The category's flat `sources`/`concepts` arrays are the union across subcategories plus any uncategorized.

---

## Fix 4: `generator.rs` — `profile.json` shape

Replace the flat shape with the nested shape per addendum 0005 §8.1:

```json
{
  "personality": "encyclopedia",
  "name": "Haleiki Demo Wiki",
  "trust": {
    "enabled": true,
    "default": "credible",
    "levels": {
      "corroborated": {
        "label": "Corroborated",
        "description": "Verified from multiple reliable sources"
      },
      "credible": {
        "label": "Credible",
        "description": "Well-sourced, reasonable interpretation"
      },
      "provisional": {
        "label": "Provisional",
        "description": "Partial evidence, subject to revision"
      },
      "speculative": {
        "label": "Speculative",
        "description": "Limited or fragmentary basis"
      }
    },
    "level_order": ["corroborated", "credible", "provisional", "speculative"]
  },
  "canon": {
    "enabled": false
  },
  "page_types": ["source", "concept"],
  "relationships": ["prerequisite", "extends", "related", "contrasts_with"],
  "taxonomy": {
    "categories": [
      {"name": "tibetan-buddhism", "description": "..."},
      ...
    ],
    "tiers": [
      {"name": "foundational", "description": "...", "order": 1},
      ...
    ]
  }
}
```

Note `level_order` is required — M6's trust-bar visualization needs to render levels in the order corroborated → speculative regardless of HashMap iteration order. Also include the `taxonomy` block (categories and tiers from `taxonomy.yaml`) so templates have one source of truth.

---

## Fix 5: Cross-reference resolution in `graph.rs`

The see-also generator currently picks source-page when both source and concept share a slug. Per § "Cross-Reference Resolution" (v1.3), the rule is:

- `derived_from` → always source page
- `concepts_generated` → always concept card
- `prerequisites`, `extends`, `related`, `contrasts_with` → concept-first; if no concept card exists with that slug, fall back to source page; otherwise unresolved (drop from see-also output silently)

Update the resolver method (likely `see_also(slug)` or whatever it's called) to apply this rule per edge type. Look at the see-also output for `nyingma` (the concept) before/after — under v1.3 resolution, references to `kagyu` should resolve to `/kagyu/` (the concept card) not `/source/kagyu/`.

Document the rule in the function's doc comment.

---

## Fix 6: Deterministic ordering in all generated JSON

Build outputs must be byte-for-byte reproducible across runs. The current generator uses `HashMap` for keys and `Vec` for unordered collections, so JSON output reorders unpredictably and diffs are noisy.

Apply consistently across all six data files:

- **`categories.json`** — outer keys (categories) and inner subcategory keys must be in stable order. Use `BTreeMap` for the top-level structure. Within each category, sort `sources` and `concepts` slug arrays alphabetically.
- **`breadcrumbs.json`** — top-level keys (slugs) sorted; breadcrumb trails are inherently ordered by hierarchy depth.
- **`see-also.json`** — top-level keys (slugs) sorted; per-slug, sort entries by relationship type then slug within each type.
- **`provenance.json`** — top-level keys (slugs) sorted; `derived_from` and `concepts_generated` arrays sorted by slug.
- **`graph.json`** — `nodes` keys sorted; `edges` array sorted by `(from, to, type)` tuple.
- **`profile.json`** — keys appear in a deliberate order (per the spec example structure); use `BTreeMap` or a struct with `#[serde(rename_all)]` ensuring stable ordering. `trust.levels` keys sorted alphabetically; `level_order` is the explicit semantic ordering.

Verification: run `haleiki demo build` twice in a row, `diff -r` the two `_data/` outputs — should be empty. Ditto for any change-irrelevant rebuild.

---

## Smaller cleanup while you're in there

- **Validator output grouping**: when reporting many issues, group by kind ("23 unknown-category errors, 8 dangling-provenance errors") rather than line-by-line. Stats-style summary first, then per-issue detail. Reduces noise during validation runs.

---

## Verification

After all fixes:

1. **Build**: `cargo build --release --features demo --manifest-path tools/Cargo.toml` succeeds without warnings.

2. **Validate**: `./target/release/haleiki validate --content-dir demo`
   - Should exit 0
   - Should report 0 errors
   - Should report 0 broken-cross-reference warnings (the check is gone)
   - Should report any orphans, bidirectional mismatches, etc. that legitimately exist
   - Should NOT flag the 17 source/concept slug overlaps as duplicates (they're correct-by-design under Path B)

3. **Stats**: `./target/release/haleiki stats --content-dir demo`
   - May include the new "X typed-relationship slugs reference content not yet in the corpus" line — cosmetic, count-only, not an error.

4. **Demo build**: `./target/release/haleiki demo build` runs end-to-end, regenerates `demo/_build/_data/`, cobalt build still succeeds.

5. **Spot-check the data files**:

   - `demo/_build/_data/categories.json` — paste the `tibetan-buddhism` entry. Should have `name`, `description`, top-level `sources` and `concepts` arrays, nested `subcategories` with `{sources, concepts}` per subcategory. No duplicates within any single array.

   - `demo/_build/_data/profile.json` — paste the full content. Should be the nested shape with `trust.enabled`, `trust.levels.corroborated.label`, `trust.level_order`, `canon.enabled`, etc., plus the `taxonomy` block.

   - `demo/_build/_data/see-also.json` — paste the `nyingma` (concept-card) entry. Compare to before: references to `kagyu`, `vajrayana`, `dzogchen` (which exist as both source and concept) should now resolve to the concept-card URLs (`/kagyu/`, `/vajrayana/`, `/dzogchen/`), not the source-page URLs (`/source/kagyu/`, etc.) — *except* `derived_from` entries which always resolve to source URLs.

6. **Doc check**: `docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md` and `0002-a-guide-for-parallel-concept-card-extraction.md` updated to v1.3 with the changes above.

---

## Reporting

Standard pattern. Report:

1. **Branch/commit details**.
2. **Doc diff stat** for 0001 and 0002.
3. **Code diff stat** for the four Rust modules.
4. **Validation output** verbatim (`haleiki validate --content-dir demo`).
5. **Spot-check pastes** for the three data files (categories tibetan-buddhism, profile, nyingma see-also).
6. **Anything that surprised you** during implementation.

Don't commit before reporting — Duncan will review the fix-pass before deciding to land.
