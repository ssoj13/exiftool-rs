---
name: gitnexus-rs-cli
description: "Use when the user needs to run gitnexus-rs CLI commands like analyze/index a repo, check status, clean the index, generate a wiki, or watch for changes. Examples: \"Index this repo\", \"Reanalyze the codebase\", \"Generate a wiki\""
---

# gitnexus-rs CLI Commands

`gitnexus-rs` is a single native binary: no Node.js, no runner script, no
package manager. Commands below assume it is on `PATH`; run them from the
project root or pass the repository path explicitly.

## Commands

### analyze — Build or refresh the index

```bash
gitnexus-rs analyze .
```

The repository path is a required positional argument. This parses all source
files, builds the knowledge graph in `<repo>/.gitnexus/lbug`, registers the
repository in `~/.gitnexus/registry.json` (`GITNEXUS_HOME` overrides the
location), writes the gitnexus-rs block into `AGENTS.md` / `CLAUDE.md`, and
installs the standard gitnexus-rs skills into `.claude/skills/` (mirrored to
`.agents/skills/` when that directory exists).

| Flag | Effect |
| --- | --- |
| `--force` | Full re-index even when the repository is up to date |
| `--embeddings auto\|off\|only` | `auto` (default): index, then embed; `off`: index only; `only`: embed an existing index without re-indexing |
| `--skip-agents-md` | Do not write the block into `AGENTS.md` / `CLAUDE.md` |
| `--skip-skills` | Do not install the standard skills into `.claude/skills/` |
| `--no-stats` | Omit the volatile symbol/relationship counts from the block (stable diffs) |
| `--name <alias>` | Register the repository under a custom name instead of its directory name |
| `--out <dir>` | Write the database somewhere other than `<repo>/.gitnexus` |

**When to run:** first time in a project, after large changes, or when
`graph_status` reports `is_stale: true` and the change set is beyond what MCP
`reanalyze` handles (50 files). A re-run on unchanged code rewrites nothing: the
block and the skills are only written when their content differs.

A skill file you edited by hand is kept: `analyze` warns and leaves it alone.
Delete it to receive the current version again.

### ai-context — Refresh agent files without analyzing

```bash
gitnexus-rs ai-context --repo . --write
```

Writes the gitnexus-rs block into `AGENTS.md` / `CLAUDE.md` and installs the
standard skills - the same step `analyze` ends with, without touching the
index. Run it after upgrading gitnexus-rs. Without `--write` it prints the
block (`--format json` adds the counts). `--skip-skills`, `--no-stats` and
`--name` work as for `analyze`.

### watch — Keep the index current

```bash
gitnexus-rs watch --repo .
```

Blocks and re-indexes changed files incrementally. `--debounce-ms <ms>` sets
the quiet period after the last change; `--max-debounce-resets <n>` forces a
re-index after that many timer restarts in one burst (editor save storms).
Watch updates only the graph — run a one-shot `analyze` when the `AGENTS.md`
block or the skills need refreshing.

### status — Check the index

```bash
gitnexus-rs status --repo .
```

Summary of the repository's graph: metadata and per-label node counts.
`--format human|json|yaml`. Freshness against git is the MCP `graph_status`
tool.

### detect-changes — What do my changes affect

```bash
gitnexus-rs detect-changes --repo . --scope all
gitnexus-rs detect-changes --repo . --scope compare --base-ref main --fail-on high
```

`--scope unstaged|staged|all|compare` (default `unstaged`), `--base-ref` for
`compare`. `--fail-on low|medium|high|critical` exits 1 when the risk reaches
that level; without it the command only reports.

### hook — Pre-commit gate

```bash
gitnexus-rs hook install . --fail-on high
gitnexus-rs hook uninstall .
```

Writes (or removes) `.git/hooks/pre-commit` that runs `detect-changes` and
blocks commits at or above `--fail-on` (default `high`). Uninstall only
removes a hook this tool installed.

### tool — Call an MCP tool from the shell

```bash
gitnexus-rs tool --name impact --args '{"target": "validateUser", "direction": "upstream"}'
```

The same tools the MCP server exposes, for scripts or when the MCP connection
is down.

### clean — Delete the index

```bash
gitnexus-rs clean --repo .            # dry run: prints what would be deleted
gitnexus-rs clean --repo . --force    # deletes
```

Deletes `<repo>/.gitnexus/`. `--all` cleans every registered repository.
`--yes` is an alias for `--force`. Without either flag nothing is deleted.

### wiki — Generate documentation from the graph

```bash
gitnexus-rs wiki --repo .
```

One page per functional area (`Community`): label, cohesion, members and
outgoing calls to other areas, plus an index page. Deterministic and offline —
no LLM, no API key. `--output <dir>` (default `<repo>/.gitnexus/wiki`),
`--format markdown|html`.

### list — Enumerate graph nodes

```bash
gitnexus-rs list --repo . --label Function --file src/auth
```

Lists nodes filtered by `--label`, `--file` or `--package` (`--limit`,
`--format`). To list indexed repositories use the MCP `list_repos` tool.

### doctor — Diagnose the install

```bash
gitnexus-rs doctor --repo .
```

Providers, runtime, database schema, environment.

## After Indexing

1. `list_repos {}` and `graph_status({repo})` to verify the index loaded
2. Use the other gitnexus-rs skills (`exploring`, `impact-analysis`,
   `debugging`, `refactoring`) for your task

## Troubleshooting

- **`graph_status` still stale after `analyze`**: check that you analyzed the
  checkout the repository is registered at (`list_repos` shows its path)
- **`reanalyze` refuses a large change set**: it is capped at 50 files — run
  `gitnexus-rs analyze .`
- **Embeddings slow or unwanted**: `--embeddings off`

<!-- gitnexus-rs:skill fnv1a64=a4a48f1bb4e07807 - generated by `gitnexus-rs analyze`; edits are kept, delete the file to restore it -->
