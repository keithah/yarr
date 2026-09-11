use crate::config::ServiceKind;

use super::SafetyRow;

pub(super) const ROWS: &[SafetyRow] = &[
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_applications",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_applications_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_applications_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_applications_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_appprofile",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_command",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_customfilter",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_downloadclient",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_downloadclient_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_downloadclient_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_downloadclient_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_indexer",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_indexer_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_indexer_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_indexer_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_indexerproxy",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_indexerproxy_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_indexerproxy_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_indexerproxy_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_login",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_notification",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_notification_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_notification_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_notification_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_search",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_search_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_system_backup_restore_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_system_backup_restore_upload",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_system_restart",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_system_shutdown",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "post_tag",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_applications_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_applications_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_appprofile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_config_development_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_config_downloadclient_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_config_host_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_config_ui_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_customfilter_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_downloadclient_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_downloadclient_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_indexer_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_indexer_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_indexerproxy_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_notification_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Prowlarr,
        operation: "put_tag_by_id",
        destructive: false,
    },
];

#[cfg(test)]
#[path = "prowlarr_tests.rs"]
mod tests;
