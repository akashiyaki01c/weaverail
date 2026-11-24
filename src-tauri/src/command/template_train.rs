use std::sync::Mutex;

use weaverail_model::model::{
    template_train::{
        TemplateTrainId, TemplateTrainSection, TemplateTrainSegment,
        TemplateTrainSegmentId, TemplateTrainStation, TemplateTrainStationId,
    },
};
use weaverail_operation::{
    app::AppState,
    command::{
        CommandError,
        template_train_segment::{
            AddTemplateTrainSegmentCommand, PopBackTemplateTrainSegmentCommand,
            PopFrontTemplateTrainSegmentCommand, UpdateTemplateTrainSectionCommand,
        },
    },
};

#[tauri::command]
pub async fn new_template_train_segment_id(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<TemplateTrainSegmentId, CommandError> {
    let state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    Ok(TemplateTrainSegmentId::new(
        state.command_manager.root.id_issuer.next(),
    ))
}

#[tauri::command]
pub async fn new_template_train_station_id(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<TemplateTrainStationId, CommandError> {
    let state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    Ok(TemplateTrainStationId::new(
        state.command_manager.root.id_issuer.next(),
    ))
}

#[tauri::command]
pub async fn add_template_train_segment(
    state: tauri::State<'_, Mutex<AppState>>,
    template_train_id: TemplateTrainId,
    segment: TemplateTrainSegment,
    station: TemplateTrainStation,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    state.command_manager.execute(Box::new(
        AddTemplateTrainSegmentCommand::new(template_train_id, segment, station),
    ));
    Ok(())
}

#[tauri::command]
pub async fn pop_front_template_train_segment(
    state: tauri::State<'_, Mutex<AppState>>,
    template_train_id: TemplateTrainId,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    state.command_manager.execute(Box::new(
        PopFrontTemplateTrainSegmentCommand::new(template_train_id),
    ));
    Ok(())
}

#[tauri::command]
pub async fn pop_back_template_train_segment(
    state: tauri::State<'_, Mutex<AppState>>,
    template_train_id: TemplateTrainId,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    state.command_manager.execute(Box::new(
        PopBackTemplateTrainSegmentCommand::new(template_train_id),
    ));
    Ok(())
}

#[tauri::command]
pub async fn update_template_train_section(
    state: tauri::State<'_, Mutex<AppState>>,
    template_train_id: TemplateTrainId,
    index: usize,
    section: TemplateTrainSection,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    state.command_manager.execute(Box::new(
        UpdateTemplateTrainSectionCommand::new(template_train_id, index, section),
    ));
    Ok(())
}
