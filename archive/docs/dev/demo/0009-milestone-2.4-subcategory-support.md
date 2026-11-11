# Milestone 2.4 — Subcategory Support

**Version:** 1.0
**Depends on:** Milestone 1.3 (manifest parsing), should be done before Phase 3 begins
**Produces:** `subcategory` field supported in manifest, frontmatter, and validation

---

## Overview

Add an optional `subcategory` field to the manifest article schema and the generated source page frontmatter. Subcategories provide finer-grained classification within a category (e.g., category "music" → subcategory "jazz" or "baroque"). They are:

- **Optional** — articles can omit subcategory
- **Free-form** — not enumerated in taxonomy.yaml (unlike categories)
- **Validated** — `haleiki validate` warns on inconsistent usage patterns (same subcategory in different categories)
- **Used by** — breadcrumb paths, category listing pages, graph grouping

---

## Step 1: Update manifest types

### `tools/src/demo/manifest.rs`

Add `subcategory` to the `Article` struct:

```rust
/// A single article entry in the manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub slug: String,
    pub category: String,

    /// Optional subcategory within the category.
    #[serde(default)]
    pub subcategory: Option<String>,

    pub tier: String,

    #[serde(default)]
    pub project: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub media: Option<ArticleMedia>,
}
```

### Validation additions

Add to `Manifest::validate()`:

```rust
// Check subcategory consistency: warn if the same subcategory appears in different categories
let mut subcategory_to_category: HashMap<&str, &str> = HashMap::new();
for article in &self.articles {
    if let Some(ref subcat) = article.subcategory {
        if let Some(&existing_cat) = subcategory_to_category.get(subcat.as_str()) {
            if existing_cat != article.category.as_str() {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: format!(
                        "subcategory \"{}\" used in both \"{}\" and \"{}\" categories",
                        subcat, existing_cat, article.category,
                    ),
                });
            }
        } else {
            subcategory_to_category.insert(subcat.as_str(), article.category.as_str());
        }

        // Validate subcategory format (kebab-case)
        if subcat != &subcat.to_lowercase() {
            issues.push(ValidationIssue {
                article: Some(article.slug.clone()),
                message: "subcategory contains uppercase characters".to_string(),
            });
        }
        if subcat.contains(' ') {
            issues.push(ValidationIssue {
                article: Some(article.slug.clone()),
                message: "subcategory contains spaces (use hyphens)".to_string(),
            });
        }
    }
}
```

---

## Step 2: Add subcategories to demo manifest

Update `demo/manifest.yaml` — add subcategories where they make sense. Examples:

```yaml
# Tibetan Buddhism subcategories
  - title: "Dzogchen"
    slug: "dzogchen"
    category: "tibetan-buddhism"
    subcategory: "practice"
    # ...

  - title: "Chagdud Tulku Rinpoche"
    slug: "chagdud-tulku-rinpoche"
    category: "tibetan-buddhism"
    subcategory: "teachers"
    # ...

  - title: "Nyingma"
    slug: "nyingma"
    category: "tibetan-buddhism"
    subcategory: "schools"
    # ...

# Music subcategories
  - title: "Johann Sebastian Bach"
    slug: "johann-sebastian-bach"
    category: "music"
    subcategory: "composers"
    # ...

  - title: "Jazz"
    slug: "jazz"
    category: "music"
    subcategory: "genres"
    # ...

  - title: "Music theory"
    slug: "music-theory"
    category: "music"
    subcategory: "theory"
    # ...

# Geology subcategories
  - title: "Earth's crust"
    slug: "earths-crust"
    category: "geology"
    subcategory: "earth-structure"
    # ...

  - title: "Tectonics"
    slug: "tectonics"
    category: "geology"
    subcategory: "processes"
    # ...
```

Not every article needs a subcategory. Overview/foundational articles (e.g., "Physics", "Language") may omit it.

---

## Step 3: Update frontmatter generation (milestone 5.3 impact)

When `frontmatter.rs` generates YAML frontmatter for converted source pages, include `subcategory` if present in the manifest:

```yaml
# === CLASSIFICATION ===
category: "tibetan-buddhism"
subcategory: "teachers"          # Only present if set in manifest
tier: "intermediate"
```

This is a note for the implementer of milestone 5.3, not code to write now.

---

## Step 4: Update status display

### `tools/src/demo/status.rs`

Optionally show subcategory in the status table. Since column space is limited, consider showing it as `category/subcategory`:

```
  SLUG                         CATEGORY                       TIER         STATUS
  ───────────────────────────────────────────────────────────────────────────────────
  dzogchen                     tibetan-buddhism/practice      foundational missing
  chagdud-tulku-rinpoche       tibetan-buddhism/teachers      intermediate missing
  nyingma                      tibetan-buddhism/schools        foundational missing
  physics                      theoretical-physics             foundational missing
```

---

## Step 5: Write tests

Add to the `#[cfg(test)]` module in `manifest.rs`:

```rust
#[test]
fn test_article_subcategory_optional() {
    let yaml = r#"
defaults:
  project: "en.wikipedia.org"
  license: "CC BY-SA 4.0"
  media:
    max_width: 1024
    formats: ["png"]
    skip_patterns: []
taxonomy:
  categories: ["music"]
  tiers: ["foundational"]
articles:
  - title: "Jazz"
    slug: "jazz"
    category: "music"
    tier: "foundational"
"#;
    let m: Manifest = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(m.articles[0].subcategory, None);
}

#[test]
fn test_article_subcategory_present() {
    let yaml = r#"
defaults:
  project: "en.wikipedia.org"
  license: "CC BY-SA 4.0"
  media:
    max_width: 1024
    formats: ["png"]
    skip_patterns: []
taxonomy:
  categories: ["music"]
  tiers: ["foundational"]
articles:
  - title: "Jazz"
    slug: "jazz"
    category: "music"
    subcategory: "genres"
    tier: "foundational"
"#;
    let m: Manifest = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(m.articles[0].subcategory, Some("genres".to_string()));
}

#[test]
fn test_validate_subcategory_cross_category_warns() {
    let mut m = sample_manifest();
    m.articles[0].subcategory = Some("overlap".to_string());
    m.articles[0].category = "cat-a".to_string();
    m.articles[1].subcategory = Some("overlap".to_string());
    m.articles[1].category = "cat-b".to_string();
    // Add both categories to taxonomy so they pass category validation
    m.taxonomy.categories.push("cat-a".to_string());
    m.taxonomy.categories.push("cat-b".to_string());

    let issues = m.validate();
    assert!(
        issues.iter().any(|i| i.message.contains("subcategory") && i.message.contains("both")),
        "Expected cross-category subcategory warning, got: {issues:?}"
    );
}

#[test]
fn test_validate_subcategory_uppercase_warns() {
    let mut m = sample_manifest();
    m.articles[0].subcategory = Some("BadCase".to_string());
    let issues = m.validate();
    assert!(
        issues.iter().any(|i| i.message.contains("subcategory") && i.message.contains("uppercase")),
        "Expected uppercase warning, got: {issues:?}"
    );
}
```

---

## Verification

```bash
# Parse manifest with subcategories
cargo test --features demo

# Status shows subcategories
cargo run --features demo -- demo status

# Lint passes
make lint
```

---

## Acceptance Criteria

- [ ] `Article` struct has `subcategory: Option<String>` with `#[serde(default)]`
- [ ] Manifest YAML with and without `subcategory` parses correctly
- [ ] Validation warns on subcategory used across different categories
- [ ] Validation warns on uppercase or space-containing subcategories
- [ ] `haleiki demo status` displays subcategory when present
- [ ] Demo manifest updated with subcategories for appropriate articles
- [ ] All tests pass
- [ ] `make lint` passes

---

## Gotchas

1. **Backward compatibility**: Since `subcategory` is `Option<String>` with `#[serde(default)]`, existing manifests without subcategory fields will parse without changes. No migration needed.

2. **Taxonomy file**: Subcategories are NOT enumerated in `taxonomy.yaml`. They're free-form, validated only for consistency (same subcategory shouldn't appear in different categories). This is deliberate — enumerating them would create maintenance burden as content grows.

3. **Impact on graph**: Subcategories may affect breadcrumb paths and category listing pages. The graph builder (future milestone) should use subcategories for finer grouping when present.

4. **Impact on frontmatter generation**: Milestone 5.3 (`frontmatter.rs`) needs to include `subcategory` in generated frontmatter. This is a small addition, noted here for tracking.
