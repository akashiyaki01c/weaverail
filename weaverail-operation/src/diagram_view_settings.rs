use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, DiagramViewSettingsId},
};

/// ダイヤグラム表示設定の参照整合性を検証する。
///
/// `StationBetween` の中に存在しない駅間 ID が含まれている場合は
/// `ModelError::ObjectNotFound` を返す。
pub fn validate_diagram_view_settings(
    root: &DiagramRoot,
    settings_id: DiagramViewSettingsId,
) -> Result<(), ModelError> {
    root.validate_diagram_view_settings(settings_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{
        DiagramViewSettings, LineSegment, LineSegmentId, SegmentRef, Station, StationId,
        id::WeaverailId,
    };

    #[test]
    fn operation_validate_diagram_view_settings_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let start = StationId::new(WeaverailId::new(1));
        let end = StationId::new(WeaverailId::new(2));
        let segment_id = LineSegmentId::new(WeaverailId::new(3));
        let settings_id = DiagramViewSettingsId::new(WeaverailId::new(4));

        root.add_station(Station::new(start, "梅田")).unwrap();
        root.add_station(Station::new(end, "大阪")).unwrap();
        root.add_segment(LineSegment::new(segment_id, start, end))
            .unwrap();

        let settings = DiagramViewSettings {
            id: settings_id,
            name: "標準表示".to_string(),
            segments: vec![weaverail_model::model::DiagramViewSegment::StationBetween {
                segment: SegmentRef {
                    segment_id,
                    is_reversed: false,
                },
            }],
            properties: weaverail_model::model::ExtensionProperty::new(),
        };
        root.diagram_view_settings.insert(settings_id, settings);

        assert!(validate_diagram_view_settings(&root, settings_id).is_ok());
        assert!(
            validate_diagram_view_settings(&root, DiagramViewSettingsId::new(WeaverailId::new(99)))
                .is_err()
        );
    }
}
