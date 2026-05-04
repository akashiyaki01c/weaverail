use std::cell::Cell;

use serde::{Deserialize, Serialize};

use crate::model::id::WeaverailId;

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct IdIssuer {
    current: Cell<u32>,
}
impl IdIssuer {
    pub fn new() -> Self {
        Self {
            current: Cell::new(0),
        }
    }

    pub fn next(&self) -> WeaverailId {
        let value = self.current.get();
        let result = WeaverailId(value);
        self.current.set(value + 1);
        result
    }
}
