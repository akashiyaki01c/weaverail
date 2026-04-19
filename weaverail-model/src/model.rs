//! Weaverail上で用いられる全てのデータ構造を定義するモジュール

pub mod id;
pub mod line;
pub mod segment_train_order;
pub mod station;
pub mod template_train;
pub mod time;
pub mod timetable;
pub mod train;
pub mod train_adjustment;
pub mod train_type;
pub mod track;
pub mod line_segment;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{id_issuer::IdIssuer, model::{
    line::{Line, LineId}, line_segment::{LineSegment, LineSegmentId}, station::{Station, StationId}, template_train::{TemplateTrain, TemplateTrainId}, timetable::{Timetable, TimetableId}, track::{Track, TrackId}, train_type::{TrainType, TrainTypeId}
}};

/// ユーザ定義で拡張が行える拡張プロパティを表す構造体
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

/// ダイヤグラムプロジェクトファイルを表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct DiagramRoot {
    /// 駅の集合
    pub stations: HashMap<StationId, Station>,
    /// 番線の集合
    pub tracks: HashMap<TrackId, Track>,
    /// 駅間の集合
    pub segments: HashMap<LineSegmentId, LineSegment>,
    /// 路線の集合
    pub lines: HashMap<LineId, Line>,
    /// 列車種別の集合
    pub train_types: HashMap<TrainTypeId, TrainType>,
    /// テンプレート列車の集合
    pub template_trains: HashMap<TemplateTrainId, TemplateTrain>,
    /// 時刻表の集合
    pub timetables: HashMap<TimetableId, Timetable>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
    /// ID発行
    pub id_issuer: IdIssuer,
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
