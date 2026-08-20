use serde::{Deserialize, Serialize};

use crate::model::{
    ExtensionProperty, PropertiableObject, line_segment::LineSegmentId, train::TrainId,
};
use crate::path::Heddle;

/// 駅間での列車の順序を表す
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{id::WeaverailId, line_segment::LineSegmentId, train::TrainId};

    #[test]
    fn test_segment_train_order_properties() {
        let train_id_1 = TrainId::new(WeaverailId::new(1));
        let train_id_2 = TrainId::new(WeaverailId::new(2));
        let segment_id = LineSegmentId::new(WeaverailId::new(3));
        let mut order = SegmentTrainOrder {
            segment_id,
            is_reversed: false,
            order: vec![train_id_1, train_id_2],
            properties: ExtensionProperty::new(),
        };
        let value = Heddle::String("priority".to_string());

        assert!(order.set_property("rank", value.clone()).is_none());
        assert_eq!(order.get_property("rank").unwrap(), &value);
        assert!(order.remove_property("rank").is_some());
        assert!(order.get_property("rank").is_none());
    }

    #[test]
    fn test_segment_train_order_default_and_ordering() {
        let order = SegmentTrainOrder::default();
        assert_eq!(order.segment_id, LineSegmentId::new(WeaverailId::new(0)));
        assert!(!order.is_reversed);
        assert!(order.order.is_empty());
        assert_eq!(order.properties, ExtensionProperty::new());
    }
}
