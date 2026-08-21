// Client-side faceted search for /search/, wired to Pagefind's JS API
// against the /pagefind/ bundle (Arc 04). Classic script (house style: see
// theme.js, browse-scrollspy.js) — no build step, no ES modules; the
// dynamic import() below is a runtime call, valid in a classic script.
//
// Escaping boundary: only result.excerpt is HTML (Pagefind's <mark>
// highlight) and is set via innerHTML. Every other field (title, url,
// badge text, facet labels) is set via textContent.
(function () {
  var GROUP_ORDER = ['category', 'tier', 'type', 'trust'];
  var GROUP_TITLES = { category: 'Category', tier: 'Tier', type: 'Type', trust: 'Trust' };
  var DEBOUNCE_MS = 150;

  // ---- Pure helpers (exposed on window.__haleikiSearch for the headless
  // test harness; none of these touch the DOM or Pagefind). ----

  function parseQuery(search) {
    if (!search) return '';
    var params = new URLSearchParams(search);
    return params.get('q') || '';
  }

  function mapResultData(data) {
    data = data || {};
    var meta = data.meta || {};
    return {
      type: meta.type || '',
      category: meta.category || '',
      tier: meta.tier || '',
      title: meta.title || '',
      url: meta.url || data.url || '',
      excerpt: data.excerpt || ''
    };
  }

  // Toggles `value` in `group` on `state` ({group: [values]}), returning a
  // new state object (never mutates `state`). Multiple values in the same
  // group's array are Pagefind's own within-group-OR shape; multiple group
  // keys on the object are Pagefind's own across-group-AND shape — this
  // reducer only maintains that shape, it doesn't need to implement OR/AND
  // itself.
  function toggleFilter(state, group, value) {
    var next = {};
    Object.keys(state || {}).forEach(function (g) {
      next[g] = state[g].slice();
    });
    var values = next[group] ? next[group].slice() : [];
    var index = values.indexOf(value);
    if (index === -1) {
      values.push(value);
    } else {
      values.splice(index, 1);
    }
    if (values.length > 0) {
      next[group] = values;
    } else {
      delete next[group];
    }
    return next;
  }

  function isFilterActive(state, group, value) {
    return !!(state && state[group] && state[group].indexOf(value) !== -1);
  }

  function formatCount(n) {
    if (typeof n !== 'number' || !Number.isFinite(n) || n < 0) return '0';
    return String(Math.round(n));
  }

  function humanizeLabel(value) {
    if (!value) return '';
    return String(value)
      .split(/[-_]+/)
      .filter(function (part) { return part.length > 0; })
      .map(function (part) { return part.charAt(0).toUpperCase() + part.slice(1); })
      .join(' ');
  }

  // Merges a full facet structure (from pagefind.filters(), which defines
  // every group + value that exists in the whole corpus) with a narrower
  // live-count map from a search response, so values a query has narrowed
  // to zero still render (at 0) instead of disappearing from the sidebar.
  function mergeFacetCounts(structure, liveCounts) {
    var merged = {};
    Object.keys(structure || {}).forEach(function (group) {
      merged[group] = {};
      Object.keys(structure[group]).forEach(function (value) {
        var live = liveCounts && liveCounts[group] ? liveCounts[group][value] : undefined;
        merged[group][value] = live === undefined ? 0 : live;
      });
    });
    return merged;
  }

  function debounce(fn, delay) {
    var timer = null;
    return function () {
      var args = arguments;
      var context = this;
      if (timer !== null) clearTimeout(timer);
      timer = setTimeout(function () {
        timer = null;
        fn.apply(context, args);
      }, delay);
    };
  }

  window.__haleikiSearch = {
    parseQuery: parseQuery,
    mapResultData: mapResultData,
    toggleFilter: toggleFilter,
    isFilterActive: isFilterActive,
    formatCount: formatCount,
    humanizeLabel: humanizeLabel,
    mergeFacetCounts: mergeFacetCounts
  };

  // ---- DOM wiring (only on the /search/ page — every element below is
  // guarded, since this script's shared markup is search.jinja-only). ----

  var input = document.querySelector('.search-hero-input');
  if (!input) return;

  input.focus();
  document.addEventListener('keydown', function (e) {
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      input.focus();
      input.select();
    }
  });

  var form = document.querySelector('.search-hero-input-wrap');
  var searchHero = document.querySelector('.search-hero');
  var searchMain = document.querySelector('.search-main');
  var emptyState = document.querySelector('.search-empty');
  var filterSidebar = document.querySelector('.filter-sidebar');
  var resultTemplate = document.getElementById('search-result-template');
  var filterOptionTemplate = document.getElementById('filter-option-template');

  if (!searchHero || !searchMain || !emptyState || !filterSidebar || !resultTemplate || !filterOptionTemplate) {
    return;
  }

  // render_search's shell always renders with an empty query and zero
  // results (Arc 02 contract, untouched by this slice), so search.jinja's
  // `{% if %}` branches mean `.search-results-list`/`.search-status`/
  // `.search-results-count` are *never* actually present pre-hydration —
  // only `.search-empty` (the else branch) is. Create the missing ones
  // rather than assuming they exist; only `.search-empty` is ever reused.
  var resultsCount = document.querySelector('.search-results-count');
  if (!resultsCount) {
    resultsCount = document.createElement('div');
    resultsCount.className = 'search-results-count';
    searchHero.appendChild(resultsCount);
  }

  var resultsList = document.querySelector('.search-results-list');
  if (!resultsList) {
    resultsList = document.createElement('ol');
    resultsList.className = 'search-results-list';
    searchMain.insertBefore(resultsList, emptyState);
  }

  var searchStatus = document.querySelector('.search-status');
  if (!searchStatus) {
    searchStatus = document.createElement('div');
    searchStatus.className = 'search-status';
    searchMain.insertBefore(searchStatus, emptyState);
  }

  var pagefindPromise = null;
  var allFacets = null;
  var activeFilters = {};

  function loadPagefind() {
    if (!pagefindPromise) {
      pagefindPromise = import('/pagefind/pagefind.js').then(function (mod) {
        return mod.init().then(function () { return mod; });
      });
    }
    return pagefindPromise;
  }

  function setUrlQuery(query) {
    var url = new URL(window.location.href);
    if (query) {
      url.searchParams.set('q', query);
    } else {
      url.searchParams.delete('q');
    }
    window.history.replaceState(null, '', url.pathname + url.search);
  }

  function clearResultsArea() {
    resultsList.replaceChildren();
    resultsList.hidden = true;
    if (searchStatus) searchStatus.hidden = true;
    if (resultsCount) resultsCount.hidden = true;
    emptyState.hidden = true;
  }

  function renderQueryResults(items, query, total) {
    resultsList.replaceChildren();

    if (items.length === 0) {
      resultsList.hidden = true;
      if (searchStatus) searchStatus.hidden = true;
      emptyState.hidden = false;
      var title = emptyState.querySelector('.search-empty-title');
      var body = emptyState.querySelector('p');
      if (title) title.textContent = 'No results found';
      if (body) body.textContent = 'No articles matched “' + query + '”. Try a different search term or browse by category.';
    } else {
      emptyState.hidden = true;
      resultsList.hidden = false;
      items.forEach(function (item) {
        resultsList.appendChild(buildResultNode(item));
      });
      if (searchStatus) {
        searchStatus.hidden = false;
        searchStatus.textContent = 'Showing all ' + formatCount(items.length) + ' results';
      }
    }

    if (resultsCount) {
      resultsCount.hidden = false;
      resultsCount.replaceChildren();
      resultsCount.append(formatCount(total) + ' results for ');
      var term = document.createElement('span');
      term.className = 'query-term';
      term.textContent = '“' + query + '”';
      resultsCount.appendChild(term);
    }
  }

  function buildResultNode(item) {
    var node = resultTemplate.content.firstElementChild.cloneNode(true);

    var typeBadge = node.querySelector('[data-field="type-badge"]');
    if (item.type === 'concept') {
      typeBadge.className = 'result-badge result-badge--type-concept';
      typeBadge.textContent = 'Concept';
    } else {
      typeBadge.className = 'result-badge result-badge--type';
      typeBadge.textContent = 'Source';
    }

    node.querySelector('[data-field="category-badge"]').textContent = item.category;
    node.querySelector('[data-field="tier-badge"]').textContent = item.tier;

    var titleLink = node.querySelector('[data-field="title-link"]');
    titleLink.href = item.url;
    titleLink.textContent = item.title;

    // The one field allowed to be HTML: Pagefind's <mark>-highlighted excerpt.
    node.querySelector('[data-field="excerpt"]').innerHTML = item.excerpt;

    node.querySelector('[data-field="url"]').textContent = item.url;

    return node;
  }

  function renderFacetSidebar(counts) {
    filterSidebar.querySelectorAll('.filter-group').forEach(function (el) { el.remove(); });

    GROUP_ORDER.forEach(function (group) {
      var values = counts[group];
      if (!values) return;

      var groupEl = document.createElement('div');
      groupEl.className = 'filter-group';

      var titleEl = document.createElement('div');
      titleEl.className = 'filter-group-title';
      titleEl.textContent = GROUP_TITLES[group] || humanizeLabel(group);
      groupEl.appendChild(titleEl);

      Object.keys(values).sort().forEach(function (value) {
        groupEl.appendChild(buildFilterOptionNode(group, value, values[value]));
      });

      filterSidebar.appendChild(groupEl);
    });
  }

  function buildFilterOptionNode(group, value, count) {
    var node = filterOptionTemplate.content.firstElementChild.cloneNode(true);
    var active = isFilterActive(activeFilters, group, value);

    node.classList.toggle('active', active);
    node.setAttribute('role', 'checkbox');
    node.setAttribute('aria-checked', active ? 'true' : 'false');
    node.querySelector('[data-field="label"]').textContent = humanizeLabel(value);
    node.querySelector('[data-field="count"]').textContent = formatCount(count);

    node.addEventListener('click', function (e) {
      e.preventDefault();
      activeFilters = toggleFilter(activeFilters, group, value);
      runSearch(input.value.trim());
    });

    return node;
  }

  // Converts {group: [values]} into Pagefind's own filter query shape.
  // Pagefind's *default* is AND: a bare array like {category: ["a", "b"]}
  // means "matches a AND matches b" (impossible for a single-valued field
  // like ours) — confirmed empirically against the real bundle, and by
  // Pagefind's own docs ("When unspecified, all filtering defaults to
  // 'AND' filtering"). Within-group OR needs the explicit `{any: [...]}`
  // form; a single active value needs no wrapping at all. Multiple *keys*
  // on the returned object are Pagefind's native across-group AND, so no
  // extra work is needed for that half of decision #4.
  function toPagefindFilters(state) {
    var filters = {};
    Object.keys(state).forEach(function (group) {
      var values = state[group];
      filters[group] = values.length > 1 ? { any: values } : values[0];
    });
    return filters;
  }

  function runSearch(query) {
    setUrlQuery(query);

    if (!query) {
      clearResultsArea();
      if (allFacets) renderFacetSidebar(allFacets);
      return Promise.resolve();
    }

    return loadPagefind().then(function (pagefind) {
      return pagefind.search(query, { filters: toPagefindFilters(activeFilters) });
    }).then(function (response) {
      return Promise.all(response.results.map(function (r) { return r.data(); })).then(function (dataList) {
        var items = dataList.map(mapResultData);
        renderQueryResults(items, query, items.length);
        if (allFacets) {
          renderFacetSidebar(mergeFacetCounts(allFacets, response.filters));
        }
      });
    }).catch(function (err) {
      console.error('haleiki search: query failed', err);
    });
  }

  var debouncedSearch = debounce(function () {
    runSearch(input.value.trim());
  }, DEBOUNCE_MS);

  input.addEventListener('input', debouncedSearch);

  if (form) {
    form.addEventListener('submit', function (e) {
      e.preventDefault();
      runSearch(input.value.trim());
    });
  }

  function init() {
    clearResultsArea();

    loadPagefind().then(function (pagefind) {
      return pagefind.filters();
    }).then(function (facets) {
      allFacets = facets;
      renderFacetSidebar(allFacets);

      var seeded = parseQuery(window.location.search);
      if (seeded) {
        input.value = seeded;
        runSearch(seeded);
      }
    }).catch(function (err) {
      console.error('haleiki search: failed to initialize Pagefind', err);
    });
  }

  init();
})();
