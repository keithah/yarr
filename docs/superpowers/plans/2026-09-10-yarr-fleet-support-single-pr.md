# Yarr Fleet Support Single-PR Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Deliver the approved fleet-support platform as one reviewable PR without broadening yarr’s one-tool MCP surface or leaking credentials.

**Architecture:** Add small application-layer modules for generated-operation authority, fleet configuration, discovery/pairing, and host-owned fleet execution. Preserve the existing Code Mode engine as a bounded selection surface; it passes one planned invocation per instance to host dispatch, which retains authorization, concurrency, deadline, truncation, and metrics ownership.

**Tech Stack:** Rust 2024; Rust 1.97.1; Tokio; reqwest 0.13; rquickjs; serde/serde_json; TOML; `serde_yaml_ng`; rmcp `=3.0.0-beta.2`; axum-prometheus.

**Spec:** `docs/superpowers/specs/2026-09-09-fleet-support-single-pr-design.md`

## Global Constraints

- Preserve Rust 2024, MSRV Rust 1.97.1, and `rmcp = "=3.0.0-beta.2"`.
- `src/mcp/tools.rs` and `src/cli.rs` remain parse-and-delegate shims with zero business logic.
- The server exposes one Code Mode `yarr` MCP tool; fleet support must not create one tool per instance.
- Environment values are authoritative for credentials; never commit, log, metric-label, fixture, or report a credential or authenticated URL.
- The only allowed normal runtime service configuration is existing environment config plus additive `YARR_FLEET_FILE` YAML/TOML metadata.
- Inline credentials in fleet files are invalid. Generated discovery secrets are atomically written at `0600` outside YAML/TOML.
- Runtime Plex discovery is CLI-only and explicit. It never runs on normal server request paths.
- A destructive fleet action is pre-authorized once for its complete, sorted target set, has a maximum of three targets, and fails closed on refusal or unavailable elicitation.
- Each fleet invocation returns one stable instance envelope that is complete, failed, or truncated; final MCP response truncation never represents fleet completion.
- Do not claim process-level memory isolation where only rquickjs heap/stack/input/output limits exist.
- Any blocked or stale Steward run, invalid reviewer output, or incomplete review artifact is not review evidence and blocks PR publication.

---

## File Structure

| Path | Responsibility |
|---|---|
| `src/openapi/safety.rs` | Generated operation safety classification and coverage validation. |
| `src/openapi/safety_tests.rs` | Table-driven generated mutation classification tests. |
| `src/fleet.rs` | Fleet selection, execution contracts, bounded result envelopes, and canonical snippet metadata. |
| `src/fleet/dispatch.rs` | Host-owned bounded-concurrency invocation executor. |
| `src/fleet/discovery.rs` | CLI-only Plex resource parser, connection ranking, atomic output, and drift comparison. |
| `src/fleet/pairing.rs` | Tautulli-to-Plex identifier pairing model. |
| `src/fleet_tests.rs`, `src/fleet/*_tests.rs` | Offline unit coverage for execution, discovery, pairing, truncation, and snippets. |
| `src/config/fleet_file.rs` | YAML/TOML fleet-file parsing, environment-reference resolution, source diagnostics, and deterministic merge. |
| `src/config/services.rs` | Existing env loader integration and namespace/identity validation. |
| `src/config.rs` | `YarrConfig` policy helpers and `YARR_FLEET_FILE` load orchestration. |
| `src/app/fleet.rs` | `YarrService` fleet status and invocation bridge. |
| `src/app/codemode.rs`, `src/app/codemode_runtime.rs` | Absolute deadline propagation and fleet Code Mode bridge. |
| `src/codemode.rs`, `src/codemode/proxy.rs`, `src/codemode/catalog.rs` | 120-second default, accurately scoped limit documentation, `fleet` JavaScript facade and discovery. |
| `src/mcp/elicit.rs`, `src/mcp/rmcp_server.rs` | Aggregate destructive authorization and read-only enforcement. |
| `src/cli/{command,router,usage}.rs`, `src/main.rs` | `discover plex` CLI-only command routing. |
| `src/yarr.rs`, `src/yarr/helpers.rs` | Deadline-aware HTTP request API and service-name/kind telemetry labels. |
| `xtask/src/tool_docs.rs` | Generated-write classification report and check failure for unclassified writes. |
| `docs/CONFIG.md`, `docs/TOOLS_ACTIONS_ENDPOINTS.md`, `README.md`, `CHANGELOG.md` | Supported configuration, limitations, discovery contract, and release notes. |

### Task 1: Stabilize Code Mode boundaries

**Files:**
- Modify: `src/codemode.rs`, `src/codemode/engine.rs`, `src/app/codemode.rs`, `src/app/codemode_runtime.rs`, `src/yarr.rs`, `src/yarr/helpers.rs`
- Modify: `src/codemode/engine_tests.rs`, `src/app/codemode_runtime_tests.rs`, `src/yarr_tests.rs`, `src/yarr/helpers_tests.rs`, `docs/CONFIG.md`, `README.md`

**Interfaces:**
- Produces `CODEMODE_TIMEOUT: Duration = Duration::from_secs(120)`.
- Produces `RequestDeadline { instant: tokio::time::Instant }` passed from `YarrService::run_script` to every Code Mode-originated service invocation.
- Produces `YarrClient::{get_json_until,post_json_until,put_json_until,delete_json_until}` that applies `tokio::time::timeout_at` before request completion.

- [ ] **Step 1: Write deadline and limit-contract failures**

Add tests showing a stalled native Code Mode call returns `"codemode absolute deadline exceeded"`, a 120-second configured default is used, and docs no longer state that the rquickjs heap cap is process isolation.

- [ ] **Step 2: Run focused tests and verify RED**

Run: `cargo test codemode::engine::tests::stalled_calltool_is_bounded_by_deadline app::codemode::tests::runtime --lib`

Expected: deadline propagation or default-duration assertions fail before implementation.

- [ ] **Step 3: Implement one deadline source**

Set `CODEMODE_TIMEOUT` to 120 seconds. Carry one `tokio::time::Instant` from `run_script` through `ToolRequest` to `codemode_dispatch`, action execution, and deadline-aware `YarrClient` helpers. Preserve the JS interrupt handler as a second layer; do not create independent per-hop deadlines.

- [ ] **Step 4: Classify local side effects honestly**

Add explicit effect metadata to curated command descriptors. Mark every command that writes an artifact/download/cache/local file as `mutates: true`, route it through write scope and existing authorization, or remove its local file side effect. Add a regression that asserts a supposedly read-only command cannot create a file without write authorization.

- [ ] **Step 5: Run focused GREEN tests**

Run: `cargo test codemode:: app::codemode:: yarr:: --lib`

Expected: all Code Mode deadline, native request, and local-write tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/codemode.rs src/codemode/ src/app/codemode.rs src/app/codemode_runtime.rs src/yarr.rs src/yarr/ docs/CONFIG.md README.md
git commit -m "fix: propagate Code Mode execution deadlines"
```

### Task 2: Make generated-operation safety authoritative

**Files:**
- Create: `src/openapi/safety.rs`, `src/openapi/safety_tests.rs`
- Modify: `src/openapi.rs`, `src/app/openapi_ops.rs`, `src/app/codemode_dispatch.rs`, `src/mcp/rmcp_server.rs`, `src/lib.rs`
- Modify: `xtask/src/tool_docs.rs`, `xtask/src/main.rs`, `docs/TOOLS_ACTIONS_ENDPOINTS.md`

**Interfaces:**
- Produces `OperationSafety { mutates: bool, destructive: bool, elicitation_required: bool }`.
- Produces `operation_safety(kind: ServiceKind, operation: &str) -> Option<OperationSafety>`.
- Produces `classify_operation(kind: ServiceKind, spec: &OperationSpec) -> Result<OperationSafety, String>`.
- Produces `validate_generated_write_classification() -> Result<(), String>`.

- [ ] **Step 1: Write table-driven RED tests**

Cover GET read-only, DELETE destructive, an audited non-destructive POST, Plex session termination destructive, and an unknown generated POST returning an actionable unclassified-write error containing kind and operation.

- [ ] **Step 2: Run RED tests**

Run: `cargo test openapi::safety --lib`

Expected: compile failure because the safety module and APIs do not exist.

- [ ] **Step 3: Implement classification table and validator**

Default DELETE to destructive. Require explicit table rows for every generated POST/PUT/PATCH. Seed audited rows for Plex, Sonarr, Radarr, Overseerr, and Jellyfin mutations. Make generated execution consult the classifier before upstream dispatch; do not duplicate method inference in Code Mode or MCP.

- [ ] **Step 4: Generate documentation from authority**

Extend `cargo xtask tool-docs` to render each generated write’s HTTP method, mutation class, and elicitation requirement, and fail `--check` if `validate_generated_write_classification()` fails.

- [ ] **Step 5: Verify GREEN**

Run: `cargo test openapi::safety --lib && cargo xtask tool-docs --check`

Expected: all rows classified and generated docs unchanged or regenerated from the table.

- [ ] **Step 6: Commit**

```bash
git add src/openapi src/app/openapi_ops.rs src/app/codemode_dispatch.rs src/mcp/rmcp_server.rs src/lib.rs xtask docs/TOOLS_ACTIONS_ENDPOINTS.md
git commit -m "feat: classify generated operation safety"
```

### Task 3: Add fleet policy and aggregate destructive authorization

**Files:**
- Modify: `src/config/mcp.rs`, `src/config.rs`, `src/mcp/elicit.rs`, `src/mcp/elicit_tests.rs`, `src/mcp/rmcp_server.rs`, `src/app/codemode.rs`
- Modify: `src/config_tests.rs`, `src/mcp/rmcp_server_errors_tests.rs`, `docs/CONFIG.md`

**Interfaces:**
- Produces `McpConfig { destructive_fanout_max: usize }` with default `3`.
- Produces `YarrConfig::is_readonly(&self, service: &str) -> bool` backed by `YARR_FLEET_READONLY`.
- Replaces `gate_destructive(peer, action, service)` with `gate_destructive(peer, action, services: &[String])`.

- [ ] **Step 1: Write failing policy tests**

Assert `YARR_FLEET_READONLY=true` rejects a generated POST before transport; four destructive targets are rejected without eliciting; a three-target action emits one message containing the sorted names; declined confirmation dispatches no target.

- [ ] **Step 2: Run RED tests**

Run: `cargo test mcp::elicit config::tests --lib`

Expected: old single-service prompt and absent policy fields fail new assertions.

- [ ] **Step 3: Implement pre-dispatch policy**

Parse and validate the read-only flag and fanout maximum. Apply read-only and fanout checks before all generated, curated, generic, and Code Mode-originated mutation execution. Make the MCP guard build its entire target set first, sort/deduplicate it, then elicit exactly once.

- [ ] **Step 4: Verify GREEN**

Run: `cargo test mcp::elicit mcp::rmcp_server config:: --lib`

Expected: one aggregate authorization and zero dispatch after a decline.

- [ ] **Step 5: Commit**

```bash
git add src/config src/mcp src/app/codemode.rs docs/CONFIG.md
git commit -m "feat: enforce fleet mutation authority"
```

### Task 4: Load and validate fleet-file configuration

**Files:**
- Add dependency: `Cargo.toml` — `serde_yaml_ng`
- Create: `src/config/fleet_file.rs`, `src/config/fleet_file_tests.rs`
- Modify: `src/config.rs`, `src/config/services.rs`, `src/config/services_tests.rs`, `src/lib.rs`, `docs/CONFIG.md`, `docs/ENV.md`

**Interfaces:**
- Produces `load_fleet_file(path: &Path) -> Result<Vec<ServiceConfig>>`.
- Produces `merge_service_sources(file: Vec<ServiceConfig>, env: Vec<ServiceConfig>) -> Result<Vec<ServiceConfig>>`.
- Produces `validate_env_reference(name: &str) -> Result<(), String>`.

- [ ] **Step 1: Write fixture-driven RED tests**

Create temporary YAML and TOML fixtures covering public metadata, `api_key_env`, environment replacement by case-insensitive name, deterministic name sort, duplicate-source names, normalized namespace collision, reserved global collision, invalid env reference, and inline `token` rejection.

- [ ] **Step 2: Run RED tests**

Run: `cargo test config::fleet_file --lib`

Expected: module/import errors until parser and merge are implemented.

- [ ] **Step 3: Implement strict parser and merge**

Use `#[serde(deny_unknown_fields)]` fleet entries with only name, kind, base URL, and credential-reference fields. Resolve referenced values only from the installed environment overlay. Load `YARR_FLEET_FILE` after overlay installation, merge environment over file, validate all final configured names, and produce errors naming source and service.

- [ ] **Step 4: Verify GREEN**

Run: `cargo test config:: --lib`

Expected: all config loading and source-merge cases pass without secret-bearing fixtures.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock src/config src/lib.rs docs/CONFIG.md docs/ENV.md
git commit -m "feat: load fleet service configuration files"
```

### Task 5: Implement Plex discovery and Tautulli pairing

**Files:**
- Create: `src/fleet/discovery.rs`, `src/fleet/discovery_tests.rs`, `src/fleet/pairing.rs`, `src/fleet/pairing_tests.rs`
- Modify: `src/cli/command.rs`, `src/cli/router.rs`, `src/cli/usage.rs`, `src/main.rs`, `src/lib.rs`, `docs/CONFIG.md`, `README.md`

**Interfaces:**
- Produces `discover_plex(options: PlexDiscoveryOptions) -> Result<PlexDiscoveryReport>`.
- Produces `select_connection(resource: &PlexResource) -> Option<SelectedConnection>`.
- Produces `pair_tautulli_to_plex(tautulli: &[TautulliIdentity], plex: &[PlexIdentity]) -> PairingReport`.

- [ ] **Step 1: Write parser, selection, output, and pairing RED tests**

Use a redacted `plex.tv` resource payload fixture. Assert non-server resources are ignored; local wins over direct HTTPS and relay; relay-only is flagged; owned-only filtering is default; `plex_<slug>` names suffix deterministically; `--diff` writes nothing; secret files are `0600`; exact identifier pairs succeed while absent and duplicate identifiers are reported unpaired/ambiguous.

- [ ] **Step 2: Run RED tests**

Run: `cargo test fleet::discovery fleet::pairing --lib`

Expected: missing modules and CLI command variants.

- [ ] **Step 3: Implement dedicated CLI-only discovery**

Add `Command::DiscoverPlex { token_env, fleet_file, secret_file, include_shared, diff }`, route it only from CLI, and invoke dedicated discovery transport with standard Plex headers. Do not add Plex discovery to server routes, MCP actions, or Code Mode. Atomically render public fleet data and secret env file separately; secret target is created with `0600`; `--diff` emits a report without mutations.

- [ ] **Step 4: Implement read-only pairing**

Fetch only required identifiers from configured Tautulli/Plex services with the Task 1 deadline-aware client, form matches by exact identifier, and return a typed report without persisting secrets or querying plex.tv during normal service operation.

- [ ] **Step 5: Verify GREEN**

Run: `cargo test fleet::discovery fleet::pairing cli:: --lib && cargo test --test cli_parse`

Expected: fixture parser, file mode, no-write diff, CLI routing, and pairing ambiguity tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/fleet src/cli src/main.rs src/lib.rs docs/CONFIG.md README.md
git commit -m "feat: discover and pair fleet Plex services"
```

### Task 6: Add host-owned fleet dispatch and Code Mode facade

**Files:**
- Create: `src/fleet.rs`, `src/fleet/dispatch.rs`, `src/fleet/dispatch_tests.rs`, `src/fleet_tests.rs`, `src/app/fleet.rs`, `src/app/fleet_tests.rs`
- Modify: `src/app.rs`, `src/app/codemode.rs`, `src/codemode/proxy.rs`, `src/codemode/catalog.rs`, `src/codemode/proxy_tests.rs`, `src/codemode/catalog_tests.rs`, `src/lib.rs`

**Interfaces:**
- Produces `FleetSelector::{Of, All}` and `FleetInvocation { action, params, services }`.
- Produces `FleetResult { service, kind, ok, elapsed_ms, truncated, value, error }`.
- Produces `YarrService::{fleet_status, dispatch_fleet}`.

- [ ] **Step 1: Write dispatcher RED tests**

Use a 20-instance mixed-kind fixture. Assert stable name ordering, bounded concurrency, per-instance timeout isolation, partial failures, one aggregate authorization before dispatch, and oversized instance values converted to `{ "truncated": true, "summary": { "type": ..., "item_count": ..., "observed_bytes": ... }, "value": null }`.

- [ ] **Step 2: Run RED tests**

Run: `cargo test fleet::dispatch app::fleet --lib`

Expected: missing host dispatcher APIs and result envelopes.

- [ ] **Step 3: Implement host dispatcher**

Build selection and call plans before launching work. Perform policy/authorization once, run per-instance futures through a semaphore, deadline, and stable result collector, then bound each value before response serialization. Never use the outer token cap to label a fleet result complete.

- [ ] **Step 4: Expose minimal JavaScript surface**

Extend the generated preamble with `fleet.of(name)`, `fleet.all(kind?)`, `fleet.map(selector, action, params)`, and `fleet.status()`. Keep the callback limited to one service invocation per selected instance and pass raw configured identity to backend dispatch.

- [ ] **Step 5: Verify GREEN**

Run: `cargo test fleet:: app::fleet codemode:: --lib`

Expected: all 20-instance, timeout, authorization, and discovery-to-execution tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/fleet.rs src/fleet src/app.rs src/app/fleet.rs src/app/fleet_tests.rs src/app/codemode.rs src/codemode src/lib.rs
git commit -m "feat: dispatch bounded fleet Code Mode operations"
```

### Task 7: Add status metrics and canonical fleet snippets

**Files:**
- Create: `src/fleet/snippets.rs`, `src/fleet/snippets_tests.rs`
- Modify: `src/fleet.rs`, `src/app/fleet.rs`, `src/codemode/store.rs`, `src/app/codemode_snippets.rs`, `src/yarr.rs`, `src/server/routes.rs`, `src/server/routes_tests/metrics_tests.rs`, `docs/TOOLS_ACTIONS_ENDPOINTS.md`, `CHANGELOG.md`

**Interfaces:**
- Produces built-ins `fleet_activity`, `fleet_health`, `fleet_library_sizes`, and `fleet_transcode_load`.
- Produces `fleet.status()` values with service name, kind, reachability, version, and latency.

- [ ] **Step 1: Write observability and snippet RED tests**

Assert status output includes exactly one record per configured service; latency is numeric; labels contain configured service name and kind but no base URL/token; built-in snippets list and run but saving/deleting one returns a protected-snippet error.

- [ ] **Step 2: Run RED tests**

Run: `cargo test fleet::snippets server::routes::tests::metrics app::codemode::tests::snippets --lib`

Expected: absent built-ins and fleet labels fail assertions.

- [ ] **Step 3: Implement status, telemetry, and immutable built-ins**

Instrument each upstream request with bounded-cardinality service name/kind labels and elapsed time. Make status calls use bounded per-instance deadlines. Merge built-ins into snippet discovery while rejecting user overwrite/delete and retaining current user-snippet atomic store behavior.

- [ ] **Step 4: Verify GREEN**

Run: `cargo test fleet::snippets server::routes::tests::metrics app::codemode::tests::snippets --lib`

Expected: labels, status envelopes, and protected snippets pass without secrets.

- [ ] **Step 5: Commit**

```bash
git add src/fleet src/app/fleet.rs src/codemode/store.rs src/app/codemode_snippets.rs src/yarr.rs src/server docs/TOOLS_ACTIONS_ENDPOINTS.md CHANGELOG.md
git commit -m "feat: expose fleet status and canonical snippets"
```

### Task 8: Regenerate documentation, run acceptance, and prepare PR evidence

**Files:**
- Modify: `README.md`, `docs/CONFIG.md`, `docs/ENV.md`, `docs/TOOLS_ACTIONS_ENDPOINTS.md`, `CHANGELOG.md`
- Create local-only: `/private/tmp/yarr-fleet-live-acceptance/` or another `0700` private path for redacted live output; do not add it to Git.

- [ ] **Step 1: Update maintained documentation from executable contracts**

Document `YARR_FLEET_FILE`, read-only/fanout policy, true Code Mode resource limits, CLI-only discovery, output modes, pairing ambiguity behavior, and fleet result envelopes. Mark live discovery/pairing as supervised read-only acceptance rather than fixture evidence.

- [ ] **Step 2: Regenerate owned references**

Run: `cargo xtask tool-docs`

Expected: generated write safety table and Code Mode/fleet references reflect the new authorities.

- [ ] **Step 3: Run complete offline gates**

Run:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test parity
python3 scripts/check-doc-links.py
cargo xtask tool-docs --check
```

Expected: every command exits zero.

- [ ] **Step 4: Run supervised live acceptance only with configured read-only credentials**

Run CLI discovery with `--diff` first and perform read-only Plex/Tautulli pairing. Verify output files are not modified by `--diff`, verify no logs contain secrets, and store only redacted summaries in the private acceptance directory. If no configured read-only account exists, record this as a live-acceptance blocker; do not invent provider evidence.

- [ ] **Step 5: Commit docs only after checks are current**

```bash
git add README.md docs/CONFIG.md docs/ENV.md docs/TOOLS_ACTIONS_ENDPOINTS.md CHANGELOG.md
git commit -m "docs: document fleet support operations"
```

- [ ] **Step 6: Run exact-head Steward and prepare publication**

Confirm clean committed checkout, run `steward_review.py` followed by `steward_llm_review.py` against the exact final head, read back artifact role/provider/model/SHA/mode, and stop if the manifest is blocked, artifacts are incomplete, or reviewer output is invalid. Only after current exact-SHA evidence and explicit publication approval may push/create the one large PR.
