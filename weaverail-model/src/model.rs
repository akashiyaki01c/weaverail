//! Weaverail上で用いられる全てのデータ構造を定義するモジュール

pub mod diagram_view_settings;
pub mod id;
pub mod line;
pub mod line_segment;
pub mod segment_train_order;
pub mod station;
pub mod template_train;
pub mod time;
pub mod timetable;
pub mod track;
pub mod train;
pub mod train_type;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    error::ModelError,
    id_issuer::IdIssuer,
    path::Heddle,
};

pub use station::Station;
pub use track::Track;
pub use track::TrackId;
pub use station::StationId;
pub use line::Line;
pub use line::LineId;
pub use line::SegmentRef;
pub use line_segment::LineSegment;
pub use line_segment::LineSegmentId;
pub use train_type::TrainType;
pub use train_type::TrainTypeId;
pub use template_train::TemplateTrain;
pub use template_train::TemplateTrainId;
pub use template_train::TemplateTrainSection;
pub use template_train::TemplateTrainSegment;
pub use template_train::TemplateTrainSegmentId;
pub use template_train::TemplateTrainStation;
pub use template_train::TemplateTrainStationId;
pub use template_train::StopType;
pub use timetable::Timetable;
pub use timetable::TimetableId;
pub use timetable::SegmentTrainOrders;
pub use segment_train_order::SegmentTrainOrder;
pub use train::Train;
pub use train::TrainId;
pub use train::TemplateSegment;
pub use time::Time;
pub use diagram_view_settings::DiagramViewSegment;
pub use diagram_view_settings::DiagramViewSettings;
pub use diagram_view_settings::DiagramViewSettingsId;

/// ユーザ定義で拡張が行える拡張プロパティを表す構造体
#[derive(
    ts_rs::TS,
    Clone,
    PartialEq,
    Debug,
    Default,
    Serialize,
    Deserialize,
    weaverail_object::RnaObjectable,
)]
pub struct ExtensionProperty(IndexMap<String, Heddle>);
impl ExtensionProperty {
    pub fn new() -> Self {
        Self(IndexMap::new())
    }
    /// 値を取得する
    pub fn get(&self, id: &str) -> Option<&Heddle> {
        self.0.get(id)
    }
    /// 値を設定する
    pub fn set(&mut self, id: &str, value: Heddle) -> Option<Heddle> {
        self.0.insert(id.to_string(), value)
    }
    /// 値を削除する
    pub fn remove(&mut self, id: &str) -> Option<Heddle> {
        self.0.shift_remove(id)
    }
}

/// 拡張プロパティを保持する構造体を表すトレイト
pub trait PropertiableObject {
    /// 拡張プロパティの値を取得する
    fn get_property(&self, id: &str) -> Option<&Heddle>;
    /// 拡張プロパティの値を設定する
    fn set_property(&mut self, id: &str, value: Heddle) -> Option<Heddle>;
    /// 拡張プロパティの値を削除する
    fn remove_property(&mut self, id: &str) -> Option<Heddle>;
}

/// ダイヤグラムプロジェクトファイルを表す構造体
#[derive(
    weaverail_object::RnaObjectable,
    ts_rs::TS,
    Clone,
    PartialEq,
    Debug,
    Default,
    Serialize,
    Deserialize,
)]
pub struct DiagramRoot {
    /// 駅の集合
    pub stations: IndexMap<StationId, Station>,
    /// 番線の集合
    pub tracks: IndexMap<TrackId, Track>,
    /// 駅間の集合
    pub segments: IndexMap<LineSegmentId, LineSegment>,
    /// 路線の集合
    pub lines: IndexMap<LineId, Line>,
    /// 列車種別の集合
    pub train_types: IndexMap<TrainTypeId, TrainType>,
    /// テンプレート列車の集合
    pub template_trains: IndexMap<TemplateTrainId, TemplateTrain>,
    /// 時刻表の集合
    pub timetables: IndexMap<TimetableId, Timetable>,
    /// 列車の集合
    pub trains: IndexMap<TrainId, Train>,
    /// ダイヤグラムの表示設定の集合
    pub diagram_view_settings: IndexMap<DiagramViewSettingsId, DiagramViewSettings>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
    /// ID発行
    pub id_issuer: IdIssuer,
    /// バージョン
    pub version: u32,
}
impl DiagramRoot {
    /// データが正常な値であるかを検証する
    pub fn validate(&self) -> Result<(), ModelError> {
        for sta in self.stations.keys() {
            self.validate_station(*sta)?;
        }
        for track in self.tracks.keys() {
            self.validate_track(*track)?;
        }
        for segment in self.segments.keys() {
            self.validate_segment(*segment)?;
        }
        for line in self.lines.keys() {
            self.validate_line(*line)?;
        }
        for train_type in self.train_types.keys() {
            self.validate_train_type(*train_type)?;
        }
        for template_train in self.template_trains.keys() {
            self.validate_template_train(*template_train)?;
        }
        for timetable in self.timetables.keys() {
            self.validate_timetable(*timetable)?;
        }
        for train in self.trains.keys() {
            self.validate_train(*train)?;
        }
        for diagram_view_setting in self.diagram_view_settings.keys() {
            self.validate_diagram_view_settings(*diagram_view_setting)?;
        }

        Ok(())
    }
}
impl PropertiableObject for DiagramRoot {
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

use std::{any::Any, cell::Cell, hash::Hash};

/// 動的アクセスのためのエラー型
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum RnaError {
    #[error("フィールド '{0}' が見つかりません")]
    FieldNotFound(String),

    #[error("型の不一致: 値をこのフィールドの型に変換できませんでした")]
    TypeMismatch,

    #[error("このフィールドは読み取り専用です")]
    ReadOnly,
}

/// 全ての DNA 構造体・基本型が実装する「動的アクセス窓口」トレイト
pub trait RnaObject: Any {
    /// 1. フィールドの参照を取得する (Read)
    ///    指定された key (例: "name") に対応するフィールドの &dyn RnaObject を返す
    fn rna_get(&self, _key: &str) -> Option<&dyn RnaObject> {
        None // デフォルト実装（基本型や子を持たない型用）
    }

    /// 2. フィールドの可変参照を取得する (Write/Mut)
    fn rna_get_mut(&mut self, _key: &str) -> Option<&mut dyn RnaObject> {
        None
    }

    /// 3. Heddle（抽象値）を使って直接値を書き換える (Set)
    fn rna_set(&mut self, key: &str, _value: Heddle) -> Result<(), RnaError> {
        Err(RnaError::FieldNotFound(key.to_string()))
    }

    /// 4. 自身を Heddle（抽象値）に変換して取り出す (Value)
    ///    String や u32 などの末端の型（プリミティブ）がオーバーライドする
    fn to_heddle(&self) -> Option<Heddle> {
        None
    }

    /// Downcast 用の Any 参照取得（必要に応じて型チェックに使用）
    fn as_any(&self) -> &dyn Any;
}

macro_rules! impl_rna_obj {
    ($t:ty, $e:ident) => {
        impl RnaObject for $t {
            fn to_heddle(&self) -> Option<Heddle> {
                Some(Heddle::$e(*self))
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        impl TryFrom<Heddle> for $t {
            type Error = crate::model::RnaError;

            fn try_from(value: Heddle) -> Result<Self, Self::Error> {
                match value {
                    Heddle::$e(s) => Ok(s),
                    _ => Err(crate::model::RnaError::TypeMismatch),
                }
            }
        }
    };
}
impl_rna_obj!(bool, Boolean);
impl_rna_obj!(u8, U8);
impl_rna_obj!(i8, I8);
impl_rna_obj!(u16, U16);
impl_rna_obj!(i16, I16);
impl_rna_obj!(u32, U32);
impl_rna_obj!(i32, I32);
impl_rna_obj!(u64, U64);
impl_rna_obj!(i64, I64);
impl_rna_obj!(u128, U128);
impl_rna_obj!(i128, I128);
impl_rna_obj!(f32, F32);
impl_rna_obj!(f64, F64);
impl<T: RnaObject + Copy> RnaObject for Cell<T> {
    fn to_heddle(&self) -> Option<Heddle> {
        self.get().to_heddle()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl<T: RnaObject + Copy + TryFrom<Heddle>> TryFrom<Heddle> for Cell<T> {
    type Error = crate::model::RnaError;

    fn try_from(value: Heddle) -> Result<Self, Self::Error> {
        Ok(Cell::new(
            T::try_from(value.clone()).map_err(|_| RnaError::TypeMismatch)?,
        ))
    }
}
impl RnaObject for String {
    fn to_heddle(&self) -> Option<Heddle> {
        Some(Heddle::String(self.clone()))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl TryFrom<Heddle> for String {
    type Error = crate::model::RnaError;

    fn try_from(value: Heddle) -> Result<Self, Self::Error> {
        match value {
            Heddle::String(s) => Ok(s),
            _ => Err(crate::model::RnaError::TypeMismatch),
        }
    }
}
impl RnaObject for Vec<Heddle> {
    fn to_heddle(&self) -> Option<Heddle> {
        Some(Heddle::Array(self.clone()))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl TryFrom<Heddle> for Vec<Heddle> {
    type Error = crate::model::RnaError;

    fn try_from(value: Heddle) -> Result<Self, Self::Error> {
        match value {
            Heddle::Array(s) => Ok(s),
            _ => Err(crate::model::RnaError::TypeMismatch),
        }
    }
}
impl<T: RnaObject> RnaObject for Vec<T> {
    fn to_heddle(&self) -> Option<Heddle> {
        let value: Vec<_> = self.iter().map(|v| v.to_heddle()).collect();
        if value.iter().all(|v| v.is_some()) {
            Some(Heddle::Array(
                value.iter().map(|v| v.clone().unwrap()).collect(),
            ))
        } else {
            None
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl<T> TryFrom<Heddle> for Vec<T>
where
    T: TryFrom<Heddle, Error = crate::model::RnaError>,
{
    type Error = crate::model::RnaError;

    fn try_from(value: Heddle) -> Result<Self, Self::Error> {
        match value {
            // Heddle::Array(s) の所有権をそのまま展開する (into_iter)
            Heddle::Array(s) => {
                s.into_iter()
                    // 各 Heddle 要素を T::try_from(v) で変換 (Result<T, RnaError> が返る)
                    .map(|v| T::try_from(v))
                    // Result の性質を利用して、全成功なら Ok(Vec<T>)、1つでも失敗なら Err にまとめて回収！
                    .collect::<Result<Vec<T>, _>>()
            }
            _ => Err(crate::model::RnaError::TypeMismatch),
        }
    }
}
impl RnaObject for IndexMap<String, Heddle> {
    fn to_heddle(&self) -> Option<Heddle> {
        let mut map = IndexMap::new();
        for (key, value) in self.iter() {
            map.insert(Heddle::String(key.clone()), value.clone());
        }
        Some(Heddle::Compound(map))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl TryFrom<Heddle> for IndexMap<String, Heddle> {
    type Error = crate::model::RnaError;

    fn try_from(value: Heddle) -> Result<Self, Self::Error> {
        match value {
            Heddle::Compound(s) => {
                let mut map = IndexMap::new();
                for (key, value) in s.iter() {
                    map.insert(String::try_from(key.clone())?, value.clone());
                }
                Ok(map)
            }
            _ => Err(crate::model::RnaError::TypeMismatch),
        }
    }
}
impl RnaObject for IndexMap<Heddle, Heddle> {
    fn to_heddle(&self) -> Option<Heddle> {
        Some(Heddle::Compound(self.clone()))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl TryFrom<Heddle> for IndexMap<Heddle, Heddle> {
    type Error = crate::model::RnaError;

    fn try_from(value: Heddle) -> Result<Self, Self::Error> {
        match value {
            Heddle::Compound(s) => Ok(s),
            _ => Err(crate::model::RnaError::TypeMismatch),
        }
    }
}
impl<T: RnaObject + Hash, S: RnaObject> RnaObject for IndexMap<T, S> {
    fn to_heddle(&self) -> Option<Heddle> {
        let mut map = IndexMap::new();
        for (key, value) in self.iter() {
            map.insert(key.to_heddle()?, value.to_heddle()?);
        }
        Some(Heddle::Compound(map))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl<T: RnaObject + Hash + TryFrom<Heddle> + Eq, S: RnaObject + TryFrom<Heddle>> TryFrom<Heddle>
    for IndexMap<T, S>
{
    type Error = crate::model::RnaError;

    fn try_from(value: Heddle) -> Result<Self, Self::Error> {
        match value {
            Heddle::Compound(s) => {
                let mut map = IndexMap::new();
                for (key, value) in s.iter() {
                    map.insert(
                        T::try_from(key.clone()).map_err(|_| RnaError::TypeMismatch)?,
                        S::try_from(value.clone()).map_err(|_| RnaError::TypeMismatch)?,
                    );
                }
                Ok(map)
            }
            _ => Err(crate::model::RnaError::TypeMismatch),
        }
    }
}
impl RnaObject for Time {
    fn to_heddle(&self) -> Option<Heddle> {
        Some(Heddle::Time(*self))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl TryFrom<Heddle> for Time {
    type Error = crate::model::RnaError;

    fn try_from(value: Heddle) -> Result<Self, Self::Error> {
        match value {
            Heddle::Time(s) => Ok(s),
            _ => Err(crate::model::RnaError::TypeMismatch),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::id::WeaverailId, result_weft::ResultWeftTrain};

    use super::*;
    use ts_rs::{Config, TS};

    #[test]
    fn export_types() {
        let cfg = Config::new();
        DiagramRoot::export_all(&cfg).expect("TS export failed");
        ResultWeftTrain::export_all(&cfg).expect("TS export failed");
    }

    #[test]
    fn test_rna() {
        let data = crate::test_data::diagram_root::get_test_data_shortly();
        let station = Station::new(StationId(WeaverailId(0)), "test");
        println!("{:#?}", data.root.to_heddle());
    }
}
