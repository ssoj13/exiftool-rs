---
name: gitnexus-rs-impact-analysis
description: "Use when the user wants to know what will break if they change something, or needs safety analysis before editing code, with gitnexus-rs. Examples: \"Is it safe to change X?\", \"What depends on this?\", \"What will break?\""
---

# Impact Analysis with gitnexus-rs

## When to Use

- "Is it safe to change this function?"
- "What will break if I modify X?"
- "Show me the blast radius"
- "Who uses this code?"
- Before making non-trivial code changes
- Before committing — to understand what your changes affect

## Bind the repository first

Impact analysis is the gate that authorizes an edit, so it must answer for the
repository you are about to edit.

Call `list_repos {}` before the first tool call. With one indexed repository,
use the examples below as written. With more than one, pass `repo` on every
call. If you cannot tell which repository is meant, stop and ask — every result
below an ambiguous identity inherits the ambiguity.

`detect_changes` diffs the checkout the repository is registered at. If you
edited a different checkout (a linked worktree, a copy), it reports zero
changed symbols — a false clean check. Confirm the `repo` in the result is the
checkout you edited; in a terminal, `gitnexus-rs detect-changes --repo <path>`
names the checkout explicitly.

State the bound identity with your risk report:

```
Repository: <name> (<path>)   Index: <last_indexed_commit> vs HEAD <head_commit>
```

## Workflow

```
0. list_repos {} + graph_status({repo})                    → Bind repo, check freshness
1. impact({target: "X", direction: "upstream"})            → Dependents by depth
2. READ gitnexus://repo/{name}/processes                   → Check affected execution flows
3. detect_changes({scope: "all"})                          → What your edits touch
4. Assess risk and report to the user, echoing repo/index identity
```

> Stale index (`graph_status`: `is_stale: true`) → `reanalyze` (MCP, up to 50
> files), `gitnexus-rs watch`, or `gitnexus-rs analyze <repo>`. An impact on a
> stale graph describes the code from before your edits.
> MCP unavailable? Every tool runs from a terminal:
> `gitnexus-rs tool --name impact --args '{"target":"X","direction":"upstream"}'`
> and `gitnexus-rs detect-changes --repo . --scope all`.

## Checklist

```
- [ ] list_repos {} — bind repo; explicit repo when >1 indexed, ask if ambiguous
- [ ] graph_status — index fresh, local edits indexed if they matter
- [ ] impact({target, direction: "upstream"}) to find dependents
- [ ] Review depth 1 first (these WILL BREAK)
- [ ] Check high-confidence (>0.8) dependencies
- [ ] READ processes to check affected execution flows
- [ ] detect_changes({scope: "all"}) for the pre-commit check
- [ ] Confirm the checkout you edited is the checkout that was diffed
- [ ] Assess risk level and report, stating repo/index identity
```

## Understanding Output

`impact` groups results in `byDepth`:

| Depth | Risk Level | Meaning |
| --- | --- | --- |
| 1 | **WILL BREAK** | Direct callers/importers |
| 2 | LIKELY AFFECTED | Indirect dependencies |
| 3 | MAY NEED TESTING | Transitive effects |

It also returns `risk`, `affected_processes`, `affected_modules`, and
`partial: true` when the traversal hit a limit before finishing.

## Risk Assessment

`impact` computes `risk` from direct callers, all impacted symbols, processes
and modules:

| Affected | Risk |
| --- | --- |
| <5 direct and <30 impacted | LOW |
| ≥5 direct or ≥30 impacted | MEDIUM |
| ≥15 direct, ≥3 processes, ≥3 modules or ≥100 impacted | HIGH |
| ≥30 direct, ≥5 processes, ≥5 modules or ≥200 impacted | CRITICAL |
| Target not found or ambiguous | **UNKNOWN** |

`UNKNOWN` is not a low rung on this scale — the tool could not answer. For an
ambiguous name it returns `candidates`: repeat with `target_uid`, `file_path`
or `kind`.

**Zero callers is not proof of "unused".** An empty depth 1 still scores LOW,
but it is equally consistent with callers the index cannot resolve (dynamic
dispatch, trait objects, macros, FFI, cross-language calls, reflection).
Confirm with a text search before treating the symbol as safe to change or
delete; do not proceed on the strength of a zero.

`partial: true` means the walk stopped early: the counts are a lower bound and
the risk may be higher. Narrow the query (`maxDepth`, `relationTypes`,
`minConfidence`) and run it again before relying on it.

Warn the user on HIGH/CRITICAL before editing; stop on UNKNOWN until the
ambiguity is resolved.

## Tools

**impact** — the primary tool for symbol blast radius:

```
impact({
  target: "validateUser",
  repo: "my-app",          // required once >1 repository is indexed
  direction: "upstream",   // "downstream" = what X depends on
  minConfidence: 0.8,
  maxDepth: 3
})

→ depth 1 (WILL BREAK):
  - loginHandler (src/auth/login.rs:42) [CALLS, 100%]
  - apiMiddleware (src/api/middleware.rs:15) [CALLS, 100%]

→ depth 2 (LIKELY AFFECTED):
  - authRouter (src/routes/auth.rs:22) [CALLS, 95%]
```

Tests are excluded unless `includeTests: true`; `relationTypes` limits the
walk to chosen edge types (e.g. `["CALLS"]`).

**detect_changes** — git-diff based impact analysis. `scope` is `unstaged`
(default), `staged`, `all`, or `compare` with `base_ref`:

```
detect_changes({scope: "all"})

→ changed_files: 3, changed_symbols: 5
→ affected_processes: LoginFlow, TokenRefresh, APIMiddlewarePipeline
→ risk_level: medium
```

`risk_level` is `none`, `low`, `medium`, `high` or `critical`. In a terminal,
`--fail-on <level>` makes `gitnexus-rs detect-changes` exit 1 at or above a
level — usable as a pre-commit gate.

A wrong-checkout zero is shape-identical to a genuine clean result, so confirm
the checkout you edited is the one that was diffed before treating an empty
change set as a passed check.

## Example: "What breaks if I change validateUser?"

```
0. list_repos {}
   → two repos (my-app, billing-api) — both define validateUser, so bind explicitly
   graph_status({repo: "my-app"}) → fresh

1. impact({target: "validateUser", repo: "my-app", direction: "upstream"})
   → depth 1: loginHandler, apiMiddleware (WILL BREAK)
   → depth 2: authRouter, sessionManager (LIKELY AFFECTED)

2. READ gitnexus://repo/my-app/processes
   → LoginFlow and TokenRefresh touch validateUser

3. Risk: 2 direct callers, 2 processes = LOW by the table, but LoginFlow is
   the auth path — report it as critical to the user
   Repository: my-app (/abs/path/my-app)  Index: fresh
```

With a single indexed repository, the `repo` argument drops out of every call
above.

<!-- gitnexus-rs:skill fnv1a64=fdabce575902c0fe - generated by `gitnexus-rs analyze`; edits are kept, delete the file to restore it -->
