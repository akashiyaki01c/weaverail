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

    /// 指定の駅間の時刻を取得する関数
    pub fn get_segment_time(
        &self,
        start_station_id: Uuid,
        end_station_id: Uuid,
        start_departure_time: Time,
    ) -> Vec<ResultStationTime> {
        let segments = self.get_filtered_segment(start_station_id, end_station_id);
        let mut result: Vec<ResultStationTime> = Vec::new();
        let mut current_time = start_departure_time;

        // 始発駅
        result.push(ResultStationTime {
            station_id: segments.0.station_id,
            arrival_time: None,
            departure_time: Some(current_time),
            stop_type: ResultStationStopType::Stop,
        });

        // 途中駅〜終着駅
        for segment in segments.1 {
            let mut time = ResultStationTime {
                station_id: segment.1.station_id,
                arrival_time: None,
                departure_time: None,
                stop_type: ResultStationStopType::Stop,
            };
            // 到着時刻
            current_time += segment.0.running_time;
            time.arrival_time = Some(current_time);

            // 発車時刻
            match segment.1.stop_time {
                StopType::Stop(stop_time) => {
                    current_time += stop_time;
                    time.departure_time = Some(current_time);
                }
                StopType::Pass => {
                    time.stop_type = ResultStationStopType::Pass;
                    time.departure_time = Some(current_time);
                }
            }
            result.push(time);
        }

        result
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

#[cfg(test)]
mod tests {
    use super::*;
    // テスト用のデータを生成するヘルパー
    fn create_test_train() -> (TemplateTrain, Vec<Uuid>) {
        let station_ids: Vec<Uuid> = (0..4).map(|_| Uuid::new_v4()).collect();
        let segment_ids: Vec<Uuid> = (0..3).map(|_| Uuid::new_v4()).collect();

        let start_station = TemplateTrainStation {
            station_id: station_ids[0],
            ..Default::default()
        };

        let segments = vec![
            (
                TemplateTrainSegment {
                    segment_id: segment_ids[0],
                    ..Default::default()
                },
                TemplateTrainStation {
                    station_id: station_ids[1],
                    ..Default::default()
                },
            ),
            (
                TemplateTrainSegment {
                    segment_id: segment_ids[1],
                    ..Default::default()
                },
                TemplateTrainStation {
                    station_id: station_ids[2],
                    ..Default::default()
                },
            ),
            (
                TemplateTrainSegment {
                    segment_id: segment_ids[2],
                    ..Default::default()
                },
                TemplateTrainStation {
                    station_id: station_ids[3],
                    ..Default::default()
                },
            ),
        ];

        let train = TemplateTrain {
            id: Uuid::new_v4(),
            start_station,
            segments,
            ..Default::default()
        };

        (train, station_ids)
    }

    #[test]
    fn test_get_filtered_segment_full_range() {
        let (train, ids) = create_test_train();
        // 始発(0)から終点(3)まで
        let (first, segments) = train.get_filtered_segment(ids[0], ids[3]);

        assert_eq!(first.station_id, ids[0]);
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].1.station_id, ids[1]);
        assert_eq!(segments[2].1.station_id, ids[3]);
    }

    #[test]
    fn test_get_filtered_segment_middle() {
        let (train, ids) = create_test_train();
        // 中間の駅1から駅2まで
        let (first, segments) = train.get_filtered_segment(ids[1], ids[2]);

        assert_eq!(first.station_id, ids[1]);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].1.station_id, ids[2]);
    }

    #[test]
    #[should_panic]
    fn test_get_filtered_segment_invalid_id() {
        let (train, _ids) = create_test_train();
        // 存在しないUUIDを指定してパニックするか（unreachable! を通るか）
        train.get_filtered_segment(Uuid::new_v4(), Uuid::new_v4());
    }

    #[test]
    fn test_get_segment_time_calculation() {
        // --- Setup ---
        let station_ids: Vec<Uuid> = (0..4).map(|_| Uuid::new_v4()).collect();

        // 1. 駅0 (始発)
        let start_station = TemplateTrainStation {
            station_id: station_ids[0],
            ..Default::default()
        };

        // 2. 駅0 -> 駅1 (5分走行、30秒停車)
        let seg0_1 = TemplateTrainSegment {
            running_time: Time::new(0, 5, 0),
            ..Default::default()
        };
        let stat1 = TemplateTrainStation {
            station_id: station_ids[1],
            stop_time: StopType::Stop(Time::new(0, 0, 30)),
            ..Default::default()
        };

        // 3. 駅1 -> 駅2 (3分走行、通過)
        let seg1_2 = TemplateTrainSegment {
            running_time: Time::new(0, 3, 0),
            ..Default::default()
        };
        let stat2 = TemplateTrainStation {
            station_id: station_ids[2],
            stop_time: StopType::Pass,
            ..Default::default()
        };

        // 4. 駅2 -> 駅3 (3分走行、停車)
        let seg2_3 = TemplateTrainSegment {
            running_time: Time::new(0, 3, 0),
            ..Default::default()
        };
        let stat3 = TemplateTrainStation {
            station_id: station_ids[3],
            stop_time: StopType::Stop(Time::new(0, 10, 0)),
            ..Default::default()
        };

        let train = TemplateTrain {
            start_station,
            segments: vec![(seg0_1, stat1), (seg1_2, stat2), (seg2_3, stat3)],
            ..Default::default()
        };

        // --- Execute ---
        // 12:00:00 出発
        let start_time = Time::new(12, 0, 0);
        let results = train.get_segment_time(station_ids[0], station_ids[3], start_time);

        // --- Assert ---
        assert_eq!(results.len(), 3);

        // 駅0: 12:00:00発
        assert_eq!(results[0].station_id, station_ids[0]);
        assert_eq!(results[0].departure_time, Some(Time::new(12, 0, 0)));

        // 駅1: 12:05:00着 -> 30秒停車 -> 12:05:30発
        assert_eq!(results[1].station_id, station_ids[1]);
        assert_eq!(results[1].arrival_time, Some(Time::new(12, 5, 0)));
        assert_eq!(results[1].departure_time, Some(Time::new(12, 5, 30)));

        // 駅2: 12:05:30から3分走行 -> 12:08:30着 (通過なので発も同時)
        assert_eq!(results[2].station_id, station_ids[2]);
        assert_eq!(results[2].arrival_time, Some(Time::new(12, 8, 30)));
        assert_eq!(results[2].departure_time, Some(Time::new(12, 8, 30)));
    }
}
