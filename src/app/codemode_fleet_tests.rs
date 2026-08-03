use super::{FleetMapRequest, YarrService};
use crate::config::{ServiceConfig, ServiceKind, YarrConfig};
use crate::yarr::YarrClient;

fn fleet_service() -> YarrService {
    let config = YarrConfig {
        services: vec![
            ServiceConfig {
                name: "plex_z".into(),
                kind: ServiceKind::Plex,
                base_url: "http://127.0.0.1:1".into(),
                ..Default::default()
            },
            ServiceConfig {
                name: "plex_a".into(),
                kind: ServiceKind::Plex,
                base_url: "http://127.0.0.1:2".into(),
                ..Default::default()
            },
            ServiceConfig {
                name: "sonarr".into(),
                kind: ServiceKind::Sonarr,
                base_url: "http://127.0.0.1:3".into(),
                ..Default::default()
            },
        ],
    };
    YarrService::new(YarrClient::new(&config).unwrap(), config)
        .with_fleet_limits(2, std::time::Duration::from_millis(100))
}

#[test]
fn targets_are_filtered_and_sorted_by_configured_name() {
    let request = FleetMapRequest {
        kind: "plex".into(),
        method: "service_status".into(),
        args: serde_json::json!({}),
    };
    assert_eq!(
        fleet_service().fleet_targets(&request).unwrap(),
        vec!["plex_a", "plex_z"]
    );
}

#[tokio::test]
async fn individual_failures_never_reject_the_fleet_result() {
    let request = FleetMapRequest {
        kind: "plex".into(),
        method: "service_status".into(),
        args: serde_json::json!({}),
    };
    let result = fleet_service().fleet_map(&request).await.unwrap();
    let rows = result.as_array().unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["name"], "plex_a");
    assert_eq!(rows[1]["name"], "plex_z");
    assert!(rows.iter().all(|row| row["ok"] == false));
    assert!(rows.iter().all(|row| row["error"].as_str().is_some()));
}

#[tokio::test]
async fn twenty_way_fanout_isolates_one_dead_instance_under_ten_seconds() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = axum::Router::new().route(
        "/identity",
        axum::routing::get(|| async { axum::Json(serde_json::json!({"version": "test"})) }),
    );
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let mut services = (0..19)
        .map(|index| ServiceConfig {
            name: format!("plex_{index:02}"),
            kind: ServiceKind::Plex,
            base_url: format!("http://{address}"),
            token: Some("test".into()),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    services.push(ServiceConfig {
        name: "plex_dead".into(),
        kind: ServiceKind::Plex,
        base_url: "http://127.0.0.1:1".into(),
        token: Some("test".into()),
        ..Default::default()
    });
    let config = YarrConfig { services };
    let service = YarrService::new(YarrClient::new(&config).unwrap(), config)
        .with_fleet_limits(8, std::time::Duration::from_secs(2));
    let request = FleetMapRequest {
        kind: "plex".into(),
        method: "service_status".into(),
        args: serde_json::json!({}),
    };

    let started = std::time::Instant::now();
    let result = service.fleet_map(&request).await.unwrap();
    let rows = result.as_array().unwrap();
    assert!(started.elapsed() < std::time::Duration::from_secs(10));
    assert_eq!(rows.iter().filter(|row| row["ok"] == true).count(), 19);
    assert_eq!(rows.iter().filter(|row| row["ok"] == false).count(), 1);
    assert_eq!(rows.last().unwrap()["name"], "plex_dead");
    server.abort();
}

#[tokio::test]
async fn per_instance_timeout_is_reported_without_rejecting_map() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = axum::Router::new().route(
        "/identity",
        axum::routing::get(|| async {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            axum::Json(serde_json::json!({"version": "slow"}))
        }),
    );
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let config = YarrConfig {
        services: vec![ServiceConfig {
            name: "plex_slow".into(),
            kind: ServiceKind::Plex,
            base_url: format!("http://{address}"),
            token: Some("test".into()),
            ..Default::default()
        }],
    };
    let service = YarrService::new(YarrClient::new(&config).unwrap(), config)
        .with_fleet_limits(1, std::time::Duration::from_millis(50));
    let request = FleetMapRequest {
        kind: "plex".into(),
        method: "service_status".into(),
        args: serde_json::json!({}),
    };

    let result = service.fleet_map(&request).await.unwrap();
    assert_eq!(result[0]["ok"], false);
    assert!(result[0]["error"].as_str().unwrap().contains("timed out"));
    server.abort();
}

#[test]
fn fleet_rows_are_bounded_before_the_quickjs_bridge() {
    let mut row = serde_json::json!({
        "name": "plex_den",
        "ok": true,
        "value": { "items": ["x".repeat(128 * 1024)] },
        "truncated": false,
        "elapsed_ms": 1,
    });

    crate::codemode::truncate::truncate_fleet_row(&mut row, 2048);

    assert_eq!(row["truncated"], true);
    assert!(row["value"].is_null());
    assert!(row["summary"]["original_bytes"].as_u64().unwrap() > 100_000);
    assert!(serde_json::to_vec(&row).unwrap().len() <= 2048);
}

#[tokio::test]
async fn mutating_fanout_is_not_cancelled_by_the_read_timeout() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = axum::Router::new().fallback(|| async {
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        axum::Json(serde_json::json!({"status": true}))
    });
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let config = YarrConfig {
        services: vec![ServiceConfig {
            name: "sab_slow".into(),
            kind: ServiceKind::Sabnzbd,
            base_url: format!("http://{address}"),
            api_key: Some("test".into()),
            ..Default::default()
        }],
    };
    let service = YarrService::new(YarrClient::new(&config).unwrap(), config)
        .with_fleet_limits(1, std::time::Duration::from_millis(50));
    let request = FleetMapRequest {
        kind: "sabnzbd".into(),
        method: "download_add".into(),
        args: serde_json::json!({"url": "https://example.invalid/item.nzb"}),
    };

    let started = std::time::Instant::now();
    let result = service.fleet_map(&request).await.unwrap();
    assert_eq!(result[0]["ok"], true, "{result}");
    assert!(started.elapsed() >= std::time::Duration::from_millis(125));
    server.abort();
}

#[tokio::test]
async fn mutation_fanout_stops_admitting_targets_after_the_script_deadline() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let calls = std::sync::Arc::new(AtomicUsize::new(0));
    let observed = calls.clone();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = axum::Router::new().fallback(move || {
        let observed = observed.clone();
        async move {
            observed.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(std::time::Duration::from_millis(75)).await;
            axum::Json(serde_json::json!({"status": true}))
        }
    });
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let config = YarrConfig {
        services: (0..4)
            .map(|index| ServiceConfig {
                name: format!("sab_{index}"),
                kind: ServiceKind::Sabnzbd,
                base_url: format!("http://{address}"),
                api_key: Some("test".into()),
                ..Default::default()
            })
            .collect(),
    };
    let service = YarrService::new(YarrClient::new(&config).unwrap(), config)
        .with_fleet_limits(1, std::time::Duration::from_millis(20));
    let request = FleetMapRequest {
        kind: "sabnzbd".into(),
        method: "download_add".into(),
        args: serde_json::json!({"url": "https://example.invalid/item.nzb"}),
    };
    let prepared = service.prepare_fleet(&request).unwrap();

    let result = service
        .execute_prepared_fleet(
            prepared,
            Some(tokio::time::Instant::now() + std::time::Duration::from_millis(100)),
        )
        .await
        .unwrap();

    assert!(calls.load(Ordering::SeqCst) < 4, "{result}");
    assert!(
        result
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row["outcome"] == "not_dispatched")
    );
    server.abort();
}

#[test]
fn destructive_generated_fanout_exposes_all_targets_to_the_guard() {
    let request = FleetMapRequest {
        kind: "plex".into(),
        method: "terminate_session".into(),
        args: serde_json::json!({"sessionId": "x", "reason": "test"}),
    };
    let authorization = fleet_service().fleet_authorization(&request).unwrap();
    assert_eq!(
        authorization.impact,
        crate::actions::ActionImpact::Destructive
    );
    assert_eq!(authorization.action, "terminate_session");
    assert_eq!(authorization.targets, vec!["plex_a", "plex_z"]);
}

#[tokio::test]
async fn fleet_status_shapes_reachability_version_and_latency() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = axum::Router::new().route(
        "/identity",
        axum::routing::get(|| async { axum::Json(serde_json::json!({"version": "1.2.3"})) }),
    );
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let config = YarrConfig {
        services: vec![ServiceConfig {
            name: "plex_den".into(),
            kind: ServiceKind::Plex,
            base_url: format!("http://{address}"),
            token: Some("test".into()),
            ..Default::default()
        }],
    };
    let service = YarrService::new(YarrClient::new(&config).unwrap(), config);

    let status = service.fleet_status().await.unwrap();
    assert_eq!(status[0]["name"], "plex_den");
    assert_eq!(status[0]["kind"], "plex");
    assert_eq!(status[0]["reachable"], true);
    assert_eq!(status[0]["version"], "1.2.3");
    assert!(status[0]["latency_ms"].is_number());
    server.abort();
}
