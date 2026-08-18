pub mod command;

use std::sync::Mutex;

use tauri::Manager;
use weaverail_model::result_weft::WeftTempStore;
use weaverail_operation::app::AppState;

use crate::command::TauriEmitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            let mut app_state = AppState::new(Box::new(TauriEmitter::new(handle.clone())));
            app_state.command_manager =
                weaverail_testdata::diagram_root::get_test_data_shortly();
            app_state.command_manager.emitter = Box::new(TauriEmitter::new(handle.clone()));

            app.manage(Mutex::new(app_state));
            app.manage::<Mutex<Option<WeftTempStore>>>(Mutex::new(None));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::command::data::get_root,
            crate::command::data::get_stations,
            crate::command::data::get_tracks,
            crate::command::data::get_segments,
            crate::command::data::get_lines,
            crate::command::data::get_train_types,
            crate::command::data::get_template_trains,
            crate::command::data::get_timetables,
            crate::command::data::get_trains,
            crate::command::redo,
            crate::command::undo,
            crate::command::redoable,
            crate::command::undoable,
            crate::command::get_svg,
            crate::command::get_warp_coords,
            crate::command::get_warp_stations,
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
            crate::command::weave::weave,
            crate::command::weave::debug_insert_train
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
