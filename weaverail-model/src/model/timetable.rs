//! Weaverail上の「時刻表」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Timetable (時刻表)

use std::collections::{HashMap, hash_map::Entry};

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Timetable {
    /// 識別ID
    pub id: TimetableId,
    /// 時刻表名
    pub name: String,
    /// 駅間の列車順序 (順行 / 逆行を表す)
    pub segment_train_orders: HashMap<LineSegmentId, (SegmentTrainOrder, SegmentTrainOrder)>,
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
            .remove(&timetable_id)
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
                .get(&order.0.segment_id)
                .ok_or(ModelError::ObjectNotFound)?;
            let _ = self
                .segments
                .get(&order.1.segment_id)
                .ok_or(ModelError::ObjectNotFound)?;
            for forward in order.0.order.iter() {
                let _ = self.trains.get(forward).ok_or(ModelError::ObjectNotFound)?;
            }
            for forward in order.1.order.iter() {
                let _ = self.trains.get(forward).ok_or(ModelError::ObjectNotFound)?;
            }
        }
        Ok(())
    }
}
impl PropertiableObject for Timetable {
    fn get_property(&self, id: &str) -> Option<&Value> {
        self.properties.get(id)
    }

    fn set_property(&mut self, id: &str, value: Value) -> Option<Value> {
        self.properties.set(id, value)
    }

    fn remove_property(&mut self, id: &str) -> Option<Value> {
        self.properties.remove(id)
    }
}
