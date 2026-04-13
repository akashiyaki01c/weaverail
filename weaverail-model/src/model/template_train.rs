//! Weaverail上の「テンプレート列車」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - TemplateTrain (テンプレート列車)
//!   - TemplateTrainSegment (テンプレート列車の駅間情報)
//!   - TemplateTrainStation (テンプレート列車の駅情報)
//!     - StopType (停車種別)

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    model::{
        DiagramRoot, ExtensionProperty,
        line::{Line, LineSegmentId},
        station::{StationId, TrackId},
        time::Time,
        train_type::TrainTypeId,
    },
    weaverail_id,
};

weaverail_id!(TemplateTrainId, "TMPT");

/// Weaverail上の1つのテンプレート列車を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TemplateTrain {
    /// 識別ID
    pub id: TemplateTrainId,
    /// テンプレート列車名 (例: "本線下り普通列車")
    pub name: String,
    /// 列車種別ID
    pub train_type_id: TrainTypeId,
    /// 開始駅情報
    pub start_station: TemplateTrainStation,
    /// 駅間/駅情報の一覧
    pub segments: Vec<(TemplateTrainSegment, TemplateTrainStation)>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl TemplateTrain {
    /// テンプレート列車が指定駅間を参照しているか
    pub fn contains_segment(&self, segment_id: LineSegmentId) -> bool {
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
    pub fn contains_station(&self, station_id: StationId) -> bool {
        let stations = self.get_stations();
        stations
            .iter()
            .any(|station| station.station_id == station_id)
    }

    /// 指定の駅が何番目にあるか
    pub fn get_station_index(&self, station_id: StationId) -> usize {
        let stations = self.get_stations();
        stations
            .iter()
            .position(|station| station.station_id == station_id)
            .expect("Station ID not found in template train")
    }

    /// 指定の駅間を抽出して返す関数
    pub fn get_filtered_segment(
        &self,
        start_station_id: StationId,
        end_station_id: StationId,
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
        start_station_id: StationId,
        end_station_id: StationId,
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

weaverail_id!(TemplateTrainSegmentId, "TTSG");

/// Weaverail上のテンプレート列車の駅間情報を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TemplateTrainSegment {
    /// 識別ID
    pub id: TemplateTrainSegmentId,
    /// 駅間ID
    pub segment_id: LineSegmentId,
    /// 駅間が反転しているか
    pub is_reversed: bool,
    /// 基準運転時分
    pub running_time: Time,
}

weaverail_id!(TemplateTrainStationId, "TTST");

/// Weaverail上のテンプレート列車の駅情報を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TemplateTrainStation {
    /// 識別ID
    pub id: TemplateTrainStationId,
    /// 駅ID
    pub station_id: StationId,
    /// 駅到着番線ID
    pub track_id: TrackId,
    /// 停車時間
    pub stop_time: StopType,
}

/// テンプレート列車の停車種別を表す列挙体
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
