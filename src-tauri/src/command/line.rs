use std::sync::Mutex;

use weaverail_model::{
    app::AppState,
    command::line::{AddLineCommand, RemoveLineCommand},
    model::line::{Line, LineId},
};

#[tauri::command]
pub async fn new_line_id() -> Result<LineId, String> {
    Ok(LineId::new())
}

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
    line_id: LineId,
) -> Result<(), String> {
    let command = RemoveLineCommand::new(line_id);
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}
