# Source-Page Back-Link Back-Fill, v1

One-time back-fill of source-page frontmatter to populate `extraction_status`, `concepts_generated`, and `last_extracted`. The pilot and corpus concept-card extractions were done before v1.2 of the howto/parallel-guide docs called for source-page updates, so all 91 source pages still show `extraction_status: pending` and `concepts_generated: []` despite 186 concept cards having been extracted from them. This pass closes the loop.

---

## Reference docs

- `docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md` (v1.2) — see § "Source Page Updates" and Golden Rule 5
- `docs/dev/concept-cards/0002-a-guide-for-parallel-concept-card-extraction.md` (v1.2) — see § Step 3.5.5 (the validation step we'll run after the back-fill)

---

## Goal

For every source page in `demo/sources/`, populate three frontmatter fields based on a scan of `demo/concepts/`:

```yaml
extraction_status: complete       # was: pending
concepts_generated:
  - <concept-slug-1>              # extracted from this source page
  - <concept-slug-2>
  - ...
last_extracted: 2026-05-04        # date of this back-fill, ISO format
```

Source pages with zero substantively-extracted concepts (none of their slug appears in any concept card's `derived_from`) get `extraction_status: skipped` and `concepts_generated: []`. This is rare but legitimate — the pre-v1.2 extractions may have processed a source page and concluded it had no substantive concepts.

---

## Approach

Write a small one-off Python script (`workbench/backfill-source-pages.py` or wherever fits — this is a throwaway, not committed source code) that:

1. **Builds the source→concept mapping** by scanning `demo/concepts/*.md`:
   - Parse each concept card's YAML frontmatter (use a YAML library; PyYAML is fine)
   - Extract the concept's `slug` and the list of source-page slugs in `derived_from`
   - For each source-page slug in `derived_from`, append the concept slug to that source's list

2. **Updates each source page** in `demo/sources/*.md`:
   - Read the file
   - Locate the frontmatter block (between the first two `---` lines)
   - **Use line-based edits, NOT YAML round-trip**, to avoid reordering or reformatting the existing fields. The demo source pages have a deliberate frontmatter structure that's nice to preserve.
   - Replace `extraction_status: pending` with `extraction_status: complete` (or `skipped` if the source has no extracted concepts)
   - Replace `concepts_generated: []` with a multi-line YAML list of the concept slugs (sorted alphabetically for stability)
   - Insert `last_extracted: 2026-05-04` (use today's date, ISO format) after the `concepts_generated` block. If the field already exists, replace it.
   - Write the updated file

3. **Reports counts** at the end:
   - Total source pages updated
   - Total source pages marked `complete` vs `skipped`
   - Total concept-card→source-page links represented (sum of all `concepts_generated` entries)
   - Any source pages that produced an unexpected error or missing slug

---

## Important details

- **Line-based, not parse-and-dump.** A naive `yaml.load` followed by `yaml.dump` will reorder fields, change quoting style, possibly break multi-line strings. Don't do that. Use targeted regex or string replacement on the frontmatter region. The existing fields you're modifying have predictable patterns (`extraction_status: pending`, `concepts_generated: []`).

- **Sort `concepts_generated` alphabetically** for diff stability. Future runs will produce the same ordering.

- **Multi-source case is automatic.** A concept like `nyingma` (derived from both `padmasambhava` and `longchenpa`) will naturally appear in both `padmasambhava`'s and `longchenpa`'s `concepts_generated` lists, because the script aggregates by source slug.

- **Skipped vs. complete:** if a source page's slug appears in zero `derived_from` entries across the 186 concept cards, mark it `skipped`. If it appears in any, mark it `complete`. With 91 source pages and 186 concepts, expect most to be `complete`; a handful may be `skipped` if they were processed but yielded no substantive concepts.

- **Existing `last_reviewed` or other fields** in the source-page frontmatter should NOT be modified. Only touch the three target fields.

---

## Verification

After running the script, verify the back-fill:

1. **Spot-check 5 source pages** — pick `padmasambhava`, `longchenpa`, `quantum-mechanics`, `rust-language`, `group-theory` (the pilot's five). Show me the resulting frontmatter for each. The pilot extracted 27 cards from these 5 source pages, so each should have several concepts in its `concepts_generated`.

2. **Run the bidirectional consistency check from `0002-a-guide-for-parallel-concept-card-extraction.md` § 3.5.5** — both shell loops, asserting that every `derived_from` entry has a matching `concepts_generated` entry and vice versa. There should be zero mismatches.

3. **Count final state**:
   ```bash
   echo "Source pages still pending: $(grep -l '^extraction_status: pending' demo/sources/*.md | wc -l)"
   echo "Source pages complete: $(grep -l '^extraction_status: complete' demo/sources/*.md | wc -l)"
   echo "Source pages skipped: $(grep -l '^extraction_status: skipped' demo/sources/*.md | wc -l)"
   ```

   Total complete + skipped should equal 91. Pending should be 0.

---

## Reporting

Don't commit. Report:

1. **Script** — paste the final script you used (or a link if it's longer than a few dozen lines).
2. **Counts** — total updated, complete vs. skipped breakdown, total source-concept links written.
3. **Spot-check frontmatter** — pasted frontmatter (full top `---` block to second `---`) for the five pilot source pages.
4. **Validation report** — output of the bidirectional consistency check (Step 3.5.5). Should be empty (zero mismatches). If non-empty, paste the mismatches and stop for triage.
5. **Anything surprising** — source pages with unexpectedly-empty `concepts_generated`, source pages that errored, anything that didn't fit the script's assumptions.

---

This back-fill is reversible (the original `extraction_status: pending` and `concepts_generated: []` values were the demo pipeline's defaults; we can re-set them by running the demo fetch with `--force` for any specific page). So if anything looks wrong post-fill, easy to course-correct.
