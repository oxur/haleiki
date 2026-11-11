#!/usr/bin/env python3
"""Verify the frozen planning migration, without running historical instructions."""

import argparse
import hashlib
import json
import re
import subprocess
from pathlib import Path
from urllib.parse import unquote


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    source = args.source.resolve()
    manifest = json.loads((root / 'migration-manifest.json').read_text())
    baseline = manifest['source_commit']

    def git(*argv):
        return subprocess.check_output(['git', '-C', str(source), *argv])

    def digest(data):
        return hashlib.sha256(data).hexdigest()

    entries = manifest['files']
    assert len({e['source'] for e in entries}) == len(entries), 'Duplicate source mapping'
    assert len({e['destination'] for e in entries}) == len(entries), 'Duplicate destination'
    present = pinned = ignored_absent = 0
    for entry in entries:
        retained = root / entry['destination']
        assert retained.is_file(), f'Missing retained file: {retained}'
        data = retained.read_bytes()
        assert len(data) == entry['bytes'], f'Size mismatch: {retained}'
        assert digest(data) == entry['sha256'], f'Payload mismatch: {retained}'
        assert retained.stat().st_mode & 0o777 == entry['mode'], f'Mode mismatch: {retained}'
        old = source / entry['source']
        if old.is_file():
            assert digest(old.read_bytes()) == entry['sha256'], f'Changed original: {old}'
            present += 1
        elif entry['tracked']:
            assert digest(git('show', f'{baseline}:{entry["source"]}')) == entry['sha256']
            pinned += 1
        else:
            # These were checked while present before deletion, and their exact
            # retained bytes are still checked above. Git never held them.
            ignored_absent += 1
    print(f'PASS: {len(entries)} byte-identical payloads; original present={present}, '
          f'pinned Git original={pinned}, formerly ignored/untracked={ignored_absent}')

    scope = ('docs/design/', 'docs/dev/', 'workbench/', 'docs/tiny-http/')
    tracked = set(git('ls-tree', '-r', '--name-only', baseline).decode().splitlines())
    expected_tracked = {p for p in tracked if p.startswith(scope)} | {'odm.toml'}
    actual_tracked = {e['source'] for e in entries if e['tracked']}
    assert expected_tracked == actual_tracked, 'Tracked population mismatch'
    for prefix in scope:
        old_root = source / prefix
        if old_root.exists():
            files = {str(p.relative_to(source)) for p in old_root.rglob('*') if p.is_file()}
            expected = {e['source'] for e in entries if e['source'].startswith(prefix)}
            assert files == expected, f'Current source population differs: {prefix}'
    print(f'PASS: complete tracked population ({len(expected_tracked)}); existing local roots covered')

    pivot = root / 'project04-tiny-http'
    ledger_rows = 0
    for original in (root / 'archive/docs/tiny-http').rglob('*plan.md'):
        relative = original.relative_to(root / 'archive/docs/tiny-http')
        current = pivot / relative
        ledger = current.with_name('ledger.md')
        pattern = r'^\| [A-Z]-\d+ .*\|$'
        before = re.findall(pattern, original.read_text(), flags=re.M)
        after = re.findall(pattern, ledger.read_text(), flags=re.M)
        assert before and before == after, f'Ledger rows changed: {current}'
        assert not re.findall(pattern, current.read_text(), flags=re.M), f'Duplicate ledger: {current}'
        ledger_rows += len(before)
    print(f'PASS: {ledger_rows} project/arc ledger rows extracted without changes')

    retained_paths = {e['destination'] for e in entries}
    checked_links = 0
    for document in root.rglob('*.md'):
        relative = str(document.relative_to(root))
        if relative in retained_paths:
            continue  # Preserve, rather than rewrite, old paths inside history.
        text = document.read_text()
        for match in re.finditer(r'\[[^\]]*\]\(([^)]+)\)', text):
            target = match[1].strip('<>')
            if target.startswith(('http:', 'https:', '#', 'mailto:')):
                continue
            target = unquote(target.split('#', 1)[0])
            assert (document.parent / target).exists(), f'Broken authored link: {document}: {target}'
            checked_links += 1
    print(f'PASS: {checked_links} links in authored/canonical Markdown; historical links retained as-is')

    names = set(git('diff', '--name-only', baseline).decode().splitlines())
    allowed = {e['source'] for e in entries} | {'AGENTS.md', 'README.md', '.gitignore'}
    assert not (names - allowed), f'Unexpected source edits: {sorted(names - allowed)}'
    print('PASS: implementation, fixtures, assets and CI unchanged from migration baseline')

    branch = subprocess.check_output(['git', '-C', str(root), 'branch', '--show-current'], text=True).strip()
    assert branch == 'planning', branch
    history = subprocess.run(['git', '-C', str(root), 'rev-parse', '--verify', 'HEAD'],
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if history.returncode == 0:
        roots = subprocess.check_output(['git', '-C', str(root), 'rev-list', '--max-parents=0', 'HEAD'], text=True).splitlines()
        assert len(roots) == 1
        assert subprocess.run(['git', '-C', str(root), 'merge-base', '--is-ancestor', baseline, 'HEAD']).returncode == 1
        print('PASS: orphan planning history; no implementation ancestry')
    else:
        print('PASS: unborn planning branch before initial commit')


if __name__ == '__main__':
    main()
