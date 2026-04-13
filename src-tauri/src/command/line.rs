use std::sync::Mutex;

use uuid::Uuid;
use weaverail_model::{
    app::AppState,
    command::line::{AddLineCommand, RemoveLineCommand},
    model::line::Line,
};

#[tauri::command]
pub async fn add_line(state: tauri::State<'_, Mutex<AppState>>, line: Line) -> Result<(), String> {
    let command = AddLineCommand::new(line.clone());
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}

#[tauri::command]
pub async fn remove_line(
    state: tauri::State<'_, Mutex<AppState>>,
    line_id: Uuid,
) -> Result<(), String> {
    let command = RemoveLineCommand::new(line_id);
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}
