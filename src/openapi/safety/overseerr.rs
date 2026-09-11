use crate::config::ServiceKind;

use super::SafetyRow;

pub(super) const ROWS: &[SafetyRow] = &[
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_auth_local",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_auth_logout",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_auth_plex",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_auth_reset_password",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_auth_reset_password_by_guid",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_issue",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_issue_by_issue_id_status",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_issue_comment_by_issue_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_media_by_media_id_status",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_request",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_request_by_request_id_status",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_request_retry_by_request_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_cache_flush_by_cache_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_discover",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_discover_add",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_initialize",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_jobs_cancel_by_job_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_jobs_run_by_job_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_jobs_schedule_by_job_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_main",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_main_regenerate",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_discord",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_discord_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_email",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_email_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_gotify",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_gotify_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_lunasea",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_lunasea_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_pushbullet",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_pushbullet_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_pushover",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_pushover_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_slack",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_slack_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_telegram",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_telegram_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_webhook",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_webhook_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_webpush",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_notifications_webpush_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_plex",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_plex_sync",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_radarr",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_radarr_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_sonarr",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_sonarr_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_settings_tautulli",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_user",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_user_import_from_plex",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_user_register_push_subscription",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_user_settings_main_by_user_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_user_settings_notifications_by_user_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_user_settings_password_by_user_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "post_user_settings_permissions_by_user_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "put_issue_comment_by_comment_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "put_request_by_request_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "put_settings_discover_by_slider_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "put_settings_radarr_by_radarr_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "put_settings_sonarr_by_sonarr_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "put_user",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Overseerr,
        operation: "put_user_by_user_id",
        destructive: false,
    },
];

#[cfg(test)]
#[path = "overseerr_tests.rs"]
mod tests;
