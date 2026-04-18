mod commands;

use commands::AppState;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::get_timeline,
            commands::create_new_timeline,
            commands::add_track_cmd,
            commands::remove_track_cmd,
            commands::add_clip_cmd,
            commands::remove_clip_cmd,
            commands::split_clip_cmd,
            commands::probe_media,
            commands::export_media,
            commands::undo_cmd,
            commands::redo_cmd,
            commands::read_file_as_data_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
