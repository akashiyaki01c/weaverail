use crate::command::{EventEmitter, command_manager::CommandManager};

/// アプリの状態
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
