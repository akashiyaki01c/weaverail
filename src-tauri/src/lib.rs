use std::sync::Mutex;

use tauri::Manager;
use weaverail_model::app::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            app.manage(Mutex::new(AppState::new(handle)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            weaverail_model::command::get_root,
            weaverail_model::command::redo,
            weaverail_model::command::undo,
            weaverail_model::command::redoable,
            weaverail_model::command::undoable,
            weaverail_model::command::station::add_station,
            weaverail_model::command::station::remove_station,
            weaverail_model::command::line::add_line,
            weaverail_model::command::line::remove_line,
            weaverail_model::command::train_type::add_train_type,
            weaverail_model::command::train_type::remove_train_type,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
