// Arc 04 Slice 02 — headless-browser verification (decision #1, CONFIRMED:
// Deno + Playwright). One Deno.test suite: pure-helper unit checks (against
// window.__haleikiSearch, exposed by the real shipped search.js) plus full
// end-to-end scenarios against BOTH sinks — a live `haleiki serve` and a
// statically-hosted built `_site/` — including sink parity (H-7/D-6).
//
// Setup (build + export + start both servers + launch Chromium) runs once,
// wrapped in try/finally so teardown always runs even on a failing step.

import { assert, assertEquals } from "@std/assert";
import { chromium } from "playwright";
import {
  buildHaleiki,
  exportDemoSite,
  startLiveServer,
  startStaticServer,
} from "./harness.js";

const EXPORT_DIR = await Deno.makeTempDir({ prefix: "haleiki-e2e-site-" });

Deno.test({
  name: "search.js: unit helpers + end-to-end faceted search (both sinks)",
  sanitizeOps: false,
  sanitizeResources: false,
  fn: async (t) => {
    await buildHaleiki();
    await exportDemoSite(EXPORT_DIR);

    const live = await startLiveServer();
    const staticSite = startStaticServer(EXPORT_DIR);
    const browser = await chromium.launch();

    try {
      await t.step(
        "unit: parseQuery reads ?q= from a location.search string",
        async () => {
          const page = await browser.newPage();
          await page.goto(live.url + "/search/");
          const result = await page.evaluate(() => ({
            withTerm: window.__haleikiSearch.parseQuery("?q=hello%20world"),
            empty: window.__haleikiSearch.parseQuery(""),
            other: window.__haleikiSearch.parseQuery("?other=1"),
          }));
          assertEquals(result.withTerm, "hello world");
          assertEquals(result.empty, "");
          assertEquals(result.other, "");
          await page.close();
        },
      );

      await t.step(
        "unit: mapResultData maps a Pagefind result to view fields",
        async () => {
          const page = await browser.newPage();
          await page.goto(live.url + "/search/");
          const mapped = await page.evaluate(() =>
            window.__haleikiSearch.mapResultData({
              url: "/abelian-group/",
              meta: {
                type: "concept",
                category: "mathematics",
                tier: "intermediate",
                title: "Abelian Group",
                url: "/abelian-group/",
              },
              excerpt: "A <mark>commutative</mark> group.",
            })
          );
          assertEquals(mapped, {
            type: "concept",
            category: "mathematics",
            tier: "intermediate",
            title: "Abelian Group",
            url: "/abelian-group/",
            excerpt: "A <mark>commutative</mark> group.",
          });
          await page.close();
        },
      );

      await t.step(
        "unit: toggleFilter/isFilterActive implement a pure OR-within-group reducer",
        async () => {
          const page = await browser.newPage();
          await page.goto(live.url + "/search/");
          const result = await page.evaluate(() => {
            const s = window.__haleikiSearch;
            const afterAdd = s.toggleFilter({}, "category", "mathematics");
            const afterAddSecond = s.toggleFilter(
              afterAdd,
              "category",
              "physics",
            );
            const afterRemove = s.toggleFilter(
              afterAddSecond,
              "category",
              "mathematics",
            );
            return {
              afterAdd,
              afterAddSecond,
              afterRemove,
              activeAfterAdd: s.isFilterActive(
                afterAdd,
                "category",
                "mathematics",
              ),
              activeAfterRemove: s.isFilterActive(
                afterRemove,
                "category",
                "mathematics",
              ),
            };
          });
          assertEquals(result.afterAdd, { category: ["mathematics"] });
          assertEquals(result.afterAddSecond, {
            category: ["mathematics", "physics"],
          });
          assertEquals(result.afterRemove, { category: ["physics"] });
          assertEquals(result.activeAfterAdd, true);
          assertEquals(result.activeAfterRemove, false);
          await page.close();
        },
      );

      await t.step("unit: formatCount and humanizeLabel", async () => {
        const page = await browser.newPage();
        await page.goto(live.url + "/search/");
        const result = await page.evaluate(() => ({
          count14: window.__haleikiSearch.formatCount(14),
          countNaN: window.__haleikiSearch.formatCount(NaN),
          countNegative: window.__haleikiSearch.formatCount(-3),
          label: window.__haleikiSearch.humanizeLabel("tibetan-buddhism"),
          labelSingle: window.__haleikiSearch.humanizeLabel("mathematics"),
        }));
        assertEquals(result.count14, "14");
        assertEquals(result.countNaN, "0");
        assertEquals(result.countNegative, "0");
        assertEquals(result.label, "Tibetan Buddhism");
        assertEquals(result.labelSingle, "Mathematics");
        await page.close();
      });

      await t.step(
        "unit: mergeFacetCounts keeps every value, defaulting unmatched to 0",
        async () => {
          const page = await browser.newPage();
          await page.goto(live.url + "/search/");
          const merged = await page.evaluate(() =>
            window.__haleikiSearch.mergeFacetCounts(
              { category: { a: 5, b: 3 } },
              { category: { a: 2 } },
            )
          );
          assertEquals(merged, { category: { a: 2, b: 0 } });
          await page.close();
        },
      );

      // ---- End-to-end, live server ----

      await t.step(
        "e2e (live): ?q=garbage renders results with correct fields + highlighted excerpt",
        async () => {
          const page = await browser.newPage();
          await page.goto(live.url + "/search/?q=garbage");
          await page.waitForSelector(".search-result");

          assertEquals(await page.inputValue(".search-hero-input"), "garbage");

          const items = await page.$$eval(
            ".search-result",
            (nodes) =>
              nodes.map((n) => ({
                title: n.querySelector(".result-title a")?.textContent,
                href: n.querySelector(".result-title a")?.getAttribute("href"),
                hasMark: !!n.querySelector(".result-excerpt mark"),
                url: n.querySelector(".result-url")?.textContent,
              })),
          );
          assert(
            items.length > 0,
            'expected at least one result for "garbage"',
          );
          for (const item of items) {
            assert(
              item.title && item.title.length > 0,
              "result should have a title",
            );
            assert(
              item.href && item.href.startsWith("/"),
              "result href should be a site-relative url",
            );
            assertEquals(item.href, item.url);
            assert(
              item.hasMark,
              "excerpt should contain a Pagefind <mark> highlight",
            );
          }
          await page.close();
        },
      );

      await t.step(
        "e2e (live): .filter-sidebar populates the four facet groups with non-zero counts",
        async () => {
          const page = await browser.newPage();
          await page.goto(live.url + "/search/");
          await page.waitForSelector(".filter-group");

          const groupTitles = await page.$$eval(
            ".filter-group-title",
            (nodes) => nodes.map((n) => n.textContent),
          );
          assertEquals(groupTitles, ["Category", "Tier", "Type", "Trust"]);

          const counts = await page.$$eval(
            ".filter-count",
            (nodes) => nodes.map((n) => Number(n.textContent)),
          );
          assert(counts.length > 0, "expected at least one facet option");
          assert(
            counts.some((c) => c > 0),
            "expected at least one facet option with a non-zero count",
          );
          await page.close();
        },
      );

      await t.step(
        "e2e (live): toggling a category facet narrows results and updates counts",
        async () => {
          const page = await browser.newPage();
          await page.goto(live.url + "/search/?q=garbage");
          await page.waitForSelector(".search-result");
          const before = (await page.$$(".search-result")).length;

          // "garbage" (garbage collection) is entirely programming-languages
          // content (spiked against the real bundle) — selecting a
          // *different* category should narrow the result set to zero
          // without erroring, proving the filter actually applies.
          await page.locator(".filter-option", { hasText: "Mathematics" })
            .click();
          await page.waitForFunction(
            (n) => document.querySelectorAll(".search-result").length !== n,
            before,
          );
          const afterWrongCategory = (await page.$$(".search-result")).length;
          assertEquals(afterWrongCategory, 0);
          assert(
            !(await page.isHidden(".search-empty")),
            "narrowing to zero results should show the empty state",
          );

          await page.locator(".filter-option", { hasText: "Mathematics" })
            .click(); // toggle back off
          await page.locator(".filter-option", {
            hasText: "Programming Languages",
          }).click();
          await page.waitForSelector(".search-result");
          const afterRightCategory = (await page.$$(".search-result")).length;
          assertEquals(afterRightCategory, before);
          await page.close();
        },
      );

      await t.step(
        "e2e (live): a no-match query shows .search-empty, not a stale list",
        async () => {
          // Pagefind's matching is forgiving (fuzzy/partial-credit): even long
          // nonsense ASCII strings ("zzzzz...", "xqzjklvwpq", repeated letters)
          // returned real, non-zero matches against this corpus when spiked —
          // there is no character overlap at all with the demo corpus's
          // English/Tibetan-transliteration/Rust content in CJK text, which
          // reliably returns zero results (verified empirically).
          const noMatchQuery = "日本語テスト";
          const page = await browser.newPage();
          await page.goto(
            live.url + "/search/?q=" + encodeURIComponent(noMatchQuery),
          );
          await page.waitForSelector(".search-empty:not([hidden])");
          assert(
            await page.isHidden(".search-results-list"),
            "results list should be hidden on no match",
          );
          await page.close();
        },
      );

      await t.step(
        "e2e (live): clearing the query returns to the initial (empty) state",
        async () => {
          const page = await browser.newPage();
          await page.goto(live.url + "/search/?q=garbage");
          await page.waitForSelector(".search-result");
          await page.fill(".search-hero-input", "");
          await page.dispatchEvent(".search-hero-input", "input");
          await page.waitForFunction(() =>
            document.querySelectorAll(".search-result").length === 0
          );
          assert(
            await page.isHidden(".search-empty"),
            "empty-state block should also be hidden, not just the list",
          );
          await page.close();
        },
      );

      await t.step(
        "e2e (live): q is reflected in the URL and reload reproduces the results",
        async () => {
          const page = await browser.newPage();
          await page.goto(live.url + "/search/");
          await page.fill(".search-hero-input", "garbage");
          await page.dispatchEvent(".search-hero-input", "input");
          await page.waitForSelector(".search-result");
          await page.waitForFunction(() =>
            new URLSearchParams(location.search).get("q") === "garbage"
          );

          const url = page.url();
          assert(
            url.includes("q=garbage"),
            `expected reflected url to carry q=garbage, got ${url}`,
          );

          const reloaded = await browser.newPage();
          await reloaded.goto(url);
          await reloaded.waitForSelector(".search-result");
          const countAfterReload = (await reloaded.$$(".search-result")).length;
          assert(
            countAfterReload > 0,
            "reloading the shareable url should reproduce results",
          );
          await page.close();
          await reloaded.close();
        },
      );

      // ---- Sink parity (H-7 / D-6) ----

      await t.step(
        "e2e: identical query returns identical result urls on both sinks",
        async () => {
          const livePage = await browser.newPage();
          await livePage.goto(live.url + "/search/?q=garbage");
          await livePage.waitForSelector(".search-result");
          const liveUrls = await livePage.$$eval(
            ".result-url",
            (nodes) => nodes.map((n) => n.textContent).sort(),
          );

          const staticPage = await browser.newPage();
          await staticPage.goto(staticSite.url + "/search/?q=garbage");
          await staticPage.waitForSelector(".search-result");
          const staticUrls = await staticPage.$$eval(
            ".result-url",
            (nodes) => nodes.map((n) => n.textContent).sort(),
          );

          assert(liveUrls.length > 0, "expected results on the live sink");
          assertEquals(liveUrls, staticUrls);

          await livePage.close();
          await staticPage.close();
        },
      );
    } finally {
      await browser.close();
      await live.close();
      await staticSite.close();
      await Deno.remove(EXPORT_DIR, { recursive: true });
    }
  },
});
