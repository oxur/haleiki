// Client-side search wiring for the /search/ page.
//
// The template renders whatever the server put in `search.results`. If you use a
// client index (e.g. Pagefind), initialise it here and re-render
// .search-results-list from the query string. Left as an integration point.
(function () {
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
})();
