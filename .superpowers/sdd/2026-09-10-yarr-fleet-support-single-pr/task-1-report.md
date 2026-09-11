# Task 1 — Local-effect metadata enforcement repair

## Finding repaired

`LocalEffect` previously had only `None`, and action dispatch/scope authorization did not consume it. A future curated command could therefore create a yarr-local file while declaring read scope and `mutates: false`.

## Implementation

- Added `LocalEffect::WritesFile`, whose presence is authorization-relevant.
- Derived effective curated-command scope from local-effect metadata: any non-`None` effect requires `yarr:write`.
- Added dispatch-time descriptor validation: a local file writer must explicitly declare both `required_scope: yarr:write` and `mutates: true`. This applies to trusted CLI dispatch as well, preserving its no-elicitation semantics while refusing malformed metadata.
- Factored the MCP Code Mode inner scope decision into the shared `authorize_codemode_action_scopes` helper used by `McpCodeModeGuard`.
- Added a cfg(test)-only curated descriptor registration seam and a Code Mode regression. The regression invokes a test-only `WritesFile` descriptor with read authorization, verifies the Code Mode result is rejected, and verifies its target file was not created.

## Strict TDD evidence

1. Added the Code Mode regression with the test descriptor initially declaring `READ_SCOPE` and `mutates: false`.
2. RED observed with:
   ```text
   cargo test mcp::tools::tests::read_authorized_codemode_cannot_invoke_local_file_writer --lib -- --exact
   ```
   The test failed at `read authorization must reject the local writer`; the pre-repair path authorized and invoked the local-writing descriptor.
3. Implemented the smallest metadata-driven scope derivation and dispatch validation.
4. GREEN observed with the same focused test; it passed and the file-absence assertion held.

## Verification

- `cargo test mcp::tools::tests --lib` — 4 passed.
- `cargo test actions::registry::tests --lib` — 18 passed.
- `cargo fmt -- --check` — passed.
- `cargo clippy --all-targets -- -D warnings` — passed.
- `cargo check` — passed.
- `cargo test -- --test-threads=1` — 610 unit tests, 59 integration tests, and doctests passed.
- `git diff --check` — passed.

A default-parallel `cargo test` run had one failure in the pre-existing deadline timing test `app::codemode::tests::runtime::native_http_call_honors_the_codemode_absolute_deadline` (`preamble error: Error: interrupted`). Its isolated rerun passed, and the complete serialized suite passed. `just verify` could not be run because `just` is not installed on this host.

## Scope

Modified only the curated-command metadata/dispatch boundary, MCP Code Mode authorization plumbing, their tests, and this task report. No network, live credentials, push, or PR actions were performed.
