use std::{collections::HashMap, sync::Mutex};

use weaverail_model::{
    app::AppState,
    model::{
        DiagramRoot, line::{Line, LineId}, line_segment::{LineSegment, LineSegmentId}, station::{Station, StationId}, template_train::{TemplateTrain, TemplateTrainId}, timetable::{Timetable, TimetableId}, track::{Track, TrackId}, train::{Train, TrainId}, train_type::{TrainType, TrainTypeId}
    },
};

#[tauri::command]
pub async fn get_root(state: tauri::State<'_, Mutex<AppState>>) -> Result<DiagramRoot, String> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .root
        .clone())
}

#[tauri::command]
pub async fn get_stations(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<HashMap<StationId, Station>, String> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .root
        .stations
        .clone())
}

#[tauri::command]
pub async fn get_tracks(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<HashMap<TrackId, Track>, String> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .root
        .tracks
        .clone())
}

#[tauri::command]
pub async fn get_segments(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<HashMap<LineSegmentId, LineSegment>, String> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .root
        .segments
        .clone())
}

#[tauri::command]
pub async fn get_lines(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<HashMap<LineId, Line>, String> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .root
        .lines
        .clone())
}

#[tauri::command]
pub async fn get_train_types(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<HashMap<TrainTypeId, TrainType>, String> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .root
        .train_types
        .clone())
}

#[tauri::command]
pub async fn get_template_trains(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<HashMap<TemplateTrainId, TemplateTrain>, String> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .root
        .template_trains
        .clone())
}

#[tauri::command]
pub async fn get_timetables(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<HashMap<TimetableId, Timetable>, String> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .root
        .timetables
        .clone())
}

#[tauri::command]
pub async fn get_trains(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<HashMap<TrainId, Train>, String> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .root
        .trains
        .clone())
}

