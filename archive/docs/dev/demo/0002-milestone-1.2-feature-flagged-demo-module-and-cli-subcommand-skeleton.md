# Milestone 1.2 — Feature-Flagged `demo` Module and CLI Subcommand Skeleton

**Version:** 1.0
**Depends on:** Milestone 1.1 (directory structure exists, but not strictly required for compilation)
**Produces:** Compiling Haleiki CLI with `haleiki demo fetch`, `haleiki demo build`, etc. all recognized

---

## Overview

This milestone creates the entire Rust project from scratch. There is some  existing placeholder Rust code in the repo. We must:

1. Create `tools/`
2. Move `Cargo.toml` to `tools/Cargo.toml` and update with core + optional (demo) dependencies
3. Move `src/` to `tools/`
4. Update the CLI entry point (`tools/src/main.rs`) with clap derive for ALL commands from the architecture doc
5. Create stub modules for core commands (`parser.rs`, `graph.rs`, `validator.rs`, `generator.rs`, `merger.rs`, `search.rs`)
6. Create `tools/src/demo/mod.rs` with the `DemoCommand` enum, feature-gated
7. Verify compilation with and without the `demo` feature
8. Update the `Makefile` to reference `tools/` instead of `crates/haleiki` or `src`

---

## Step 1

Per the instrcutions above.

## Step 2: Update `tools/Cargo.toml`

After moving `Cargo.toml` to `tools/Cargo.toml` you will need to update it.

### File: `tools/Cargo.toml`

```toml
[package]
name = "haleiki"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
description = "A static-site knowledge base framework"
license = "MIT OR Apache-2.0"

[[bin]]
name = "haleiki"
path = "src/main.rs"

[features]
default = []
demo = [
    "dep:reqwest",
    "dep:tokio",
    "dep:scraper",
    "dep:htmd",
    "dep:globset",
    "dep:indicatif",
]

[dependencies]
# ─── Core (always compiled) ───
clap = { version = "4", features = ["derive", "cargo", "wrap_help"] }
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1"
petgraph = "0.7"
thiserror = "2"
anyhow = "1"

# ─── Demo feature (optional) ───
reqwest = { version = "0.12", features = ["rustls-tls", "json"], optional = true }
tokio = { version = "1", features = ["full"], optional = true }
scraper = { version = "0.22", optional = true }
htmd = { version = "0.1", optional = true }
globset = { version = "0.4", optional = true }
indicatif = { version = "0.17", optional = true }

[dev-dependencies]
assert_cmd = "2"
assert_fs = "1"
predicates = "3"

[lints.rust]
unsafe_code = "deny"
missing_debug_implementations = "warn"
unused_lifetimes = "warn"

[lints.clippy]
pedantic = { level = "warn", priority = -1 }
cargo = { level = "warn", priority = -1 }
# Allow common pedantic noise
module_name_repetitions = "allow"
must_use_candidate = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
```

### Notes on dependency versions

- `petgraph`: Use `0.7` (latest as of early 2026), not `0.6` as listed in the design doc
- `scraper`: Use `0.22` (latest), not `0.20` as listed in the design doc
- `thiserror`: Use `2` (latest), the design doc doesn't specify
- Check actual latest versions with `cargo search <crate>` before writing. The versions above are best-effort — adjust to whatever is current.

Note that since this will be a workspace setup, you will need to create a new Cargo.toml file according to best practices for Rust workspace set ups.

---

## Step 3: Per the Summary Above

Move `src/` to `tools/`

## Step 4: Create the error module

### File: `tools/src/error.rs`

```rust
//! Centralized error types for Haleiki.

use thiserror::Error;

/// Top-level error type for the Haleiki CLI.
#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    Config(String),
}

/// Crate-level Result alias.
pub type Result<T> = std::result::Result<T, Error>;
```

---

## Step 5: Create stub modules for core commands

These are empty placeholder modules. They exist so the `mod` declarations in `main.rs`/`lib.rs` compile. Each will be implemented in future milestones.

### File: `tools/src/parser.rs`

```rust
//! Content parsing for source pages and concept cards.
```

### File: `tools/src/graph.rs`

```rust
//! Relationship graph construction and derived data computation.
```

### File: `tools/src/validator.rs`

```rust
//! Content validation: broken refs, orphans, conflicts, etc.
```

### File: `tools/src/generator.rs`

```rust
//! JSON data generation for the `_data/` directory.
```

### File: `tools/src/merger.rs`

```rust
//! Concept card merging logic.
```

### File: `tools/src/search.rs`

```rust
//! Search index generation.
```

---

## Step 6: Create `tools/src/cli.rs` — CLI argument definitions

This is the clap derive definition for the entire CLI surface. Following CLI-03 (separate library and binary) and CLI-05 (use clap derive for all argument parsing).

### File: `tools/src/cli.rs`

```rust
//! CLI argument definitions using clap derive.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Haleiki — A static-site knowledge base framework
#[derive(Debug, Parser)]
#[command(name = "haleiki", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run the full build pipeline (parse, graph, validate, generate)
    Build,

    /// Validate content without building
    Validate,

    /// Show graph statistics and health
    Stats,

    /// Generate search index only
    Search,

    /// Start development server with file watching
    Dev,

    /// Scaffold new content
    New {
        #[command(subcommand)]
        kind: NewCommand,
    },

    /// Extract concepts from a source page (AI-driven)
    Extract {
        /// Path to the source page to extract from
        source: PathBuf,
    },

    /// Manage pending concept merges
    Merges {
        #[command(subcommand)]
        action: MergesCommand,
    },

    /// Demo site management (fetch from Wikimedia, build, serve)
    #[cfg(feature = "demo")]
    Demo {
        #[command(subcommand)]
        command: super::demo::DemoCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum NewCommand {
    /// Scaffold a new source page
    Source {
        /// Title for the new source page
        title: String,
    },

    /// Scaffold a new concept card
    Concept {
        /// Name for the new concept card
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum MergesCommand {
    /// Show pending concept merges
    Pending,

    /// Accept a pending merge
    Accept {
        /// Slug of the concept to accept
        slug: String,
    },
}
```

### Design Rationale

- All commands from architecture doc section 5.1 are represented
- `Demo` variant is `#[cfg(feature = "demo")]` so the entire subcommand tree vanishes without the feature
- `New` and `Merges` use nested subcommands (CLI-10 pattern)
- `Extract` takes a `PathBuf` positional argument (CLI-06 pattern)
- Doc comments on every variant become `--help` text (CLI-14 pattern)

---

## Step 7: Create `tools/src/demo/mod.rs` — Demo subcommand module

### File: `tools/src/demo/mod.rs`

```rust
//! Demo site management — fetch Wikimedia content, build, serve.
//!
//! This entire module is compiled only when the `demo` feature is enabled.

pub mod manifest;

use clap::Subcommand;

/// Demo site subcommands.
#[derive(Debug, Subcommand)]
pub enum DemoCommand {
    /// Fetch articles from Wikimedia and convert to Haleiki source pages
    Fetch {
        /// Fetch only this article (by slug from manifest)
        #[arg(long)]
        article: Option<String>,

        /// Show what would be fetched without writing files
        #[arg(long)]
        dry_run: bool,

        /// Re-fetch even if content already exists locally
        #[arg(long)]
        force: bool,

        /// Use pandoc for HTML-to-Markdown instead of the built-in converter
        #[arg(long)]
        pandoc: bool,
    },

    /// Wire demo content into content/ and run the full build pipeline
    Build,

    /// Build and serve locally with file watching
    Serve,

    /// Show manifest vs. on-disk state for each article
    Status,

    /// Validate demo content integrity
    Validate,

    /// Generate the attribution page
    Attribution,

    /// Remove all generated demo content (preserves manifest and taxonomy)
    Clean,

    /// Clean and re-fetch all articles (full regeneration)
    Refresh,
}

/// Execute a demo subcommand.
pub fn run(cmd: &DemoCommand) -> anyhow::Result<()> {
    match cmd {
        DemoCommand::Fetch { article, dry_run, force, pandoc } => {
            let _ = (article, dry_run, force, pandoc);
            eprintln!("haleiki demo fetch: not yet implemented");
        }
        DemoCommand::Build => {
            eprintln!("haleiki demo build: not yet implemented");
        }
        DemoCommand::Serve => {
            eprintln!("haleiki demo serve: not yet implemented");
        }
        DemoCommand::Status => {
            eprintln!("haleiki demo status: not yet implemented");
        }
        DemoCommand::Validate => {
            eprintln!("haleiki demo validate: not yet implemented");
        }
        DemoCommand::Attribution => {
            eprintln!("haleiki demo attribution: not yet implemented");
        }
        DemoCommand::Clean => {
            eprintln!("haleiki demo clean: not yet implemented");
        }
        DemoCommand::Refresh => {
            eprintln!("haleiki demo refresh: not yet implemented");
        }
    }
    Ok(())
}
```

### File: `tools/src/demo/manifest.rs`

Placeholder for milestone 1.3:

```rust
//! Demo manifest parsing and validation.
//!
//! Deserializes `demo/manifest.yaml` and validates article entries
//! against the embedded taxonomy.
```

---

## Step 8: Create `tools/src/main.rs` — Entry point

Following CLI-04 (main function structure): thin main that parses args, delegates to a `run()` function, and handles exit codes.

### File: `tools/src/main.rs`

```rust
//! Haleiki CLI — A static-site knowledge base framework.

mod cli;
#[cfg(feature = "demo")]
mod demo;
mod error;
mod generator;
mod graph;
mod merger;
mod parser;
mod search;
mod validator;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(&cli.command) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(cmd: &Command) -> anyhow::Result<()> {
    match cmd {
        Command::Build => {
            eprintln!("haleiki build: not yet implemented");
        }
        Command::Validate => {
            eprintln!("haleiki validate: not yet implemented");
        }
        Command::Stats => {
            eprintln!("haleiki stats: not yet implemented");
        }
        Command::Search => {
            eprintln!("haleiki search: not yet implemented");
        }
        Command::Dev => {
            eprintln!("haleiki dev: not yet implemented");
        }
        Command::New { kind } => {
            let _ = kind;
            eprintln!("haleiki new: not yet implemented");
        }
        Command::Extract { source } => {
            let _ = source;
            eprintln!("haleiki extract: not yet implemented");
        }
        Command::Merges { action } => {
            let _ = action;
            eprintln!("haleiki merges: not yet implemented");
        }
        #[cfg(feature = "demo")]
        Command::Demo { command } => {
            demo::run(command)?;
        }
    }
    Ok(())
}
```

---

## Step 9: Update the Makefile

The existing Makefile references `crates/$(CODE_NAME)` for coverage but the project uses `tools/` for the CLI crate. The Makefile also has build targets that need to work with the new structure.

### Changes needed

1. **Coverage target**: Change `cd crates/$(CODE_NAME)` to `cd tools`
2. **Build targets**: The existing `cargo build` commands should work since they use workspace-level commands. But since there's no workspace Cargo.toml, we need to either:
   - Add a root `Cargo.toml` workspace file, OR
   - Change build commands to use `--manifest-path tools/Cargo.toml`

**Recommended approach**: Add a root `Cargo.toml` as a workspace manifest.

### File: `Cargo.toml` (project root — workspace manifest)

```toml
[workspace]
members = ["tools"]
resolver = "2"
```

### Makefile changes

In the Makefile, update the coverage target:

**Old:**

```makefile
coverage:
 ...
 @cd crates/$(CODE_NAME) && cargo llvm-cov --lib --no-default-features
```

**New:**

```makefile
coverage:
 ...
 @cd tools && cargo llvm-cov --lib --no-default-features
```

**Old:**

```makefile
coverage-html:
 ...
 @cd crates/$(CODE_NAME) && cargo llvm-cov --html --lib --no-default-features
```

**New:**

```makefile
coverage-html:
 ...
 @cd tools && cargo llvm-cov --html --lib --no-default-features
```

Also note: the Makefile's `BINARIES := $(CODE_NAME)` and `TARGET := ./target/$(MODE)` should work correctly with the workspace since cargo puts binaries in the workspace root's `target/` directory.

---

## Step 10: Verification

### 8.1: Build without demo feature

```bash
cd /Users/oubiwann/lab/oxur/haleiki
cargo build
echo $?  # Should be 0
```

The `demo` module and its dependencies should NOT be compiled.

### 8.2: Build with demo feature

```bash
cargo build --features demo
echo $?  # Should be 0
```

All demo dependencies (`reqwest`, `tokio`, `scraper`, `htmd`, `globset`, `indicatif`) should compile.

### 8.3: CLI help text shows all commands

```bash
# Core commands visible
cargo run -- --help
# Should show: build, validate, stats, search, dev, new, extract, merges

# Demo subcommands visible when feature enabled
cargo run --features demo -- --help
# Should show all of the above PLUS: demo

# Demo sub-subcommands
cargo run --features demo -- demo --help
# Should show: fetch, build, serve, status, validate, attribution, clean, refresh

# Fetch sub-options
cargo run --features demo -- demo fetch --help
# Should show: --article, --dry-run, --force, --pandoc
```

### 8.4: Stub commands print "not yet implemented"

```bash
cargo run --features demo -- demo status
# Should print: "haleiki demo status: not yet implemented"

cargo run -- build
# Should print: "haleiki build: not yet implemented"

cargo run -- new source "Test"
# Should print: "haleiki new: not yet implemented"
```

### 8.5: Linting and formatting pass

```bash
make lint
make format
```

### 8.6: Demo command not available without feature

```bash
cargo run -- demo status 2>&1
# Should fail with clap error: unrecognized subcommand 'demo'
```

---

## Acceptance Criteria

- [ ] `tools/Cargo.toml` exists with correct dependencies and feature configuration
- [ ] Root `Cargo.toml` exists as workspace manifest
- [ ] `cargo build` succeeds (no `demo` feature)
- [ ] `cargo build --features demo` succeeds
- [ ] `cargo run -- --help` shows all core commands (build, validate, stats, search, dev, new, extract, merges)
- [ ] `cargo run --features demo -- --help` additionally shows `demo`
- [ ] `cargo run --features demo -- demo --help` shows all 8 demo subcommands
- [ ] All stub commands print "not yet implemented" to stderr and exit 0
- [ ] `demo` subcommand is not available without the feature flag
- [ ] `make lint` passes (clippy + format check)
- [ ] `make format` produces no changes
- [ ] No compiler warnings
- [ ] All files have doc comments on public items

---

## Gotchas

1. **Workspace vs. single crate**: The Makefile assumes workspace-level `cargo` commands. A root `Cargo.toml` workspace manifest is needed to make `cargo build` from the repo root work with `tools/` as a member.

2. **`#[cfg(feature = "demo")]` placement**: The feature gate goes on:
   - The `mod demo;` declaration in `main.rs`
   - The `Demo` variant in the `Command` enum in `cli.rs`
   - The match arm for `Command::Demo` in `main.rs`
   All three are needed, or you'll get compilation errors.

3. **`super::demo::DemoCommand`**: In `cli.rs`, the `Demo` variant references `super::demo::DemoCommand`. This works because `cli.rs` is a sibling module to `demo/mod.rs`, and `super` goes up to the crate root where `mod demo` is declared.

4. **Clippy pedantic**: The `[lints.clippy]` section enables pedantic warnings. You may need to add `#[expect()]` annotations or `allow` directives for some patterns. Common ones:
   - `clippy::module_name_repetitions` — already allowed in Cargo.toml
   - `clippy::must_use_candidate` — already allowed
   - `clippy::wildcard_imports` — may trigger on `use super::*` in tests

5. **Dependency versions**: The versions in the design doc may be outdated. Run `cargo search <crate> --limit 1` for each to get the latest. Key ones to verify:
   - `petgraph` (design doc says 0.6, latest may be 0.7)
   - `scraper` (design doc says 0.20, latest may be 0.22)
   - `htmd` (design doc says 0.1, verify this exists and is the right crate)

6. **`let _ = (article, dry_run, force, pandoc);`**: This pattern in the stub `run()` function suppresses unused variable warnings without renaming parameters with `_` prefix. When the implementations are filled in, these lines are simply removed.

7. **`anyhow` vs custom `Error`**: The stub uses `anyhow::Result` for the demo `run()` function. Milestone 1.3 will decide whether to use `anyhow` or the custom `Error` type. Both are available.

8. **The `--force` flag**: The design doc's `DemoCommand::Fetch` doesn't explicitly list `--force`, but section 2.2 of the project plan says "skip articles whose staging HTML already exists unless `--force` is passed." Add it now to avoid a breaking CLI change later.

9. **Makefile `PROJECT_NAME`**: The Makefile header says `# Makefile for the Fermata Project` but `PROJECT_NAME := Haleiki`. The comment is from a template; leave it or fix it, but don't let it block.
