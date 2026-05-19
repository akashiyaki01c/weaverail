use serde::{Deserialize, Serialize};

use crate::model::{line_segment::LineSegmentId, train::TrainId};

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultSvg {
    pub trains: Vec<ResultSvgTrain>,
}

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultSvgTrain {
    pub train_id: TrainId,
    pub path_string: String,
}

#[cfg(test)]
mod tests {
    use ts_rs::{Config, TS};

    use crate::result_svg::ResultSvg;
    #[test]
    fn export_types() {
        let cfg = Config::new();

        ResultSvg::export_all(&cfg).expect("TS export failed");
    }
}
