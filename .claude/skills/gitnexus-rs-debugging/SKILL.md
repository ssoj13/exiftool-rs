---
name: gitnexus-rs-debugging
description: "Use when the user is debugging a bug, tracing an error, or asking why something fails, with gitnexus-rs. Examples: \"Why is X failing?\", \"Where does this error come from?\", \"Trace this bug\""
---

# Debugging with gitnexus-rs

## When to Use

- "Why is this function failing?"
- "Trace where this error comes from"
- "Who calls this method?"
- "This endpoint returns 500"
- Investigating bugs, errors, or unexpected behavior

## Bind the repository first

A root cause traced in the wrong repository is a wrong root cause.

Call `list_repos {}` before the first tool call. With one indexed repository,
use the examples below as written. With more than one, pass `repo` on every
call. If you cannot tell which repository is meant, stop and ask. This matters
most for `cypher`, whose statement carries no in-band hint of which database it
ran against.

A stale index describes the code from before your bug, so check `graph_status`
and refresh before trusting a trace, and state the repository and index
freshness with the diagnosis.

## Workflow

```
0. list_repos {} + graph_status({repo})            → Bind repo, check freshness
1. query({query: "<error or symptom>"})            → Find related execution flows
2. context({name: "<suspect>"})                    → See callers/callees/processes
3. READ gitnexus://repo/{name}/process/{processName} → Trace the execution flow
4. cypher({query: "MATCH path..."})                → Custom traces if needed
```

> Stale (`is_stale: true`) → `reanalyze` (MCP, up to 50 files),
> `gitnexus-rs watch`, or `gitnexus-rs analyze <repo>`.
> `worktree_has_unstaged: true` → the graph does not see your local edits yet:
> `reanalyze({scope: "unstaged"})`.

## Checklist

```
- [ ] list_repos {} — bind repo; explicit repo when >1 indexed, ask if ambiguous
- [ ] graph_status — index fresh
- [ ] Understand the symptom (error message, unexpected behavior)
- [ ] query for error text or related code
- [ ] Identify the suspect function from returned processes
- [ ] context to see callers and callees
- [ ] Trace execution flow via process resource if applicable
- [ ] cypher for custom call chain traces if needed
- [ ] Read source files to confirm root cause
- [ ] State the repository and index freshness with the diagnosis
```

## Debugging Patterns

| Symptom | gitnexus-rs Approach |
| --- | --- |
| Error message | `query` for error text → `context` on throw/`Err` sites |
| Wrong return value | `context` on the function → trace callees for data flow |
| Intermittent failure | `context` → look for external calls, async deps, locks |
| Performance issue | `context` → find symbols with many callers (hot paths) |
| Recent regression | `detect_changes({scope: "compare", base_ref: "main"})` — what changed since the last good ref |
| "How does A reach B?" | `cypher` with a variable-length CALLS path between the two symbols |

## Tools

**query** — find code related to the error:

```
query({query: "payment validation error", repo: "my-app"})
→ Processes: CheckoutFlow, ErrorHandling
→ Symbols: validatePayment, handlePaymentError, PaymentError
```

**context** — full context for a suspect:

```
context({name: "validatePayment", repo: "my-app"})
→ Incoming calls: processCheckout, webhookHandler
→ Outgoing calls: verifyCard, fetchRates (external API!)
→ Processes: CheckoutFlow (step 3/7)
```

**cypher** — custom call chain traces (read-only). Pass `repo` alongside the
query; the Cypher text itself names no repository. Read
`gitnexus://repo/{name}/schema` first for the exact labels and relation types.

```cypher
MATCH path = (a)-[:CodeRelation {type: 'CALLS'}*1..2]->(b:Function {name: "validatePayment"})
RETURN [n IN nodes(path) | n.name] AS chain
```

"How does A reach B?" — the shortest chain is the first row:

```cypher
MATCH path = (a {name: "processCheckout"})-[:CodeRelation {type: 'CALLS'}*1..6]->(b {name: "fetchRates"})
RETURN [n IN nodes(path) | n.name] AS chain, length(path) AS hops
ORDER BY hops LIMIT 1
```

No row means no CALLS chain the index can resolve within 6 hops — the break is
often dynamic dispatch, a trait object, a callback or an external boundary;
walk it by hand with `context` from each end.

## Example: "Payment endpoint returns 500 intermittently"

```
0. list_repos {}
   → two repos (my-app, billing-api) — bind my-app explicitly on every call
   graph_status({repo: "my-app"}) → fresh

1. query({query: "payment error handling", repo: "my-app"})
   → Processes: CheckoutFlow, ErrorHandling
   → Symbols: validatePayment, handlePaymentError

2. context({name: "validatePayment", repo: "my-app"})
   → Outgoing calls: verifyCard, fetchRates (external API!)

3. READ gitnexus://repo/my-app/process/CheckoutFlow
   → Step 3: validatePayment → calls fetchRates (external)

4. Root cause: fetchRates calls the external API without a timeout
   Repository: my-app  Index: fresh
```

With a single indexed repository, the `repo` argument drops out of every call
above.

<!-- gitnexus-rs:skill fnv1a64=bd6bfc6fc18724df - generated by `gitnexus-rs analyze`; edits are kept, delete the file to restore it -->
