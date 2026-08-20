use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, TrainType, TrainTypeId},
};

/// 列車種別を新規登録する。
///
/// すでに同じ ID が存在する場合は `ModelError::DuplicateKey` を返す。
pub fn add_train_type(root: &mut DiagramRoot, train_type: TrainType) -> Result<(), ModelError> {
    root.add_train_type(train_type)
}

/// 指定 ID の列車種別を削除し、削除前のデータを返す。
pub fn delete_train_type(
    root: &mut DiagramRoot,
    train_type_id: TrainTypeId,
) -> Result<TrainType, ModelError> {
    root.delete_train_type(train_type_id)
}

/// 列車種別名で検索する。
///
/// `None` を返すのは見つからない場合のみで、検索結果は `&TrainType` の参照として返す。
pub fn find_train_type_by_name<'a>(root: &'a DiagramRoot, name: &str) -> Option<&'a TrainType> {
    root.find_train_type_by_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::id::WeaverailId;

    #[test]
    fn operation_add_and_delete_train_type_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let id = TrainTypeId::new(WeaverailId::new(1));
        let train_type = TrainType::new(id, "普通");

        assert!(add_train_type(&mut root, train_type.clone()).is_ok());
        assert_eq!(root.train_types.len(), 1);

        let removed = delete_train_type(&mut root, id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().name, "普通");
    }

    #[test]
    fn operation_find_train_type_by_name_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        root.add_train_type(TrainType::new(
            TrainTypeId::new(WeaverailId::new(1)),
            "普通",
        ))
        .unwrap();
        root.add_train_type(TrainType::new(
            TrainTypeId::new(WeaverailId::new(2)),
            "快速",
        ))
        .unwrap();

        assert_eq!(find_train_type_by_name(&root, "普通").unwrap().name, "普通");
        assert!(find_train_type_by_name(&root, "特急").is_none());
    }
}
