/// Rust側からTypeScript側へ渡す際のイベントの種類定義
#[derive(
    Debug,
    PartialEq,
    Clone,
    Copy,
    strum::EnumString,
    strum::Display,
    strum::IntoStaticStr,
    strum::EnumIter,
)]
pub enum EmitEventType {
	/// 駅が追加された
    StationAdded,
	/// 駅が削除された
    StationDeleted,
	/// 駅名が変更された
    StationRenamed,
}
