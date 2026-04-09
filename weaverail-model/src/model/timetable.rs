use std::collections::{HashMap, hash_map::Entry};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{command::CommandError, model::{DiagramRoot, ExtensionProperty, train::Train, train_adjustment::TrainsAdjustment}};

/// 一つの時刻表を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Timetable {
    /// 識別ID
    pub id: Uuid,
    /// 時刻表名
    pub name: String,
    /// 時刻表に含まれる列車
    pub trains: HashMap<Uuid, Train>,
    /// 時間調整
    pub adjustments: HashMap<Uuid, TrainsAdjustment>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Timetable {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
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
    pub fn delete_timetable(&mut self, timetable_id: Uuid) -> Result<Timetable, CommandError> {
        self.timetables.remove(&timetable_id).ok_or(CommandError::TargetObjectNotFound)
    }
}