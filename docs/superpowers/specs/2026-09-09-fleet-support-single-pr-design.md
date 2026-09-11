# Fleet Support Single-PR Design

**Status:** approved for planning

## Goal

Deliver a single upstream PR that makes yarr safe and practical for a 20-instance media fleet while preserving one Code Mode MCP tool and the existing thin CLI/MCP shims. It includes the current Code Mode boundary repairs, fleet policy/configuration, Plex discovery and Plex/Tautulli pairing, fleet dispatch, observability, and canonical snippets.

## Scope and constraints

- Preserve Rust 2024, MSRV Rust 1.97.1, and `rmcp = "=3.0.0-beta.2"`.
- Keep `src/mcp/tools.rs` and `src/cli.rs` as parse-and-delegate shims only.
- Keep environment configuration authoritative for every credential value. No config file, discovery file, test, metric, or log may retain a credential or authenticated URL.
- Preserve the one `yarr` Code Mode tool. Fleet operations must not expand MCP schema cost with one tool per instance.
- All offline behavior is testable without provider credentials. Live discovery and pairing acceptance are supervised, read-only, opt-in operations.
- Every generated mutation must be classified. Unclassified generated writes fail CI and do not execute.

## Code Mode containment

The default Code Mode deadline becomes 120 seconds. A single execution deadline is carried through the JS engine, the bridge, native action dispatch, and upstream HTTP work. Cancellation stops the script and cancels outstanding operation futures.

Filesystem side effects are part of the write classification. A command that creates, downloads, or updates a local file cannot be advertised or authorized as read-only. The implementation must either classify and gate it as a local mutation or remove the side effect from the command.

The documentation distinguishes QuickJS-enforced resource limits from host-level isolation. The runtime may claim only limits directly enforced by rquickjs; it must not claim a hard process-wide memory sandbox. If an operation requires a stronger guarantee, it must use a process-isolated execution path or carry an explicit unsupported limitation.

The public JavaScript namespace remains the normalized configured name, while backend dispatch uses the original configured identity. Callable, action, and type discovery must round-trip through the same namespace. All config sources reject namespace collisions before dispatch.

## Generated-operation authority

`src/openapi/safety.rs` is the authority for generated-operation safety. It exposes `OperationSafety`, `operation_safety(kind, operation)`, `classify_operation(kind, spec)`, and `validate_generated_write_classification()`.

HTTP DELETE is destructive by default. Audited non-DELETE mutations are classified explicitly, including Plex session termination and high-impact library mutations. The generated documentation report lists operation method, mutation class, and elicitation requirement. A new generated mutation without a decision is a build failure.

`YARR_FLEET_READONLY` rejects all mutations before any upstream dispatch. Destructive fleet calls allow at most three targets and require one MCP elicitation naming every sorted target. Refusal or unavailable elicitation leaves all targets untouched.

## Fleet configuration

`YARR_FLEET_FILE` adds YAML or TOML service entries to the existing environment configuration. Fleet files contain public connection metadata and references such as `token_env` or `api_key_env`; inline `token`, `api_key`, `username`, and `password` are invalid.

Environment-defined services replace file services with the same case-insensitive configured name. Other entries form a deterministic name-sorted union. Duplicates within one source, cross-source normalized environment-name collisions, reserved-global collisions, invalid credential-reference identifiers, and normalized Code Mode namespace collisions fail startup with source, entry, service name, and collision details.

## Plex discovery and pairing

`yarr discover plex` is a CLI-only scaffolding command. It uses a dedicated plex.tv client and standard Plex headers, validates the observed API shape, and selects only resources providing `server`. It defaults to owned servers and ranks connections local, direct HTTPS, then relay; relay-only choices are flagged.

Discovery generates deterministic `plex_<slug>` names with suffixes for collisions. It writes public fleet configuration separately from secret material. Secret output is atomic, mode `0600`, outside YAML/TOML, and never appears in logs, reports, or test fixtures. `--diff` reports drift without writing either output.

For each configured Tautulli service, discovery reads `pms_identifier` and pairs it with a Plex `clientIdentifier`. It records only unambiguous matches and reports unpaired or multiply matched identities. Pairing tests use redacted fixtures; live acceptance requires explicit supervision and read-only access.

## Fleet dispatch and observability

A host-owned dispatcher backs `fleet.of`, `fleet.all`, `fleet.map`, and `fleet.status`. JavaScript selects a target and invokes one operation per instance; it does not own network concurrency or authorization.

The host applies bounded concurrency, per-instance timeouts, stable sorted results, pre-dispatch aggregate destructive authorization, and partial-failure isolation. Every instance produces a distinguishable complete, failed, or truncated envelope. Oversized instance data becomes a valid `{ truncated: true, summary, value: null }` result; the final MCP token cap is not used to imply fleet completeness.

`fleet.status()` reports reachability, version, and latency. Upstream metrics and structured logs carry bounded-cardinality `service.name` and `service.kind` labels without credentials or response bodies.

Built-in read-only snippets `fleet_activity`, `fleet_health`, `fleet_library_sizes`, and `fleet_transcode_load` are discoverable but cannot be overwritten or deleted.

## Verification

The PR includes table-driven tests for operation classification, config merge and diagnostics, discovery parsing and `--diff`, pairing success/ambiguity, deadline propagation, local-write classification, namespace round trips, fan-out authorization/timeouts/failures/truncation, snippets, and metric labels. The acceptance gate is:

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test parity
python3 scripts/check-doc-links.py
```

Before PR creation, run Steward against the exact committed head. A blocked manifest, invalid reviewer artifact, stale state, or incomplete artifact is not review evidence and stops publication. Live Plex/Tautulli acceptance occurs only after offline gates pass and is reported separately from fixture coverage.

## Non-goals

- No provider discovery on normal server request paths.
- No automatic writes to upstream media services during discovery or pairing.
- No credentials in committed files, diagnostics, metrics, or artifacts.
- No claim that QuickJS input/output limits are process-level memory isolation.
