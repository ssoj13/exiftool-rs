---
name: gitnexus-rs-refactoring
description: "Use when the user wants to rename, extract, split, move, or restructure code safely with gitnexus-rs. Examples: \"Rename this function\", \"Extract this into a module\", \"Refactor this class\", \"Move this to a separate file\""
---

# Refactoring with gitnexus-rs

## When to Use

- "Rename this function safely"
- "Extract this into a module"
- "Split this service"
- "Move this to a new file"
- Any task involving renaming, extracting, splitting, or restructuring code

## Bind the repository first

Refactoring writes to disk. `rename` with `dry_run: false` edits files in
whichever repository was resolved, so binding identity here is a safety gate,
not bookkeeping.

Call `list_repos {}` before the first tool call. With one indexed repository,
use the examples below as written. With more than one, pass `repo` on every
call. If you cannot tell which repository is meant, stop and ask. `rename`
previews by default (`dry_run` defaults to `true`); never run it with
`dry_run: false` until the preview in the same bound repository has been
reviewed — its `file_path` values show which checkout is about to be written,
so read them as a confirmation of identity.

`detect_changes` diffs the checkout the repository is registered at; if you
edited a different checkout it reports nothing changed, which reads as a
verified refactor. Confirm the `repo` in the result is the checkout you edited.

## Workflow

```
0. list_repos {} + graph_status({repo})           → Bind repo, check freshness
1. impact({target: "X", direction: "upstream"})   → Map all dependents
2. query({query: "X"})                            → Find execution flows involving X
3. context({name: "X"})                           → See all incoming/outgoing refs
4. Plan update order: interfaces/traits → implementations → callers → tests
```

> Stale (`is_stale: true`) → `reanalyze` (MCP, up to 50 files),
> `gitnexus-rs watch`, or `gitnexus-rs analyze <repo>`. A rename planned on a
> stale graph misses references added since the last index.

## Checklists

### Rename Symbol

```
- [ ] list_repos {} — bind repo; explicit repo when >1 indexed, ask if ambiguous
- [ ] rename({symbol_name: "oldName", new_name: "newName"}) — preview (dry_run defaults to true)
- [ ] Confirm the previewed file paths are in the bound repository
- [ ] Review graph_edits (high confidence) and text_search_edits (review carefully)
- [ ] If satisfied: rename({..., dry_run: false}) — apply edits
- [ ] detect_changes({scope: "all"}) — verify only expected files changed
- [ ] Build and run tests for affected processes
```

### Extract Module

```
- [ ] list_repos {} — bind repo; explicit repo when >1 indexed, ask if ambiguous
- [ ] context({name: target}) — see all incoming/outgoing refs
- [ ] impact({target, direction: "upstream"}) — find all external callers
- [ ] Define new module interface
- [ ] Extract code, update imports/uses
- [ ] detect_changes({scope: "all"}) — verify affected scope
- [ ] Build and run tests for affected processes
```

### Split Function/Service

```
- [ ] list_repos {} — bind repo; explicit repo when >1 indexed, ask if ambiguous
- [ ] context({name: target}) — understand all callees
- [ ] Group callees by responsibility
- [ ] impact({target, direction: "upstream"}) — map callers to update
- [ ] Create new functions/services
- [ ] Update callers
- [ ] detect_changes({scope: "all"}) — verify affected scope
- [ ] Build and run tests for affected processes
```

## Tools

**rename** — coordinated multi-file rename (disambiguate with `symbol_uid` or
`file_path`):

```
rename({symbol_name: "validateUser", new_name: "authenticateUser", repo: "my-app"})
→ total_edits: 12, files_affected: 8, applied: false
→ graph_edits: 10 (high confidence), text_search_edits: 2 (review)
→ changes: [{file_path, edits: [{line, old_text, new_text, confidence}]}]
```

**impact** — map all dependents first:

```
impact({target: "validateUser", repo: "my-app", direction: "upstream"})
→ depth 1: loginHandler, apiMiddleware, testUtils
→ affected_processes: LoginFlow, TokenRefresh
```

**detect_changes** — verify your changes after refactoring:

```
detect_changes({scope: "all"})
→ changed_files: 8, changed_symbols: 12
→ affected_processes: LoginFlow, TokenRefresh
→ risk_level: medium
```

A wrong-checkout zero is indistinguishable from a clean verification, so
confirm the diffed checkout is the one you edited. Run `reanalyze` after a
large refactor before the next impact question: the graph still describes the
old names until then.

**cypher** — custom reference queries (read-only):

```cypher
MATCH (caller)-[:CodeRelation {type: 'CALLS'}]->(f:Function {name: "validateUser"})
RETURN caller.name, caller.filePath ORDER BY caller.filePath
```

## Risk Rules

| Risk Factor | Mitigation |
| --- | --- |
| Many callers (>5) | Use rename for automated updates |
| Cross-area refs | Use detect_changes after to verify scope |
| String/dynamic refs, macros | query and text search to find them |
| External/public API | Version and deprecate properly |
| Same name in another indexed repo | Bind `repo`; verify previewed paths before applying |

## Example: Rename `validateUser` to `authenticateUser`

```
0. list_repos {}
   → two repos (my-app, billing-api) — both define validateUser, so bind explicitly

1. rename({symbol_name: "validateUser", new_name: "authenticateUser", repo: "my-app"})
   → 12 edits: 10 graph (safe), 2 text_search (review)
   → Files: validator.rs, login.rs, middleware.rs, config.toml...

2. Review text_search edits (config.toml: dynamic reference!)

3. rename({symbol_name: "validateUser", new_name: "authenticateUser", repo: "my-app", dry_run: false})
   → applied: true, 12 edits across 8 files

4. detect_changes({scope: "all", repo: "my-app"})
   → affected_processes: LoginFlow, TokenRefresh
   → risk_level: medium — run tests for these flows
   Repository: my-app (/abs/path/my-app)  Index: fresh
```

With a single indexed repository, the `repo` argument drops out of every call
above.

<!-- gitnexus-rs:skill fnv1a64=37f6ac5f65789b01 - generated by `gitnexus-rs analyze`; edits are kept, delete the file to restore it -->
