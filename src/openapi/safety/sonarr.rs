use crate::config::ServiceKind;

use super::SafetyRow;

pub(super) const ROWS: &[SafetyRow] = &[
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_autotagging",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_command",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_customfilter",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_customformat",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_delayprofile",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_downloadclient",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_downloadclient_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_downloadclient_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_downloadclient_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_history_failed_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_importlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_importlist_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_importlist_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_importlist_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_importlistexclusion",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_indexer",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_indexer_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_indexer_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_indexer_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_languageprofile",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_login",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_manualimport",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_metadata",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_metadata_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_metadata_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_metadata_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_notification",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_notification_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_notification_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_notification_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_qualityprofile",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_queue_grab_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_queue_grab_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_release",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_release_push",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_releaseprofile",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_remotepathmapping",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_rootfolder",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_seasonpass",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_series",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_series_import",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_system_backup_restore_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_system_backup_restore_upload",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_system_restart",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_system_shutdown",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "post_tag",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_autotagging_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_config_downloadclient_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_config_host_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_config_importlist_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_config_indexer_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_config_mediamanagement_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_config_naming_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_config_ui_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_customfilter_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_customformat_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_customformat_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_delayprofile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_delayprofile_reorder_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_downloadclient_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_downloadclient_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_episode_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_episode_monitor",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_episodefile_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_episodefile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_episodefile_editor",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_importlist_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_importlist_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_importlistexclusion_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_indexer_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_indexer_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_languageprofile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_metadata_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_notification_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_qualitydefinition_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_qualitydefinition_update",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_qualityprofile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_releaseprofile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_remotepathmapping_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_series_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_series_editor",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Sonarr,
        operation: "put_tag_by_id",
        destructive: false,
    },
];

#[cfg(test)]
#[path = "sonarr_tests.rs"]
mod tests;
