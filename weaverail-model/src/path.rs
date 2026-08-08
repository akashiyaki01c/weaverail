//! Weaverailのオブジェクトに対し、パスを用いてアクセスできるようにするモジュール

use std::hash::Hasher;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::model::{id::WeaverailId, time::Time};

/// オブジェクトを走査する際のオブジェクトを表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum Heddle {
    /// Null value
    #[default]
    Null,
    /// Id value
    Id(WeaverailId),
    /// Boolean value
    Boolean(bool),
    /// Unsigned 8bit integer
    U8(u8),
    /// Signed 8bit integer
    I8(i8),
    /// Unsigned 16bit integer
    U16(u16),
    /// Signed 16bit integer
    I16(i16),
    /// Unsigned 32bit integer
    U32(u32),
    /// Signed 32bit integer
    I32(i32),
    /// Unsigned 64bit integer
    U64(u64),
    /// Signed 64bit integer
    I64(i64),
    /// Unsigned 128bit integer
    U128(u128),
    /// Signed 128bit integer
    I128(i128),
    /// 32bit float point number
    F32(f32),
    /// 64bit float point number
    F64(f64),
    /// String value
    String(String),
    /// Array value
    Array(Vec<Heddle>),
    /// Compount value
    Compound(IndexMap<Heddle, Heddle>),
    /// Time value
    Time(Time),
}

impl Eq for Heddle {}

impl std::hash::Hash for Heddle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // enum の discriminant（どのバリアントか）をハッシュ化
        std::mem::discriminant(self).hash(state);

        match self {
            Heddle::Null => {}
            Heddle::Id(v) => v.hash(state),
            Heddle::Boolean(v) => v.hash(state),
            Heddle::U8(v) => v.hash(state),
            Heddle::I8(v) => v.hash(state),
            Heddle::U16(v) => v.hash(state),
            Heddle::I16(v) => v.hash(state),
            Heddle::U32(v) => v.hash(state),
            Heddle::I32(v) => v.hash(state),
            Heddle::U64(v) => v.hash(state),
            Heddle::I64(v) => v.hash(state),
            Heddle::U128(v) => v.hash(state),
            Heddle::I128(v) => v.hash(state),

            // float はビット列でハッシュ化 (-0.0 と +0.0、NaN の揺れを正規化)
            Heddle::F32(v) => canonicalize_f32(*v).hash(state),
            Heddle::F64(v) => canonicalize_f64(*v).hash(state),

            Heddle::String(v) => v.hash(state),
            Heddle::Array(v) => v.hash(state),
            Heddle::Compound(v) => {
                // IndexMap は内部要素を順序通りにハッシュ化
                for (k, val) in v {
                    k.hash(state);
                    val.hash(state);
                }
            }
            Heddle::Time(v) => v.hash(state),
        }
    }
}

// 浮動小数点のビット表現の揺れを抑える補助関数
fn canonicalize_f32(val: f32) -> u32 {
    if val.is_nan() {
        0x7fc00000 // 標準的な NaN のビット
    } else if val == 0.0 {
        0 // -0.0 も +0.0 と同じビットに揃える
    } else {
        val.to_bits()
    }
}

fn canonicalize_f64(val: f64) -> u64 {
    if val.is_nan() {
        0x7ff8000000000000
    } else if val == 0.0 {
        0
    } else {
        val.to_bits()
    }
}
