pub mod command_manager;
pub mod line;
pub mod station;
pub mod train_type;

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::{app::AppState, model::DiagramRoot};

/// モデルに対する「操作」を表すトレイト
pub trait Command: Send + Sync {
    /// やり直す動作
    fn redo(&mut self, obj: &mut DiagramRoot, app: Option<&AppHandle>) -> Result<(), CommandError>;
    /// 元に戻す動作
    fn undo(&mut self, obj: &mut DiagramRoot, app: Option<&AppHandle>) -> Result<(), CommandError>;
}

/// コマンドのエラー一覧
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CommandError {
    /// 対象オブジェクトが見つからない
    TargetObjectNotFound,
    /// オブジェクトのキーが重複している
    DuplicateKey,
    /// 外部参照されている
    ExternalReference,
    /// 範囲外の要素を参照した
    IndexOutOfBounds,
    /// 整合性がない
    Inconsistent,
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

#[test]
fn check_env() {
    println!("TS_RS_EXPORT_DIR: {:?}", std::env::var("TS_RS_EXPORT_DIR"));
}
