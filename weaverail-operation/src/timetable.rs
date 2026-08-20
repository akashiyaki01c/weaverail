use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, Timetable, TimetableId},
};

/// 時刻表を追加する。
///
/// 既存の ID と衝突した場合は `ModelError::DuplicateKey` を返す。
pub fn add_timetable(root: &mut DiagramRoot, timetable: Timetable) -> Result<(), ModelError> {
    root.add_timetable(timetable)
}

/// 指定した時刻表 ID を削除して、削除前の時刻表を返す。
pub fn delete_timetable(
    root: &mut DiagramRoot,
    timetable_id: TimetableId,
) -> Result<Timetable, ModelError> {
    root.delete_timetable(timetable_id)
}

/// 時刻表に含まれる列車順序と駅間参照が整合しているか検証する。
pub fn validate_timetable(root: &DiagramRoot, timetable_id: TimetableId) -> Result<(), ModelError> {
    root.validate_timetable(timetable_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::id::WeaverailId;

    #[test]
    fn operation_add_and_delete_timetable_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let id = TimetableId::new(WeaverailId::new(1));
        let timetable = Timetable::new(id, "平日時刻表");

        assert!(add_timetable(&mut root, timetable.clone()).is_ok());
        assert_eq!(root.timetables.len(), 1);

        let removed = delete_timetable(&mut root, id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().name, "平日時刻表");
    }

    #[test]
    fn operation_validate_timetable_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let id = TimetableId::new(WeaverailId::new(1));
        let timetable = Timetable::new(id, "平日時刻表");
        add_timetable(&mut root, timetable).unwrap();

        assert!(validate_timetable(&root, id).is_ok());
        assert!(validate_timetable(&root, TimetableId::new(WeaverailId::new(99))).is_err());
    }
}
