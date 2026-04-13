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
#[derive(Default)]
pub enum ResultStationStopType {
    /// 停車
    #[default]
    Stop,
    /// 通過
    Pass,
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
        stations
            .iter()
            .any(|station| station.station_id == station_id)
    }

    /// 指定の駅が何番目にあるか
    pub fn get_station_index(&self, station_id: Uuid) -> usize {
        let stations = self.get_stations();
        stations
            .iter()
            .position(|station| station.station_id == station_id)
            .expect("Station ID not found in template train")
    }

    /// 指定の駅間を抽出して返す関数
    pub fn get_filtered_segment(
        &self,
        start_station_id: Uuid,
        end_station_id: Uuid,
    ) -> (
        &TemplateTrainStation,
        Vec<(&TemplateTrainSegment, &TemplateTrainStation)>,
    ) {
        // 対象駅がない場合
        if !self.contains_station(start_station_id) || !self.contains_station(end_station_id) {
            unreachable!();
        }

        let first_index = self.get_station_index(start_station_id);
        let end_index = self.get_station_index(end_station_id);
        let first_station = if first_index == 0 {
            &self.start_station
        } else {
            &self.segments.get(first_index - 1).expect("index error").1
        };

        let mut segments: Vec<(&TemplateTrainSegment, &TemplateTrainStation)> = Vec::new();
        for i in (first_index)..end_index {
            let segment = self.segments.get(i).expect("index error");
            segments.push((&segment.0, &segment.1));
        }

        (first_station, segments)
    }
    pub fn get_filtered_segment_iter(
        &self,
        start_station_id: Uuid,
        end_station_id: Uuid,
    ) -> Vec<(
        &TemplateTrainStation,
        &TemplateTrainSegment,
        &TemplateTrainStation,
    )> {
        let segments = self.get_filtered_segment(start_station_id, end_station_id);
        let mut result = Vec::new();

        let get_station_by_index = |index: isize| {
            if index < 0 {
                segments.0
            } else {
                segments.1.get(index as usize).unwrap().1
            }
        };

        for i in 0..segments.1.len() {
            let start = get_station_by_index(i as isize - 1);
            let segment = segments.1.get(i).unwrap().0;
            let end = segments.1.get(i).unwrap().1;
            if segment.is_reversed {
                result.push((end, segment, start));
            } else {
                result.push((start, segment, end));
            }
        }
        result
    }
}
impl DiagramRoot {
    /// テンプレート列車名からテンプレート列車を検索する関数
    /// 見つからない場合は None を返す
    pub fn find_template_train_by_name(&self, template_train_name: &str) -> Option<&TemplateTrain> {
        self.template_trains
            .values()
            .find(|template_train| template_train.name == template_train_name)
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
    /// 駅到着番線ID
    pub track_id: Uuid,
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
