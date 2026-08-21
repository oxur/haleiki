// Browser-asset quality gate for shipped classic scripts under assets/js/.
// This is Deno tooling, but it does not apply Deno runtime lint/fmt policy to
// browser assets. It verifies the browser-script contract directly.

const decoder = new TextDecoder();

const forbiddenPatterns = [
  {
    name: "CommonJS require",
    pattern: /\brequire\s*\(/,
  },
  {
    name: "CommonJS module.exports",
    pattern: /\bmodule\.exports\b/,
  },
  {
    name: "Node process global",
    pattern: /\bprocess\./,
  },
  {
    name: "Deno runtime global",
    pattern: /\bDeno\./,
  },
  {
    name: "Node compatibility import",
    pattern: /\bnode:/,
  },
  {
    name: "Deno npm/jsr specifier",
    pattern: /\b(?:npm|jsr):/,
  },
  {
    name: "Node project manifest",
    pattern: /\bpackage(?:-lock)?\.json\b/,
  },
  {
    name: "local node_modules dependency tree",
    pattern: /\bnode_modules\b/,
  },
  {
    name: "npm install workflow",
    pattern: /\bnpm\s+install\b/,
  },
  {
    name: "npm run workflow",
    pattern: /\bnpm\s+run\b/,
  },
];

function fail(message) {
  console.error(message);
  Deno.exit(1);
}

async function trackedBrowserAssets() {
  const command = new Deno.Command("git", {
    args: ["ls-files", "assets/js/*.js"],
    stdout: "piped",
    stderr: "piped",
  });
  const result = await command.output();
  if (!result.success) {
    fail(decoder.decode(result.stderr).trim() || "git ls-files failed");
  }
  return decoder.decode(result.stdout).trim().split("\n").filter(Boolean);
}

async function checkClassicScriptSyntax(path) {
  const source = await Deno.readTextFile(path);
  try {
    // Parse as a classic script, not an ES module. This catches static
    // import/export while allowing the runtime dynamic import() used by search.js.
    new Function(source);
  } catch (error) {
    throw new Error(`${path}: not valid classic browser JavaScript`, {
      cause: error,
    });
  }
  return source;
}

function checkForbiddenPatterns(path, source) {
  const failures = [];
  for (const { name, pattern } of forbiddenPatterns) {
    const lines = source.split("\n");
    for (let index = 0; index < lines.length; index++) {
      const line = lines[index];
      if (pattern.test(line)) {
        failures.push(`${path}:${index + 1}: ${name}: ${line.trim()}`);
      }
    }
  }
  return failures;
}

const files = await trackedBrowserAssets();
if (files.length === 0) {
  fail("No tracked browser assets found under assets/js/*.js");
}

const failures = [];
for (const file of files) {
  try {
    const source = await checkClassicScriptSyntax(file);
    failures.push(...checkForbiddenPatterns(file, source));
  } catch (error) {
    failures.push(error.message);
    if (error.cause) failures.push(`  cause: ${error.cause.message}`);
  }
}

if (failures.length > 0) {
  console.error("Browser JS gate failed:");
  for (const failure of failures) console.error(`  ${failure}`);
  Deno.exit(1);
}

console.log("Browser JS gate passed for tracked shipped assets:");
for (const file of files) console.log(`  ${file}`);
