use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct DiagramLogicalCoord {
	x: f64,
	y: f64,
}

