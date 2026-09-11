# Task 6 report — dispatcher and aggregate-authorization regression repair

## Status

Completed and committed as a scoped test-coverage repair. The unrelated unstaged `packages/yarr-mcp/README.md` change remains untouched.

## RED/GREEN evidence

- **RED:** temporarily changed the host dispatcher from `buffer_unordered(FLEET_MAX_CONCURRENT)` to `buffer_unordered(1)`. The new controlled-server dispatcher test failed with `dispatcher must actually overlap leaf work` (exit 101).
- **GREEN:** restored the bounded dispatcher; `cargo test app::fleet::tests --lib` passed **7/7**.

## Added regression coverage

`src/app/fleet_tests.rs`, wired from `src/app/fleet.rs`, now proves:

- local controlled upstreams overlap work (`>1`) without exceeding `FLEET_MAX_CONCURRENT`, while completion inversion preserves configured-name plan order;
- per-leaf upstream failure and timeout produce distinct non-truncated null-value envelopes;
- oversized values become exactly the `summary { type, item_count, observed_bytes }` marker and complete values do not claim truncation;
- `fleet.map` reaches every selected host leaf and is not a public action or MCP-schema action;
- guarded preflight authorizes all sorted leaves, receives one sorted destructive target vector, and runtime authorizes every leaf;
- a destructive runtime leaf absent from preflight authorization fails closed before transport.

## Fresh verification

- `cargo fmt --check`
- `cargo test app::fleet::tests --lib` — 7 passed
- `cargo test fleet:: --lib` — 34 passed
- `cargo test codemode:: --lib` — 102 passed
- `cargo test mcp::tools:: --lib` — 8 passed
- `cargo test --test tool_dispatch` — 18 passed
- `cargo test --workspace --all-targets` — passed (697 library tests plus all target suites)
- `cargo clippy -p yarr --all-targets -- -D warnings`
- `cargo xtask tool-docs --check`
- `python3 scripts/check-doc-links.py`
- `python3 scripts/check-schema-docs.py --check`
- `git diff --check`

## Commit

`test: cover fleet dispatcher authorization boundaries` (this scoped repair commit)
