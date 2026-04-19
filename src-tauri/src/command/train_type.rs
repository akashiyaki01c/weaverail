use std::sync::Mutex;

use weaverail_model::{
    app::AppState,
    command::train_type::{AddTrainTypeCommand, RemoveTrainTypeCommand},
    model::train_type::{TrainType, TrainTypeId},
};

#[tauri::command]
pub async fn new_train_type_id(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<TrainTypeId, String> {
    let state = state.lock().expect("mutex lock error");
    Ok(TrainTypeId::new(
        state.command_manager.root.id_issuer.next(),
    ))
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

#[tauri::command]
pub async fn remove_train_type(
    state: tauri::State<'_, Mutex<AppState>>,
    train_type_id: TrainTypeId,
) -> Result<(), String> {
    let command = RemoveTrainTypeCommand::new(train_type_id);
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}
