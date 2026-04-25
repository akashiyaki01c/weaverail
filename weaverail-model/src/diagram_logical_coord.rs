use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct DiagramLogicalCoord {
    pub x: f64,
    pub y: f64,
}
impl DiagramLogicalCoord {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct DiagramLogicalConvert {
    pub scale_x: f64,
    pub scale_y: f64,
    pub offset_x: f64,
    pub offset_y: f64,
}
impl DiagramLogicalConvert {
    pub fn convert(&self, coord: DiagramLogicalCoord) -> DiagramLogicalCoord {
        DiagramLogicalCoord::new(
            coord.x * self.scale_x + self.offset_x,
            coord.y * self.scale_y + self.offset_y,
        )
    }
}

#[cfg(test)]
mod tests {
    use ts_rs::{Config, TS};
    use crate::diagram_logical_coord::{DiagramLogicalConvert, DiagramLogicalCoord};

    #[test]
    fn export_types() {
        let cfg = Config::new();
        DiagramLogicalCoord::export_all(&cfg).expect("TS export failed");
        DiagramLogicalConvert::export_all(&cfg).expect("TS export failed");
    }
}
