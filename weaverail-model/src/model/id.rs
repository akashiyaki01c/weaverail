//! Weaverail上の識別子を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - WeaverailId

use std::fmt::Display;

use base64::Engine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// Weaverail上の識別子を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Default, Eq, Hash, Copy)]
pub struct WeaverailId(pub Uuid);
impl WeaverailId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn to_string(&self) -> String {
        base64::prelude::BASE64_STANDARD.encode(self.0.as_bytes())
    }
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(uuid::Uuid::from_bytes(bytes))
    }
}
impl Serialize for WeaverailId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for WeaverailId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let vec = base64::prelude::BASE64_STANDARD
            .decode(s)
            .map_err(|_| serde::de::Error::custom("base64 is invalid"))?;
        let bytes: [u8; 16] = vec
            .try_into()
            .map_err(|_| serde::de::Error::custom("decoded data length is not 16 bytes"))?;
        Ok(Self(Uuid::from_bytes(bytes)))
    }
}
impl Display for WeaverailId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
impl std::fmt::Debug for WeaverailId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:08x}", self.0.as_fields().0))
    }
}

#[macro_export]
macro_rules! weaverail_id {
    ($name: ident, $id: expr) => {
        #[derive(ts_rs::TS, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name(pub $crate::model::id::WeaverailId);

        impl $name {
            pub fn new() -> Self {
                Self(crate::model::id::WeaverailId::new())
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
                use base64::Engine;

                let s = String::deserialize(deserializer)?;
                let id = &s[..4];
                let uuid = &s[4..];

                let vec = base64::prelude::BASE64_STANDARD
                    .decode(uuid)
                    .map_err(|_| serde::de::Error::custom("base64 is invalid"))?;
                let bytes: [u8; 16] = vec
                    .try_into()
                    .map_err(|_| serde::de::Error::custom("decoded data length is not 16 bytes"))?;
                if id != $id {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
                Ok(Self(crate::model::id::WeaverailId(Uuid::from_bytes(bytes))))
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&format!("{}{:08x}", $id, self.0.0.as_fields().0))
            }
        }
    };
}
