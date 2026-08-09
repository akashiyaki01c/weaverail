//! Weaverailのオブジェクト定義に用いられるプリミティブ型を定義するモジュール
//!
//! Weaverailのオブジェクト定義でのプリミティブ型は以下の型である
//!
//! - 真偽値型
//! 	- `Boolean`
//! - 整数型
//! 	- `u8` / `i8`
//! 	- `u16` / `i16`
//! 	- `u32` / `i32`
//! 	- `u64` / `i64`
//! 	- `u128` / `i128`
//! - 浮動小数点数型
//! 	- `f32`
//! 	- `f64`
//! - 文字型
//! 	- `char`
//! - 文字列型
//! 	- `String`
//! - ID型
//! 	- `WeaverailId`
//! - 時刻型
//! 	- `Time`
//! - 配列型
//! 	- `Vec<T>`
//! - 連想配列型
//! 	- `HashMap<T>`
//! 	- `FxHashMap<T>`
//! 	- `IndexMap<T>`
//! - スマートポインタ型
//! 	- `Cell<T>`

pub mod total_sizable;

use serde::{Deserialize, Serialize};

use crate::model::PropertiableObject;

pub use total_sizable::TotalSizable;

pub trait WeaverailObjectable<'a>: Serialize + Deserialize<'a> + PropertiableObject {}
