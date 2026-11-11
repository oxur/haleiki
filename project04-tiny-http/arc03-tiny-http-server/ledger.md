# Arc ledger

> Extracted without row changes on 2026-11-11 from the August plan. Existing dispositions/evidence are retained, not freshly reproduced by the migration.

Opens here; closes (per-row walk) in `closing-report.md` at arc close. Class (a)
= slices closed; class (b) = composition, **reproduced at arc scale** (run a live
server and exercise it); class (c) = bubble-up findings dispositioned.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| C-1 | Slice 01 (route map + core server) closed | ptr: `slice01-route-map-server/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 01 CDC-closed 2026-07-28, 8/8 rows; traversal guard audited live |
| C-2 | Slice 02 (hardening + composition) closed | ptr: `slice02-hardening/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 02 CDC-closed 2026-07-29, 8/8 rows; hardened surface (HEAD/405/nosniff/log) audited live |
| C-3 | A live `haleiki serve` against the demo serves every route type (home, browse, category, concept, source, about, attribution, search) with `200` + `text/html`, assets with `200` + correct content-type, and unknown paths with `404` + the 404 body | reproduce at arc scale: start the server, HTTP-GET a sample of each | serious | arc-plan | done | reproduced at arc scale — live sweep of all 8 route kinds + `/assets/css/haleiki.css` all `200`; `/nope/` `404` (see `closing-report.md`) |
| C-4 | Server ↔ export parity: a served route's body is identical to the file the export writes for that route (one pipeline, two sinks) | reproduce: diff a served body against the exported file | serious | arc-plan | done | reproduced — parity test (served body == exported file) green, incl. through the hardened `handle()`; export suite unchanged |
| C-5 | Hardened for self-host: configurable bind/port; a per-request error does not crash the server; requests are logged | reproduce: bind a chosen port; induce a bad request; observe the server survives + logs | serious | arc-plan | done | reproduced — `--bind` honored; live panic-isolation (worker survives); per-request stderr access log; HEAD empty-body/Content-Length, POST 405+Allow, nosniff confirmed on the wire |

_(No class-(c) bubble-up rows surfaced within the arc; forward findings are
dispositioned in `closing-report.md` §"Bubble-up to the project".)_
