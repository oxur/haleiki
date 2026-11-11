#!/usr/bin/env python3
"""Back-fill source-page frontmatter with extraction status and concept lists."""

import os
import re
import yaml
from collections import defaultdict
from pathlib import Path

CONCEPTS_DIR = Path("demo/concepts")
SOURCES_DIR = Path("demo/sources")
BACKFILL_DATE = "2026-05-04"


def build_source_to_concepts_map():
    """Scan concept cards and build source-slug → [concept-slugs] mapping."""
    source_map = defaultdict(list)

    for card_path in sorted(CONCEPTS_DIR.glob("*.md")):
        if card_path.name == ".gitkeep":
            continue

        content = card_path.read_text()

        # Extract frontmatter
        parts = content.split("---", 2)
        if len(parts) < 3:
            print(f"  WARNING: {card_path.name} has no valid frontmatter")
            continue

        try:
            fm = yaml.safe_load(parts[1])
        except yaml.YAMLError as e:
            print(f"  WARNING: {card_path.name} YAML parse error: {e}")
            continue

        if not fm or not isinstance(fm, dict):
            continue

        concept_slug = fm.get("slug", "")
        derived_from = fm.get("derived_from", [])

        if not derived_from or not isinstance(derived_from, list):
            continue

        for entry in derived_from:
            if isinstance(entry, dict):
                source_slug = entry.get("slug", "")
            elif isinstance(entry, str):
                source_slug = entry
            else:
                continue

            if source_slug:
                source_map[source_slug].append(concept_slug)

    # Sort each list alphabetically for stability
    for slug in source_map:
        source_map[slug] = sorted(set(source_map[slug]))

    return source_map


def update_source_page(source_path, concept_slugs):
    """Update a source page's frontmatter with extraction status fields."""
    content = source_path.read_text()

    # Determine status
    if concept_slugs:
        status = "complete"
    else:
        status = "skipped"

    # Build the concepts_generated YAML block
    if concept_slugs:
        lines = ["concepts_generated:"]
        for slug in concept_slugs:
            lines.append(f"  - {slug}")
        concepts_block = "\n".join(lines)
    else:
        concepts_block = "concepts_generated: []"

    # Replace extraction_status
    content = re.sub(
        r"^extraction_status:.*$",
        f"extraction_status: {status}",
        content,
        count=1,
        flags=re.MULTILINE,
    )

    # Replace concepts_generated (handle both [] and multi-line forms)
    # First try the simple [] form
    if "concepts_generated: []" in content:
        content = content.replace("concepts_generated: []", concepts_block, 1)
    else:
        # Handle existing multi-line form
        content = re.sub(
            r"^concepts_generated:.*?(?=^\w|\Z)",
            concepts_block + "\n",
            content,
            count=1,
            flags=re.MULTILINE | re.DOTALL,
        )

    # Insert or replace last_extracted after concepts_generated block
    if "last_extracted:" in content:
        content = re.sub(
            r"^last_extracted:.*$",
            f"last_extracted: {BACKFILL_DATE}",
            content,
            count=1,
            flags=re.MULTILINE,
        )
    else:
        # Insert after the concepts_generated block (before status: line)
        content = re.sub(
            r"(concepts_generated:.*?\n(?:  - .*\n)*)(status:)",
            rf"\g<1>last_extracted: {BACKFILL_DATE}\n\g<2>",
            content,
            count=1,
        )

    source_path.write_text(content)
    return status


def main():
    print("Building source → concept mapping...")
    source_map = build_source_to_concepts_map()

    print(f"Found {len(source_map)} source pages with extracted concepts")
    total_links = sum(len(v) for v in source_map.values())
    print(f"Total concept-card → source-page links: {total_links}")

    complete = 0
    skipped = 0
    errors = []

    source_pages = sorted(SOURCES_DIR.glob("*.md"))
    source_pages = [p for p in source_pages if p.name != ".gitkeep"]

    for source_path in source_pages:
        slug = source_path.stem
        concept_slugs = source_map.get(slug, [])

        try:
            status = update_source_page(source_path, concept_slugs)
            if status == "complete":
                complete += 1
            else:
                skipped += 1
        except Exception as e:
            errors.append((slug, str(e)))
            print(f"  ERROR: {slug}: {e}")

    print()
    print("=== Results ===")
    print(f"Total source pages updated: {complete + skipped}")
    print(f"  Complete: {complete}")
    print(f"  Skipped: {skipped}")
    print(f"Total concept links written: {total_links}")

    if errors:
        print(f"\nErrors ({len(errors)}):")
        for slug, err in errors:
            print(f"  {slug}: {err}")

    if skipped > 0:
        print(f"\nSkipped source pages (no concepts extracted):")
        for source_path in source_pages:
            slug = source_path.stem
            if slug not in source_map:
                print(f"  {slug}")


if __name__ == "__main__":
    main()
