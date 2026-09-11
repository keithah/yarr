use super::*;
use crate::{
    config::{ServiceConfig, ServiceKind, YarrConfig},
    yarr::YarrClient,
};

fn tautulli(service: &str, id: &str) -> TautulliIdentity {
    TautulliIdentity {
        service: service.into(),
        pms_identifier: id.into(),
    }
}
fn plex(service: &str, id: &str) -> PlexIdentity {
    PlexIdentity {
        service: service.into(),
        client_identifier: id.into(),
    }
}

#[test]
fn pairs_exact_identifiers_and_reports_unmatched_sides() {
    let report = pair_tautulli_to_plex(
        &[
            tautulli("tautulli_a", "match"),
            tautulli("tautulli_b", "tautulli-only"),
        ],
        &[plex("plex_a", "match"), plex("plex_b", "plex-only")],
    );
    assert_eq!(report.pairs.len(), 1);
    assert_eq!(report.pairs[0].tautulli_service, "tautulli_a");
    assert_eq!(report.pairs[0].plex_service, "plex_a");
    assert_eq!(
        report.unpaired_tautulli,
        vec![tautulli("tautulli_b", "tautulli-only")]
    );
    assert_eq!(report.unpaired_plex, vec![plex("plex_b", "plex-only")]);
    assert!(report.ambiguous.is_empty());
}

#[test]
fn duplicate_identifier_is_ambiguous_and_not_paired() {
    let report = pair_tautulli_to_plex(
        &[
            tautulli("tautulli_a", "duplicate"),
            tautulli("tautulli_b", "duplicate"),
        ],
        &[plex("plex_a", "duplicate")],
    );
    assert!(report.pairs.is_empty());
    assert_eq!(report.ambiguous.len(), 1);
    assert_eq!(report.ambiguous[0].identifier, "duplicate");
    assert_eq!(report.unpaired_tautulli.len(), 2);
    assert_eq!(report.unpaired_plex.len(), 1);
}

#[tokio::test]
async fn pairs_identifiers_read_from_configured_tautulli_and_plex_services() {
    let app = axum::Router::new()
        .route(
            "/tautulli/api/v2",
            axum::routing::get(|| async {
                axum::Json(serde_json::json!({
                    "response": {
                        "result": "success",
                        "data": { "pms_identifier": "server-identifier" }
                    }
                }))
            }),
        )
        .route(
            "/plex/identity",
            axum::routing::get(|| async {
                (
                    [("content-type", "application/xml")],
                    r#"<?xml version="1.0" encoding="UTF-8"?><MediaContainer machineIdentifier="server-identifier" size="0"/>"#,
                )
            }),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    let config = YarrConfig {
        services: vec![
            ServiceConfig {
                name: "tautulli-main".into(),
                kind: ServiceKind::Tautulli,
                base_url: format!("http://{address}/tautulli"),
                ..Default::default()
            },
            ServiceConfig {
                name: "plex-main".into(),
                kind: ServiceKind::Plex,
                base_url: format!("http://{address}/plex"),
                ..Default::default()
            },
        ],
    };
    let client = YarrClient::new(&config).unwrap();

    let report = pair_configured_tautulli_to_plex(&client, &config.services)
        .await
        .expect("configured identity reads must pair exactly");
    assert_eq!(
        report.pairs,
        vec![Pair {
            tautulli_service: "tautulli-main".into(),
            plex_service: "plex-main".into(),
        }]
    );
    assert!(report.unpaired_tautulli.is_empty());
    assert!(report.unpaired_plex.is_empty());
    assert!(report.ambiguous.is_empty());
}
