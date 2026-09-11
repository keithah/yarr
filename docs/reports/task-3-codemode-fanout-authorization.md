---
title: "Task 3 Code Mode destructive fan-out authorization"
status: "verified"
date: 2026-09-11
---

# Task 3 Code Mode destructive fan-out authorization

## Root cause

The Code Mode guard passed only the current inner action's service to
`destructive_targets`. A script could therefore dispatch destructive actions to
multiple services while each authorization saw a one-service target list, which
bypassed the configured fleet fan-out bound.

## RED evidence

Command:

```sh
cargo test mcp::tools::tests::codemode_destructive_targets_cover_the_entire_configured_fleet_before_eliciting --lib
```

Exit status: `101`

```text
error[E0425]: cannot find function `codemode_destructive_targets` in module `super`
  --> src/mcp/tools_tests.rs:96:16
...
error[E0425]: cannot find function `codemode_destructive_targets` in module `super`
   --> src/mcp/tools_tests.rs:101:24
```

## GREEN evidence

Command:

```sh
cargo test mcp::tools::tests::codemode_destructive_targets_cover_the_entire_configured_fleet_before_eliciting --lib
```

Exit status: `0`

```text
running 1 test
test mcp::tools::tests::codemode_destructive_targets_cover_the_entire_configured_fleet_before_eliciting ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 619 filtered out
```

The complete focused verification command also exited `0`:

```sh
cargo fmt --check && cargo test mcp::elicit --lib && cargo test mcp::rmcp_server --lib && cargo test mcp::tools --lib && cargo test config:: --lib
```

Its test totals were 7 elicitation, 24 RMCP server, 6 MCP tools, and 43 configuration tests: all passed.

## Follow-up repair: actual script targets

The initial repair was too broad: it aggregated every configured service, not the
services reached by destructive calls in the submitted script. Code Mode now runs a
non-dispatching QuickJS preflight, records actual bridge calls, parses them with the
normal `YarrAction` parser, sorts/deduplicates destructive service targets, and applies
the fan-out cap before one MCP elicitation. The real execution still reauthorizes every
inner call and rejects a destructive call that was not preflight-authorized.

RED evidence:

```sh
cargo test mcp::tools::tests::codemode_preflight --lib
# exit 101: codemode_script_destructive_targets was absent
```

GREEN evidence:

```sh
cargo fmt --check
cargo test mcp::tools::tests::codemode_preflight --lib
cargo test --lib
cargo clippy --all-targets -- -D warnings
```

The focused regression uses real Code Mode scripts: a four-target script is rejected
at a cap of three without host dispatch, while a one-target script on the same
four-service fleet produces only `["sonarr"]` for authorization.

## Follow-up repair: saved snippets and bounded preflight admission

The preflight previously ran in `execute_tool` before `YarrService::run_script`.
That bypassed Code Mode's source-size check and semaphore admission, and the top-level
`codemode.run(...)` plan saw only `snippet_run`, not the saved source it executes.

### RED evidence

```sh
cargo test guarded_codemode_rejects_oversize_code_before_preflight --lib
cargo test guarded_saved_snippet_preflights_its_loaded_source_before_execution --lib
```

Both initially exited `101` with `E0407`: `preflight` was not a member of
`CodeModeCallGuard`. The tests therefore established the missing guarded execution
boundary before production changes.

### GREEN evidence

```sh
cargo fmt --check
cargo test mcp::tools --lib
cargo test app::codemode --lib
cargo clippy --all-targets -- -D warnings
```

All commands exited `0`. The focused suites reported 8 MCP-tools tests and 28 Code
Mode tests passing. The added coverage proves an oversized guarded source never enters
preflight, a preflight retains the sole configured admission permit until it completes,
a guarded saved snippet preflights its loaded source, and `codemode.run(...)` expands a
saved destructive snippet to its actual `sonarr` target before authorization.

## Follow-up repair: data-dependent planning and nonblocking QuickJS

Null-only planning could not take a branch selected by an upstream read. Guarded
planning now runs on the existing Code Mode blocking boundary, sends each
non-destructive call through the normal guarded dispatcher, and returns its real
JSON to QuickJS. Destructive calls are parsed, scope/readonly-checked, collected
as exact targets, and return `null` without transport. The sorted/deduplicated
set is capped and confirmed once before the actual script runs; runtime target
enforcement remains in place. Planning uses the same admission permit and absolute
deadline as execution, including deadline-aware read dispatch.

### RED evidence

```sh
cargo test --lib app::codemode::tests::guarded_codemode_plans_data_dependent_delete_after_real_read -- --nocapture
# exit 101: CodeModeCallGuard lacked the planning target/authorization hooks
```

### GREEN evidence

```sh
cargo fmt --check
cargo test --lib app::codemode -- --nocapture
cargo test --lib mcp::tools -- --nocapture
cargo test --lib
```

All commands exited `0` on 2026-09-11. The focused integration regression uses a
controlled local HTTP service and `const x = await api.sonarr.get(...)`; its
nonempty response selects `api.sonarr.delete(...)`. It observes `GET, GET,
DELETE`: the first GET is planning, no DELETE reaches transport before the target
is recorded/authorized, and the second GET plus DELETE is actual execution. The
single-worker regression proves an infinite-loop planning run yields Tokio while
QuickJS runs on `spawn_blocking`; the run itself still terminates at the absolute
deadline. The full library suite reported 627 passing tests.
