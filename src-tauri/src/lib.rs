pub mod command;

use std::sync::Mutex;

use tauri::Manager;
use weaverail_model::app::AppState;

use crate::command::TauriEmitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            app.manage(Mutex::new(AppState::new(Box::new(TauriEmitter::new(
                handle.clone(),
            )))));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::command::get_root,
            crate::command::redo,
            crate::command::undo,
            crate::command::redoable,
            crate::command::undoable,
            crate::command::station::new_station_id,
            crate::command::station::new_track_id,
            crate::command::station::add_station,
            crate::command::station::remove_station,
            crate::command::line::new_line_id,
            crate::command::line::add_line,
            crate::command::line::remove_line,
            crate::command::train_type::new_train_type_id,
            crate::command::train_type::add_train_type,
            crate::command::train_type::remove_train_type,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
