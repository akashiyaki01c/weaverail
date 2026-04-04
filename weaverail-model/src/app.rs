use tauri::AppHandle;

use crate::command::command_manager::CommandManager;

/// アプリの状態
pub struct AppState {
    pub command_manager: CommandManager,
}

impl AppState {
    pub fn new(app_handle: &AppHandle) -> Self {
        Self {
            command_manager: CommandManager::new(Some(app_handle.clone())),
        }
    }
}
