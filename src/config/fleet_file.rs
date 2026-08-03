//! Additive fleet-file parsing and environment-secret resolution.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use super::services::{ServiceConfig, ServiceKind, validate_service_identities};

#[derive(Debug, Clone, Copy)]
pub(crate) enum FleetFormat {
    Yaml,
    Toml,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FleetDocument {
    services: Vec<FleetServiceEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FleetServiceEntry {
    name: String,
    kind: ServiceKind,
    url: String,
    token_env: Option<String>,
    api_key_env: Option<String>,
    username_env: Option<String>,
    password_env: Option<String>,
    client_identifier: Option<String>,
    plex: Option<String>,
    #[serde(default)]
    relay_only: bool,
}

fn format_for_path(path: &Path) -> Result<FleetFormat> {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("yaml" | "yml") => Ok(FleetFormat::Yaml),
        Some("toml") => Ok(FleetFormat::Toml),
        _ => anyhow::bail!(
            "fleet file {} must have a .yaml, .yml, or .toml extension",
            path.display()
        ),
    }
}

pub(crate) fn load_fleet_file_with_overrides(
    path: &Path,
    higher_precedence: Vec<ServiceConfig>,
) -> Result<Vec<ServiceConfig>> {
    validate_service_identities(&higher_precedence)
        .context("invalid higher-precedence service configuration")?;
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read fleet file {}", path.display()))?;
    let entries = parse_entries(&contents, format_for_path(path)?, path)?;
    let mut higher = higher_precedence
        .into_iter()
        .map(|service| (service.name.to_ascii_lowercase(), service))
        .collect::<BTreeMap<_, _>>();
    let mut lower = Vec::new();

    let mut seen = BTreeSet::new();
    for (entry, line) in entries {
        entry.validate(path, line)?;
        let normalized_name = entry.name.trim().to_ascii_lowercase();
        if !seen.insert(normalized_name.clone()) {
            anyhow::bail!(
                "{}:{line}: duplicate fleet service name {normalized_name:?}",
                path.display()
            );
        }
        if let Some(service) = higher.get_mut(&normalized_name) {
            service.client_identifier = service
                .client_identifier
                .take()
                .or_else(|| nonempty(entry.client_identifier));
            service.plex = service.plex.take().or_else(|| nonempty(entry.plex));
            service.relay_only |= entry.relay_only;
        } else {
            lower.push(entry.resolve(path, line)?);
        }
    }

    merge_service_sources(lower, higher.into_values().collect())
}

#[cfg(test)]
pub(crate) fn parse_and_resolve(
    contents: &str,
    format: FleetFormat,
    source: &Path,
) -> Result<Vec<ServiceConfig>> {
    let entries = parse_entries(contents, format, source)?;
    let mut services = Vec::with_capacity(entries.len());
    for (entry, line) in entries {
        services.push(entry.resolve(source, line)?);
    }
    services.sort_by(|left, right| left.name.cmp(&right.name));
    validate_service_identities(&services)
        .with_context(|| format!("invalid fleet file {}", source.display()))?;
    Ok(services)
}

fn parse_entries(
    contents: &str,
    format: FleetFormat,
    source: &Path,
) -> Result<Vec<(FleetServiceEntry, usize)>> {
    let document: FleetDocument = match format {
        FleetFormat::Yaml => serde_yaml::from_str(contents).map_err(|error| {
            let line = error.location().map_or(1, |location| location.line());
            fleet_parse_error(source, contents, line, &error)
        })?,
        FleetFormat::Toml => toml::from_str(contents).map_err(|error| {
            let line = error
                .span()
                .map_or(1, |span| contents[..span.start].lines().count());
            fleet_parse_error(source, contents, line, &error)
        })?,
    };
    let lines = entry_lines(contents, format);
    Ok(document
        .services
        .into_iter()
        .enumerate()
        .map(|(index, entry)| {
            let line = lines.get(index).copied().unwrap_or(1);
            (entry, line)
        })
        .collect())
}

fn fleet_parse_error(
    source: &Path,
    contents: &str,
    line: usize,
    error: &dyn std::fmt::Display,
) -> anyhow::Error {
    let (entry_line, name) = contents
        .lines()
        .take(line)
        .enumerate()
        .filter_map(|(index, text)| parse_name_line(text).map(|name| (index + 1, name)))
        .last()
        .unwrap_or_else(|| (line, "<unnamed>".into()));
    anyhow::anyhow!(
        "{}:{entry_line}: invalid fleet service {name:?} (invalid field at line {line}): {error}; credentials must use token_env, api_key_env, username_env, or password_env (inline secrets are forbidden)",
        source.display()
    )
}

impl FleetServiceEntry {
    fn validate(&self, source: &Path, line: usize) -> Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!(
                "{}:{line}: fleet service name must not be empty",
                source.display()
            );
        }
        if self.url.trim().is_empty() {
            anyhow::bail!(
                "{}:{line}: fleet service {:?} url must not be empty",
                source.display(),
                self.name.trim().to_ascii_lowercase()
            );
        }
        for (reference, field) in [
            (&self.api_key_env, "api_key_env"),
            (&self.username_env, "username_env"),
            (&self.password_env, "password_env"),
            (&self.token_env, "token_env"),
        ] {
            if let Some(variable) = reference.as_deref().map(str::trim)
                && !valid_env_name(variable)
            {
                anyhow::bail!(
                    "{}:{line}: {field} value {variable:?} is not a valid environment variable name",
                    source.display()
                );
            }
        }
        Ok(())
    }

    fn resolve(self, source: &Path, line: usize) -> Result<ServiceConfig> {
        self.validate(source, line)?;
        let name = self.name.trim().to_ascii_lowercase();
        let base_url = self.url.trim().to_owned();
        Ok(ServiceConfig {
            name,
            kind: self.kind,
            base_url,
            api_key: resolve_env(&self.api_key_env, "api_key_env", source, line)?,
            username: resolve_env(&self.username_env, "username_env", source, line)?,
            password: resolve_env(&self.password_env, "password_env", source, line)?,
            token: resolve_env(&self.token_env, "token_env", source, line)?,
            read_only: false,
            client_identifier: nonempty(self.client_identifier),
            plex: nonempty(self.plex),
            relay_only: self.relay_only,
        })
    }
}

fn resolve_env(
    reference: &Option<String>,
    field: &str,
    source: &Path,
    line: usize,
) -> Result<Option<String>> {
    let Some(variable) = reference.as_deref().map(str::trim) else {
        return Ok(None);
    };
    debug_assert!(valid_env_name(variable));
    let value = super::env_value(variable)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "{}:{line}: {field} references unset or empty environment variable {variable}",
                source.display()
            )
        })?;
    Ok(Some(value))
}

fn valid_env_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn nonempty(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn entry_lines(contents: &str, format: FleetFormat) -> Vec<usize> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(line, text)| match format {
            FleetFormat::Yaml if text.trim_start().starts_with("- name:") => Some(line + 1),
            FleetFormat::Toml if text.trim() == "[[services]]" => Some(line + 1),
            _ => None,
        })
        .collect()
}

fn parse_name_line(line: &str) -> Option<String> {
    let line = line.trim_start().strip_prefix('-').unwrap_or(line).trim();
    let value = line
        .strip_prefix("name:")
        .or_else(|| line.strip_prefix("name ="))?
        .trim();
    Some(value.trim_matches(['\'', '"']).to_owned())
}

pub(crate) fn merge_service_sources(
    lower_precedence: Vec<ServiceConfig>,
    higher_precedence: Vec<ServiceConfig>,
) -> Result<Vec<ServiceConfig>> {
    validate_service_identities(&lower_precedence)?;
    validate_service_identities(&higher_precedence)?;
    let mut merged = BTreeMap::<String, ServiceConfig>::new();
    for service in lower_precedence {
        merged.insert(service.name.to_ascii_lowercase(), service);
    }
    for mut service in higher_precedence {
        if let Some(lower) = merged.get(&service.name.to_ascii_lowercase()) {
            service.client_identifier = service
                .client_identifier
                .or_else(|| lower.client_identifier.clone());
            service.plex = service.plex.or_else(|| lower.plex.clone());
            service.relay_only |= lower.relay_only;
        }
        merged.insert(service.name.to_ascii_lowercase(), service);
    }
    let services = merged.into_values().collect::<Vec<_>>();
    validate_service_identities(&services)?;
    Ok(services)
}
