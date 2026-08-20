//! `weaverail-model` のデータを扱うためのドメイン操作層。
//!
//! このクレートは「モデルそのもの」ではなく、`DiagramRoot` を入力にして
//! 追加・削除・検証・検索を行う操作を集約する役割を持つ。
//! すべての公開 API は、状態を持つルートを受け取り、結果を返す
//! ことを前提としているため、数学的な純粋関数というよりも
//! 「状態遷移を表すドメイン操作」として扱うのが正確である。

pub mod app;
pub mod command;
pub mod diagram_view_settings;
pub mod line;
pub mod line_segment;
pub mod station;
pub mod template_train;
pub mod timetable;
pub mod track;
pub mod train;
pub mod train_type;

pub use diagram_view_settings::validate_diagram_view_settings;
pub use line::{
    add_line, delete_line, find_segment_by_name, get_line_stations, get_segment,
    pop_back_line_segment, pop_front_line_segment, push_back_line_segment, push_front_line_segment,
    validate_line,
};
pub use line_segment::{add_segment, delete_segment, validate_segment};
pub use station::{add_station, delete_station, find_station_by_name};
pub use template_train::{
    add_template_train, delete_template_train, find_template_train_by_name, get_template_segments,
    pop_back_template_segment, pop_front_template_segment, push_back_template_segment,
    push_front_template_segment, validate_template_train,
};
pub use timetable::{add_timetable, delete_timetable, validate_timetable};
pub use track::{add_track, delete_track, station_of_track, validate_track};
pub use train::{add_train, delete_train, get_train_segment, get_train_stations, validate_train};
pub use train_type::{add_train_type, delete_train_type, find_train_type_by_name};

#[cfg(test)]
mod tests {
    use weaverail_model::{
        model::id::WeaverailId,
        model::{Station, station::StationId},
    };

    #[test]
    fn command_manager_handles_station_add_and_undo() {
        let mut manager =
            crate::command::CommandManager::new(Box::new(crate::command::EmptyEventEmitter));
        let station = Station::new(StationId::new(WeaverailId::new(1)), "梅田");

        manager.execute(Box::new(crate::command::station::AddStationCommand::new(
            station.clone(),
        )));
        assert_eq!(manager.root.stations.len(), 1);

        let result = manager.undo();
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert!(manager.root.stations.is_empty());
    }
}
