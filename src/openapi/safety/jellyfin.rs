use crate::config::ServiceKind;

use super::SafetyRow;

pub(super) const ROWS: &[SafetyRow] = &[
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
];

#[cfg(test)]
#[path = "jellyfin_tests.rs"]
mod tests;
