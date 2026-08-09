use std::sync::Mutex;

use indexmap::IndexMap;
use weaverail_model::{app::AppState, command::CommandError, model::*};

#[tauri::command]
pub async fn get_root(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<DiagramRoot, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root
        .clone())
}

#[tauri::command]
pub async fn get_stations(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<IndexMap<StationId, Station>, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root
        .stations
        .clone())
}

#[tauri::command]
pub async fn get_tracks(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<IndexMap<TrackId, Track>, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root
        .tracks
        .clone())
}

#[tauri::command]
pub async fn get_segments(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<IndexMap<LineSegmentId, LineSegment>, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root
        .segments
        .clone())
}

#[tauri::command]
pub async fn get_lines(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<IndexMap<LineId, Line>, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root
        .lines
        .clone())
}

#[tauri::command]
pub async fn get_train_types(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<IndexMap<TrainTypeId, TrainType>, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root
        .train_types
        .clone())
}

#[tauri::command]
pub async fn get_template_trains(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<IndexMap<TemplateTrainId, TemplateTrain>, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root
        .template_trains
        .clone())
}

#[tauri::command]
pub async fn get_timetables(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<IndexMap<TimetableId, Timetable>, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root
        .timetables
        .clone())
}

#[tauri::command]
pub async fn get_trains(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<IndexMap<TrainId, Train>, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root
        .trains
        .clone())
}
