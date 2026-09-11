//! MCP tool dispatch — thin shims only.

use std::sync::Arc;

use lab_auth::AuthContext;
use rmcp::{RoleServer, service::Peer};
use serde_json::{Map, Value};

use crate::actions::{YarrAction, execute_service_action, required_scope_for_action};
use crate::app::codemode::CodeModeCallGuard;
use crate::server::AppState;

use super::schemas::YARR_TOOL_NAME;

pub(super) async fn execute_tool(
    state: &AppState,
    name: &str,
    args: Value,
    peer: &Peer<RoleServer>,
    auth: Option<AuthContext>,
) -> anyhow::Result<Value> {
    let guarded_script = name == YARR_TOOL_NAME
        || args
            .get("action")
            .and_then(Value::as_str)
            .is_some_and(|action| matches!(action, "codemode" | "snippet_run"));
    if guarded_script {
        let guard = Arc::new(McpCodeModeGuard {
            state: state.clone(),
            peer: peer.clone(),
            auth,
            destructive_authorized_targets: std::sync::Mutex::new(std::collections::BTreeSet::new()),
        });
        if name == YARR_TOOL_NAME
            && let Some(code) = args.get("code").and_then(Value::as_str)
        {
            guard.authorize_script(code).await?;
        }
        return dispatch_script_with_guard(state, name, args, guard).await;
    }
    dispatch_tool(state, name, args).await
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub async fn execute_tool_without_peer_for_test(
    state: &AppState,
    name: &str,
    args: Value,
) -> anyhow::Result<Value> {
    dispatch_tool(state, name, args).await
}

/// Route a tool call. In `codemode` mode (default) the only tool `list_tools`
/// ever advertises is `yarr` (→ the `codemode` action) — but this function has
/// always accepted service-named calls too, since a `yarr` script's own
/// `callTool` dispatches through this same path internally, and it's also
/// exercised directly by the dispatch-layer test helper. In `flat`
/// [`crate::config::ToolMode`], `list_tools` advertises those service-named
/// tools for real, so this same branch becomes the live MCP surface instead of
/// an internal-only one.
async fn dispatch_tool(state: &AppState, name: &str, args: Value) -> anyhow::Result<Value> {
    if name == YARR_TOOL_NAME {
        return dispatch_yarr(state, args).await;
    }
    match state.service.kind_of(name)? {
        Some(_) => dispatch_service_tool(state, name, args).await,
        None => Err(anyhow::anyhow!("unknown tool: {name}")),
    }
}

/// The `yarr` tool's only param is `code`; it dispatches the `codemode` action.
async fn dispatch_yarr(state: &AppState, args: Value) -> anyhow::Result<Value> {
    let mut object = match args {
        Value::Object(map) => map,
        _ => Map::new(),
    };
    object.insert("action".to_owned(), Value::String("codemode".to_owned()));
    let action = YarrAction::from_mcp_args(&Value::Object(object))?;
    execute_service_action(&state.service, &action).await
}

async fn dispatch_script_with_guard(
    state: &AppState,
    tool_name: &str,
    args: Value,
    guard: Arc<dyn CodeModeCallGuard>,
) -> anyhow::Result<Value> {
    let mut object = match args {
        Value::Object(map) => map,
        _ => Map::new(),
    };
    if tool_name == YARR_TOOL_NAME {
        object.insert("action".to_owned(), Value::String("codemode".to_owned()));
    } else {
        object.insert("service".to_owned(), Value::String(tool_name.to_owned()));
    }
    let action = YarrAction::from_mcp_args(&Value::Object(object))?;
    match action {
        YarrAction::CodeMode { code } => state.service.codemode_with_guard(&code, guard).await,
        YarrAction::SnippetRun { name, input } => {
            state
                .service
                .snippet_run_with_guard(&name, &input, Some(guard))
                .await
        }
        _ => unreachable!("only Code Mode and snippet execution use the guarded script path"),
    }
}

struct McpCodeModeGuard {
    state: AppState,
    peer: Peer<RoleServer>,
    auth: Option<AuthContext>,
    /// Script-local set authorized by the preflight prompt. Runtime calls must
    /// still match this exact target set; a branch that changes after real reads
    /// therefore fails closed rather than inheriting fleet-wide authority.
    destructive_authorized_targets: std::sync::Mutex<std::collections::BTreeSet<String>>,
}

pub(crate) fn authorize_codemode_action_scopes(
    token_scopes: &[String],
    action: &YarrAction,
) -> Result<(), String> {
    if let Some(required) = required_scope_for_action(action.name())
        && !crate::actions::scopes_satisfy(token_scopes, required)
    {
        return Err(format!(
            "forbidden inner Code Mode action `{}`: requires scope {required}",
            action.name()
        ));
    }
    Ok(())
}

impl McpCodeModeGuard {
    async fn authorize_script(&self, code: &str) -> anyhow::Result<()> {
        let targets = codemode_script_destructive_targets(
            &self.state.service,
            code,
            self.state.config.destructive_fanout_max,
        )
        .map_err(anyhow::Error::msg)?;
        if targets.is_empty() {
            return Ok(());
        }
        if self.peer.supported_elicitation_modes().is_empty() {
            anyhow::bail!(
                "destructive Code Mode script requires an elicitation-capable MCP client; nothing changed"
            );
        }
        if super::elicit::gate_destructive(&self.peer, "codemode", &targets).await
            == super::elicit::DeleteGate::Declined
        {
            anyhow::bail!("destructive Code Mode script was not confirmed; nothing changed");
        }
        *self.destructive_authorized_targets.lock().map_err(|_| {
            anyhow::anyhow!("Code Mode destructive authorization state is unavailable")
        })? = targets.into_iter().collect();
        Ok(())
    }
}

impl CodeModeCallGuard for McpCodeModeGuard {
    fn authorize<'a>(
        &'a self,
        action: &'a YarrAction,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            if let Some(auth) = self.auth.as_ref() {
                authorize_codemode_action_scopes(&auth.scopes, action)?;
            }
            reject_fleet_readonly_mutation(&self.state, action)?;

            let (destructive, target) = destructive_inner_call(&self.state, action);
            if !destructive {
                return Ok(());
            }
            if !self
                .destructive_authorized_targets
                .lock()
                .map_err(|_| "Code Mode destructive authorization state is unavailable")?
                .contains(target)
            {
                return Err(format!(
                    "destructive inner Code Mode action `{}` targeting `{target}` was not authorized by the script preflight; nothing changed",
                    action.name()
                ));
            }
            Ok(())
        })
    }
}

/// Run a Code Mode script in a non-dispatching QuickJS preflight and derive its
/// destructive targets from the bridge calls it actually reaches. This preserves
/// the engine's JavaScript semantics instead of attempting to parse source text.
pub(crate) fn codemode_script_destructive_targets(
    service: &crate::app::YarrService,
    code: &str,
    fanout_max: usize,
) -> Result<Vec<String>, String> {
    let limits = crate::codemode::EngineLimits {
        memory_bytes: crate::codemode::CODEMODE_MEMORY_LIMIT,
        stack_bytes: crate::codemode::CODEMODE_STACK_LIMIT,
        deadline: std::time::Instant::now() + crate::codemode::CODEMODE_TIMEOUT,
    };
    let calls =
        crate::codemode::plan_tool_calls(code, &service.codemode_preamble(), &limits, None)?;
    let mut targets = Vec::new();
    for call in calls {
        let mut args: Map<String, Value> = serde_json::from_str(&call.params_json)
            .map_err(|error| format!("invalid params for `{}`: {error}", call.id))?;
        args.insert("action".to_owned(), Value::String(call.id));
        let action =
            YarrAction::from_mcp_args(&Value::Object(args)).map_err(|error| error.to_string())?;
        if destructive_script_action(service, &action)
            && let Some(service) = script_action_service(&action)
        {
            targets.push(service.to_owned());
        }
    }
    super::rmcp_server::destructive_targets(&targets, fanout_max)
}

fn destructive_script_action(service: &crate::app::YarrService, action: &YarrAction) -> bool {
    crate::actions::action_is_destructive(action.name())
        || matches!(action, YarrAction::Op { service: target, op, .. } if service
            .kind_of(target)
            .ok()
            .flatten()
            .and_then(|kind| crate::openapi::safety::operation_safety(kind, op))
            .is_some_and(|safety| safety.elicitation_required))
}

fn script_action_service(action: &YarrAction) -> Option<&str> {
    match action {
        YarrAction::ServiceStatus { service }
        | YarrAction::ApiGet { service, .. }
        | YarrAction::ApiPost { service, .. }
        | YarrAction::ApiPut { service, .. }
        | YarrAction::ApiDelete { service, .. }
        | YarrAction::Op { service, .. } => Some(service),
        YarrAction::Curated { params, .. } => params.get("service").and_then(Value::as_str),
        _ => None,
    }
}

fn destructive_inner_call<'a>(state: &AppState, action: &'a YarrAction) -> (bool, &'a str) {
    let service = match action {
        YarrAction::ServiceStatus { service }
        | YarrAction::ApiGet { service, .. }
        | YarrAction::ApiPost { service, .. }
        | YarrAction::ApiPut { service, .. }
        | YarrAction::ApiDelete { service, .. }
        | YarrAction::Op { service, .. } => service.as_str(),
        YarrAction::Curated { params, .. } => params
            .get("service")
            .and_then(Value::as_str)
            .unwrap_or(YARR_TOOL_NAME),
        _ => YARR_TOOL_NAME,
    };
    let generated_elicitation = match action {
        YarrAction::Op { service, op, .. } => state
            .service
            .kind_of(service)
            .ok()
            .flatten()
            .and_then(|kind| crate::openapi::safety::operation_safety(kind, op))
            .is_some_and(|safety| safety.elicitation_required),
        _ => false,
    };
    (
        crate::actions::action_is_destructive(action.name()) || generated_elicitation,
        service,
    )
}

async fn dispatch_service_tool(
    state: &AppState,
    service: &str,
    args: Value,
) -> anyhow::Result<Value> {
    // Thin shim: parse args and route EVERY action (including `help`) through the
    // shared service-layer dispatch. No special cases or business logic here.
    let args = inject_service(args, service);
    let action = YarrAction::from_mcp_args(&args)?;
    reject_fleet_readonly_mutation(state, &action).map_err(anyhow::Error::msg)?;
    execute_service_action(&state.service, &action).await
}

/// Fleet read-only policy is enforced on every MCP mutation before shared action
/// dispatch can create an upstream request. Generated operations consult their
/// authoritative safety classification so generated GETs remain available.
fn reject_fleet_readonly_mutation(state: &AppState, action: &YarrAction) -> Result<(), String> {
    if !state.config.fleet_readonly || !action_mutates(state, action) {
        return Ok(());
    }
    Err(format!(
        "YARR_FLEET_READONLY rejects mutating action `{}`; nothing changed",
        action.name()
    ))
}

fn action_mutates(state: &AppState, action: &YarrAction) -> bool {
    match action {
        YarrAction::Op { service, op, .. } => state
            .service
            .kind_of(service)
            .ok()
            .flatten()
            .and_then(|kind| crate::openapi::safety::operation_safety(kind, op))
            .map(|safety| safety.mutates)
            .unwrap_or(true),
        YarrAction::Curated { name, .. } => {
            crate::actions::curated_command(name).is_some_and(|command| command.mutates)
        }
        _ => crate::actions::action_spec(action.name()).is_some_and(|spec| spec.mutates),
    }
}

fn inject_service(args: Value, service: &str) -> Value {
    let mut object = match args {
        Value::Object(map) => map,
        _ => Map::new(),
    };
    object.insert("service".to_owned(), Value::String(service.to_owned()));
    Value::Object(object)
}

#[cfg(test)]
#[path = "tools_tests.rs"]
mod tests;
