use std::sync::Mutex;

use tauri::{AppHandle, Emitter};
use weaverail_model::{
    app::AppState,
    command::CommandError,
    event::EmitEventType,
    model::{
        time::Time,
        timetable::TimetableId,
        train::{TemplateSegment, Train, TrainId},
    },
    result_weft::ResultWeftTrain,
};
use weft_rail::shuutle::insert_train_order;

#[tauri::command]
pub async fn weave(
    timetable_id: TimetableId,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<ResultWeftTrain>, CommandError> {
    let state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    Ok(weft_rail::weave(&state.command_manager.root, timetable_id)?)
}

#[tauri::command]
pub async fn debug_insert_train(
    starting_departure_time: Time,
    timetable_id: TimetableId,
    state: tauri::State<'_, Mutex<AppState>>,
    handle: AppHandle,
) -> Result<(), CommandError> {
    let start = std::time::Instant::now();

    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    let root = &mut state.command_manager.root;

    let mut add_train = Train::new(TrainId(root.id_issuer.next()), timetable_id);
    add_train.template_segments = vec![TemplateSegment {
        template_train_id: root
            .find_template_train_by_name("神姫線下り-普通")
            .unwrap()
            .id,
        start_station_id: root.find_station_by_name("湊川").unwrap().id,
        end_station_id: root.find_station_by_name("姫路").unwrap().id,
    }];
    add_train.start_departure_time = starting_departure_time;

    let _ = &root.trains.insert(add_train.id, add_train.clone());

    let _ = insert_train_order(root, timetable_id, add_train.id);

    root.version += 1;

    let _ = handle.emit(EmitEventType::TrainAdded.into(), ());

    let duration = start.elapsed();
    println!("insert-train {:?}us", duration.as_micros());

    Ok(())
}
