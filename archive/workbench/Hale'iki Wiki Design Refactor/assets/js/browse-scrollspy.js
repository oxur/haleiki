// Highlights the active category in the browse-page left index while scrolling.
(function () {
  var sections = document.querySelectorAll('.browse-category-section');
  var navLinks = document.querySelectorAll('.toc-list a');
  if (!sections.length || !navLinks.length) return;
  var observer = new IntersectionObserver(function (entries) {
    entries.forEach(function (entry) {
      if (!entry.isIntersecting) return;
      var id = entry.target.getAttribute('id');
      navLinks.forEach(function (link) {
        link.classList.toggle('active', link.getAttribute('href') === '#' + id);
      });
    });
  }, { rootMargin: '-80px 0px -65% 0px', threshold: 0 });
  sections.forEach(function (s) { observer.observe(s); });
})();
