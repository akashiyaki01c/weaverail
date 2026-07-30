use serde::{Deserialize, Serialize};

use crate::model::{
    ExtensionProperty, PropertiableObject, line_segment::LineSegmentId, train::TrainId,
};
use crate::path::Heddle;

/// 駅間での列車の順序を表す
#[derive(weaverail_object::RnaObjectable, ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SegmentTrainOrder {
    pub segment_id: LineSegmentId,
    pub is_reversed: bool,
    pub order: Vec<TrainId>,
    pub properties: ExtensionProperty,
}
impl PropertiableObject for SegmentTrainOrder {
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
