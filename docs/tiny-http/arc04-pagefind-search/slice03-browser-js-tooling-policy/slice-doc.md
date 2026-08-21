# Slice 03 — browser JS tooling policy

> Plan-of-record for the third slice of Arc 04 (Search / Pagefind).
> Open set: this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **proposed-done** · 2026-08-21.

---

## Goal

Define and implement Haleiki's canonical quality gate for **shipped browser
JavaScript** (`assets/js/*.js`) now that Arc 04 has introduced a Deno/Playwright
browser harness.

Slice 02 proved that the search UI works in a real browser, but CDC review found
that its original tooling claim was muddy: `tests/e2e/` is Deno code and should
be linted/formatted by Deno, while `assets/js/search.js` is a classic browser
script that intentionally uses browser globals (`window`, `document`) and
matches the established house style in `theme.js` and `browse-scrollspy.js`.
Deno's default lints are not automatically the right authority for that asset.

This slice closes the tooling gap without changing the search feature's
behavior.

## Context

- Slice 02 is CDC-closed. Its functional evidence is reproduced by
  `make test-e2e`: Pagefind loads, `?q=` seeds search, results render, facets
  filter with live counts, no-match/clear states work, and live/static sinks
  return identical result URLs.
- Slice 02's H-8 was amended. Direct CDC reproduction showed:
  - `deno lint assets/js/search.js` fails with browser/house-style findings
    (`no-var`, `no-window`, plus `no-this-alias`/`no-inner-declarations`).
  - `deno fmt --check assets/js/search.js` would reformat the file.
  - `make lint-e2e`, `make test-e2e`, and elevated `make check` pass.
- Existing shipped browser assets are classic scripts loaded by ordinary
  `<script>` tags; there is no browser bundler, transpiler, or Node toolchain.
- The project intentionally keeps Node out. The Deno harness may use
  `npm:playwright` through Deno's npm-specifier support, but there should be no
  `package.json`, `node_modules`, `npm install`, or `npm run` workflow.

## What to decide

Answer one question and encode it in repo-visible tooling/docs:

**What is the canonical quality gate for shipped browser JS in Haleiki?**

Reasonable outcomes include:

- Keep Deno as the harness/tool runner, but add a browser-asset-specific Deno
  lint/fmt configuration if the team wants Deno to govern `assets/js/*.js`.
- Use Deno only for `tests/e2e/` and define the browser-asset gate as
  real-browser execution plus targeted grep/static checks.
- Introduce another browser-aware, no-Node static checker only if it is small,
  justified, and does not create a hidden dependency burden.

The chosen policy must be explicit. No future slice should have to infer whether
Deno's default runtime lints apply to browser assets.

## Decision

Shipped browser JavaScript under `assets/js/*.js` is governed by a targeted
browser-asset gate, not by Deno's default lint/fmt rules. The canonical command
is `make check-browser-js`.

The gate treats these files as classic browser scripts loaded by ordinary
`<script>` tags. It parses each tracked `assets/js/*.js` file as classic
JavaScript and rejects dependency/build-step patterns that would violate the
current architecture: CommonJS, Node/Deno runtime globals, `node:`/`npm:`/`jsr:`
specifier use, local `node_modules`, `package.json`, and `npm install`/`npm run`
workflows. It intentionally does not reject house-style `var`, IIFEs, `window`,
or `document`, because those are part of the existing shipped asset style.

Deno remains the test-tooling runtime for `tests/e2e/`, where `make lint-e2e`
continues to run `deno lint` and `deno fmt --check`. The two policies are
separate by design.

## Scope

### In

- A documented browser-JS tooling policy for `assets/js/*.js`.
- Make targets that expose the policy clearly, either by adjusting existing e2e
  targets or adding narrow browser-asset targets.
- Any required config files, kept scoped so Deno harness rules and browser asset
  rules do not accidentally fight each other.
- Updating Slice 02 H-8 / Arc 04 references only as needed to point at the
  settled policy.
- Verifying the policy against all existing browser assets:
  `assets/js/search.js`, `assets/js/theme.js`, `assets/js/browse-scrollspy.js`,
  and any other tracked `assets/js/*.js`.

### Out

- Changing search behavior, facet semantics, Pagefind index generation, server
  bundle hosting, or rendered template contracts.
- Converting the shipped browser assets to modules or adding a browser build
  step unless the operator explicitly approves that larger architecture change.
- Adding Node/npm project infrastructure.
- Wiring the browser harness into default `make check` or CI. Arc 06 still owns
  CI/deployment-readiness policy.
- Broad cosmetic formatting churn outside `assets/js/*.js`, `tests/e2e/`, and
  directly related docs/Make targets.

## Verification approach

- Run the chosen browser-JS gate and show that it passes on all tracked shipped
  browser assets.
- Run `make lint-e2e` to keep the Deno harness clean.
- Run `make test-e2e` to ensure the browser policy did not break the already
  closed functional search evidence.
- Run `make check` to confirm no Rust regression.
- Grep for Node/npm project infrastructure and confirm none was introduced.

## Exit criteria

Ledger rows I-1…I-7 reach final status. The project has a clear, reproducible
answer for shipped browser JS quality; the Deno harness remains clean; the
search browser suite still passes; the Rust suite remains green; no Node/npm
toolchain is added; Arc 04 can proceed to arc-scale close.

## Relationship to the arc close

This slice is a tooling-policy follow-up created by CDC review of Slice 02. It
does not add new user-facing search behavior, but it is required before Arc 04
closes so the arc does not carry an unresolved verification contract. After this
slice CDC-closes, the independent reviewer should reproduce D-3…D-6 at arc scale
and write `arc04-pagefind-search/closing-report.md`.
