# CDC verification — Arc 05 Slice 01

## Current verdict — 2026-11-11

**Returned for a bounded test correction; Slice 01 remains open.**
Current reviewed source: `cb31dc92caa667ce6f3a7133163c3da8749549b1`, clean
`framework-base/tiny-http`. Governing assignment was `cc-prompt-iteration01.md`,
SHA-256 `2e78d1481c57c89054ab47f756d78c4564a5fb838c43f023086212f2f6740002`,
issued at planning `2d5e1486e2c9ba5bc111b42603bce9ab4a46c736`.

Independent reviewer: CDC Locke, context
`01a1ccec-0192-7ea3-81ca-f0ba2dad69a9`, requested/tool-selected OpenAI
`gpt-6.1-sol / high` per parent dispatch. Implementer: CC Kierkegaard,
`01a1ccf7-3e7c-7ab2-ae59-72c9be8f457b`, requested/tool-selected
`gpt-6-luna / high`. Backend identities remain unreported. This is the existing
CDC + CC arrangement, not an additional role. CC returned source mutation
ownership before these checks; CDC made no source repairs.

## Independent review of iteration 01

I read the actual complete 117-line CC closing report, all twelve changed paths
against `75d2593d51c986346f0239bed36a248ac5b4e6f7`, the complete new 195-line
news module, new parser/date/model/loading code and detailed tests, and relevant
caller wiring. The source and planning branches/dirty states were revalidated;
only the CC report was untracked in planning at handback. Original plans and
normative/Rust/testing/verification guidance loaded during assignment authoring
remain applicable; the complete 524-line prompt-authoring guide was reloaded
before the corrective assignment. Truncated combined reads were recovered.
The CC intake/readback addresses the consequential contract; those reading
receipts remain attestation, distinct from the independent source/checks here.

No production behavior defect was found in this pass. Strict Value string/list
inspection avoids YAML scalar coercion; NewsDate guards byte slicing and checks
Gregorian boundaries; missing news directory is empty but other I/O errors
propagate; direct regular Markdown entries are sorted; duplicate normalized
filenames fail before insertion. Metadata stays normally escaped, body uses the
existing real-template safe slot, and the shared route insertion checks for
existing keys. Wiki graph, stats, source/concept models, template/browser assets
and Pagefind record construction remain unchanged apart from the additive map
and mechanical test-fixture fields. News is deliberately not indexed yet.

### R1 — P2: null assertions do not prove key presence

At this revision, `tools/src/news.rs:173–174` uses
`assert!(value["prev"].is_null())` and its `next` equivalent. The locked
serde_json version is 1.0.151. Its actual cached `src/value/index.rs:209–214`
documents and implements undefined-key indexing as a reference to `Value::Null`
(`index_into(self).unwrap_or(&NULL)`). Thus both assertions pass for missing
fields, the precise alternative that the assignment's F-5/F-7 oracle promises
to reject (“Undefined fields masquerading as explicit null”).

The current production context includes both fields, without skip-serialization
attributes. This finding concerns the discrimination of the required test,
not evidence that the current rendered product omits them. Replace the two
assertions with presence-aware `get` equality checks (Some(Null)); preserve
the related=[] and real-template absent-navigation assertions. The negative
case must be distinguishable as None, not silently accepted as null.
Assigned repair: `cc-prompt-iteration02.md`, source test only. F-7 and the
affected F-5 context evidence remain unresolved until this is checked.

## Independently reproduced checks at cb31dc92

All commands used absolute cwd `/Users/oubiwann/lab/oxur/haleiki`, serially,
with the required escalation for local writes/builds. All listed commands
exited 0 unless a negative input's expected exit 1 is explicitly named. No
sandbox-denied bind or project failure occurred in this independent pass.

| Command / check | Observed population and scope |
|---|---|
| `cargo test --all-features --workspace news::tests:: -- --nocapture` | 4 passed, 0 failed, 420 filtered; existing weak assertion passes, which is not proof of key-presence discrimination |
| `cargo test --all-features --workspace parser::tests::` | 19 passed, 1 ignored, 404 filtered; all strict controls and existing parser/body checks selected |
| `cargo test --all-features --workspace server::tests::test_render_routes` | 2 passed, 422 filtered; full route-set and negative legacy segmented-concept collision |
| `cargo test --all-features --workspace server::tests::test_news_route_coexists` | 1 passed, 423 filtered; positive concept/source/news namespace coexistence |
| `cargo test --all-features --workspace search::tests::test_build_records` | 2 passed, 422 filtered; existing source/concept record count/order and no news URL |
| `cargo test --all-features --workspace --test export_cli` | 1 passed; real CLI static export includes news and prior wiki paths |
| Separate real CLI export, exact invocation in the retained focused log | 294 HTML files including 404; exactly one news HTML file; approved title/date/category, three tag links, author and Markdown paragraph present; no post navigation/related/trust-bar markup |
| Twelve independent isolated CLI validation specimens | 4 accepted: base, leap-2000, maximum year 9999, null optionals. 8 rejected with exit 1 and filename context: numeric title, boolean title, numeric category, boolean author, null tag element, invalid 1900 leap day, Unicode date digits, leading date space |
| Actual unreadable news directory | As uid 501, changed only an owned temporary fixture directory to mode 000, invoked real CLI validate, observed exit 1 with failed-read/news-path diagnostic, restored mode in finally; distinguishes a permission error from an absent news directory |
| Full corpus frontmatter scan | demo 753 recursive Markdown files, direct authored sources=91/concepts=186/news=1; exactly the approved news file matches news/blog type; content has 0 Markdown; no read errors or generated-tree exclusions |
| `git diff --check 75d2593 HEAD`; exact Git tree preservation checks | Exit 0; template tree, browser JS, Cargo pins/manifests, authored source and concept files unchanged |

Focused log is retained at `artifacts/cdc-review01-focused.log` to preserve
original independent command outputs and exact real-export invocation for this
review. Its duplicate zero-selected integration summaries are not counted as
passing behavior tests. Current real exported post:
`/private/tmp/haleiki-cdc-cb31dc92-export-kkpfr854/site/news/search-lands-in-the-demo-wiki/index.html`,
SHA-256 `e352ae4cc2044b172cab2974955f9505887742b8696fcb98ec7aac4676026a0a`.
It is a real export, not a standalone styled mockup; root-relative asset links
require serving the export root. The post's listing/category/tag targets remain
pending surfaces as approved. Reconstruction uses the exact log command.

Both `/private/tmp/haleiki-slice01-original-bin` and source `bin/haleiki` were
independently checked: regular non-symlink files, uid 501, mode 0755, SHA-256
`4df7d22f32bbeb7bcde62c62e100917987199fc71729179f85b877a1c3405f6e`.
No make-clean operation or bin replacement was performed in this pass.

`make check`, direct dependency query and the no-default full suite are **not
yet independently run**: their fresh execution is assigned to the corrected
candidate before final acceptance. They remain CC-attested for cb31dc92, not
inherited green CDC gates. The parent's relay reports the earlier 415 progress
count was corrected by CC to 416; this pass derived 424 total unit entries
from filtered/selected counts, but has not independently derived the full
pass/ignore split. Final gates will derive it directly. Browser gates are not
triggered by the test-only correction and remain required at arc composition.

## All-row accounting and retained-evidence proposal

| Row | Current independent disposition |
|---|---|
| F-1 | Canonical approved log/example and assignment verified; no new approval inferred |
| F-2 | Corpus query and sole approved fixture verified |
| F-3 | Strict model/identity/optional/body parser checks reproduced; full parse/export works |
| F-4 | Parser rejection matrix reproduced plus independent real CLI and permission controls |
| F-5 | Real template/escaping reproduced; explicit-null assertion gap R1 needs correction |
| F-6 | Shared route-set, positive/negative collision controls, search preservation and real export reproduced |
| F-7 | Current context has null/null/[] by source inspection, but test must discriminate omitted prev/next; open R1 |
| F-8 | Required full native gates remain pending independent reproduction on the corrected candidate |

All eight live ledger statuses remain open; this is not a partial slice-close
or a final acceptance verdict. For a strictly test-only delta, the unchanged
parser, route, data, templates, dependencies and exported-output claims above
may retain this review's independent evidence under correction-review rules.
CDC must inspect the actual next diff and explicitly confirm applicability;
re-run the corrected renderer/context tests and required full gates fresh.

## Bubble-up and next action

The increment supplies its assigned direct post through real routes/export,
without silently adding listing, archive, taxonomy, feed, news search or trust
surfaces. No arc structural or scope change is indicated by this review; R1
remains a slice correction. The concrete post links reveal exactly the later
surfaces that remain pending, not a new grant to implement them. E-1…E-7 and
all existing project dispositions remain unchanged. No slice/arc close or
retrospective is claimed yet. After the test repair and final verification,
CDC will complete the slice retrospective and continue permitted Arc 05 work
to the next actual discussion gate, retaining all later Operator decisions.

Next owner: parent dispatches/resumes the same selected CC with iteration 02;
CC owns only the stated test edit and appends dated evidence to its report.
CDC makes no concurrent source mutations/gates while CC owns the source tree.
