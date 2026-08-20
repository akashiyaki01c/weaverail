use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, StationId, TemplateTrainSegment, Train, TrainId},
};

/// 列車を新規追加する。
///
/// 同一 ID がすでに存在する場合は `ModelError::DuplicateKey` を返す。
pub fn add_train(root: &mut DiagramRoot, train: Train) -> Result<(), ModelError> {
    root.add_train(train)
}

/// 指定した列車 ID を削除し、削除前の列車を返す。
pub fn delete_train(root: &mut DiagramRoot, train_id: TrainId) -> Result<Train, ModelError> {
    root.delete_train(train_id)
}

/// 列車が通過する駅 ID を順序付きで取得する。
pub fn get_train_stations<'a>(
    root: &'a DiagramRoot,
    train: &Train,
) -> Result<Vec<StationId>, ModelError> {
    root.get_train_stations(train)
}

/// 列車が通過するテンプレート駅間の一覧を取得する。
pub fn get_train_segment<'a>(
    root: &'a DiagramRoot,
    train: &Train,
) -> Result<Vec<TemplateTrainSegment>, ModelError> {
    root.get_train_segment(train)
}

/// 列車の参照整合性を検証する。
pub fn validate_train(root: &DiagramRoot, train_id: TrainId) -> Result<(), ModelError> {
    root.validate_train(train_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{id::WeaverailId, timetable::TimetableId};

    #[test]
    fn operation_add_and_delete_train_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let train_id = TrainId::new(WeaverailId::new(2));
        let train = Train::new(train_id, timetable_id);

        assert!(add_train(&mut root, train.clone()).is_ok());
        assert_eq!(root.trains.len(), 1);

        let removed = delete_train(&mut root, train_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().id, train_id);
    }

    #[test]
    fn operation_validate_train_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let train_id = TrainId::new(WeaverailId::new(2));
        let train = Train::new(train_id, timetable_id);
        add_train(&mut root, train).unwrap();

        assert!(validate_train(&root, train_id).is_ok());
        assert!(validate_train(&root, TrainId::new(WeaverailId::new(99))).is_err());
    }

    #[test]
    fn operation_get_train_segment_works() {
        let root = DiagramRoot::default();
        let train_id = TrainId::new(WeaverailId::new(2));
        let train = Train::new(train_id, TimetableId::new(WeaverailId::new(1)));

        let segments = get_train_segment(&root, &train);
        assert!(segments.is_ok());
        assert!(segments.unwrap().is_empty());
    }
}
