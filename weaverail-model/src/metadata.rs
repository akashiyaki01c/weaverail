use serde::{Deserialize, Serialize};

use crate::model::ExtensionProperty;

/// プロジェクトのメタデータを表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
	/// プロジェクト名
	pub project_name: String,
	/// 拡張プロパティ
	pub properties: ExtensionProperty,
}
