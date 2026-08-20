use serde::{Deserialize, Serialize};

use crate::{
    error::ModelError,
    model::{DiagramRoot, ExtensionProperty, PropertiableObject, line::SegmentRef},
    path::Heddle,
    weaverail_id,
};

weaverail_id!(DiagramViewSettingsId, "DVS_");

/// ダイヤグラムを表示する際の設定を表す
#[derive(
    weaverail_object::RnaObjectable,
    ts_rs::TS,
    Clone,
    PartialEq,
    Debug,
    Default,
    Serialize,
    Deserialize,
)]
pub struct DiagramViewSettings {
    /// 設定の識別子
    pub id: DiagramViewSettingsId,
    /// 設定の名前
    pub name: String,
    /// ダイヤグラムの縦軸の区間の一覧を表す
    pub segments: Vec<DiagramViewSegment>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl DiagramRoot {
    /// 駅間データが正常な値であるかを検証する
    pub fn validate_diagram_view_settings(
        &self,
        diagram_view_settings_id: DiagramViewSettingsId,
    ) -> Result<(), ModelError> {
        let diagram_view_settings = self
            .diagram_view_settings
            .get(&diagram_view_settings_id)
            .ok_or(ModelError::ObjectNotFound)?;
        for seg in &diagram_view_settings.segments {
            if let DiagramViewSegment::StationBetween { segment } = seg {
                let _ = self
                    .segments
                    .get(&segment.segment_id)
                    .ok_or(ModelError::ObjectNotFound)?;
            }
        }

        Ok(())
    }
}
impl PropertiableObject for DiagramViewSettings {
    fn get_property(&self, id: &str) -> Option<&Heddle> {
        self.properties.get(id)
    }

    fn set_property(&mut self, id: &str, value: Heddle) -> Option<Heddle> {
        self.properties.set(id, value)
    }

    fn remove_property(&mut self, id: &str) -> Option<Heddle> {
        self.properties.remove(id)
    }
}

#[derive(
    weaverail_object::RnaObjectable,
    ts_rs::TS,
    Clone,
    PartialEq,
    Debug,
    Serialize,
    Deserialize,
    strum::EnumString,
    strum::Display,
)]
/// ダイヤグラムの縦軸の区間を表す
pub enum DiagramViewSegment {
    /// 空白
    Black { scale: f32 },
    /// 駅間
    StationBetween { segment: SegmentRef },
}
impl Default for DiagramViewSegment {
    fn default() -> Self {
        Self::Black { scale: 1.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        id::WeaverailId,
        line::SegmentRef,
        line_segment::{LineSegment, LineSegmentId},
        station::Station,
        station::StationId,
    };

    #[test]
    fn test_diagram_view_segment_default() {
        let segment = DiagramViewSegment::default();
        assert!(matches!(segment, DiagramViewSegment::Black { scale: 1.0 }));
    }

    #[test]
    fn test_validate_diagram_view_settings() {
        let mut root = DiagramRoot::default();
        let start_station = StationId::new(WeaverailId::new(1));
        let end_station = StationId::new(WeaverailId::new(2));
        let segment_id = LineSegmentId::new(WeaverailId::new(3));
        let settings_id = DiagramViewSettingsId::new(WeaverailId::new(4));

        root.add_station(Station::new(start_station, "梅田"))
            .unwrap();
        root.add_station(Station::new(end_station, "大阪")).unwrap();
        root.add_segment(LineSegment::new(segment_id, start_station, end_station))
            .unwrap();

        let settings = DiagramViewSettings {
            id: settings_id,
            name: "標準表示".to_string(),
            segments: vec![DiagramViewSegment::StationBetween {
                segment: SegmentRef {
                    segment_id,
                    is_reversed: false,
                },
            }],
            properties: ExtensionProperty::new(),
        };
        root.diagram_view_settings.insert(settings_id, settings);

        assert!(root.validate_diagram_view_settings(settings_id).is_ok());
        assert!(
            root.validate_diagram_view_settings(DiagramViewSettingsId::new(WeaverailId::new(99)))
                .is_err()
        );
    }

    #[test]
    fn test_diagram_view_settings_properties() {
        let mut settings = DiagramViewSettings {
            id: DiagramViewSettingsId::new(WeaverailId::new(1)),
            name: "標準表示".to_string(),
            segments: vec![DiagramViewSegment::default()],
            properties: ExtensionProperty::new(),
        };
        let value = Heddle::String("zoom".to_string());

        assert!(
            settings
                .set_property("display_mode", value.clone())
                .is_none()
        );
        assert_eq!(settings.get_property("display_mode").unwrap(), &value);
        assert!(settings.remove_property("display_mode").is_some());
        assert!(settings.get_property("display_mode").is_none());
    }
}
