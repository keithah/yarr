use crate::config::ServiceKind;

use super::SafetyRow;

pub(super) const ROWS: &[SafetyRow] = &[
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
];

#[cfg(test)]
#[path = "plex_tests.rs"]
mod tests;
