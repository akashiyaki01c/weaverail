//! Weaverail上の識別子を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - WeaverailId

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Weaverail上の識別子を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Default, Eq, Hash, Copy, Serialize, Deserialize)]
#[ts(as = "String")]
pub struct WeaverailId(pub u32);
impl WeaverailId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}
impl Display for WeaverailId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::fmt::Debug for WeaverailId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[macro_export]
macro_rules! weaverail_id {
    ($name: ident, $id: expr) => {
        #[derive(ts_rs::TS, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name(pub $crate::model::id::WeaverailId);

        impl $name {
            pub fn new(id: $crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                format!("{}{}", $id, self.0.to_string())
            }
        }
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use std::str::FromStr;

                let s = String::deserialize(deserializer)?;
                let id = &s[..4];
                let uuid = &s[4..];

                let number = u32::from_str(uuid);
                if let Ok(number) = number {
                    if id != $id {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self($crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
    };
}
