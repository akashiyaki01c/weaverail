use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{DiagramRoot, ExtensionProperty, line::Line, time::Time};

/// 駅時刻の計算結果を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultStationTime {
    /// 駅ID
    pub station_id: Uuid,
    /// 到着時刻
    pub arrival_time: Option<Time>,
    /// 出発時刻
    pub departure_time: Option<Time>,
    /// 停車種別
    pub stop_type: ResultStationStopType,
}

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ResultStationStopType {
    /// 停車
    Stop,
    /// 通過
    Pass,
}
impl Default for ResultStationStopType {
    fn default() -> Self {
        Self::Stop
    }
}

impl ResultStationTime {
    pub fn new(
        station_id: Uuid,
        arrival_time: Option<Time>,
        departure_time: Option<Time>,
        stop_type: ResultStationStopType,
    ) -> Self {
        Self {
            station_id,
            arrival_time,
            departure_time,
            stop_type,
        }
    }
}

/// 一つのテンプレート列車を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TemplateTrain {
    /// 識別ID
    pub id: Uuid,
    /// テンプレート列車名
    pub name: String,
    /// 列車種別ID
    pub train_type_id: Uuid,
    pub start_station: TemplateTrainStation,
    /// 駅間情報群
    pub segments: Vec<(TemplateTrainSegment, TemplateTrainStation)>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl TemplateTrain {
    /// テンプレート列車が指定駅間を参照しているか
    pub fn contains_segment(&self, segment_id: Uuid) -> bool {
        self.segments
            .iter()
            .any(|segment| segment.0.segment_id == segment_id)
    }

    /// テンプレート列車が指定路線を参照しているか
    pub fn contains_line(&self, line: &Line) -> bool {
        line.segments
            .iter()
            .any(|segment| self.contains_segment(segment.id))
    }

    /// 全ての駅を取得する関数
    pub fn get_stations(&self) -> Vec<&TemplateTrainStation> {
        std::iter::once(&self.start_station)
            .chain(self.segments.iter().map(|segment| &segment.1))
            .collect()
    }

    /// 指定の駅が含まれているか
    pub fn contains_station(&self, station_id: Uuid) -> bool {
        let stations = self.get_stations();
        return stations
            .iter()
            .any(|station| station.station_id == station_id);
    }

    /// 指定の駅が何番目にあるか
    pub fn get_station_index(&self, station_id: Uuid) -> usize {
        let stations = self.get_stations();
        return stations
            .iter()
            .position(|station| station.station_id == station_id)
            .expect("Station ID not found in template train");
    }
}
impl DiagramRoot {
    /// テンプレート列車名からテンプレート列車を検索する関数
    /// 見つからない場合は None を返す
    pub fn find_template_train_by_name(&self, template_train_name: &str) -> Option<&TemplateTrain> {
        self.template_trains.values().find(|template_train| &template_train.name == template_train_name)
    }
}

/// 一つのテンプレート列車の駅間情報を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TemplateTrainSegment {
    /// 識別ID
    pub id: Uuid,
    /// 駅間ID
    pub segment_id: Uuid,
    /// 駅間が反転しているか
    pub is_reversed: bool,
    /// 基準運転時分
    pub running_time: Time,
}

/// 一つのテンプレート列車の駅情報を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TemplateTrainStation {
    /// 識別ID
    pub id: Uuid,
    /// 駅ID
    pub station_id: Uuid,
    /// 停車時間
    pub stop_time: StopType,
}

/// 停車種別
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StopType {
    /// 停車（停車時分）
    Stop(Time),
    /// 通過
    Pass,
}
impl Default for StopType {
    fn default() -> Self {
        Self::Stop(Time::new(0, 0, 30))
    }
}