//! Host-backed fleet fanout with bounded concurrency and isolated failures.

use std::time::Instant;

use anyhow::Result;
use futures_util::{StreamExt, stream};
use serde_json::{Value, json};

use super::FleetMapRequest;
use crate::actions::{ActionImpact, YarrAction, action_impact, execute_service_action};
use crate::app::YarrService;
use crate::config::ServiceKind;

#[derive(Debug)]
pub(crate) struct FleetAuthorization {
    pub targets: Vec<String>,
    pub action: String,
    pub scope_action: &'static str,
    pub impact: ActionImpact,
}

pub(super) struct PreparedFleet {
    pub authorization: FleetAuthorization,
    request: std::sync::Arc<FleetMapRequest>,
    status: bool,
}

impl YarrService {
    pub(crate) fn fleet_targets(&self, request: &FleetMapRequest) -> Result<Vec<String>> {
        let mut targets: Vec<String> = if request.kind == "*" {
            if request.method != "service_status" {
                anyhow::bail!("fleet kind `*` is supported only by fleet.status()");
            }
            self.services
                .iter()
                .map(|service| service.name.clone())
                .collect()
        } else {
            let kind = request
                .kind
                .parse::<ServiceKind>()
                .map_err(|_| anyhow::anyhow!("unknown fleet service kind `{}`", request.kind))?;
            self.services
                .iter()
                .filter(|service| service.kind == kind)
                .map(|service| service.name.clone())
                .collect()
        };
        targets.sort();
        Ok(targets)
    }

    #[cfg(test)]
    pub(crate) fn fleet_authorization(
        &self,
        request: &FleetMapRequest,
    ) -> Result<FleetAuthorization> {
        Ok(self.prepare_fleet(request)?.authorization)
    }

    pub(super) fn prepare_fleet(&self, request: &FleetMapRequest) -> Result<PreparedFleet> {
        let targets = self.fleet_targets(request)?;
        let Some(first) = targets.first() else {
            return Ok(PreparedFleet {
                authorization: FleetAuthorization {
                    targets,
                    action: "service_status".into(),
                    scope_action: "service_status",
                    impact: ActionImpact::Read,
                },
                request: std::sync::Arc::new(request.clone()),
                status: request.kind == "*" && request.method == "service_status",
            });
        };
        // Every non-`*` fleet target has the same kind, so operation validity and
        // impact are identical. Build one representative action here; target
        // actions are materialized lazily inside the bounded stream.
        let first_action = self.fleet_action(first, request)?;
        let first_impact = action_impact(self, &first_action)?;
        let scope_action = first_action.name();
        Ok(PreparedFleet {
            authorization: FleetAuthorization {
                targets,
                action: request.method.clone(),
                scope_action,
                impact: first_impact,
            },
            request: std::sync::Arc::new(request.clone()),
            status: request.kind == "*" && request.method == "service_status",
        })
    }

    pub(crate) async fn fleet_map(&self, request: &FleetMapRequest) -> Result<Value> {
        let prepared = self.prepare_fleet(request)?;
        self.execute_prepared_fleet(prepared, None).await
    }

    pub(super) async fn execute_prepared_fleet(
        &self,
        prepared: PreparedFleet,
        deadline: Option<tokio::time::Instant>,
    ) -> Result<Value> {
        let PreparedFleet {
            authorization,
            request,
            status,
        } = prepared;
        let targets = authorization.targets;
        let impact = authorization.impact;
        let timeout = self.fleet_instance_timeout;
        let per_instance_budget = crate::codemode::truncate::FLEET_RESULT_BUDGET
            .checked_div(targets.len())
            .unwrap_or(0)
            .max(256);
        let mut results = stream::iter(targets.into_iter().map(|name| {
            let request = request.clone();
            async move {
                let started = Instant::now();
                let result = if impact.mutates()
                    && deadline.is_some_and(|deadline| tokio::time::Instant::now() >= deadline)
                {
                    None
                } else {
                    Some(match self.fleet_action(&name, &request) {
                        Ok(action) if impact.uses_instance_timeout() => match tokio::time::timeout(
                            timeout,
                            execute_service_action(self, &action),
                        )
                        .await
                        {
                            Ok(result) => result,
                            Err(_) => Err(anyhow::anyhow!(
                                "instance timed out after {} ms",
                                timeout.as_millis()
                            )),
                        },
                        Ok(action) => execute_service_action(self, &action).await,
                        Err(error) => Err(error),
                    })
                };
                let mut row = match result {
                    None => json!({
                        "name": name, "ok": false,
                        "outcome": "not_dispatched",
                        "error": "Code Mode deadline expired before this mutation was dispatched",
                        "truncated": false, "elapsed_ms": started.elapsed().as_millis(),
                    }),
                    Some(Ok(value)) => json!({
                        "name": name, "ok": true, "value": value,
                        "outcome": if impact.mutates() { "confirmed" } else { "read" },
                        "truncated": false, "elapsed_ms": started.elapsed().as_millis(),
                    }),
                    Some(Err(error)) => json!({
                        "name": name, "ok": false, "error": error.to_string(),
                        "outcome": if impact.mutates() { "indeterminate" } else { "failed" },
                        "truncated": false, "elapsed_ms": started.elapsed().as_millis(),
                    }),
                };
                crate::codemode::truncate::truncate_fleet_row(&mut row, per_instance_budget);
                row
            }
        }))
        .buffer_unordered(self.fleet_max_concurrent)
        .collect::<Vec<_>>()
        .await;
        results.sort_by(|left, right| left["name"].as_str().cmp(&right["name"].as_str()));
        if status {
            return Ok(Value::Array(
                results
                    .into_iter()
                    .map(|row| self.status_row(row))
                    .collect(),
            ));
        }
        Ok(Value::Array(results))
    }

    /// Per-instance reachability, version, and latency over the same bounded
    /// dispatcher used by Code Mode `fleet.status()`.
    pub async fn fleet_status(&self) -> Result<Value> {
        self.fleet_map(&FleetMapRequest {
            kind: "*".into(),
            method: "service_status".into(),
            args: json!({}),
        })
        .await
    }

    fn status_row(&self, row: Value) -> Value {
        let name = row["name"].as_str().unwrap_or("<unknown>");
        let kind = self.kind_of(name).ok().flatten().map(|kind| kind.as_str());
        json!({
            "name": name,
            "kind": kind,
            "reachable": row["ok"],
            "version": find_version(kind, &row["value"]),
            "latency_ms": row["elapsed_ms"],
            "error": row.get("error").cloned().unwrap_or(Value::Null),
            "truncated": row["truncated"],
        })
    }

    fn fleet_action(&self, service_name: &str, request: &FleetMapRequest) -> Result<YarrAction> {
        let kind = self
            .kind_of(service_name)?
            .ok_or_else(|| anyhow::anyhow!("unknown fleet service `{service_name}`"))?;
        if request.method == "service_status" {
            return Ok(YarrAction::ServiceStatus {
                service: service_name.to_owned(),
            });
        }
        if crate::openapi::is_generated(kind) {
            if crate::openapi::classify_operation(kind, &request.method).is_none() {
                anyhow::bail!(
                    "operation `{}` is not available for kind {}",
                    request.method,
                    kind.as_str()
                );
            }
            return Ok(YarrAction::Op {
                service: service_name.to_owned(),
                op: request.method.clone(),
                args: request.args.clone(),
            });
        }
        if !crate::codemode::catalog::service_action_names(kind).contains(&request.method.as_str())
        {
            anyhow::bail!(
                "action `{}` is not available for kind {}",
                request.method,
                kind.as_str()
            );
        }
        let mut params = request.args.as_object().cloned().ok_or_else(|| {
            anyhow::anyhow!("fleet.map service method params must be a JSON object")
        })?;
        params.insert("action".into(), Value::String(request.method.clone()));
        params.insert("service".into(), Value::String(service_name.to_owned()));
        YarrAction::from_mcp_args(&Value::Object(params))
    }
}

fn find_version<'a>(kind: Option<&str>, value: &'a Value) -> Option<&'a str> {
    let paths: &[&str] = match kind {
        Some("plex") => &["/MediaContainer/version", "/version"],
        Some("tautulli") => &[
            "/response/data/tautulli_version",
            "/response/data/pms_version",
        ],
        Some("jellyfin") => &["/Version", "/version"],
        Some("qbittorrent") => &["/version", "/app/version"],
        _ => &["/version", "/productVersion"],
    };
    paths
        .iter()
        .find_map(|path| value.pointer(path).and_then(Value::as_str))
}
