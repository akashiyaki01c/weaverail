use std::sync::Mutex;

use weaverail_model::{
    app::AppState, command::CommandError, model::timetable::TimetableId,
    result_weft::ResultWeftTrain,
};

#[tauri::command]
pub async fn weave(
    timetable_id: TimetableId,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<ResultWeftTrain>, CommandError> {
    let state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    Ok(weft_rail::weave(&state.command_manager.root, timetable_id)?)
}
