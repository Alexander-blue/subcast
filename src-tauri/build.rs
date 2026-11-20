const COMMANDS: &[&str] = &["add_subscription", "refresh_show", "refresh_all", "list_shows", "list_episodes", "recent_episodes", "get_episode", "remove_show", "set_progress", "set_watched", "mark_show_watched", "search_podcasts", "search_show", "load_transcript", "fetch_official_transcript", "transcribe", "cancel_job", "asr_models", "download_model", "delete_model", "get_blob", "set_blob", "get_settings", "set_settings", "set_api_key", "has_api_key", "llm_chat", "translate_texts", "shared_transcript", "shared_exists", "shared_guide", "share_guide", "asr_elsewhere", "log_listen", "listening_stats", "rescan_models", "youtube_subscriptions", "open_privacy_settings", "export_opml", "local_agents", "probe_agent", "agent_models", "warm_agent", "share_translation", "ui_ready", "ui_check", "refresh_config", "youtube_stream", "prepare_youtube", "youtube_sign_in", "youtube_signed_in", "youtube_sign_out", "youtube_tracks", "use_youtube_track", "qiaomu_detail", "preview_show", "list_models", "tts_speak", "tts_synthesize", "tts_system_voices", "clear_tts_cache", "local_tts_status", "local_tts_install", "local_tts_delete", "cached_show", "save_image", "learn_clip", "save_text", "export_anki", "set_menu_labels", "download_episode", "list_downloads", "delete_download", "clear_downloads", "align_audio", "save_time_map", "list_audio_cache", "clear_audio_cache", "storage_info", "open_data_folder", "reveal_download", "clear_temp", "full_notes", "app_update_check", "app_update_download", "app_update_install", "shadow_start", "shadow_stop", "shadow_clear", "shadow_ready", "shadow_hear", "open_mic_settings", "video_clip_env", "video_clip_warm", "video_clip_prepare", "video_clip_render", "reveal_file"];

fn main() {
    // On Windows the Common-Controls manifest is linked into every target, not only the app as tauri-build does,
    // or the `cargo test` binary dies at start with STATUS_ENTRYPOINT_NOT_FOUND.
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    let mut attributes = tauri_build::Attributes::new().app_manifest(tauri_build::AppManifest::new().commands(COMMANDS));
    if msvc {
        attributes = attributes.windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest());
        let manifest = std::env::current_dir().unwrap().join("windows-app-manifest.xml");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
    }
    tauri_build::try_build(attributes).expect("failed to run tauri-build");
}
