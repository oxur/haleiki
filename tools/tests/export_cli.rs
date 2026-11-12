//! Integration test for `haleiki export` — Arc 02 Slice 04, F-6: the CLI
//! subcommand drives the static export end-to-end.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_export_cli_produces_site_from_demo_content() {
    let out_dir = std::env::temp_dir().join("haleiki-export-cli-test");
    let _ = std::fs::remove_dir_all(&out_dir);

    Command::cargo_bin("haleiki")
        .unwrap()
        .arg("export")
        .arg("--content-dir")
        .arg("demo")
        .arg("--out-dir")
        .arg(&out_dir)
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stderr(predicate::str::contains("Exporting").and(predicate::str::contains("Done.")));

    assert!(out_dir.join("index.html").exists());
    assert!(out_dir.join("404.html").exists());
    assert!(out_dir.join("browse/index.html").exists());
    assert!(out_dir.join("category/mathematics/index.html").exists());
    assert!(out_dir.join("abelian-group/index.html").exists());
    assert!(out_dir.join("source/vimalamitra/index.html").exists());
    let news_post = out_dir.join("news/search-lands-in-the-demo-wiki/index.html");
    assert!(news_post.exists());
    let news_html = std::fs::read_to_string(news_post).expect("news post is readable");
    assert!(news_html.contains("Search Lands in the Demo Wiki"));
    assert!(out_dir.join("assets/css/haleiki.css").exists());
    assert!(!out_dir.join("assets/ai").exists());

    let _ = std::fs::remove_dir_all(&out_dir);
}
