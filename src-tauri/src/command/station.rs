use std::sync::Mutex;

use weaverail_model::model::{
    station::{Station, StationId},
    track::TrackId,
};
use weaverail_operation::{
    app::AppState,
    command::{
        CommandError,
        station::{AddStationCommand, RemoveStationCommand},
    },
};

#[tauri::command]
#[specta::specta]
pub async fn new_station_id(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<StationId, CommandError> {
    let state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    Ok(StationId::new(state.command_manager.root.id_issuer.next()))
}

#[tauri::command]
#[specta::specta]
pub async fn new_track_id(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<TrackId, CommandError> {
    let state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    Ok(TrackId::new(state.command_manager.root.id_issuer.next()))
}

#[tauri::command]
#[specta::specta]
pub async fn add_station(
    state: tauri::State<'_, Mutex<AppState>>,
    station: Station,
) -> Result<(), CommandError> {
    let command = AddStationCommand::new(station.clone());
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn remove_station(
    state: tauri::State<'_, Mutex<AppState>>,
    station_id: StationId,
) -> Result<(), CommandError> {
    let command = RemoveStationCommand::new(station_id);
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}
