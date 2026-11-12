# CC assignment — Arc 05 Slice 01, iteration 02: prove explicit null keys

Issued 2026-11-11 by CDC Locke (`01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`).
Project-relative path:
`arc05-news-blog-content-type/slice01-news-model-and-post/cc-prompt-iteration02.md`.
Predecessor `cc-prompt-iteration01.md` remains unchanged. This is the first
review-driven correction, after the first executed implementation pass.

## Authority, candidate and ownership

Operator authorization remains Autonomous Mode through end of Arc 05, local
scoped commits, stopping at discussion gates. No change to selected roles or
settings: CDC Locke OpenAI `gpt-6.1-sol / high`, CC Kierkegaard
`01a1ccf7-3e7c-7ab2-ae59-72c9be8f457b` explicitly selected `gpt-6-luna / high`.
Requested/tool-selected identities are recorded; backend identity is unreported.
Parent may resume that CC with the exact settings; no substitute/extra role.

`SOURCE=/Users/oubiwann/lab/oxur/haleiki`, branch `framework-base/tiny-http`,
clean candidate `cb31dc92caa667ce6f3a7133163c3da8749549b1`.
`PLANNING=/Users/oubiwann/lab/oxur/haleiki/.worktrees/planning`, branch `planning`.
`SLICE=$PLANNING/project04-tiny-http/arc05-news-blog-content-type/slice01-news-model-and-post`.
`SKILLS=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Planning review/issuance follows `2d5e148`; record actual HEAD and this prompt's
hash at intake. CDC owns planning mutation/staging and independent acceptance.
On parent dispatch, CC owns source mutation, generation and source index plus
only `$SLICE/closing-report.md` for its appended intake/return. Do not rewrite
the previous report; append a dated iteration-02 section preserving all attempts.
CDC will not edit it or run source gates during CC ownership. Return ownership
explicitly with a stable committed candidate.

Only source edit authorized: the existing test
`test_post_context_serializes_empty_navigation_and_related_collection_explicitly`
in `tools/src/news.rs`. No production behavior or other source changes. No
push/merge/publish/deploy, external messages, new chats/worktrees, Arc 06,
listing/archive/taxonomy/feed/search decisions or new editorial fixtures.
Use absolute shell working directories; use the existing escalation route for
necessary live-root/build/bind writes. Preserve ignored outputs and other work.

## Required reading/intake manifest

Read before dependent edits. Current source ranges are at cb31dc92; planning
text is the current CDC review/issuance state. Exact prefix roots resolve above.

| Order | Path, state and scope | Why |
|---|---|---|
| 1 | This prompt, `$SLICE/slice-plan.md`, `$SLICE/ledger.md`, `$SLICE/cdc-verification.md` — required-full | Active correction, all rows and independent R1 evidence |
| 2 | `$SLICE/closing-report.md` and `cc-prompt-iteration01.md` — required-full | Preserve original attestation and unchanged binding scope/gates; new prompt narrows source edits only |
| 3 | `$SOURCE/AGENTS.md`, `$PLANNING/AGENTS.md` — required-full | Repo/migration/model/gate constraints |
| 4 | `$SOURCE/tools/src/news.rs` — required-full (195 lines); `$SOURCE/Cargo.lock` complete serde_json 1.0.151 package block — required-data | Actual serialization/test and pinned dependency |
| 5 | `/Users/oubiwann/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.151/src/value/index.rs` lines 178–215 and `src/value/mod.rs` full `get` method/doc section (locate definition, read complete section) — required-section | Verify Index's missing→Null fallback and get's Option result against pinned source |
| 6 | `$SKILLS/engineering-methods/guides/07-implementation-prompt-authoring.md` complete “Required reading and CC intake” and “Corrective iterations” sections; `$SKILLS/work-verification/guides/05-independent-verification.md` complete “Evidence During Correction Review”; `$SKILLS/testing/guides/03-validation-gates.md` complete “Proportionate Proof” — required-section | Intake, affected checks, retained evidence, discriminating assertion |
| 7 | Retain the fully loaded collaboration-framework, repo Rust entrypoint/full anti-patterns chapter, applicable Rust ID-30/API-02, testing/verification, Autonomous model/ownership and iteration contracts from iteration 01 if reliably present in this same CC context; otherwise reload their complete prior manifest extents before writing Rust — conditional, explicitly attest applicability | No silent skim or assumption that a receipt proves content is still available |

Record actual roots/branches/HEADs/dirty paths, required reading ranges and any
recovered truncation, plus a short contract readback in the appended report:
why missing differs from explicit null, the exact replacement assertions, unchanged
production/fixtures, and fresh versus retained gates. Hash/counts alone do not
prove reading. The active prompt, plan and ledger are required-full even when
prior context is retained. This bounded packet leaves ample recovery headroom;
there is no new investigation or new approval request.

## Reproduced finding and concrete repair

R1 (P2, F-5/F-7): at `tools/src/news.rs:173–174`, both
`value["prev"].is_null()` and `value["next"].is_null()` pass if the key is
missing. The locked serde_json Index implementation returns a static Null when
index_into returns None (`src/value/index.rs:209–214`). The current serialized
context correctly declares both fields without skip attributes; production
behavior is not reported broken. The test fails the explicit discrimination
required by iteration 01's null/null/[] contract.

**Binding repair:** keep serialization of the actual `PostContext::from_post`
fixture in that test. Assert both key presence and null value via `get`:

```rust
assert_eq!(value.get("prev"), Some(&serde_json::Value::Null));
assert_eq!(value.get("next"), Some(&serde_json::Value::Null));
```

This illustrative shape is not presented as compiled code. It uses the pinned
Value API; local syntax may adapt without changing behavior. A missing key is
None and must fail the same equality predicate. Keep the existing exact empty
related-list assertion and real-template no-byline/no-navigation/no-related
checks. Do not alter PostContext, serde attributes, renderer, parser, fixtures,
or binding product choices to make a test pass. No public helper or new test
harness is needed. Explain why the replacement rejects omission; a source-
mutation campaign or a third-party-library test suite is not required.

Applied guidance: ID-30 MUST distinguishes Option absence from a sentinel;
the expected Some(Null) is externally meaningful context shape. Testing's
behavior/proportionate-proof rules and prompt-authoring's discriminating-oracle
requirement select this small existing test correction. Existing Rust private
invariants/borrowed inputs and template safety remain intact. No lifecycle,
resource, concurrency, dependency or API change is involved.

## Checks and return

Run in absolute cwd SOURCE, serially on the final candidate:

```sh
cargo test --all-features --workspace news::tests::
cargo test --no-default-features --workspace
cargo outdated --root-deps-only --ignore-external-rel
make check
git diff --check
```

Expect the four renderer tests to run, including the stronger actual-context
assertion. Derive full test/ignore/integration populations afresh. Preserve any
failures and exact commands/exits; a filtered zero-test summary is not coverage.
`make check` remains mandatory, without skipping/redefining freshness, lint,
format, build or tests. Direct cargo-outdated exit/stderr must be recorded,
because the Makefile wrapper can mask query failure; distinguish query success
from claims that no newer packages exist. Do not upgrade pins or change Makefile.
No browser assets/harness behavior changes, so browser gates are not triggered;
their full arc-composition requirements remain unchanged. The frozen migration
checker is not a new-development gate and must remain unchanged.

Before make's bin cleanup, independently verify source `bin/haleiki` and
`/private/tmp/haleiki-slice01-original-bin` are regular non-symlink uid-501 owned
files with matching SHA-256
`4df7d22f32bbeb7bcde62c62e100917987199fc71729179f85b877a1c3405f6e`;
preserve unknown outputs and restore only the verified owned original afterward.
Do not blindly overwrite a changed backup. Retain failed/privileged reruns.

CDC already independently reproduced the parser (19+1 ignored), render (4),
route set/negative collision (2), positive namespace (1), search preservation
(2), export CLI (1), real 294-page export, twelve CLI validation specimens and
actual unreadable-news-directory rejection on cb31dc92. The exact source and
limits are in cdc-verification.md. Propose retaining those unchanged production
claims; CDC decides after inspecting your actual test-only diff. Do not relabel
earlier observations as fresh. F-7's oracle and final full gates need fresh proof.
No ledger row or slice is accepted yet; all eight stay accounted for.

Commit only `tools/src/news.rs`, explicitly staged, with
`git commit --only -- tools/src/news.rs` and contiguous trailers:

```text
Co-authored-by: Codex <noreply@openai.com>
Co-authored-by: Billo AI <ai-engineering@billo.systems>
```

Return exact assignment, intake/source/planning states and requested/selected
model/settings/context; source commit and one-file diff; reading/readback;
commands/exits/counts and bin hashes/restoration; R1 disposition and F-row
evidence applicability; scope/bubble-up and all failures/unrun checks. Append
to the existing closing report; CDC stages planning later. Your result remains
attested proposed-done until independent CDC review. Hand source ownership back.

## CDC readiness

Locke read the real candidate/report and relevant pinned serde_json source,
independently reproduced the existing test and identified its exact false-
positive case. The complete prompt-authoring guide was reloaded before this
assignment; existing full Rust/anti-pattern and applied testing/verification
loads remain available. The repair changes one test's observation method with
resolved expected Some(Null) versus None behavior, leaves production untouched,
uses an existing fixture and no new verification product. The manifest, exact
scope, preservation/retention rules and mandatory gates make this executable,
falsifiable and bounded. This is author readiness, not an acceptance verdict.
