// Test-only harness for the Arc 04 Slice 02 headless-browser suite: builds
// the demo site, starts a live `haleiki serve`, and serves a built `_site/`
// statically — the "both sinks" the ledger's H-7/D-6 sink-parity row needs.
// Deno-side only; never shipped to the browser (search.js has no dependency
// on anything here).

const REPO_ROOT = new URL("../..", import.meta.url).pathname;
const HALEIKI_BIN = REPO_ROOT + "target/debug/haleiki";
const SERVE_PORT = 18080;
const STATIC_PORT = 18081;

/** @type {Record<string, string>} */
const ASSET_CONTENT_TYPES = {
  ".html": "text/html",
  ".css": "text/css",
  ".js": "application/javascript",
  ".woff2": "font/woff2",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".ico": "image/x-icon",
  ".json": "application/json",
};

/** @type {Record<string, string>} */
const PAGEFIND_CONTENT_TYPES = {
  ".js": "text/javascript",
  ".wasm": "application/wasm",
  ".pagefind": "application/wasm",
  ".json": "application/json",
};

function extensionOf(pathname) {
  const dot = pathname.lastIndexOf(".");
  return dot === -1 ? "" : pathname.slice(dot).toLowerCase();
}

function contentTypeFor(pathname) {
  const table = pathname.startsWith("/pagefind/")
    ? PAGEFIND_CONTENT_TYPES
    : ASSET_CONTENT_TYPES;
  return table[extensionOf(pathname)] || "application/octet-stream";
}

/**
 * Builds the haleiki binary (debug profile) so the harness always exercises
 * the current tree, not a stale binary.
 * @returns {Promise<void>}
 */
export async function buildHaleiki() {
  const cmd = new Deno.Command("cargo", {
    args: ["build"],
    cwd: REPO_ROOT + "tools",
    stdout: "inherit",
    stderr: "inherit",
  });
  const { success } = await cmd.output();
  if (!success) {
    throw new Error("cargo build failed");
  }
}

/**
 * Runs `haleiki export` against the demo corpus into `outDir`.
 * @param {string} outDir
 * @returns {Promise<void>}
 */
export async function exportDemoSite(outDir) {
  const cmd = new Deno.Command(HALEIKI_BIN, {
    args: [
      "export",
      "--content-dir",
      REPO_ROOT + "demo",
      "--templates-dir",
      REPO_ROOT + "templates",
      "--assets-dir",
      REPO_ROOT + "assets",
      "--out-dir",
      outDir,
    ],
    stdout: "inherit",
    stderr: "inherit",
  });
  const { success } = await cmd.output();
  if (!success) {
    throw new Error("haleiki export failed");
  }
}

async function waitForReady(url, attempts = 60, delayMs = 250) {
  for (let i = 0; i < attempts; i++) {
    try {
      const response = await fetch(url);
      await response.body?.cancel();
      return;
    } catch {
      await new Promise((resolve) => setTimeout(resolve, delayMs));
    }
  }
  throw new Error(`server at ${url} never became ready`);
}

/**
 * Starts a live `haleiki serve` against the demo corpus.
 * @returns {Promise<{ url: string, close: () => Promise<void> }>}
 */
export async function startLiveServer() {
  const bind = `127.0.0.1:${SERVE_PORT}`;
  const command = new Deno.Command(HALEIKI_BIN, {
    args: [
      "serve",
      "--content-dir",
      REPO_ROOT + "demo",
      "--templates-dir",
      REPO_ROOT + "templates",
      "--assets-dir",
      REPO_ROOT + "assets",
      "--bind",
      bind,
    ],
    stdout: "piped",
    stderr: "piped",
  });
  const child = command.spawn();
  const url = `http://${bind}`;
  await waitForReady(url + "/");
  return {
    url,
    close: async () => {
      child.kill();
      await child.status;
    },
  };
}

/**
 * Serves a built `_site/` directory over plain HTTP — the static-export
 * sink, hosted the way any static file host would.
 * @param {string} siteDir
 * @returns {{ url: string, close: () => Promise<void> }}
 */
export function startStaticServer(siteDir) {
  const server = Deno.serve(
    { port: STATIC_PORT, onListen: () => {} },
    async (req) => {
      const url = new URL(req.url);
      let pathname = url.pathname;
      if (pathname.endsWith("/")) pathname += "index.html";

      const filePath = siteDir + pathname;
      try {
        const file = await Deno.readFile(filePath);
        return new Response(file, {
          headers: { "Content-Type": contentTypeFor(pathname) },
        });
      } catch {
        try {
          const notFound = await Deno.readFile(siteDir + "/404.html");
          return new Response(notFound, {
            status: 404,
            headers: { "Content-Type": "text/html" },
          });
        } catch {
          return new Response("404 Not Found", { status: 404 });
        }
      }
    },
  );

  return {
    url: `http://127.0.0.1:${STATIC_PORT}`,
    close: () => server.shutdown(),
  };
}
