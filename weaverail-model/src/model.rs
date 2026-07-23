//! Weaverail上で用いられる全てのデータ構造を定義するモジュール

pub mod diagram_view_settings;
pub mod id;
pub mod line;
pub mod line_segment;
pub mod segment_train_order;
pub mod station;
pub mod template_train;
pub mod time;
pub mod timetable;
pub mod track;
pub mod train;
pub mod train_type;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::ModelError,
    id_issuer::IdIssuer,
    model::{
        diagram_view_settings::{DiagramViewSettings, DiagramViewSettingsId},
        line::{Line, LineId},
        line_segment::{LineSegment, LineSegmentId},
        station::{Station, StationId},
        template_train::{TemplateTrain, TemplateTrainId},
        timetable::{Timetable, TimetableId},
        track::{Track, TrackId},
        train::{Train, TrainId},
        train_type::{TrainType, TrainTypeId},
    },
};

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

/// 拡張プロパティを保持する構造体を表すトレイト
pub trait PropertiableObject {
    /// 拡張プロパティの値を取得する
    fn get_property(&self, id: &str) -> Option<&Value>;
    /// 拡張プロパティの値を設定する
    fn set_property(&mut self, id: &str, value: Value) -> Option<Value>;
    /// 拡張プロパティの値を削除する
    fn remove_property(&mut self, id: &str) -> Option<Value>;
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
    /// 列車の集合
    pub trains: HashMap<TrainId, Train>,
    /// ダイヤグラムの表示設定の集合
    pub diagram_view_settings: HashMap<DiagramViewSettingsId, DiagramViewSettings>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
    /// ID発行
    pub id_issuer: IdIssuer,
    /// バージョン
    pub version: u32,
}
impl DiagramRoot {
    /// データが正常な値であるかを検証する
    pub fn validate(&self) -> Result<(), ModelError> {
        for sta in self.stations.keys() {
            let _ = self.validate_station(*sta)?;
        }
        for track in self.tracks.keys() {
            let _ = self.validate_track(*track)?;
        }
        for segment in self.segments.keys() {
            let _ = self.validate_segment(*segment)?;
        }
        for line in self.lines.keys() {
            let _ = self.validate_line(*line)?;
        }
        for train_type in self.train_types.keys() {
            let _ = self.validate_train_type(*train_type)?;
        }
        for template_train in self.template_trains.keys() {
            let _ = self.validate_template_train(*template_train)?;
        }
        for timetable in self.timetables.keys() {
            let _ = self.validate_timetable(*timetable)?;
        }
        for train in self.trains.keys() {
            let _ = self.validate_train(*train)?;
        }
        for diagram_view_setting in self.diagram_view_settings.keys() {
            let _ = self.validate_diagram_view_settings(*diagram_view_setting)?;
        }

        Ok(())
    }
}
impl PropertiableObject for DiagramRoot {
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

#[cfg(test)]
mod tests {
    use crate::result_weft::ResultWeftTrain;

    use super::*;
    use ts_rs::{Config, TS};

    #[test]
    fn export_types() {
        let cfg = Config::new();
        DiagramRoot::export_all(&cfg).expect("TS export failed");
        ResultWeftTrain::export_all(&cfg).expect("TS export failed");
    }
}
