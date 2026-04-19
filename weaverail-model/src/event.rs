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

    /// 番線が追加された
    #[strum(serialize = "track::added")]
    TrackAdded,
    /// 番線が削除された
    #[strum(serialize = "track::deleted")]
    TrackDeleted,

    /// 路線が追加された
    #[strum(serialize = "line::added")]
    LineAdded,
    /// 路線が削除された
    #[strum(serialize = "line::deleted")]
    LineDeleted,
    /// 路線名が変更された
    #[strum(serialize = "line::renamed")]
    LineRenamed,

    /// 駅間が追加された
    #[strum(serialize = "segment::pushed")]
    SegmentPushed,
    /// 駅間が削除された
    #[strum(serialize = "segment::poped")]
    SegmentPoped,

    /// 列車種別が追加された
    #[strum(serialize = "train_type::added")]
    TrainTypeAdded,
    /// 列車種別が削除された
    #[strum(serialize = "train_type::deleted")]
    TrainTypeDeleted,

    /// テンプレート列車が追加された
    #[strum(serialize = "template_train::added")]
    TemplateTrainAdded,
    /// テンプレート列車が削除された
    #[strum(serialize = "template_train::deleted")]
    TemplateTrainDeleted,

    /// テンプレート列車の駅間が追加された
    #[strum(serialize = "template_train_segment::pushed")]
    TemplateTrainSegmentPushed,
    /// テンプレート列車の駅間が削除された
    #[strum(serialize = "template_train_segment::poped")]
    TemplateTrainSegmentPoped,

    /// 時刻表が追加された
    #[strum(serialize = "timetable::added")]
    TimetableAdded,
    /// 時刻表が削除された
    #[strum(serialize = "timetable::deleted")]
    TimetableDeleted,

    /// 列車が追加された
    #[strum(serialize = "train::added")]
    TrainAdded,
    /// 列車が削除された
    #[strum(serialize = "train::deleted")]
    TrainDeleted,
}
