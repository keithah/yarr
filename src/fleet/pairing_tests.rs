use super::*;

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
