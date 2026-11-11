// Highlights the active entry in the concept/source left-nav table of contents
// as the reader scrolls. Progressive enhancement — the TOC works without it.
(function () {
  var tocLinks = Array.prototype.slice.call(document.querySelectorAll('.toc-list a'));
  if (!tocLinks.length) return;
  var sections = tocLinks
    .map(function (l) { return document.getElementById(l.getAttribute('href').replace('#', '')); })
    .filter(Boolean);
  if (!sections.length) return;

  function update() {
    var currentId = sections[0].id;
    for (var i = 0; i < sections.length; i++) {
      if (sections[i].getBoundingClientRect().top <= 120) currentId = sections[i].id;
    }
    tocLinks.forEach(function (l) {
      l.classList.toggle('active', l.getAttribute('href') === '#' + currentId);
    });
  }

  var ticking = false;
  window.addEventListener('scroll', function () {
    if (ticking) return;
    ticking = true;
    requestAnimationFrame(function () { update(); ticking = false; });
  }, { passive: true });
  update();
})();
