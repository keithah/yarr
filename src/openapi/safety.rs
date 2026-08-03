//! Reviewed safety classification for generated OpenAPI operations.
//!
//! HTTP DELETE remains an unconditional destructive trigger. The table below
//! adds non-DELETE operations whose blast radius warrants the same MCP
//! elicitation gate. Every other non-GET operation is a normal mutation. The
//! reviewed write inventory deliberately fails when regenerated specs change a
//! write, forcing the operation through review before generated docs and CI pass.

use crate::config::ServiceKind;

use super::OperationSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationSafety {
    Read,
    Mutating,
    Destructive,
}

#[derive(Debug, Clone, Copy)]
struct SafetyRow {
    kind: ServiceKind,
    operation: &'static str,
}

const HIGH_IMPACT_ROWS: &[SafetyRow] = &[
    // Plex: stream termination, metadata/library-wide changes, and operations
    // that can start expensive work across an entire server.
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "terminate_session",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "edit_metadata_item",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "refresh_section",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "refresh_sections_metadata",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "scan",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_section",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "edit_section",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "empty_trash",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "clean_bundles",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "optimize_database",
    },
    // Arr command bodies can request library-wide scans/imports. Bulk editors,
    // restore, restart, and shutdown likewise have fleet-scale blast radius.
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_command",
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_system_backup_restore_by_id",
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_system_backup_restore_upload",
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_system_restart",
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_system_shutdown",
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_series_editor",
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_command",
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_system_backup_restore_by_id",
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_system_backup_restore_upload",
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_system_restart",
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_system_shutdown",
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_movie_editor",
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_command",
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_system_backup_restore_by_id",
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_system_backup_restore_upload",
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_system_restart",
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_system_shutdown",
    },
    // Overseerr administrative job controls and full Plex synchronization.
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_jobs_cancel_by_job_id",
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_jobs_run_by_job_id",
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_jobs_schedule_by_job_id",
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_plex_sync",
    },
    // Jellyfin server lifecycle, library-wide work, restore, and remote session
    // controls are high impact even though the API models them as POST.
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "refresh_library",
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "restart_application",
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "shutdown_application",
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "start_restore_backup",
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "start_task",
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "send_full_general_command",
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "send_general_command",
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "send_playstate_command",
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "send_system_command",
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_stop",
    },
];

// These upstream APIs incorrectly model side-effecting calls as GET.
const MUTATING_GET_ROWS: &[SafetyRow] = &[
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_subtitles",
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "start_transcode_session",
    },
];

const REVIEWED_WRITE_INVENTORY: &str = include_str!("reviewed_write_inventory.txt");

pub fn operation_safety(kind: ServiceKind, spec: &OperationSpec) -> OperationSafety {
    if spec.method.is_delete()
        || HIGH_IMPACT_ROWS
            .iter()
            .any(|row| row.kind == kind && row.operation == spec.name)
    {
        OperationSafety::Destructive
    } else if !spec.method.is_read()
        || MUTATING_GET_ROWS
            .iter()
            .any(|row| row.kind == kind && row.operation == spec.name)
    {
        OperationSafety::Mutating
    } else {
        OperationSafety::Read
    }
}

pub fn classify_operation(kind: ServiceKind, operation: &str) -> Option<OperationSafety> {
    super::find_operation(kind, operation).map(|spec| operation_safety(kind, spec))
}

pub fn validate_write_inventory() -> Result<(), String> {
    use std::collections::BTreeSet;

    let actual = ServiceKind::ALL
        .iter()
        .copied()
        .filter(|kind| super::is_generated(*kind))
        .flat_map(|kind| {
            super::operations_for_kind(kind)
                .iter()
                .filter_map(move |spec| {
                    let safety = operation_safety(kind, spec);
                    (safety != OperationSafety::Read).then(|| {
                        format!(
                            "{}.{}|{}|{safety:?}",
                            kind.as_str(),
                            spec.name,
                            spec.method.as_str()
                        )
                    })
                })
        })
        .collect::<BTreeSet<_>>();
    let reviewed = REVIEWED_WRITE_INVENTORY
        .lines()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    if actual != reviewed {
        let added = actual
            .difference(&reviewed)
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        let removed = reviewed
            .difference(&actual)
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        return Err(format!(
            "generated write inventory changed; added/changed: {added:?}; removed/changed: {removed:?}; classify and audit the exact operation, method, and safety before regenerating tool docs"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod row_coverage {
    use super::*;

    #[test]
    fn every_high_impact_row_resolves_to_a_destructive_operation() {
        for row in HIGH_IMPACT_ROWS {
            assert_eq!(
                classify_operation(row.kind, row.operation),
                Some(OperationSafety::Destructive),
                "{}.{} in HIGH_IMPACT_ROWS must resolve to a destructive operation",
                row.kind.as_str(),
                row.operation
            );
        }
    }

    #[test]
    fn every_mutating_get_row_resolves_to_a_mutating_operation() {
        for row in MUTATING_GET_ROWS {
            assert_eq!(
                classify_operation(row.kind, row.operation),
                Some(OperationSafety::Mutating),
                "{}.{} in MUTATING_GET_ROWS must resolve to a mutating operation",
                row.kind.as_str(),
                row.operation
            );
        }
    }
}
