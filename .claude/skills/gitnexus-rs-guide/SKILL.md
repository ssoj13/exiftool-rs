---
name: gitnexus-rs-guide
description: "Use when the user asks about gitnexus-rs itself — available MCP tools, how to query the knowledge graph, MCP resources, graph schema, or workflow reference. Examples: \"What gitnexus-rs tools are available?\", \"How do I use gitnexus-rs?\""
---

# gitnexus-rs Guide

Quick reference for the gitnexus-rs MCP tools, resources, and the knowledge
graph schema.

## Always Start Here

For any task involving code understanding, debugging, impact analysis, or
refactoring:

1. **`list_repos {}`** — bind the repository (pass `repo` on every call when
   more than one is indexed; ask when it is ambiguous)
2. **`graph_status({repo})`** — check index freshness
3. **Match your task to a skill below** and **read that skill file**
4. **Follow the skill's workflow and checklist**

> `is_stale: true` → `reanalyze` (MCP, incremental, up to 50 files) or
> `gitnexus-rs analyze <repo>` in a terminal (full). `worktree_has_unstaged:
> true` → `reanalyze({scope: "unstaged"})`. `detect_changes` reads git, it does
> **not** re-index.

## Skills

| Task | Skill to read |
| --- | --- |
| Understand architecture / "How does X work?" | `gitnexus-rs-exploring` |
| Blast radius / "What breaks if I change X?" | `gitnexus-rs-impact-analysis` |
| Trace bugs / "Why is X failing?" | `gitnexus-rs-debugging` |
| Rename / extract / split / refactor | `gitnexus-rs-refactoring` |
| Tools, resources, schema reference | `gitnexus-rs-guide` (this file) |
| Index, status, clean, wiki CLI commands | `gitnexus-rs-cli` |

## Tools Reference

Tool names carry no prefix: call `impact`, `query`, `context` as written.

| Tool | What it gives you |
| --- | --- |
| `list_repos` | Indexed repositories: the global registry plus a scan of `GITNEXUS_REPOS_DIR` |
| `graph_status` | Index freshness: `is_stale`, `staleness`, `worktree_has_unstaged`, `last_indexed_commit` vs `head_commit`, `recommended_next` |
| `reanalyze` | Incremental re-index of changed files (`scope: "unstaged"` for local edits; capped at 50 files — run `analyze` beyond that) |
| `query` | Process-grouped code intelligence — execution flows related to a concept |
| `context` | 360-degree symbol view — categorized refs, processes it participates in |
| `impact` | Symbol blast radius — what breaks at depth 1/2/3, `risk`, affected processes/modules |
| `detect_changes` | Git-diff impact — what your current changes affect (`scope`: unstaged/staged/all/compare) |
| `rename` | Multi-file coordinated rename with confidence-tagged edits (`dry_run` defaults to `true`) |
| `cypher` | Raw read-only graph queries (read `gitnexus://repo/{name}/schema` first) |
| `route_map` | API route mappings: handlers, middleware wrapper chains, consumers, flows |
| `shape_check` | Response-shape drift — keys a route returns vs keys its consumers access (MISMATCH) |
| `api_impact` | Pre-change report for an API route — consumers, field accesses, middleware, risk |
| `tool_map` | MCP/RPC tool definitions, their handlers and linked flows |
| `graph_validate` | Graph shape check — labels, relationship types, schema version, anomalies |
| `group_list` | Configured multi-repo groups, or one group's repos and manifest links |
| `group_sync` | Rebuild a group's Contract Registry (cross-repo HTTP contracts) |

There is no `trace` tool: answer "how does A reach B?" with a variable-length
`CALLS` path in `cypher` (see `gitnexus-rs-debugging`).

### Freshness is a separate call

Read tools do not attach a staleness field to their answers. Ask
`graph_status` once per repository before relying on `query` / `context` /
`impact` / `cypher`, and again after you edit files or switch branches:

```jsonc
{
  "is_stale": false,
  "staleness": "fresh",
  "worktree_has_unstaged": true,
  "last_indexed_commit": "4f2a1c9…",
  "head_commit": "4f2a1c9…",
  "recommended_next": "reanalyze with scope unstaged"
}
```

`is_stale: false` with `worktree_has_unstaged: true` means the commit is
indexed but your uncommitted edits are not.

### Risk levels

`impact.risk`: `LOW` / `MEDIUM` / `HIGH` / `CRITICAL`, or `UNKNOWN` when the
target was not found or is ambiguous (then `candidates` lists the matches —
retry with `target_uid` or `file_path`). `detect_changes.risk_level`: `none` /
`low` / `medium` / `high` / `critical`. Zero callers reports `LOW`, which is not
proof that a symbol is unused: dynamic dispatch, macros and string references
are invisible to the graph.

## Resources Reference

Lightweight reads for navigation:

| Resource | Content |
| --- | --- |
| `gitnexus://repos` | All indexed repositories |
| `gitnexus://repo/{name}/context` | Stats and overview of one repository |
| `gitnexus://repo/{name}/clusters` | All functional areas with cohesion scores |
| `gitnexus://repo/{name}/cluster/{clusterName}` | Area members |
| `gitnexus://repo/{name}/processes` | All execution flows |
| `gitnexus://repo/{name}/process/{processName}` | Step-by-step trace |
| `gitnexus://repo/{name}/schema` | Graph schema of that repository, for Cypher |
| `gitnexus://schema` | Global graph schema |
| `gitnexus://setup` | Setup / onboarding notes |
| `gitnexus://stats` | Server-wide statistics |
| `gitnexus://languages` | Supported languages |
| `gitnexus://group/{name}/contracts` | A group's cross-repo contracts |
| `gitnexus://group/{name}/status` | A group's sync status |

## Graph Schema

**Nodes:** Project, Package, Module, Folder, File, Class, Function, Method,
Variable, Interface, Enum, Decorator, Import, Type, CodeElement, Community,
Process, Struct, Macro, Typedef, Union, Namespace, Trait, Impl, TypeAlias,
Const, Static, Property, Record, Delegate, Annotation, Constructor, Template,
Section, Route, Tool.

**Edges (via `CodeRelation.type`):** CONTAINS, CALLS, INHERITS,
METHOD_OVERRIDES, METHOD_IMPLEMENTS, IMPORTS, USES, DEFINES, DECORATES,
IMPLEMENTS, EXTENDS, HAS_METHOD, HAS_PROPERTY, ACCESSES, MEMBER_OF,
STEP_IN_PROCESS, HANDLES_ROUTE, FETCHES, HANDLES_TOOL, ENTRY_POINT_OF, WRAPS,
QUERIES.

Read `gitnexus://repo/{name}/schema` before writing Cypher — it is the
authoritative schema for the indexed repository.

```cypher
MATCH (caller)-[:CodeRelation {type: 'CALLS'}]->(f:Function {name: "myFunc"})
RETURN caller.name, caller.filePath
```

<!-- gitnexus-rs:skill fnv1a64=6789c856810af037 - generated by `gitnexus-rs analyze`; edits are kept, delete the file to restore it -->
