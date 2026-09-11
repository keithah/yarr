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
