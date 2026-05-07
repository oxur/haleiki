(function() {
  var STORAGE_KEY = 'haleiki-theme';
  function getPreferred() {
    var s = localStorage.getItem(STORAGE_KEY);
    if (s) return s;
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) return 'night';
    return 'day';
  }
  function setTheme(t) {
    document.documentElement.setAttribute('data-theme', t);
    localStorage.setItem(STORAGE_KEY, t);
  }
  setTheme(getPreferred());
  window.haleikiToggleTheme = function() {
    setTheme(document.documentElement.getAttribute('data-theme') === 'day' ? 'night' : 'day');
  };
})();
