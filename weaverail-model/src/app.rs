//! アプリケーション上での状態保持に使用する構造体を定義するモジュール

use crate::command::{EventEmitter, command_manager::CommandManager};

/// アプリの状態を表すデータモデル
pub struct AppState {
    pub command_manager: CommandManager,
}

impl AppState {
    pub fn new(emitter: Box<dyn EventEmitter>) -> Self {
        Self {
            command_manager: CommandManager::new(emitter),
        }
    }
}
