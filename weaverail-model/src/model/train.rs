use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{DiagramRoot, ExtensionProperty, template_train::ResultStationTime, time::Time};

/// 一つの列車を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Train {
    /// 識別ID
    pub id: Uuid,
    /// テンプレート列車ID
    pub template_segments: Vec<TemplateSegment>,
    /// 開始駅の出発時刻
    pub start_departure_time: Time,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Train {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Default::default()
        }
    }
}
impl DiagramRoot {
    pub fn get_train_time(
        &self,
        train: &Train,
        start_departure_time: Time,
    ) -> Vec<ResultStationTime> {
        let mut result = Vec::new();
        let mut current_time = start_departure_time;

        for segment in &train.template_segments {
            let template = self
                .template_trains
                .get(&segment.template_train_id)
                .expect("invalid template train id");
            let mut segment_time = template.get_segment_time(
                segment.start_station_id,
                segment.end_station_id,
                current_time,
            );
            if segment_time.is_empty() {
                return Vec::new();
            }
            if result.is_empty() {
                current_time = segment_time.last().expect("").departure_time.expect("");
                result.append(&mut segment_time);
            } else {
                current_time = segment_time.last().expect("").departure_time.expect("");
                if result.last().expect("").station_id != segment_time.first().expect("").station_id
                {
                    // 駅が繋がらない場合
                    return Vec::new();
                }
                result.last_mut().expect("").departure_time =
                    segment_time.first().expect("").departure_time;
                segment_time.remove(0);
                result.append(&mut segment_time);
            }
        }

        result
    }
}

/// 一つのテンプレート列車への参照を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TemplateSegment {
    /// テンプレート列車ID
    pub template_train_id: Uuid,
    /// 開始駅ID
    pub start_station_id: Uuid,
    /// 終了駅ID
    pub end_station_id: Uuid,
}

#[cfg(test)]
mod diagram_tests {
    use super::*;
    use crate::model::template_train::{
        StopType, TemplateTrain, TemplateTrainSegment, TemplateTrainStation,
    };

    #[test]
    fn test_get_train_time_with_connection_stop_time() {
        let mut root = DiagramRoot::default();
        let s0 = Uuid::new_v4();
        let s1 = Uuid::new_v4();
        let s2 = Uuid::new_v4();

        // 1. テンプレートA: 駅0 -> 駅1 (5分走行)
        let t1_id = Uuid::new_v4();
        let train_a = TemplateTrain {
            id: t1_id,
            start_station: TemplateTrainStation {
                station_id: s0,
                ..Default::default()
            },
            segments: vec![(
                TemplateTrainSegment {
                    running_time: Time::new(0, 5, 0),
                    ..Default::default()
                },
                TemplateTrainStation {
                    station_id: s1,
                    stop_time: StopType::Stop(Time::new(0, 2, 0)), // 2分停車
                    ..Default::default()
                }, // 駅1で終了
            )],
            ..Default::default()
        };

        // 2. テンプレートB: 駅1 -> 駅2 (10分走行)
        // ★ ここで駅1に「2分停車」を設定
        let t2_id = Uuid::new_v4();
        let train_b = TemplateTrain {
            id: t2_id,
            start_station: TemplateTrainStation {
                station_id: s1,
                stop_time: StopType::Stop(Time::new(0, 2, 0)), // 2分停車
                ..Default::default()
            },
            segments: vec![(
                TemplateTrainSegment {
                    running_time: Time::new(0, 10, 0),
                    ..Default::default()
                },
                TemplateTrainStation {
                    station_id: s2,
                    ..Default::default()
                },
            )],
            ..Default::default()
        };

        root.template_trains.insert(t1_id, train_a);
        root.template_trains.insert(t2_id, train_b);

        // 3. 列車作成
        let train = Train {
            template_segments: vec![
                TemplateSegment {
                    template_train_id: t1_id,
                    start_station_id: s0,
                    end_station_id: s1,
                },
                TemplateSegment {
                    template_train_id: t2_id,
                    start_station_id: s1,
                    end_station_id: s2,
                },
            ],
            ..Default::default()
        };

        // --- 実行 (10:00:00 出発) ---
        let start_time = Time::new(10, 0, 0);
        let results = root.get_train_time(&train, start_time);

        // --- 検証 ---
        assert_eq!(results.len(), 3); // 駅0, 駅1, 駅2

        // 駅0 (始発)
        assert_eq!(results[0].station_id, s0);
        assert_eq!(results[0].departure_time, Some(Time::new(10, 0, 0)));

        // 駅1 (接続駅)
        assert_eq!(results[1].station_id, s1);
        // 到着: 10:00 + 5分走行 = 10:05
        assert_eq!(results[1].arrival_time, Some(Time::new(10, 5, 0)));
        // 出発: 10:05 + 2分停車 = 10:07  ★ここが今回の修正ポイント
        assert_eq!(results[1].departure_time, Some(Time::new(10, 7, 0)));

        // 駅2 (終点)
        assert_eq!(results[2].station_id, s2);
        // 到着: 10:07出発 + 10分走行 = 10:17
        assert_eq!(results[2].arrival_time, Some(Time::new(10, 17, 0)));
    }
}
