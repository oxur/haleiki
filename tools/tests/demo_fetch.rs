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

// --- Batch dry-run tests ---

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_batch_dry_run() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Dry run")
                .and(predicate::str::contains("articles"))
                .and(predicate::str::contains("dzogchen"))
                .and(predicate::str::contains("quantum-mechanics"))
                .and(predicate::str::contains("group-theory")),
        );
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_batch_dry_run_shows_urls() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("api/rest_v1/page/html/Dzogchen")
                .and(predicate::str::contains(
                    "api/rest_v1/page/html/Quantum_mechanics",
                ))
                .and(predicate::str::contains(
                    "api/rest_v1/page/html/Group_theory",
                )),
        );
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_batch_dry_run_shows_would_fetch_count() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(predicate::str::contains("Would fetch:"));
}

#[test]
#[ignore] // Requires network access, takes ~60s
#[cfg(feature = "demo")]
fn test_demo_fetch_batch_live() {
    use std::path::Path;

    let staging_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("demo/.staging");

    // Clean staging directory
    if staging_dir.exists() {
        for entry in std::fs::read_dir(&staging_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path
                .extension()
                .map_or(false, |e| e == "html" || e == "json")
            {
                let _ = std::fs::remove_file(&path);
            }
        }
    }

    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .timeout(std::time::Duration::from_secs(300))
        .assert()
        .success()
        .stdout(predicate::str::contains("Fetched:"));

    // Spot-check a few expected files
    let spot_check_slugs = ["dzogchen", "quantum-mechanics", "group-theory"];
    for slug in &spot_check_slugs {
        let html = staging_dir.join(format!("{slug}.html"));
        assert!(html.exists(), "Missing: {}", html.display());
        let meta = staging_dir.join(format!("{slug}.meta.json"));
        assert!(meta.exists(), "Missing: {}", meta.display());
    }

    // Second run should skip all (cached)
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(predicate::str::contains("Skipped:"));

    // Clean up
    if staging_dir.exists() {
        for entry in std::fs::read_dir(&staging_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path
                .extension()
                .map_or(false, |e| e == "html" || e == "json")
            {
                let _ = std::fs::remove_file(&path);
            }
        }
    }
}

// --- Rigpa Wiki dry-run tests ---

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_dry_run_rigpawiki_uses_action_parse() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--article", "longchenpa", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("api.php?action=parse")
                .and(predicate::str::contains("www.rigpawiki.org")),
        );
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_batch_dry_run_shows_both_api_styles() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("api/rest_v1/page/html/")
                .and(predicate::str::contains("api.php?action=parse")),
        );
}

// --- Rigpa Wiki live test ---

#[test]
#[ignore] // Requires network access
#[cfg(feature = "demo")]
fn test_demo_fetch_rigpawiki_article_live() {
    use std::path::Path;

    let staging_html = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("demo/.staging/longchenpa.html");

    // Clean up from previous runs
    let _ = std::fs::remove_file(&staging_html);
    let _ = std::fs::remove_file(staging_html.with_extension("meta.json"));

    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--article", "longchenpa"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success();

    let staging_html = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("demo/.staging/longchenpa.html");
    assert!(staging_html.exists(), "Rigpa Wiki HTML was not created");

    let html = std::fs::read_to_string(&staging_html).unwrap();
    assert!(
        html.len() > 500,
        "HTML seems too short: {} bytes",
        html.len()
    );

    // Clean up
    let _ = std::fs::remove_file(&staging_html);
    let _ = std::fs::remove_file(staging_html.with_extension("meta.json"));
}

// --- Single-article live test ---

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
