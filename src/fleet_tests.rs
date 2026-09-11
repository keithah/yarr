use super::{FleetInvocation, FleetSelector};
use crate::{ServiceConfig, ServiceKind, YarrClient, YarrConfig, YarrService};
use serde_json::{Map, Value, json};

fn service() -> YarrService {
    let services = (0..20)
        .map(|index| ServiceConfig {
            name: format!("instance-{index:02}"),
            kind: if index % 2 == 0 {
                ServiceKind::Sonarr
            } else {
                ServiceKind::Radarr
            },
            base_url: format!("http://127.0.0.1:{}", 40000 + index),
            ..ServiceConfig::default()
        })
        .rev()
        .collect::<Vec<_>>();
    let config = YarrConfig { services };
    YarrService::new(YarrClient::new(&config).unwrap(), config)
}

#[test]
fn plan_fleet_sorts_twenty_mixed_instances_and_forces_identity() {
    let svc = service();
    let mut params = Map::new();
    params.insert("service".into(), Value::String("attacker".into()));
    let plan = svc
        .plan_fleet(FleetInvocation {
            selector: FleetSelector::All {
                kind: Some(ServiceKind::Sonarr),
            },
            action: "service_status".into(),
            params,
        })
        .expect("valid selected read action");
    assert_eq!(plan.leaves().len(), 10);
    assert_eq!(
        plan.leaves()
            .iter()
            .map(|leaf| leaf.service.as_str())
            .collect::<Vec<_>>(),
        (0..20)
            .step_by(2)
            .map(|index| format!("instance-{index:02}"))
            .collect::<Vec<_>>()
    );
    assert!(
        plan.leaves()
            .iter()
            .all(|leaf| leaf.action_service() == leaf.service)
    );
}

#[test]
fn plan_fleet_of_requires_exact_configured_identity_and_rejects_invalid_actions() {
    let svc = service();
    let exact = svc
        .plan_fleet(FleetInvocation {
            selector: FleetSelector::Of {
                name: "instance-01".into(),
            },
            action: "service_status".into(),
            params: Map::new(),
        })
        .expect("configured identity is accepted");
    assert_eq!(exact.leaves()[0].service, "instance-01");

    let alias = svc
        .plan_fleet(FleetInvocation {
            selector: FleetSelector::Of {
                name: "sonarr".into(),
            },
            action: "service_status".into(),
            params: Map::new(),
        })
        .expect_err("kind alias is not an identity");
    assert!(alias.to_string().contains("configured service identity"));

    for action in ["help", "codemode", "snippet_list", "no_such_action"] {
        let error = svc
            .plan_fleet(FleetInvocation {
                selector: FleetSelector::Of {
                    name: "instance-00".into(),
                },
                action: action.into(),
                params: if action == "codemode" {
                    serde_json::from_value(json!({"code":"async () => null"})).unwrap()
                } else {
                    Map::new()
                },
            })
            .expect_err("infra/script/invalid actions cannot fan out");
        assert!(!error.to_string().is_empty());
    }
}
