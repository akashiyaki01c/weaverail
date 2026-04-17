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
	#[strum(serialize = "station::added")]
    StationAdded,
	/// 駅が削除された
	#[strum(serialize = "station::deleted")]
    StationDeleted,
	/// 駅名が変更された
    #[strum(serialize = "station::renamed")]
	StationRenamed,
}
