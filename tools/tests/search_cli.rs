//! Integration test for `haleiki search` — Arc 04 Slice 01, G-8: the CLI
//! subcommand builds the Pagefind bundle standalone (not just via `export`).

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_search_cli_writes_pagefind_bundle_from_demo_content() {
    let out_dir = std::env::temp_dir().join("haleiki-search-cli-test");
    let _ = std::fs::remove_dir_all(&out_dir);

    Command::cargo_bin("haleiki")
        .unwrap()
        .arg("search")
        .arg("--content-dir")
        .arg("demo")
        .arg("--out-dir")
        .arg(&out_dir)
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stderr(
            predicate::str::contains("Building search index")
                .and(predicate::str::contains("Done.")),
        );

    let bundle = out_dir.join("pagefind");
    assert!(bundle.join("pagefind.js").exists());
    assert!(bundle.join("pagefind-entry.json").exists());
    assert!(bundle.join("fragment").is_dir());
    assert!(bundle.join("index").is_dir());
    // No _site/ page output — `search` builds only the bundle, not the export.
    assert!(!out_dir.join("index.html").exists());

    let _ = std::fs::remove_dir_all(&out_dir);
}
