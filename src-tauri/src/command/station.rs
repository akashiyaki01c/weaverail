use std::sync::Mutex;

use weaverail_model::{
    app::AppState,
    command::station::{AddStationCommand, RemoveStationCommand},
    model::station::{Station, StationId, TrackId},
};

#[tauri::command]
pub async fn new_station_id() -> Result<StationId, String> {
    Ok(StationId::new())
}

#[tauri::command]
pub async fn new_track_id() -> Result<TrackId, String> {
    Ok(TrackId::new())
}

#[tauri::command]
pub async fn add_station(
    state: tauri::State<'_, Mutex<AppState>>,
    station: Station,
) -> Result<(), String> {
    let command = AddStationCommand::new(station.clone());
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}

#[tauri::command]
pub async fn remove_station(
    state: tauri::State<'_, Mutex<AppState>>,
    station_id: StationId,
) -> Result<(), String> {
    let command = RemoveStationCommand::new(station_id);
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}
