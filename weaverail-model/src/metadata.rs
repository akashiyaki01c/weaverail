//! プロジェクト上のメタデータを表すモジュール

use serde::{Deserialize, Serialize};

use crate::model::ExtensionProperty;

/// プロジェクトのメタデータを表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
	/// プロジェクト名
	/// (例: "2026年4月1日 摂播電気鉄道全線 ダイヤグラム")
	pub project_name: String,
	/// 拡張プロパティ
	pub properties: ExtensionProperty,
}
