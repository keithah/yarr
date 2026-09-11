use super::*;
use std::fs;

const FIXTURE: &str = include_str!("fixtures/plex_resources_redacted.json");

#[test]
fn parser_ignores_non_servers_and_defaults_to_owned_resources() {
    let resources = parse_plex_resources(FIXTURE).expect("redacted fixture parses");
    let discovered = build_report(&resources, false).expect("owned resources select connections");
    assert_eq!(discovered.resources.len(), 2);
    assert!(
        discovered
            .resources
            .iter()
            .all(|item| item.name != "Web Client")
    );
    assert!(
        discovered
            .resources
            .iter()
            .all(|item| item.name != "Shared Server")
    );
}

#[test]
fn parser_rejects_missing_observed_resources_shape() {
    let error = parse_plex_resources(r#"{"MediaContainer":{"Device":{}}}"#)
        .expect_err("Device must be the observed resource array");
    assert!(error.to_string().contains("MediaContainer.Device"));
}

#[test]
fn include_shared_retains_shared_servers() {
    let resources = parse_plex_resources(FIXTURE).unwrap();
    let discovered = build_report(&resources, true).unwrap();
    assert!(
        discovered
            .resources
            .iter()
            .any(|item| item.name == "plex_shared_server")
    );
}

#[test]
fn connection_priority_is_local_then_direct_https_then_relay() {
    let resources = parse_plex_resources(FIXTURE).unwrap();
    let first = resources
        .iter()
        .find(|resource| resource.client_identifier == "server-one")
        .unwrap();
    let chosen = select_connection(first).expect("a connection is available");
    assert_eq!(chosen.url, "http://192.0.2.10:32400");
    assert!(!chosen.relay_only);

    let relay = resources
        .iter()
        .find(|resource| resource.client_identifier == "relay-server")
        .unwrap();
    assert!(select_connection(relay).unwrap().relay_only);

    let mut unsafe_local = PlexResource::for_test("Unsafe", "unsafe-local");
    unsafe_local.connections = vec![
        PlexConnection {
            uri: "https://user:password@unsafe.invalid".into(),
            local: true,
            relay: false,
            protocol: "https".into(),
        },
        PlexConnection {
            uri: "https://direct.invalid".into(),
            local: false,
            relay: false,
            protocol: "https".into(),
        },
    ];
    assert_eq!(
        select_connection(&unsafe_local).unwrap().url,
        "https://direct.invalid"
    );
}

#[test]
fn configured_names_are_stable_and_collision_safe() {
    let resources = vec![
        PlexResource::for_test("Same Name", "client-a"),
        PlexResource::for_test("Same Name", "client-b"),
    ];
    let names = configured_names(&resources);
    assert_eq!(names.len(), 2);
    assert!(names.iter().all(|name| name.starts_with("plex_same_name")));
    assert_ne!(names[0], names[1]);
    assert_eq!(names, configured_names(&resources));
    assert!(names.iter().all(|name| {
        name.chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    }));
}

#[test]
fn public_and_secret_outputs_are_separate_and_secret_is_owner_only() {
    let temp = tempfile::tempdir().unwrap();
    let fleet = temp.path().join("fleet.yaml");
    let secret = temp.path().join("plex.env");
    let report = build_report(&parse_plex_resources(FIXTURE).unwrap(), false).unwrap();
    write_discovery_outputs(&fleet, &secret, &report).unwrap();
    let public = fs::read_to_string(&fleet).unwrap();
    let private = fs::read_to_string(&secret).unwrap();
    assert!(public.contains("token_env:"));
    assert!(!public.contains("<redacted>"));
    assert!(!public.contains("accessToken"));
    assert!(private.contains("=<redacted>"));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(&secret).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
}

#[test]
fn diff_reports_typed_drift_without_writing_files() {
    let temp = tempfile::tempdir().unwrap();
    let fleet = temp.path().join("fleet.yaml");
    let secret = temp.path().join("plex.env");
    fs::write(&fleet, "sentinel-public").unwrap();
    fs::write(&secret, "sentinel-secret").unwrap();
    let new = build_report(&parse_plex_resources(FIXTURE).unwrap(), false).unwrap();
    let mut old = PlexDiscoveryReport::empty();
    let mut prior_relay = new
        .resources
        .iter()
        .find(|item| item.relay_only)
        .expect("fixture includes a relay-only server")
        .clone();
    prior_relay.relay_only = false;
    old.resources.push(prior_relay);
    let drift = classify_drift(&old, &new);
    assert!(
        drift
            .iter()
            .any(|change| matches!(change, Drift::Added { .. }))
    );
    assert!(
        drift
            .iter()
            .any(|change| matches!(change, Drift::RelayStateChanged { .. }))
    );
    let options = PlexDiscoveryOptions::for_test(fleet.clone(), secret.clone(), true);
    write_or_diff(&options, &old, &new).unwrap();
    assert_eq!(fs::read_to_string(&fleet).unwrap(), "sentinel-public");
    assert_eq!(fs::read_to_string(&secret).unwrap(), "sentinel-secret");
}

#[test]
fn injected_client_receives_only_the_env_value_and_report_never_returns_it() {
    let mut env = crate::testing::TestEnv::new();
    env.set("YARR_TEST_PLEX_TOKEN", "<redacted>");
    let options = PlexDiscoveryOptions::for_test("unused.yaml".into(), "unused.env".into(), true);
    let report = discover_plex_with_client(options, &FixtureClient { body: FIXTURE }).unwrap();
    assert!(
        report
            .drift
            .iter()
            .any(|change| matches!(change, Drift::Added { .. }))
    );
    let rendered = serde_json::to_string(&report).unwrap();
    assert!(!rendered.contains("<redacted>"));
}

struct FixtureClient {
    body: &'static str,
}
impl PlexDiscoveryClient for FixtureClient {
    fn fetch_resources(&self, _token: &str) -> anyhow::Result<String> {
        Ok(self.body.to_owned())
    }
}
