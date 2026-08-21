use std::sync::Mutex;

use weaverail_model::model::line::{Line, LineId};
use weaverail_operation::{
    app::AppState,
    command::{
        CommandError,
        line::{AddLineCommand, RemoveLineCommand},
    },
};

#[tauri::command]
#[specta::specta]
pub async fn new_line_id(state: tauri::State<'_, Mutex<AppState>>) -> Result<LineId, CommandError> {
    let state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    Ok(LineId::new(state.command_manager.root.id_issuer.next()))
}

#[tauri::command]
#[specta::specta]
pub async fn add_line(
    state: tauri::State<'_, Mutex<AppState>>,
    line: Line,
) -> Result<(), CommandError> {
    let command = AddLineCommand::new(line.clone());
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn remove_line(
    state: tauri::State<'_, Mutex<AppState>>,
    line_id: LineId,
) -> Result<(), CommandError> {
    let command = RemoveLineCommand::new(line_id);
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}
