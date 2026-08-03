//! Dispatch and semantic-search bridges for Code Mode scripts.

use serde_json::{Map, Value};

use super::{CodeModeCallGuard, FleetMapRequest};
use crate::{
    actions::{ActionImpact, YarrAction, action_impact, execute_service_action},
    app::YarrService,
};

pub(super) enum PreparedCodeModeCall {
    Fleet {
        prepared: super::fleet::PreparedFleet,
    },
    Action {
        id: String,
        action: YarrAction,
        impact: ActionImpact,
    },
}

impl PreparedCodeModeCall {
    pub(super) fn impact(&self) -> ActionImpact {
        match self {
            Self::Fleet { prepared } => prepared.authorization.impact,
            Self::Action { impact, .. } => *impact,
        }
    }
}

impl YarrService {
    #[cfg(test)]
    pub(super) async fn codemode_dispatch(
        &self,
        id: &str,
        params_json: &str,
        in_snippet: bool,
        guard: Option<std::sync::Arc<dyn CodeModeCallGuard>>,
    ) -> Result<String, String> {
        let prepared = self.prepare_codemode_dispatch(id, params_json, in_snippet)?;
        self.authorize_prepared_codemode(&prepared, guard.as_ref())
            .await?;
        self.execute_authorized_codemode(prepared, guard, None)
            .await
    }

    pub(super) fn prepare_codemode_dispatch(
        &self,
        id: &str,
        params_json: &str,
        in_snippet: bool,
    ) -> Result<PreparedCodeModeCall, String> {
        if id == "codemode" {
            return Err("codemode cannot invoke codemode".to_owned());
        }
        if in_snippet && id == "snippet_run" {
            return Err(
                "a snippet cannot run another snippet (codemode.run is one level deep)".to_owned(),
            );
        }
        if id == "__fleet_map" {
            let request: FleetMapRequest = serde_json::from_str(params_json)
                .map_err(|error| format!("invalid fleet.map request: {error}"))?;
            let prepared = self
                .prepare_fleet(&request)
                .map_err(|error| error.to_string())?;
            return Ok(PreparedCodeModeCall::Fleet { prepared });
        }
        let params: Value = serde_json::from_str(params_json)
            .map_err(|error| format!("invalid params for `{id}`: {error}"))?;
        let mut args: Map<String, Value> = match params {
            Value::Object(map) => map,
            _ => return Err(format!("params for `{id}` must be a JSON object")),
        };
        args.insert("action".to_owned(), Value::String(id.to_owned()));

        let action =
            YarrAction::from_mcp_args(&Value::Object(args)).map_err(|error| error.to_string())?;
        let impact = action_impact(self, &action).map_err(|error| error.to_string())?;
        Ok(PreparedCodeModeCall::Action {
            id: id.to_owned(),
            action,
            impact,
        })
    }

    pub(super) async fn authorize_prepared_codemode(
        &self,
        prepared: &PreparedCodeModeCall,
        guard: Option<&std::sync::Arc<dyn CodeModeCallGuard>>,
    ) -> Result<(), String> {
        let Some(guard) = guard else {
            return Ok(());
        };
        match prepared {
            PreparedCodeModeCall::Fleet { prepared } => {
                guard.authorize_fleet(&prepared.authorization).await
            }
            PreparedCodeModeCall::Action { action, .. } => guard.authorize(action).await,
        }
    }

    pub(super) async fn execute_authorized_codemode(
        &self,
        prepared: PreparedCodeModeCall,
        guard: Option<std::sync::Arc<dyn CodeModeCallGuard>>,
        deadline: Option<tokio::time::Instant>,
    ) -> Result<String, String> {
        match prepared {
            PreparedCodeModeCall::Fleet { prepared } => {
                let value = self
                    .execute_prepared_fleet(prepared, deadline)
                    .await
                    .map_err(|error| error.to_string())?;
                serde_json::to_string(&value)
                    .map_err(|error| format!("could not serialize fleet.map result: {error}"))
            }
            PreparedCodeModeCall::Action { id, action, .. } => {
                let value = if let YarrAction::SnippetRun { name, input } = &action {
                    self.snippet_run_with_guard_until(name, input, guard, deadline)
                        .await
                        .map_err(|error| error.to_string())?
                } else {
                    // Indirection is required because `execute_service_action`
                    // can itself enter Code Mode; without it Rust detects an
                    // infinitely sized recursive future.
                    Box::pin(execute_service_action(self, &action))
                        .await
                        .map_err(|error| error.to_string())?
                };
                serde_json::to_string(&value)
                    .map_err(|error| format!("could not serialize `{id}` result: {error}"))
            }
        }
    }

    pub(super) async fn codemode_semantic_search(&self, query: &str) -> String {
        let catalog = self.codemode_catalog();
        let scores = crate::codemode::semantic_scores(
            self.semantic_cache(),
            crate::codemode::tei_url().as_deref(),
            &catalog,
            query,
        )
        .await;
        serde_json::to_string(&scores).unwrap_or_else(|_| "{}".to_owned())
    }
}

#[cfg(test)]
#[path = "codemode_dispatch_tests.rs"]
mod tests;
