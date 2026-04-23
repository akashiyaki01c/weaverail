use std::sync::Mutex;

use weaverail_model::{app::AppState, model::timetable::TimetableId, result_weft::ResultWeftTrain};

#[tauri::command]
pub async fn weave(
    timetable_id: TimetableId,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<ResultWeftTrain>, String> {
    let state = state.lock().expect("mutex lock error");
    Ok(weft_rail::weave(&state.command_manager.root, timetable_id))
}
