# Hale'iki minijinja templates

A complete template set for rendering the wiki + news site with
[minijinja](https://docs.rs/minijinja) from a small Rust server. Every page is
painted by the single unified stylesheet at `/assets/css/haleiki.css`.

## Layout

```
templates/
  base.jinja               shared shell: <head>, topnav, footer, block slots
  macros.jinja             trust_glyph, search_icon, post_row, ext
  partials/                included fragments
    topnav.jinja           unified nav — live search + one active convention
    footer.jinja           unified footer — all copy from site.data.profile
    breadcrumb.jinja
    trust-bar.jinja        epistemic-trust strip (concept + source)
    left-nav-toc.jinja
    sidebar-taxonomy.jinja
    provenance.jinja
    see-also.jinja
  pages/                   one template per route (extends base.jinja)
    home · browse · category · concept · source · attribution · about · search · 404
    news-index · news-post · news-archive · news-tags · news-categories
```

## Rendering (Rust sketch)

```rust
use minijinja::{Environment, path_loader, context};

let mut env = Environment::new();
env.set_loader(path_loader("templates"));      // resolves "partials/…", "pages/…"

let tmpl = env.get_template("pages/concept.jinja")?;
let html = tmpl.render(context! {
    site => site_ctx,   // { data: { profile, categories, titles, … } }
    page => page_ctx,   // { title, slug, content, data: { … } }
})?;
```

`page.content`, `post.body`, `result.excerpt`, and `profile.about_body` are
pre-rendered HTML and printed with `| safe`. Everything else is auto-escaped.

## Context contract

### `site.data.profile` — every page

```yaml
name: "Haleiki Demo Wiki"
name_short: "Haleiki"
tagline: "A house of knowledge — a read-only wiki for structured, sourced content."
hero_subtitle: "A knowledge base for encyclopedic exploration"
about_intro: "…"
about_body: ["<safe html paragraph>", "…"]
content_notice: "Wiki content licensed under CC BY-SA 4.0, unless otherwise stated."
trust:
  enabled: true
  default: "credible"
  level_order: [corroborated, credible, provisional, speculative]
  levels:
    credible:     { label: "Credible",     description: "Well-sourced …", note: "…" }
    corroborated: { label: "Corroborated", description: "…" }
    provisional:  { label: "Provisional",  description: "…" }
    speculative:  { label: "Speculative",  description: "…" }
canon: { enabled: false }
taxonomy:
  categories: [{ name: "memory-management" }, …]
  tiers: [{ name: "foundational", description: "…" }, …]
footer:
  columns:
    - title: "Knowledge Base"
      links: [{ label: "Front Page", url: "/" }, …]
    - title: "About"
      links: [{ label: "Attribution", url: "/attribution/" }, …]
    - title: "Development"
      links: [{ label: "GitHub", url: "https://…", external: true }, …]
  copyright: "© 2026 Oxur Contributors · Apache-2.0 / MIT"
  meta: [{ label: "Built with Haleiki", url: "/about/" }, { label: "Privacy", url: "/privacy/" }]
```

> **Single source of truth for the licence.** `content_notice` is the only place
> the content licence is stated, so the CC-BY-SA / CC0 contradiction between the
> old mockups can never recur.

### Per-page context

| Template | Needs |
|---|---|
| home | `site.data.stats {sources,concepts,categories}` · `site.data.home_categories[] {slug,name,count,description,articles[]{title,url}}` · `site.data.featured[] {title,category,tier,teaser,url}` |
| browse | `site.data.browse_categories[] {slug,name,count,description,groups[]{name,articles[]{title,url,type}}}` |
| category | `page.data.category {name,description,count,source_count,concept_count,subcategories[],groups[]{name,modifier,articles[]{title,url,type,description}}}` |
| concept | `page.content` (with section anchors) · `page.data {concept,disambiguator,category,category_slug,tier,type,facts[],subject_headings[],aliases[],toc[]{id,label},page_type:"concept"}` · `site.data.see_also[slug]` · `site.data.provenance[slug]` |
| source | as concept, `page.data.page_type:"source"` |
| attribution | `site.data.attribution {intro,general_notice,source_count,sources_by_project[]{name,sources[]{slug,title,license,url}},licenses[]{title,points[],links[]{label,url}}}` |
| about | `site.data.profile.about_intro / about_body / trust` |
| search | `search {query,results[]{type,title,url,category,tier,excerpt},facets[]{title,options[]{label,count}}}` |
| news-index | `news {posts[]{title,url,date_display,date_short,category,excerpt},categories[]{name,slug,count},tags[]{name,slug}}` |
| news-post | `post {title,body,date_display,category,category_slug,tags[]{name,slug},author,prev,next,related[]}` |
| news-archive | `news {total_posts,pagination{current,total,newer_url,older_url},months[]{label,posts[]{title,url,date_short,category}}}` |
| news-tags | `news {total_posts,tags[]{name,slug,count,size},tag_sections[]{name,slug,posts[]}}` (`size` ∈ large / medium / small) |
| news-categories | `news {categories[]{name,slug,count,description,recent[]},category_sections[]{name,slug,posts[]}}` |

### Grouping is done in Rust, not the template

The original Liquid category/browse layouts de-duplicated "top-level vs
sub-category" entries inside the template with nested loops. That logic now lives
in the pre-build: hand the templates a resolved `groups[]` list and they stay
declarative.

## Notes carried over from the design review

- **Trust on news posts** is wired but commented out in `news-post.jinja` — trust
  describes sourced knowledge, not announcements. Uncomment the `chrome` block to
  enable it per your questionnaire answer.
- **News category cards** use `.news-cat-*` classes, namespaced away from the home
  page's `.category-card` (same name, different component in the mockups).
- **Active nav** is passed via an overridable `{% block active_nav %}` captured in
  `base.jinja` (a plain child-level `{% set %}` would not reach the included topnav).
