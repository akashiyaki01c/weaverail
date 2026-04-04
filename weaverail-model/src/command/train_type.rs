use std::sync::Mutex;

use tauri::{AppHandle, Emitter};
use uuid::Uuid;

use crate::{
    app::AppState,
    command::{Command, CommandError},
    model::{DiagramRoot, train_type::TrainType},
};

/// 列車種別の追加
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTrainTypeCommand {
    train_type: TrainType,
}
impl AddTrainTypeCommand {
    pub fn new(train_type: TrainType) -> Self {
        Self { train_type }
    }
}
impl Command for AddTrainTypeCommand {
    fn redo(&mut self, obj: &mut DiagramRoot, app: Option<&AppHandle>) -> Result<(), CommandError> {
        obj.add_train_type(self.train_type.clone())?;
        if let Some(app) = app {
            let _ = app.emit_filter("train_type_changed", &obj, |_| true);
        }
        Ok(())
    }

    fn undo(&mut self, obj: &mut DiagramRoot, app: Option<&AppHandle>) -> Result<(), CommandError> {
        obj.delete_train_type(self.train_type.id)?;

        if let Some(app) = app {
            let _ = app.emit_filter("train_type_changed", &obj, |_| true);
        }

        Ok(())
    }
}
#[tauri::command]
pub async fn add_train_type(
    state: tauri::State<'_, Mutex<AppState>>,
    train_type: TrainType,
) -> Result<(), String> {
    let command = AddTrainTypeCommand::new(train_type.clone());
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}

/// 駅の削除
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveTrainTypeCommand {
    train_type_id: Uuid,
    train_type: Option<TrainType>,
}
impl RemoveTrainTypeCommand {
    pub fn new(station_id: Uuid) -> Self {
        Self {
            train_type_id: station_id,
            train_type: None,
        }
    }
}
impl Command for RemoveTrainTypeCommand {
    fn redo(&mut self, obj: &mut DiagramRoot, app: Option<&AppHandle>) -> Result<(), CommandError> {
        let train_type = obj.delete_train_type(self.train_type_id)?;
        self.train_type = Some(train_type);

        if let Some(app) = app {
            let _ = app.emit_filter("train_type_changed", &obj, |_| true);
        }

        Ok(())
    }

    fn undo(&mut self, obj: &mut DiagramRoot, app: Option<&AppHandle>) -> Result<(), CommandError> {
        if let Some(train_type) = self.train_type.clone() {
            obj.add_train_type(train_type)?;
        }

        if let Some(app) = app {
            let _ = app.emit_filter("train_type_changed", &obj, |_| true);
        }

        Ok(())
    }
}
#[tauri::command]
pub async fn remove_train_type(
    state: tauri::State<'_, Mutex<AppState>>,
    train_type_id: Uuid,
) -> Result<(), String> {
    let command = RemoveTrainTypeCommand::new(train_type_id);
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}
