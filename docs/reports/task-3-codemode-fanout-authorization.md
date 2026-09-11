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
