# Haleiki Search Architecture — A+B Plan

## Context

Haleiki needs a search experience that matches its design system. We have two interaction patterns to support:

- **A — Dedicated search results page** (`/search/?q=...`): full page with faceted sidebar, already mocked up as `assets/mockups/15-search-page.html`
- **B — Inline dropdown**: as-you-type results appearing below the nav search input on every page, for quick lookups

Pagefind (Rust-based, static-site-native) generates the search index at build time. We use its **JS API** (not the Default UI component) for full control over rendering.

## Pagefind API Summary

**What we get from Pagefind:**

| Capability | API | Notes |
|-----------|-----|-------|
| Search | `pagefind.search("query")` | Returns result stubs (url, metadata) |
| Load result data | `result.data()` | Async — loads excerpt, full meta |
| Debounced search | `pagefind.debouncedSearch("query", {}, 300)` | Returns null if superseded — perfect for as-you-type |
| Preload | `pagefind.preload("partial")` | Warms the index while user types |
| Filters with counts | `pagefind.filters()` | Returns `{ category: { "music": 17, ... } }` |
| Search with filters | `pagefind.search("q", { filters: { category: "music" } })` | AND/OR/NOT operators |
| Excerpts | `result.excerpt` | HTML string with `<mark>` tags around matches |
| Custom metadata | `result.meta.title`, `result.meta.category`, etc. | Set via `data-pagefind-meta` attributes |

**Bundle size:** Under 300KB for a 10k-page site. Chunked index — browser only downloads relevant chunks. Lazy-loaded on first search.

## Architecture

### Build-Time Setup (Cobalt templates)

Add these attributes to Cobalt Liquid templates:

```html
<!-- In source.liquid and concept.liquid -->
<article data-pagefind-body>
  {{ page.content }}
</article>

<!-- Metadata for search results -->
<h1 data-pagefind-meta="title">{{ page.title }}</h1>
<meta data-pagefind-meta="type" content="{{ page.page_type }}">

<!-- Filters for faceted search -->
<span data-pagefind-filter="category">{{ page.category }}</span>
<span data-pagefind-filter="tier">{{ page.tier }}</span>
<span data-pagefind-filter="type">{{ page.page_type }}</span>
```

Build step: `npx pagefind --site _site` (wrapped by `haleiki search` CLI command)

### Runtime — TypeScript Module (`assets/ts/search.ts`)

A single TypeScript module compiled to `assets/js/search.js`. No framework. Three responsibilities:

#### 1. Pagefind Manager (shared)

```typescript
// Lazy-init Pagefind once, share across dropdown and full page
let pagefind: any = null;

async function getPagefind() {
  if (!pagefind) {
    pagefind = await import("/pagefind/pagefind.js");
    await pagefind.init();
  }
  return pagefind;
}
```

#### 2. Inline Dropdown (B) — on every page

Activated by:
- Typing in the nav search input
- Pressing `⌘K` (focuses input + opens dropdown)

Behavior:
- On input: `pagefind.debouncedSearch(value, {}, 300)` — 300ms debounce
- While typing: `pagefind.preload(value)` — warms index
- Shows top 5-6 results in a positioned `<div>` below the search input (NOT a modal — no overlay, no backdrop, no scroll lock)
- Each result: title + category badge + excerpt snippet
- Keyboard nav: arrow keys move selection, Enter navigates to selected result
- "See all results" link at bottom → navigates to `/search/?q=...`
- Escape or click-outside closes dropdown
- If `debouncedSearch` returns null (superseded by newer query), do nothing (prevents flicker)

DOM structure (pre-existing in the topnav HTML):
```html
<div class="search-container">
  <input class="search-input" type="text" placeholder="Search Haleiki...">
  <div class="search-dropdown" hidden>
    <div class="search-dropdown-results"></div>
    <a class="search-dropdown-all" href="/search/">See all results</a>
  </div>
</div>
```

The dropdown div is always in the DOM, toggled via `hidden` attribute. JS populates `.search-dropdown-results` with result HTML.

#### 3. Full Search Page (A) — `/search/` only

On page load:
- Read query from URL params (`?q=...`)
- Pre-fill the large search input
- Run `pagefind.search(query)` + `pagefind.filters()`
- Populate results list and filter sidebar with counts
- On input change: re-search with debounce
- On filter checkbox change: re-search with active filters

Result rendering: populate pre-existing DOM containers (matching mockup 15's structure):
```html
<div class="search-results" id="search-results">
  <!-- JS populates result cards here -->
</div>
```

Each result card rendered from a template function:
```typescript
function renderResult(data: PagefindResult): string {
  return `
    <article class="result-card">
      <div class="result-badges">
        <span class="type-badge type-${data.meta.type}">${data.meta.type}</span>
        <span class="tag">${data.meta.category}</span>
      </div>
      <a href="${data.url}" class="result-title">${data.meta.title}</a>
      <p class="result-excerpt">${data.excerpt}</p>
      <span class="result-url">${data.url}</span>
    </article>`;
}
```

Filter sidebar: read `pagefind.filters()` counts and render checkboxes with live counts that update on each search.

### File Structure

```
assets/
├── ts/
│   └── search.ts          ← Source TypeScript
├── js/
│   └── search.js           ← Compiled output (committed or built in CI)
```

Compilation: `tsc assets/ts/search.ts --outDir assets/js/ --target es2020 --module es2020`

Or if we want to avoid a build step for JS: write it as vanilla JS directly in `assets/js/search.js`. TypeScript gives us type safety for the Pagefind API shapes, but the module is small enough (~200 lines) that vanilla JS is also fine.

### `⌘K` Keyboard Shortcut

```typescript
document.addEventListener("keydown", (e) => {
  if ((e.metaKey || e.ctrlKey) && e.key === "k") {
    e.preventDefault();
    const input = document.querySelector(".search-input") as HTMLInputElement;
    input.focus();
    input.select();
  }
});
```

## What We Do NOT Need to Wrap

Pagefind's API is clean enough that we don't need a wrapper library. We just need:
- A thin init/lazy-load function
- Result rendering templates (HTML string builders)
- Event wiring (input, keydown, click-outside)
- Filter state management (which checkboxes are active)

No framework, no state management library, no build toolchain beyond `tsc` (optional).

## Implementation Approach

### Phase 1: Template Attributes
Add `data-pagefind-body`, `data-pagefind-filter`, and `data-pagefind-meta` to Cobalt Liquid templates. This is part of the template conversion work (converting mockups to Cobalt templates).

### Phase 2: Search Module
Write `search.ts` (or `search.js`) with:
1. Pagefind lazy initialization
2. Dropdown controller (show/hide, populate, keyboard nav)
3. Full page controller (populate results + filters, handle filter changes)
4. `⌘K` shortcut handler

### Phase 3: Dropdown HTML
Add the `.search-dropdown` div to the topnav include (`_includes/topnav.liquid`). It's hidden by default, shown by JS.

### Phase 4: Search Page Template
Create `search.liquid` layout that has the large search input, filter sidebar, and results container — matching mockup 15. JS populates the dynamic parts.

## Verification

1. Build site: `haleiki build && cobalt build && npx pagefind --site _site`
2. Serve locally: `cobalt serve` or any static server on `_site/`
3. Test dropdown: type in nav search → results appear within 300ms
4. Test `⌘K`: keyboard shortcut focuses search, opens dropdown
5. Test full page: navigate to `/search/?q=test` → results + filters render
6. Test filters: click a category checkbox → results update with counts
7. Test both themes: toggle day/night, verify all search UI elements respond
8. Test narrow viewport: verify dropdown and search page work on mobile
