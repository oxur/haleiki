# TODOs

## In-Progress

-

## Pending

- Create a doc w/Claude for converting design mockups to Cobalt / Liquid templates
  - How the HTML mockups decompose into layouts vs. includes
  - How design tokens (CSS custom properties) map to the Cobalt/Liquid template structure
  - The template variable contract (what page.*, site.*, collection.* variables each template expects)
  - How to test templates against real content (demo source pages)

Mockups to create:

1. [x] Home page (home.liquid) — landing page, entry point to the knowledge base
2. [x] Source page (source.liquid) — full narrative article with taxonomy sidebar (your 90 demo articles render with this)
3. [x] Concept page (concept.liquid) — structured article with standardized sections (Quick Definition, Core Definition, Prerequisites, etc.) + taxonomy sidebar
4. [x] Category listing page (category.liquid) — index of all pages within a category, both source pages and concept cards
5. [x] Browse/index page (browse.liquid) — overview of all categories/tiers, entry point for exploration
6. [x] Search results (search.liquid as an include, but probably needs its own layout or modal) — the ⌘K search UX
7. [x] Attribution page — CC BY-SA compliance page listing all sources and media (milestone 7.1 generates this for the demo)
8. [x] Brand guidelines page
9. [x] Search results page

- Default/chrome layout (default.liquid) — the shared shell (topnav, footer, theme toggle, search activation) that wraps all other layouts

## Completed

-
