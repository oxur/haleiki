// Renders math emitted by the Rust markdown pipeline (comrak's `math_dollars`
// extension), not by scanning the page for `$`/`$$` delimiters. The server
// already found each math span correctly (respecting code context — `$` in
// `<code>`/`<pre>` was never treated as math) and marked it with
// `data-math-style="inline"` or `"display"`; `el.textContent` gives back the
// original TeX (the DOM un-escapes the HTML entities the renderer used to
// make it safe to embed).
document.addEventListener("DOMContentLoaded", function() {
  if (typeof katex === 'undefined') return;
  document.querySelectorAll('[data-math-style]').forEach(function (el) {
    katex.render(el.textContent, el, {
      displayMode: el.dataset.mathStyle === 'display',
      throwOnError: false
    });
  });
});
