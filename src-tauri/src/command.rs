//! TauriのIPCで使用するコマンドを定義するモジュール

use std::{collections::HashMap, sync::Mutex};

use tauri::{AppHandle, Emitter};
use warp_rail::{warp_coords, warp_stations};
use weaverail_model::{
    app::AppState,
    command::{CommandError, EventEmitter},
    diagram_logical_coord::DiagramLogicalConvert,
    event::EmitEventType,
    model::{
        diagram_view_settings::DiagramViewSettingsId, line_segment::LineSegmentId, time::Time,
        timetable::TimetableId,
    },
    result_svg::ResultSvg,
    result_warp::{ResultWarpCoords, ResultWarpStations},
    result_weft::WeftTempStore,
};
use weft_rail::{make_node_diff, ripple_diff::ripple_node_diff, sort_diff};

pub mod data;
pub mod line;
pub mod station;
pub mod train_type;
pub mod weave;

pub struct TauriEmitter {
    handle: AppHandle,
}
impl TauriEmitter {
    pub fn new(handle: AppHandle) -> Self {
        Self { handle }
    }
}
impl EventEmitter for TauriEmitter {
    fn emit(&self, event: EmitEventType, payload: &str) {
        let _ = self
            .handle
            .emit_filter(&EmitEventType::to_string(&event), payload, |_| true);
    }
}

#[tauri::command]
pub async fn undo(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), CommandError> {
    let result = state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .undo();
    result.unwrap_or(Ok(()))
}

#[tauri::command]
pub async fn undoable(state: tauri::State<'_, Mutex<AppState>>) -> Result<bool, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .undoable())
}

#[tauri::command]
pub async fn redo(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), CommandError> {
    let result = state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .redo();
    result.unwrap_or(Ok(()))
}

#[tauri::command]
pub async fn redoable(state: tauri::State<'_, Mutex<AppState>>) -> Result<bool, CommandError> {
    Ok(state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .redoable())
}

#[tauri::command]
pub async fn get_svg(
    state: tauri::State<'_, Mutex<AppState>>,
    result_svg: tauri::State<'_, Mutex<Option<WeftTempStore>>>,
    timetable_id: TimetableId,
    view_settings_id: DiagramViewSettingsId,
    settings: DiagramLogicalConvert,
    start_time: Time,
    end_time: Time,
) -> Result<ResultSvg, CommandError> {
    let start = std::time::Instant::now();
    let root = &state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .command_manager
        .root;
    let result_svg = &mut result_svg
        .lock()
        .map_err(|_| CommandError::MutexLockError)?;
    if result_svg.is_some() {
        let cached = result_svg.as_ref().unwrap();
        if cached.version == root.version {
            let coords = warp_coords(root, view_settings_id)?;
            let result = warp_rail::get_svg(
                root,
                timetable_id,
                &cached.nodes,
                &cached.node_array,
                &cached.times,
                &coords,
                settings,
                start_time,
                end_time,
            )?;
            let duration = start.elapsed();
            println!("calc-svg (cached): {}us", duration.as_micros());

            return Ok(result);
        }
    }

    let nodes = make_node_diff::make_node(root, timetable_id)?;
    let node_array = sort_diff::sort_node(&nodes)?;
    let times: Vec<Time> = ripple_node_diff(&nodes, &node_array);

    let coords = warp_coords(root, view_settings_id)?;
    let result = warp_rail::get_svg(
        root,
        timetable_id,
        &nodes,
        &node_array,
        &times,
        &coords,
        settings,
        start_time,
        end_time,
    )?;

    **result_svg = Some(WeftTempStore {
        nodes,
        node_array,
        times,
        version: root.version,
    });

    let duration = start.elapsed();
    println!("calc-svg: {}us", duration.as_micros());

    Ok(result)
}

#[tauri::command]
pub async fn get_warp_coords(
    state: tauri::State<'_, Mutex<AppState>>,
    view_settings_id: DiagramViewSettingsId,
) -> Result<HashMap<LineSegmentId, ResultWarpCoords>, CommandError> {
    let coords = warp_coords(
        &state
            .lock()
            .map_err(|_| CommandError::MutexLockError)?
            .command_manager
            .root,
        view_settings_id,
    )?;
    Ok(coords)
}

#[tauri::command]
pub async fn get_warp_stations(
    state: tauri::State<'_, Mutex<AppState>>,
    view_settings_id: DiagramViewSettingsId,
) -> Result<Vec<ResultWarpStations>, CommandError> {
    let coords = warp_stations(
        &state
            .lock()
            .map_err(|_| CommandError::MutexLockError)?
            .command_manager
            .root,
        view_settings_id,
    )?;
    Ok(coords)
}
