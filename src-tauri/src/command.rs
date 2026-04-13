use std::sync::Mutex;

use tauri::{AppHandle, Emitter};
use weaverail_model::{
    app::AppState,
    command::{CommandError, EventEmitter},
    model::DiagramRoot,
};

pub mod line;
pub mod station;
pub mod train_type;

pub struct TauriEmitter {
    handle: AppHandle,
}
impl TauriEmitter {
    pub fn new(handle: AppHandle) -> Self {
        Self { handle }
    }
}
impl EventEmitter for TauriEmitter {
    fn emit(&self, event: &str, payload: &str) {
        let _ = self.handle.emit_filter(event, payload, |_| true);
    }
}

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
