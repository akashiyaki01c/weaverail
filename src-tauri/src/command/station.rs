use std::sync::Mutex;

use weaverail_model::model::{
    station::{Station, StationId},
    track::{Track, TrackId},
};
use weaverail_operation::{
    app::AppState,
    command::{
        CommandError,
        station::RenameStationCommand,
        station::{AddStationCommand, RemoveStationCommand},
        track::{AddTrackCommand, RemoveTrackCommand, RenameTrackCommand},
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

#[tauri::command]
#[specta::specta]
pub async fn rename_station(
    state: tauri::State<'_, Mutex<AppState>>,
    station_id: StationId,
    name: String,
) -> Result<(), CommandError> {
    let command = RenameStationCommand::new(station_id, &name);
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    state.command_manager.execute(Box::new(command));
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn add_track(
    state: tauri::State<'_, Mutex<AppState>>,
    track: Track,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    state
        .command_manager
        .execute(Box::new(AddTrackCommand::new(track)));
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn remove_track(
    state: tauri::State<'_, Mutex<AppState>>,
    track_id: TrackId,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    state
        .command_manager
        .execute(Box::new(RemoveTrackCommand::new(track_id)));
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn rename_track(
    state: tauri::State<'_, Mutex<AppState>>,
    track_id: TrackId,
    name: String,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    state
        .command_manager
        .execute(Box::new(RenameTrackCommand::new(track_id, &name)));
    Ok(())
}
