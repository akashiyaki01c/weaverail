//! Weaverail上の「列車種別」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - TrainType (列車種別)

use std::collections::hash_map::Entry;

use serde::{Deserialize, Serialize};

use crate::{
    command::CommandError, model::{DiagramRoot, ExtensionProperty}, weaverail_id
};

weaverail_id!(TrainTypeId, "TYPE");

/// Weaverail上の1つの「列車種別」を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TrainType {
    /// 識別ID
    pub id: TrainTypeId,
    /// 列車種別名 (例: "普通列車")
    pub name: String,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl TrainType {
    pub fn new(id: TrainTypeId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl DiagramRoot {
    /// 列車種別を追加する関数
    /// 既に同一IDの列車種別が存在している場合はエラーを返す
    pub fn add_train_type(&mut self, train_type: TrainType) -> Result<(), CommandError> {
        match self.train_types.entry(train_type.id) {
            Entry::Vacant(entry) => {
                entry.insert(train_type);
                Ok(())
            }
            Entry::Occupied(_) => Err(CommandError::DuplicateKey),
        }
    }

    /// 列車種別を削除する関数
    /// 指定IDの列車種別が存在しない場合はエラーを返す
    /// テンプレート列車から讃匠されている場合はエラーを返す
    pub fn delete_train_type(
        &mut self,
        train_type_id: TrainTypeId,
    ) -> Result<TrainType, CommandError> {
        if self
            .template_trains
            .values()
            .any(|train| train.train_type_id == train_type_id)
        {
            return Err(CommandError::ExternalReference);
        }

        self.train_types
            .remove(&train_type_id)
            .ok_or(CommandError::TargetObjectNotFound)
    }

    /// 列車種別名から列車種別を検索する関数
    /// 見つからない場合は None を返す
    pub fn find_train_type_by_name(&self, train_type_name: &str) -> Option<&TrainType> {
        self.train_types
            .values()
            .find(|train_type| train_type.name == train_type_name)
    }
}
