//! Integration test for `haleiki demo status`.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
#[cfg(feature = "demo")]
fn test_demo_status_prints_article_table() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .arg("demo")
        .arg("status")
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("SLUG")
                .and(predicate::str::contains("CATEGORY"))
                .and(predicate::str::contains("STATUS"))
                .and(predicate::str::contains("memory-management"))
                .and(predicate::str::contains("garbage-collection"))
                .and(predicate::str::contains("12 articles"))
                .and(predicate::str::contains("missing")),
        );
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_status_shows_wikibooks_project() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .arg("demo")
        .arg("status")
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(predicate::str::contains("en.wikibooks.org"));
}
