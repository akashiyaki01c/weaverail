use std::{collections::HashMap, sync::Mutex};

use tauri::{AppHandle, Emitter};
use warp_rail::warp_coords;
use weaverail_model::{
    app::AppState,
    command::{CommandError, EventEmitter},
    diagram_logical_coord::DiagramLogicalConvert,
    event::EmitEventType,
    model::{
        diagram_view_settings::DiagramViewSettings, time::Time,
        timetable::TimetableId, train::TrainId,
    },
    result_svg::ResultSvg,
};
use weft_rail::{
    WeftNode,
    make_node::{get_node_by_nodeid, make_node},
    ripple::ripple_time,
    sort::sort_node,
};

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
        .expect("mutex lock error")
        .command_manager
        .undo();
    result.unwrap_or(Ok(()))
}

#[tauri::command]
pub async fn undoable(state: tauri::State<'_, Mutex<AppState>>) -> Result<bool, CommandError> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .undoable())
}

#[tauri::command]
pub async fn redo(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), CommandError> {
    let result = state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .redo();
    result.unwrap_or(Ok(()))
}

#[tauri::command]
pub async fn redoable(state: tauri::State<'_, Mutex<AppState>>) -> Result<bool, CommandError> {
    Ok(state
        .lock()
        .expect("mutex lock error")
        .command_manager
        .redoable())
}

#[tauri::command]
pub async fn get_svg(
    state: tauri::State<'_, Mutex<AppState>>,
    timetable_id: TimetableId,
    view_settings: DiagramViewSettings,
    settings: DiagramLogicalConvert,
    start_time: Time,
    end_time: Time,
) -> Result<ResultSvg, CommandError> {
    let root = &state.lock().expect("mutex lock error").command_manager.root;
    let nodes: (WeftNode, HashMap<TrainId, Vec<WeftNode>>) = make_node(root, timetable_id);
    let converted_nodes: Vec<&WeftNode> = get_node_by_nodeid(&nodes.0, &nodes.1);
    let node_array: Vec<&WeftNode> = sort_node(&converted_nodes);
    let times: Vec<Time> = ripple_time(&node_array);

    let coords = warp_coords(&root, &view_settings);

    Ok(warp_rail::get_svg(
        root,
        timetable_id,
        &converted_nodes,
        &times,
        &coords,
        settings,
        start_time,
        end_time
    ))
}
