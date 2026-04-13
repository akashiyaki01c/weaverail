pub mod line;
pub mod station;
pub mod template_train;
pub mod time;
pub mod timetable;
pub mod train;
pub mod train_adjustment;
pub mod train_type;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::model::{
    line::Line, station::Station, template_train::TemplateTrain, timetable::Timetable,
    train_type::TrainType,
};

/// 拡張プロパティ
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ExtensionProperty(HashMap<String, Value>);
impl ExtensionProperty {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    /// 値を取得する
    pub fn get(&self, id: &str) -> Option<&Value> {
        self.0.get(id)
    }
    /// 値を設定する
    pub fn set(&mut self, id: &str, value: Value) -> Option<Value> {
        self.0.insert(id.to_string(), value)
    }
    /// 値を削除する
    pub fn remove(&mut self, id: &str) -> Option<Value> {
        self.0.remove(id)
    }
}

/// ダイヤグラムファイル
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct DiagramRoot {
    /// 駅の集合
    pub stations: HashMap<Uuid, Station>,
    /// 路線の集合
    pub lines: HashMap<Uuid, Line>,
    /// 列車種別の集合
    pub train_types: HashMap<Uuid, TrainType>,
    /// テンプレート列車の集合
    pub template_trains: HashMap<Uuid, TemplateTrain>,
    /// 時刻表の集合
    pub timetables: HashMap<Uuid, Timetable>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts_rs::{Config, TS};

    #[test]
    fn export_types() {
        let cfg = Config::new();
        DiagramRoot::export_all(&cfg).expect("TS export failed");
    }
}
