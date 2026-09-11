//! Authoritative safety classification for generated OpenAPI operations.
//!
//! Every non-GET generated operation is classified here before execution. DELETE
//! operations are destructive by policy; POST/PUT/PATCH rows are an audited table
//! derived from the current generated registry and validated against it.

use crate::config::ServiceKind;

use super::{HttpMethod, OperationSpec, find_operation, operations_for_kind};

/// Safety authority consumed by generated execution and MCP elicitation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationSafety {
    pub mutates: bool,
    pub destructive: bool,
    pub elicitation_required: bool,
}

impl OperationSafety {
    const READ_ONLY: Self = Self {
        mutates: false,
        destructive: false,
        elicitation_required: false,
    };
    const MUTATION: Self = Self {
        mutates: true,
        destructive: false,
        elicitation_required: false,
    };
    const DESTRUCTIVE: Self = Self {
        mutates: true,
        destructive: true,
        elicitation_required: true,
    };
}

#[derive(Clone, Copy)]
struct SafetyRow {
    kind: ServiceKind,
    operation: &'static str,
    destructive: bool,
}

// This table is deliberately exhaustive for generated POST/PUT/PATCH operations.
// Regeneration drift is rejected by validate_generated_write_classification().
const AUDITED_WRITES: &[SafetyRow] = &[
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "add_item_to_playlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "add_listing_provider",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "add_media_path",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "add_to_collection",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "add_tuner_host",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "add_user_to_session",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "add_virtual_folder",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "apply_search_criteria",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "authenticate_user_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "authenticate_with_quick_connect",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "authorize_quick_connect",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "close_live_stream",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "complete_wizard",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "create_backup",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "create_collection",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "create_key",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "create_playlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "create_series_timer",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "create_timer",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "create_user_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "disable_plugin",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "display_content",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "download_remote_image",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "download_remote_lyrics",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "download_remote_subtitles",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "enable_plugin",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "forgot_password",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "forgot_password_pin",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_book_remote_search_results",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_box_set_remote_search_results",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_movie_remote_search_results",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_music_album_remote_search_results",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_music_artist_remote_search_results",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_music_video_remote_search_results",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_person_remote_search_results",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_plugin_manifest",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_posted_playback_info",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_programs",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_series_remote_search_results",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "get_trailer_remote_search_results",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "initiate_quick_connect",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "install_package",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "log_file",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "mark_favorite_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "mark_played_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "merge_versions",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "move_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "open_live_stream",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "ping_playback_session",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "play",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "post_added_movies",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "post_added_series",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "post_capabilities",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "post_full_capabilities",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "post_ping_system",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "post_updated_media",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "post_updated_movies",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "post_updated_series",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "post_user_image",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "refresh_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "refresh_library",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "rename_virtual_folder",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "report_playback_progress",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "report_playback_start",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "report_playback_stopped",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "report_session_ended",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "report_viewing",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "reset_tuner",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "restart_application",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "send_full_general_command",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "send_general_command",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "send_message_command",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "send_playstate_command",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "send_system_command",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "set_channel_mapping",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "set_item_image",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "set_item_image_by_index",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "set_remote_access",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "set_repositories",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "shutdown_application",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "start_restore_backup",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "start_task",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_buffering",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_create_group",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_join_group",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_leave_group",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_move_playlist_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_next_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_pause",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_ping",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_previous_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_queue",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_ready",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_remove_from_playlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_seek",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_set_ignore_wait",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_set_new_queue",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_set_playlist_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_set_repeat_mode",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_set_shuffle_mode",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_stop",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "sync_play_unpause",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_branding_configuration",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_configuration",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_device_options",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_display_preferences",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_initial_configuration",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_item_content_type",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_item_image_index",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_item_user_data",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_library_options",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_media_path",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_named_configuration",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_playlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_playlist_user",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_plugin_configuration",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_series_timer",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_startup_user",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_task",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_timer",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_user",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_user_configuration",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_user_item_rating",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_user_password",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "update_user_policy",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "upload_custom_splashscreen",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "upload_lyrics",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "upload_subtitle",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Jellyfin,
        operation: "validate_path",
        destructive: false,
    },
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
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_collection_items",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_device",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_device_to_dvr",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_download_queue_items",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_extras",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_lineup",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_playlist_items",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_provider",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_section",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "add_to_play_queue",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "analyze_metadata",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "apply_updates",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "check_updates",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "clean_bundles",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "create_collection",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "create_custom_hub",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "create_download_queue",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "create_dvr",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "create_marker",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "create_play_queue",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "create_playlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "create_subscription",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "delete_collection_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "detect_ads",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "detect_credits",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "detect_intros",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "detect_voice_activity",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "discover_devices",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "edit_marker",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "edit_metadata_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "edit_section",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "edit_subscription_preferences",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "empty_trash",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "enable_papertrail",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "generate_thumbs",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "get_transient_token",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "ingest_transient_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "list_matches",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "mark_played",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "match_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "merge_items",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "modify_device",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "modify_playlist_generator",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "move_collection_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "move_hub",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "move_play_queue_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "move_playlist_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "optimize_database",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "post_users_sign_in_data",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "process_subscriptions",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "refresh_items_metadata",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "refresh_playlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "refresh_providers",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "refresh_section",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "refresh_sections_metadata",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "reload_guide",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "reorder_subscription",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "report",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "reset_play_queue",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "restart_processing_download_queue_items",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "scan",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_channelmap",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_device_preferences",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_dvr_preferences",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_item_artwork",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_item_preferences",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_preferences",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_rating",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_section_preferences",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_stream_offset",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "set_stream_selection",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "shuffle",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "split_item",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "start_analysis",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "start_bif_generation",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "start_task",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "start_tasks",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "terminate_session",
        destructive: true,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "trigger_fallback",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "tune_channel",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "unmatch",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "unscrobble",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "unshuffle",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "update_hub_visibility",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "update_item_artwork",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "update_items",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "update_playlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "upload_playlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "write_log",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Plex,
        operation: "write_message",
        destructive: false,
    },
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
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_autotagging",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_command",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_customfilter",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_customformat",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_delayprofile",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_downloadclient",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_downloadclient_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_downloadclient_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_downloadclient_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_exclusions",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_exclusions_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_history_failed_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_importlist",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_importlist_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_importlist_movie",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_importlist_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_importlist_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_indexer",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_indexer_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_indexer_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_indexer_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_login",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_manualimport",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_metadata",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_metadata_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_metadata_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_metadata_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_movie",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_movie_import",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_notification",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_notification_action_by_name",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_notification_test",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_notification_testall",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_qualityprofile",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_queue_grab_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_queue_grab_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_release",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_release_push",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_releaseprofile",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_remotepathmapping",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_rootfolder",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_system_backup_restore_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_system_backup_restore_upload",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_system_restart",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_system_shutdown",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "post_tag",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_autotagging_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_collection",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_collection_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_config_downloadclient_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_config_host_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_config_importlist_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_config_indexer_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_config_mediamanagement_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_config_metadata_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_config_naming_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_config_ui_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_customfilter_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_customformat_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_customformat_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_delayprofile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_delayprofile_reorder_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_downloadclient_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_downloadclient_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_exclusions_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_importlist_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_importlist_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_indexer_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_indexer_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_metadata_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_movie_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_movie_editor",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_moviefile_bulk",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_moviefile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_moviefile_editor",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_notification_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_qualitydefinition_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_qualitydefinition_update",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_qualityprofile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_releaseprofile_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_remotepathmapping_by_id",
        destructive: false,
    },
    SafetyRow {
        kind: ServiceKind::Radarr,
        operation: "put_tag_by_id",
        destructive: false,
    },
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

/// Find the authoritative safety classification for a known generated operation.
pub fn operation_safety(kind: ServiceKind, operation: &str) -> Option<OperationSafety> {
    let spec = find_operation(kind, operation)?;
    classify_operation(kind, spec).ok()
}

/// Classify one generated operation. Unknown generated writes fail closed.
pub fn classify_operation(
    kind: ServiceKind,
    spec: &OperationSpec,
) -> Result<OperationSafety, String> {
    match spec.method {
        HttpMethod::Get => Ok(OperationSafety::READ_ONLY),
        HttpMethod::Delete => Ok(OperationSafety::DESTRUCTIVE),
        HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch => AUDITED_WRITES
            .iter()
            .find(|row| row.kind == kind && row.operation == spec.name)
            .map(|row| {
                if row.destructive {
                    OperationSafety::DESTRUCTIVE
                } else {
                    OperationSafety::MUTATION
                }
            })
            .ok_or_else(|| {
                format!(
                    "unclassified generated write: kind={} operation={}",
                    kind.as_str(),
                    spec.name
                )
            }),
    }
}

/// Prove the audited table covers exactly every generated POST/PUT/PATCH.
pub fn validate_generated_write_classification() -> Result<(), String> {
    for row in AUDITED_WRITES {
        let spec = find_operation(row.kind, row.operation).ok_or_else(|| {
            format!(
                "stale generated write classification: kind={} operation={}",
                row.kind.as_str(),
                row.operation
            )
        })?;
        if !matches!(
            spec.method,
            HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch
        ) {
            return Err(format!(
                "non-write generated classification: kind={} operation={} method={}",
                row.kind.as_str(),
                row.operation,
                spec.method.as_str()
            ));
        }
        if AUDITED_WRITES
            .iter()
            .filter(|candidate| candidate.kind == row.kind && candidate.operation == row.operation)
            .count()
            != 1
        {
            return Err(format!(
                "duplicate generated write classification: kind={} operation={}",
                row.kind.as_str(),
                row.operation
            ));
        }
    }

    for kind in ServiceKind::ALL {
        for spec in operations_for_kind(kind) {
            if matches!(
                spec.method,
                HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch
            ) {
                classify_operation(kind, spec)?;
            }
        }
    }
    Ok(())
}
