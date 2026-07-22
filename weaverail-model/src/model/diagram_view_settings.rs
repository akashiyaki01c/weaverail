use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{model::{ExtensionProperty, PropertiableObject, line::SegmentRef}, weaverail_id};

weaverail_id!(DiagramViewSettingsId, "DVS_");

/// ダイヤグラムを表示する際の設定を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
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
impl PropertiableObject for DiagramViewSettings {
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

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Serialize, Deserialize)]
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
