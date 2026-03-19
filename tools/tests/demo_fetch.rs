//! Integration tests for `haleiki demo fetch`.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_dry_run_single_article() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--article", "dzogchen", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Would fetch")
                .and(predicate::str::contains("Dzogchen"))
                .and(predicate::str::contains("en.wikipedia.org"))
                .and(predicate::str::contains("api/rest_v1/page/html")),
        );
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_unknown_slug_fails() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--article", "nonexistent-article"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found in manifest"));
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_dry_run_rigpawiki_shows_project() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args([
            "demo",
            "fetch",
            "--article",
            "yangthang-rinpoche",
            "--dry-run",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(predicate::str::contains("www.rigpawiki.org"));
}

#[test]
#[ignore] // Requires network access
#[cfg(feature = "demo")]
fn test_demo_fetch_single_article_live() {
    use std::path::Path;

    let staging_html = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("demo/.staging/dzogchen.html");

    // Clean up from previous runs
    let _ = std::fs::remove_file(&staging_html);

    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--article", "dzogchen"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stderr(predicate::str::contains("Fetching"));

    // Verify file was created
    assert!(staging_html.exists(), "Staging HTML was not created");

    let html = std::fs::read_to_string(&staging_html).unwrap();
    assert!(
        html.len() > 1000,
        "HTML seems too short: {} bytes",
        html.len()
    );

    // Verify metadata was created
    let meta_path = staging_html.with_extension("meta.json");
    assert!(meta_path.exists(), "Metadata JSON was not created");

    // Clean up
    let _ = std::fs::remove_file(&staging_html);
    let _ = std::fs::remove_file(&meta_path);
}
