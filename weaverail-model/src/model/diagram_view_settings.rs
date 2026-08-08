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
