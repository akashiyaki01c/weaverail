use weaverail_model::{
    error::ModelError,
    model::{
        DiagramRoot, LineSegmentId, TemplateTrain, TemplateTrainId, TemplateTrainSegment,
        TemplateTrainStation,
    },
};

/// テンプレート列車を新規登録する。
///
/// 同じ ID が既に存在する場合は `ModelError::DuplicateKey` を返す。
pub fn add_template_train(
    root: &mut DiagramRoot,
    template_train: TemplateTrain,
) -> Result<(), ModelError> {
    root.add_template_train(template_train)
}

/// 指定したテンプレート列車を削除し、削除前の値を返す。
pub fn delete_template_train(
    root: &mut DiagramRoot,
    template_train_id: TemplateTrainId,
) -> Result<TemplateTrain, ModelError> {
    root.delete_template_train(template_train_id)
}

/// テンプレート列車名で検索する。
pub fn find_template_train_by_name<'a>(
    root: &'a DiagramRoot,
    template_train_name: &str,
) -> Option<&'a TemplateTrain> {
    root.find_template_train_by_name(template_train_name)
}

/// テンプレート列車の末尾に駅間を追加する。
pub fn push_back_template_segment(
    root: &mut DiagramRoot,
    template_train_id: TemplateTrainId,
    template_segment: TemplateTrainSegment,
    template_station: TemplateTrainStation,
) -> Result<(), ModelError> {
    root.push_back_template_segment(template_train_id, template_segment, template_station)
}

/// テンプレート列車の先頭に駅間を追加する。
pub fn push_front_template_segment(
    root: &mut DiagramRoot,
    template_train_id: TemplateTrainId,
    template_segment: TemplateTrainSegment,
    template_station: TemplateTrainStation,
) -> Result<(), ModelError> {
    root.push_front_template_segment(template_train_id, template_segment, template_station)
}

/// テンプレート列車の末尾の駅間参照を除去する。
pub fn pop_back_template_segment(
    root: &mut DiagramRoot,
    template_train_id: TemplateTrainId,
) -> Result<(), ModelError> {
    root.pop_back_template_segment(template_train_id)
}

/// テンプレート列車の先頭の駅間参照を除去する。
pub fn pop_front_template_segment(
    root: &mut DiagramRoot,
    template_train_id: TemplateTrainId,
) -> Result<(), ModelError> {
    root.pop_front_template_segment(template_train_id)
}

/// 指定された駅間を参照しているテンプレート列車の駅間定義を取得する。
pub fn get_template_segments(root: &DiagramRoot, id: LineSegmentId) -> Vec<&TemplateTrainSegment> {
    root.get_template_segments(id)
}

/// テンプレート列車の構造と参照整合性を検証する。
pub fn validate_template_train(
    root: &DiagramRoot,
    template_train_id: TemplateTrainId,
) -> Result<(), ModelError> {
    root.validate_template_train(template_train_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{
        StopType, TemplateTrainStationId,
        id::WeaverailId,
        station::{Station, StationId},
        time::Time,
        track::Track,
        track::TrackId,
        train_type::TrainType,
        train_type::TrainTypeId,
    };

    #[test]
    fn operation_add_and_delete_template_train_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));
        let train_type_id = TrainTypeId::new(WeaverailId::new(3));
        let template_train_id = TemplateTrainId::new(WeaverailId::new(4));

        root.add_station(Station::new(station_id, "梅田")).unwrap();
        root.add_track(Track::new(track_id, station_id, "1番線"))
            .unwrap();
        root.add_train_type(TrainType::new(train_type_id, "普通"))
            .unwrap();

        let template_train = TemplateTrain {
            id: template_train_id,
            name: "本線下り普通".to_string(),
            train_type_id,
            start_station: TemplateTrainStation {
                id: TemplateTrainStationId::new(WeaverailId::new(10)),
                station_id,
                track_id,
                stop_time: StopType::Stop(Time::new(0, 0, 0)),
                properties: weaverail_model::model::ExtensionProperty::default(),
            },
            segments: vec![],
            properties: weaverail_model::model::ExtensionProperty::default(),
        };

        assert!(add_template_train(&mut root, template_train.clone()).is_ok());
        assert_eq!(root.template_trains.len(), 1);

        let removed = delete_template_train(&mut root, template_train_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().id, template_train_id);
    }

    #[test]
    fn operation_validate_template_train_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));
        let train_type_id = TrainTypeId::new(WeaverailId::new(3));
        let template_train_id = TemplateTrainId::new(WeaverailId::new(4));

        root.add_station(Station::new(station_id, "梅田")).unwrap();
        root.add_track(Track::new(track_id, station_id, "1番線"))
            .unwrap();
        root.add_train_type(TrainType::new(train_type_id, "普通"))
            .unwrap();

        let template_train = TemplateTrain {
            id: template_train_id,
            name: "本線下り普通".to_string(),
            train_type_id,
            start_station: TemplateTrainStation {
                id: TemplateTrainStationId::new(WeaverailId::new(10)),
                station_id,
                track_id,
                stop_time: StopType::Stop(Time::new(0, 0, 0)),
                properties: weaverail_model::model::ExtensionProperty::default(),
            },
            segments: vec![],
            properties: weaverail_model::model::ExtensionProperty::default(),
        };
        add_template_train(&mut root, template_train).unwrap();

        assert!(validate_template_train(&root, template_train_id).is_ok());
        assert!(
            validate_template_train(&root, TemplateTrainId::new(WeaverailId::new(99))).is_err()
        );
    }
}
