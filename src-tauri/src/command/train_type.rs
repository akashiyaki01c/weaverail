use std::sync::Mutex;

use weaverail_model::model::train_type::{TrainType, TrainTypeId};
use weaverail_operation::{
    app::AppState,
    command::{
        CommandError,
        train_type::{AddTrainTypeCommand, RemoveTrainTypeCommand},
    },
};

#[tauri::command]
pub async fn new_train_type_id(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<TrainTypeId, CommandError> {
    let state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    Ok(TrainTypeId::new(
        state.command_manager.root.id_issuer.next(),
    ))
}

#[tauri::command]
pub async fn add_train_type(
    state: tauri::State<'_, Mutex<AppState>>,
    train_type: TrainType,
) -> Result<(), CommandError> {
    let command = AddTrainTypeCommand::new(train_type.clone());
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}

#[tauri::command]
pub async fn remove_train_type(
    state: tauri::State<'_, Mutex<AppState>>,
    train_type_id: TrainTypeId,
) -> Result<(), CommandError> {
    let command = RemoveTrainTypeCommand::new(train_type_id);
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}
