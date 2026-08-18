//! Weaverail上の「時刻表」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Timetable (時刻表)

use indexmap::{IndexMap, map::Entry};

use serde::{Deserialize, Serialize};

use crate::path::Heddle;
use crate::{
    command::CommandError,
    error::ModelError,
    model::{
        DiagramRoot, ExtensionProperty, PropertiableObject, line_segment::LineSegmentId,
        segment_train_order::SegmentTrainOrder,
    },
    weaverail_id,
};

weaverail_id!(TimetableId, "TBL_");

/// Weaverail上の1つの時刻表を表す構造体
#[derive(
    weaverail_object::RnaObjectable,
    ts_rs::TS,
    Clone,
    PartialEq,
    Debug,
    Default,
    Serialize,
    Deserialize,
)]
pub struct Timetable {
    /// 識別ID
    pub id: TimetableId,
    /// 時刻表名
    pub name: String,
    /// 駅間の列車順序 (順行 / 逆行を表す)
    pub segment_train_orders: IndexMap<LineSegmentId, SegmentTrainOrders>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Timetable {
    pub fn new(id: TimetableId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            ..Default::default()
        }
    }
}
impl DiagramRoot {
    /// 時刻表を追加する関数
    /// 既に同一IDの時刻表が存在している場合はエラーを返す
    pub fn add_timetable(&mut self, timetable: Timetable) -> Result<(), CommandError> {
        match self.timetables.entry(timetable.id) {
            Entry::Vacant(entry) => {
                entry.insert(timetable);
                Ok(())
            }
            Entry::Occupied(_) => Err(CommandError::DuplicateKey),
        }
    }

    /// 時刻表を削除する関数
    /// 指定IDの時刻表が存在しない場合はエラーを返す
    pub fn delete_timetable(
        &mut self,
        timetable_id: TimetableId,
    ) -> Result<Timetable, CommandError> {
        self.timetables
            .shift_remove(&timetable_id)
            .ok_or(CommandError::TargetObjectNotFound)
    }

    /// 時刻表データが正常な値であるかを検証する
    pub fn validate_timetable(&self, timetable_id: TimetableId) -> Result<(), ModelError> {
        let timetable = self
            .timetables
            .get(&timetable_id)
            .ok_or(ModelError::ObjectNotFound)?;
        for order in timetable.segment_train_orders.values() {
            let _ = self
                .segments
                .get(&order.prograde.segment_id)
                .ok_or(ModelError::ObjectNotFound)?;
            let _ = self
                .segments
                .get(&order.retrograde.segment_id)
                .ok_or(ModelError::ObjectNotFound)?;
            for forward in order.prograde.order.iter() {
                let _ = self.trains.get(forward).ok_or(ModelError::ObjectNotFound)?;
            }
            for forward in order.retrograde.order.iter() {
                let _ = self.trains.get(forward).ok_or(ModelError::ObjectNotFound)?;
            }
        }
        Ok(())
    }
}
impl PropertiableObject for Timetable {
    fn get_property(&self, id: &str) -> Option<&Heddle> {
        self.properties.get(id)
    }

    fn set_property(&mut self, id: &str, value: Heddle) -> Option<Heddle> {
        self.properties.set(id, value)
    }

    fn remove_property(&mut self, id: &str) -> Option<Heddle> {
        self.properties.remove(id)
    }
}

#[derive(
    weaverail_object::RnaObjectable,
    ts_rs::TS,
    Clone,
    PartialEq,
    Debug,
    Default,
    Serialize,
    Deserialize,
)]
pub struct SegmentTrainOrders {
    /// 順行列車
    pub prograde: SegmentTrainOrder,
    /// 逆行列車
    pub retrograde: SegmentTrainOrder,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::id::WeaverailId;

    /// Timetable の生成と基本プロパティが正しく設定されることをテスト
    #[test]
    fn test_timetable_creation() {
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let timetable = Timetable::new(timetable_id, "平日時刻表");

        assert_eq!(timetable.id, timetable_id);
        assert_eq!(timetable.name, "平日時刻表");
        assert_eq!(timetable.segment_train_orders.len(), 0);
        assert_eq!(timetable.properties, ExtensionProperty::new());
    }

    /// Timetable の名前が正しく設定・変更されることをテスト
    #[test]
    fn test_timetable_name_change() {
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let mut timetable = Timetable::new(timetable_id, "平日時刻表");

        assert_eq!(timetable.name, "平日時刻表");

        timetable.name = "休日時刻表".to_string();
        assert_eq!(timetable.name, "休日時刻表");
    }

    /// DiagramRoot に Timetable を追加・削除できることをテスト
    #[test]
    fn test_add_and_delete_timetable() {
        let mut root = DiagramRoot::default();
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let timetable = Timetable::new(timetable_id, "平日時刻表");

        // 追加テスト
        assert!(root.add_timetable(timetable.clone()).is_ok());
        assert_eq!(root.timetables.len(), 1);
        assert_eq!(
            root.timetables.get(&timetable_id).unwrap().name,
            "平日時刻表"
        );

        // 削除テスト
        let removed_timetable = root.delete_timetable(timetable_id);
        assert!(removed_timetable.is_ok());
        assert_eq!(removed_timetable.unwrap().name, "平日時刻表");
        assert_eq!(root.timetables.len(), 0);
    }

    /// 同一IDの Timetable を2つ追加しようとするとエラーになることをテスト
    #[test]
    fn test_duplicate_timetable_id_error() {
        let mut root = DiagramRoot::default();
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let timetable1 = Timetable::new(timetable_id, "平日時刻表");
        let timetable2 = Timetable::new(timetable_id, "休日時刻表");

        assert!(root.add_timetable(timetable1).is_ok());

        // 同一IDで追加しようとする
        let result = root.add_timetable(timetable2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CommandError::DuplicateKey);
        assert_eq!(root.timetables.len(), 1);
    }

    /// 存在しない Timetable ID を削除しようとするとエラーになることをテスト
    #[test]
    fn test_delete_nonexistent_timetable() {
        let mut root = DiagramRoot::default();
        let timetable_id = TimetableId::new(WeaverailId::new(1));

        let result = root.delete_timetable(timetable_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CommandError::TargetObjectNotFound);
    }

    /// Timetable の拡張プロパティを取得・設定・削除できることをテスト
    #[test]
    fn test_timetable_properties() {
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let mut timetable = Timetable::new(timetable_id, "平日時刻表");

        // プロパティを設定
        let value = Heddle::String("weekday".to_string());
        let result = timetable.set_property("type", value.clone());
        assert!(result.is_none());

        // プロパティを取得
        let retrieved = timetable.get_property("type");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), &value);

        // プロパティを削除
        let removed = timetable.remove_property("type");
        assert!(removed.is_some());
        assert!(timetable.get_property("type").is_none());
    }

    /// 複数の Timetable を追加・管理できることをテスト
    #[test]
    fn test_multiple_timetables() {
        let mut root = DiagramRoot::default();
        let mut timetable_ids = Vec::new();

        // 5つの時刻表を追加
        for i in 1..=5 {
            let timetable_id = TimetableId::new(WeaverailId::new(i));
            let timetable = Timetable::new(timetable_id, &format!("時刻表{}", i));
            timetable_ids.push(timetable_id);
            assert!(root.add_timetable(timetable).is_ok());
        }

        assert_eq!(root.timetables.len(), 5);

        // 全時刻表を削除
        for timetable_id in timetable_ids {
            assert!(root.delete_timetable(timetable_id).is_ok());
        }

        assert_eq!(root.timetables.len(), 0);
    }
}
