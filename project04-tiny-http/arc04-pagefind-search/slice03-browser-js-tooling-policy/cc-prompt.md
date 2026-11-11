# CC Prompt — Arc 04 / Slice 03: browser JS tooling policy

You are implementing the third slice of Arc 04 of the Haleiki tiny_http /
minijinja pivot, on branch `framework-base/tiny-http`. Work to the
collaboration-framework's ledger discipline: every ledger row must reach a
final, evidence-backed status before you claim the slice done.

## What this slice is

This is a **tooling-policy slice**, not a search-feature slice.

Slice 02 is CDC-closed functionally: the Deno/Playwright harness reproduces
client-side Pagefind search, facets, URL state, empty state, and live/static
sink parity. The remaining issue is that the original H-8 wording treated
`assets/js/search.js` as if Deno's default lint/fmt rules were the canonical
authority for shipped browser assets. CDC reproduced that this is false:
`deno lint assets/js/search.js` reports browser/house-style findings and
`deno fmt --check assets/js/search.js` would reformat it.

Your job is to define the repo's canonical quality gate for shipped browser JS
and encode it so future slices do not repeat the ambiguity.

## Before you write code

1. Load the **collaboration-framework** skill and the
   **javascript-deno-guidelines** skill.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, scope, decisions.
   - [`ledger.md`](./ledger.md) — the I-1…I-7 contract.
   - [`../arc-plan.md`](../arc-plan.md) — Arc 04's updated three-slice shape.
   - [`../slice02-client-and-facets/cdc-verification.md`](../slice02-client-and-facets/cdc-verification.md)
     — the exact CDC finding this slice resolves.
   - `assets/js/search.js`, `assets/js/theme.js`, and
     `assets/js/browse-scrollspy.js` — the current shipped browser-asset style.
   - `tests/e2e/deno.json`, `tests/e2e/search_test.js`,
     `tests/e2e/harness.js`, and `Makefile` — the current harness and targets.

## Decisions to make explicitly

1. **Browser asset lint/fmt authority.** Decide whether shipped browser assets
   are governed by Deno with browser-specific config, by a targeted custom gate,
   or by real-browser execution plus grep/static checks. Record the decision in
   repo docs and in the closing report.
2. **Formatting churn.** If your chosen policy reformats existing assets, keep
   the change narrow and justified. Do not format unrelated files.
3. **Make targets.** Prefer Make-backed entrypoints so CDC can reproduce from
   the repo root. Keep the harness targets discoverable.
4. **CI/default gate.** Do not wire browser/e2e checks into default `make check`
   unless the operator explicitly changes the Arc 06 deferral.

## Boundaries

- Do **not** change search behavior, Pagefind index generation, server bundle
  serving, or rendered template contracts.
- Do **not** add `package.json`, `node_modules`, `npm install`, or `npm run`
  workflows.
- Do **not** convert browser assets to ES modules or add a browser build step
  unless the operator explicitly approves that architecture change.
- Do **not** use this slice to resolve facet URL persistence, no-JS copy, fixed
  e2e ports, or Arc 06 CI policy unless the tooling decision directly requires a
  small note.

## Definition of done

All I-1…I-7 rows reach final status with reproducible evidence. In the closing
report, walk the ledger row by row, include the exact commands and outputs, and
state the policy in plain language. Name any remaining caveat with a concrete
re-entry condition. Do not write `cdc-verification.md`; that is the independent
reviewer's file.

