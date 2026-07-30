//! プロジェクト上のメタデータを表すモジュール

use serde::{Deserialize, Serialize};

use crate::model::{ExtensionProperty, PropertiableObject};
use crate::path::Heddle;

/// プロジェクトのメタデータを表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// プロジェクト名
    /// (例: "2026年4月1日 摂播電気鉄道全線 ダイヤグラム")
    pub project_name: String,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl PropertiableObject for Metadata {
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
