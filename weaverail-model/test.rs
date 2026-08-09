pub mod model {
    //! Weaverail上で用いられる全てのデータ構造を定義するモジュール
    pub mod diagram_view_settings {
        use serde::{Deserialize, Serialize};
        use crate::{
            error::ModelError,
            model::{
                DiagramRoot, ExtensionProperty, PropertiableObject, line::SegmentRef,
            },
            path::Heddle, weaverail_id,
        };
        pub struct DiagramViewSettingsId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for DiagramViewSettingsId {
            type WithoutGenerics = DiagramViewSettingsId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("DiagramViewSettingsId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("DiagramViewSettingsId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "DiagramViewSettingsId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <DiagramViewSettingsId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "DiagramViewSettingsId",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "DiagramViewSettingsId"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<crate::model::id::WeaverailId>();
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for DiagramViewSettingsId {}
        #[automatically_derived]
        impl ::core::clone::Clone for DiagramViewSettingsId {
            #[inline]
            fn clone(&self) -> DiagramViewSettingsId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for DiagramViewSettingsId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DiagramViewSettingsId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DiagramViewSettingsId {
            #[inline]
            fn eq(&self, other: &DiagramViewSettingsId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for DiagramViewSettingsId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for DiagramViewSettingsId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DiagramViewSettingsId {
            #[inline]
            fn default() -> DiagramViewSettingsId {
                DiagramViewSettingsId(::core::default::Default::default())
            }
        }
        impl DiagramViewSettingsId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "DVS_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for DiagramViewSettingsId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for DiagramViewSettingsId {
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
                    if id != "DVS_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for DiagramViewSettingsId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for DiagramViewSettingsId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for DiagramViewSettingsId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for DiagramViewSettingsId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(DiagramViewSettingsId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<DiagramViewSettingsId>
        for DiagramViewSettingsId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// ダイヤグラムを表示する際の設定を表す
        pub struct DiagramViewSettings {
            /// 設定の識別子
            pub id: DiagramViewSettingsId,
            /// 設定の名前
            pub name: String,
            /// ダイヤグラムの縦軸の区間の一覧を表す
            pub segments: Vec<DiagramViewSegment>,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for DiagramViewSettings {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "name" => Some(&self.name as &dyn crate::model::RnaObject),
                    "segments" => Some(&self.segments as &dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "name" => Some(&mut self.name as &mut dyn crate::model::RnaObject),
                    "segments" => {
                        Some(&mut self.segments as &mut dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "name" => {
                        self.name = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "segments" => {
                        self.segments = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("name".to_string()),
                    self.name.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("segments".to_string()),
                    self.segments.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for DiagramViewSettings {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<DiagramViewSettings>
        for DiagramViewSettings {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.segments.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.segments.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for DiagramViewSettings {
            type WithoutGenerics = DiagramViewSettings;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("DiagramViewSettings").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" ダイヤグラムを表示する際の設定を表す"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("DiagramViewSettings").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "DiagramViewSettings",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <DiagramViewSettings as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "DiagramViewSettings",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 設定の識別子"]),
                                                            ),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <DiagramViewSettingsId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 設定の名前"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[" ダイヤグラムの縦軸の区間の一覧を表す"],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "segments",
                                                    if false { "?" } else { "" },
                                                    <Vec<DiagramViewSegment> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 設定の識別子"]),
                                                            ),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <DiagramViewSettingsId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 設定の名前"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[" ダイヤグラムの縦軸の区間の一覧を表す"],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "segments",
                                                    if false { "?" } else { "" },
                                                    <Vec<DiagramViewSegment> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "DiagramViewSettings"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<DiagramViewSettingsId>();
                <DiagramViewSettingsId as ::ts_rs::TS>::visit_generics(v);
                <String as ::ts_rs::TS>::visit_generics(v);
                v.visit::<Vec<DiagramViewSegment>>();
                v.visit::<ExtensionProperty>();
                <Vec<DiagramViewSegment> as ::ts_rs::TS>::visit_generics(v);
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<String>();
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DiagramViewSettings {
            #[inline]
            fn clone(&self) -> DiagramViewSettings {
                DiagramViewSettings {
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    segments: ::core::clone::Clone::clone(&self.segments),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DiagramViewSettings {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DiagramViewSettings {
            #[inline]
            fn eq(&self, other: &DiagramViewSettings) -> bool {
                self.id == other.id && self.name == other.name
                    && self.segments == other.segments
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DiagramViewSettings {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "DiagramViewSettings",
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "segments",
                    &self.segments,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DiagramViewSettings {
            #[inline]
            fn default() -> DiagramViewSettings {
                DiagramViewSettings {
                    id: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    segments: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DiagramViewSettings {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "DiagramViewSettings",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "segments",
                        &self.segments,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DiagramViewSettings {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "name" => _serde::__private229::Ok(__Field::__field1),
                                "segments" => _serde::__private229::Ok(__Field::__field2),
                                "properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"name" => _serde::__private229::Ok(__Field::__field1),
                                b"segments" => _serde::__private229::Ok(__Field::__field2),
                                b"properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<DiagramViewSettings>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DiagramViewSettings;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct DiagramViewSettings",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                DiagramViewSettingsId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DiagramViewSettings with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DiagramViewSettings with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Vec<DiagramViewSegment>,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DiagramViewSettings with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DiagramViewSettings with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(DiagramViewSettings {
                                id: __field0,
                                name: __field1,
                                segments: __field2,
                                properties: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                DiagramViewSettingsId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<String> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<
                                Vec<DiagramViewSegment>,
                            > = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                DiagramViewSettingsId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "segments",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<DiagramViewSegment>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("name")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("segments")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(DiagramViewSettings {
                                id: __field0,
                                name: __field1,
                                segments: __field2,
                                properties: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "segments",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DiagramViewSettings",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<
                                DiagramViewSettings,
                            >,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl DiagramRoot {
            /// 駅間データが正常な値であるかを検証する
            pub fn validate_diagram_view_settings(
                &self,
                diagram_view_settings_id: DiagramViewSettingsId,
            ) -> Result<(), ModelError> {
                let diagram_view_settings = self
                    .diagram_view_settings
                    .get(&diagram_view_settings_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                for seg in &diagram_view_settings.segments {
                    if let DiagramViewSegment::StationBetween { segment } = seg {
                        let _ = self
                            .segments
                            .get(&segment.segment_id)
                            .ok_or(ModelError::ObjectNotFound)?;
                    }
                }
                Ok(())
            }
        }
        impl PropertiableObject for DiagramViewSettings {
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
        /// ダイヤグラムの縦軸の区間を表す
        pub enum DiagramViewSegment {
            /// 空白
            Black { scale: f32 },
            /// 駅間
            StationBetween { segment: SegmentRef },
        }
        impl crate::model::RnaObject for DiagramViewSegment {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match self {
                    Self::Black { scale } => {
                        match key {
                            "scale" => Some(scale as &dyn crate::model::RnaObject),
                            _ => None,
                        }
                    }
                    Self::StationBetween { segment } => {
                        match key {
                            "segment" => Some(segment as &dyn crate::model::RnaObject),
                            _ => None,
                        }
                    }
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match self {
                    Self::Black { scale } => {
                        match key {
                            "scale" => Some(scale as &mut dyn crate::model::RnaObject),
                            _ => None,
                        }
                    }
                    Self::StationBetween { segment } => {
                        match key {
                            "segment" => {
                                Some(segment as &mut dyn crate::model::RnaObject)
                            }
                            _ => None,
                        }
                    }
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match self {
                    Self::Black { scale } => {
                        match key {
                            "scale" => {
                                *scale = value
                                    .try_into()
                                    .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                                Ok(())
                            }
                            _ => {
                                Err(crate::model::RnaError::FieldNotFound(key.to_string()))
                            }
                        }
                    }
                    Self::StationBetween { segment } => {
                        match key {
                            "segment" => {
                                *segment = value
                                    .try_into()
                                    .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                                Ok(())
                            }
                            _ => {
                                Err(crate::model::RnaError::FieldNotFound(key.to_string()))
                            }
                        }
                    }
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for DiagramViewSegment {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<DiagramViewSegment> for DiagramViewSegment {
            fn get_stack_memory_size(&self) -> usize {
                match self {
                    Self::Black { scale } => 0 + scale.get_stack_memory_size(),
                    Self::StationBetween { segment } => {
                        0 + segment.get_stack_memory_size()
                    }
                }
            }
            fn get_heap_memory_size(&self) -> usize {
                match self {
                    Self::Black { scale } => 0 + scale.get_heap_memory_size(),
                    Self::StationBetween { segment } => {
                        0 + segment.get_heap_memory_size()
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for DiagramViewSegment {
            type WithoutGenerics = DiagramViewSegment;
            type OptionInnerType = Self;
            const IS_ENUM: bool = true;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("DiagramViewSegment").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" ダイヤグラムの縦軸の区間を表す"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("DiagramViewSegment").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "DiagramViewSegment",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <DiagramViewSegment as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "DiagramViewSegment",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                [
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ \"{0}\": {1} }}",
                                "Black",
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{{ {0} }}",
                                                <[String]>::join(
                                                    &[
                                                        ::alloc::__export::must_use({
                                                            ::alloc::fmt::format(
                                                                format_args!(
                                                                    "{0}{1}{2}: {3},",
                                                                    "",
                                                                    "scale",
                                                                    if false { "?" } else { "" },
                                                                    <f32 as ::ts_rs::TS>::name(cfg),
                                                                ),
                                                            )
                                                        }),
                                                    ],
                                                    " ",
                                                ),
                                            ),
                                        )
                                    })
                                    .replace(" } & { ", " "),
                            ),
                        )
                    }),
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ \"{0}\": {1} }}",
                                "StationBetween",
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{{ {0} }}",
                                                <[String]>::join(
                                                    &[
                                                        ::alloc::__export::must_use({
                                                            ::alloc::fmt::format(
                                                                format_args!(
                                                                    "{0}{1}{2}: {3},",
                                                                    "",
                                                                    "segment",
                                                                    if false { "?" } else { "" },
                                                                    <SegmentRef as ::ts_rs::TS>::name(cfg),
                                                                ),
                                                            )
                                                        }),
                                                    ],
                                                    " ",
                                                ),
                                            ),
                                        )
                                    })
                                    .replace(" } & { ", " "),
                            ),
                        )
                    }),
                ]
                    .join(" | ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "({0})",
                            [
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "{{ \"{0}\": {1} }}",
                                            "Black",
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "{{ {0} }}",
                                                            <[String]>::join(
                                                                &[
                                                                    ::alloc::__export::must_use({
                                                                        ::alloc::fmt::format(
                                                                            format_args!(
                                                                                "{0}{1}{2}: {3},",
                                                                                "",
                                                                                "scale",
                                                                                if false { "?" } else { "" },
                                                                                <f32 as ::ts_rs::TS>::name(cfg),
                                                                            ),
                                                                        )
                                                                    }),
                                                                ],
                                                                " ",
                                                            ),
                                                        ),
                                                    )
                                                })
                                                .replace(" } & { ", " "),
                                        ),
                                    )
                                }),
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "{{ \"{0}\": {1} }}",
                                            "StationBetween",
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "{{ {0} }}",
                                                            <[String]>::join(
                                                                &[
                                                                    ::alloc::__export::must_use({
                                                                        ::alloc::fmt::format(
                                                                            format_args!(
                                                                                "{0}{1}{2}: {3},",
                                                                                "",
                                                                                "segment",
                                                                                if false { "?" } else { "" },
                                                                                <SegmentRef as ::ts_rs::TS>::name(cfg),
                                                                            ),
                                                                        )
                                                                    }),
                                                                ],
                                                                " ",
                                                            ),
                                                        ),
                                                    )
                                                })
                                                .replace(" } & { ", " "),
                                        ),
                                    )
                                }),
                            ]
                                .join(" | "),
                        ),
                    )
                })
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "DiagramViewSegment"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <f32 as ::ts_rs::TS>::visit_generics(v);
                v.visit::<SegmentRef>();
                v.visit::<f32>();
                <SegmentRef as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DiagramViewSegment {
            #[inline]
            fn clone(&self) -> DiagramViewSegment {
                match self {
                    DiagramViewSegment::Black { scale: __self_0 } => {
                        DiagramViewSegment::Black {
                            scale: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    DiagramViewSegment::StationBetween { segment: __self_0 } => {
                        DiagramViewSegment::StationBetween {
                            segment: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DiagramViewSegment {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DiagramViewSegment {
            #[inline]
            fn eq(&self, other: &DiagramViewSegment) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            DiagramViewSegment::Black { scale: __self_0 },
                            DiagramViewSegment::Black { scale: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            DiagramViewSegment::StationBetween { segment: __self_0 },
                            DiagramViewSegment::StationBetween { segment: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DiagramViewSegment {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    DiagramViewSegment::Black { scale: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Black",
                            "scale",
                            &__self_0,
                        )
                    }
                    DiagramViewSegment::StationBetween { segment: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "StationBetween",
                            "segment",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DiagramViewSegment {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        DiagramViewSegment::Black { ref scale } => {
                            let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                                __serializer,
                                "DiagramViewSegment",
                                0u32,
                                "Black",
                                0 + 1,
                            )?;
                            _serde::ser::SerializeStructVariant::serialize_field(
                                &mut __serde_state,
                                "scale",
                                scale,
                            )?;
                            _serde::ser::SerializeStructVariant::end(__serde_state)
                        }
                        DiagramViewSegment::StationBetween { ref segment } => {
                            let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                                __serializer,
                                "DiagramViewSegment",
                                1u32,
                                "StationBetween",
                                0 + 1,
                            )?;
                            _serde::ser::SerializeStructVariant::serialize_field(
                                &mut __serde_state,
                                "segment",
                                segment,
                            )?;
                            _serde::ser::SerializeStructVariant::end(__serde_state)
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DiagramViewSegment {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private229::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Black" => _serde::__private229::Ok(__Field::__field0),
                                "StationBetween" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                _ => {
                                    _serde::__private229::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Black" => _serde::__private229::Ok(__Field::__field0),
                                b"StationBetween" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                _ => {
                                    let __value = &_serde::__private229::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private229::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<DiagramViewSegment>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DiagramViewSegment;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "enum DiagramViewSegment",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data) {
                                _serde::__private229::Ok(
                                    (__Field::__field0, __variant),
                                ) => {
                                    #[allow(non_camel_case_types)]
                                    #[doc(hidden)]
                                    enum __Field {
                                        __field0,
                                        __ignore,
                                    }
                                    #[doc(hidden)]
                                    struct __FieldVisitor;
                                    #[automatically_derived]
                                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                        type Value = __Field;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private229::Formatter,
                                        ) -> _serde::__private229::fmt::Result {
                                            _serde::__private229::Formatter::write_str(
                                                __formatter,
                                                "field identifier",
                                            )
                                        }
                                        fn visit_u64<__E>(
                                            self,
                                            __value: u64,
                                        ) -> _serde::__private229::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                                _ => _serde::__private229::Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_str<__E>(
                                            self,
                                            __value: &str,
                                        ) -> _serde::__private229::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                "scale" => _serde::__private229::Ok(__Field::__field0),
                                                _ => _serde::__private229::Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_bytes<__E>(
                                            self,
                                            __value: &[u8],
                                        ) -> _serde::__private229::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                b"scale" => _serde::__private229::Ok(__Field::__field0),
                                                _ => _serde::__private229::Ok(__Field::__ignore),
                                            }
                                        }
                                    }
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de> for __Field {
                                        #[inline]
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private229::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::Deserializer::deserialize_identifier(
                                                __deserializer,
                                                __FieldVisitor,
                                            )
                                        }
                                    }
                                    #[doc(hidden)]
                                    struct __Visitor<'de> {
                                        marker: _serde::__private229::PhantomData<
                                            DiagramViewSegment,
                                        >,
                                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                                    }
                                    #[automatically_derived]
                                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = DiagramViewSegment;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private229::Formatter,
                                        ) -> _serde::__private229::fmt::Result {
                                            _serde::__private229::Formatter::write_str(
                                                __formatter,
                                                "struct variant DiagramViewSegment::Black",
                                            )
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(
                                            self,
                                            mut __seq: __A,
                                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::SeqAccess<'de>,
                                        {
                                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                                f32,
                                            >(&mut __seq)? {
                                                _serde::__private229::Some(__value) => __value,
                                                _serde::__private229::None => {
                                                    return _serde::__private229::Err(
                                                        _serde::de::Error::invalid_length(
                                                            0usize,
                                                            &"struct variant DiagramViewSegment::Black with 1 element",
                                                        ),
                                                    );
                                                }
                                            };
                                            _serde::__private229::Ok(DiagramViewSegment::Black {
                                                scale: __field0,
                                            })
                                        }
                                        #[inline]
                                        fn visit_map<__A>(
                                            self,
                                            mut __map: __A,
                                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::MapAccess<'de>,
                                        {
                                            let mut __field0: _serde::__private229::Option<f32> = _serde::__private229::None;
                                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map)? {
                                                match __key {
                                                    __Field::__field0 => {
                                                        if _serde::__private229::Option::is_some(&__field0) {
                                                            return _serde::__private229::Err(
                                                                <__A::Error as _serde::de::Error>::duplicate_field("scale"),
                                                            );
                                                        }
                                                        __field0 = _serde::__private229::Some(
                                                            _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                                        );
                                                    }
                                                    _ => {
                                                        let _ = _serde::de::MapAccess::next_value::<
                                                            _serde::de::IgnoredAny,
                                                        >(&mut __map)?;
                                                    }
                                                }
                                            }
                                            let __field0 = match __field0 {
                                                _serde::__private229::Some(__field0) => __field0,
                                                _serde::__private229::None => {
                                                    _serde::__private229::de::missing_field("scale")?
                                                }
                                            };
                                            _serde::__private229::Ok(DiagramViewSegment::Black {
                                                scale: __field0,
                                            })
                                        }
                                    }
                                    #[doc(hidden)]
                                    const FIELDS: &'static [&'static str] = &["scale"];
                                    _serde::de::VariantAccess::struct_variant(
                                        __variant,
                                        FIELDS,
                                        __Visitor {
                                            marker: _serde::__private229::PhantomData::<
                                                DiagramViewSegment,
                                            >,
                                            lifetime: _serde::__private229::PhantomData,
                                        },
                                    )
                                }
                                _serde::__private229::Ok(
                                    (__Field::__field1, __variant),
                                ) => {
                                    #[allow(non_camel_case_types)]
                                    #[doc(hidden)]
                                    enum __Field {
                                        __field0,
                                        __ignore,
                                    }
                                    #[doc(hidden)]
                                    struct __FieldVisitor;
                                    #[automatically_derived]
                                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                        type Value = __Field;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private229::Formatter,
                                        ) -> _serde::__private229::fmt::Result {
                                            _serde::__private229::Formatter::write_str(
                                                __formatter,
                                                "field identifier",
                                            )
                                        }
                                        fn visit_u64<__E>(
                                            self,
                                            __value: u64,
                                        ) -> _serde::__private229::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                                _ => _serde::__private229::Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_str<__E>(
                                            self,
                                            __value: &str,
                                        ) -> _serde::__private229::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                "segment" => _serde::__private229::Ok(__Field::__field0),
                                                _ => _serde::__private229::Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_bytes<__E>(
                                            self,
                                            __value: &[u8],
                                        ) -> _serde::__private229::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                b"segment" => _serde::__private229::Ok(__Field::__field0),
                                                _ => _serde::__private229::Ok(__Field::__ignore),
                                            }
                                        }
                                    }
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de> for __Field {
                                        #[inline]
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private229::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::Deserializer::deserialize_identifier(
                                                __deserializer,
                                                __FieldVisitor,
                                            )
                                        }
                                    }
                                    #[doc(hidden)]
                                    struct __Visitor<'de> {
                                        marker: _serde::__private229::PhantomData<
                                            DiagramViewSegment,
                                        >,
                                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                                    }
                                    #[automatically_derived]
                                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = DiagramViewSegment;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private229::Formatter,
                                        ) -> _serde::__private229::fmt::Result {
                                            _serde::__private229::Formatter::write_str(
                                                __formatter,
                                                "struct variant DiagramViewSegment::StationBetween",
                                            )
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(
                                            self,
                                            mut __seq: __A,
                                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::SeqAccess<'de>,
                                        {
                                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                                SegmentRef,
                                            >(&mut __seq)? {
                                                _serde::__private229::Some(__value) => __value,
                                                _serde::__private229::None => {
                                                    return _serde::__private229::Err(
                                                        _serde::de::Error::invalid_length(
                                                            0usize,
                                                            &"struct variant DiagramViewSegment::StationBetween with 1 element",
                                                        ),
                                                    );
                                                }
                                            };
                                            _serde::__private229::Ok(DiagramViewSegment::StationBetween {
                                                segment: __field0,
                                            })
                                        }
                                        #[inline]
                                        fn visit_map<__A>(
                                            self,
                                            mut __map: __A,
                                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::MapAccess<'de>,
                                        {
                                            let mut __field0: _serde::__private229::Option<
                                                SegmentRef,
                                            > = _serde::__private229::None;
                                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map)? {
                                                match __key {
                                                    __Field::__field0 => {
                                                        if _serde::__private229::Option::is_some(&__field0) {
                                                            return _serde::__private229::Err(
                                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                                    "segment",
                                                                ),
                                                            );
                                                        }
                                                        __field0 = _serde::__private229::Some(
                                                            _serde::de::MapAccess::next_value::<SegmentRef>(&mut __map)?,
                                                        );
                                                    }
                                                    _ => {
                                                        let _ = _serde::de::MapAccess::next_value::<
                                                            _serde::de::IgnoredAny,
                                                        >(&mut __map)?;
                                                    }
                                                }
                                            }
                                            let __field0 = match __field0 {
                                                _serde::__private229::Some(__field0) => __field0,
                                                _serde::__private229::None => {
                                                    _serde::__private229::de::missing_field("segment")?
                                                }
                                            };
                                            _serde::__private229::Ok(DiagramViewSegment::StationBetween {
                                                segment: __field0,
                                            })
                                        }
                                    }
                                    #[doc(hidden)]
                                    const FIELDS: &'static [&'static str] = &["segment"];
                                    _serde::de::VariantAccess::struct_variant(
                                        __variant,
                                        FIELDS,
                                        __Visitor {
                                            marker: _serde::__private229::PhantomData::<
                                                DiagramViewSegment,
                                            >,
                                            lifetime: _serde::__private229::PhantomData,
                                        },
                                    )
                                }
                                _serde::__private229::Err(__err) => {
                                    _serde::__private229::Err(__err)
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &[
                        "Black",
                        "StationBetween",
                    ];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "DiagramViewSegment",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<
                                DiagramViewSegment,
                            >,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl ::core::str::FromStr for DiagramViewSegment {
            type Err = ::strum::ParseError;
            #[inline]
            fn from_str(
                s: &str,
            ) -> ::core::result::Result<
                DiagramViewSegment,
                <Self as ::core::str::FromStr>::Err,
            > {
                ::core::result::Result::Ok(
                    match s {
                        "Black" => {
                            DiagramViewSegment::Black {
                                scale: Default::default(),
                            }
                        }
                        "StationBetween" => {
                            DiagramViewSegment::StationBetween {
                                segment: Default::default(),
                            }
                        }
                        _ => {
                            return ::core::result::Result::Err(
                                ::strum::ParseError::VariantNotFound,
                            );
                        }
                    },
                )
            }
        }
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl ::core::convert::TryFrom<&str> for DiagramViewSegment {
            type Error = ::strum::ParseError;
            #[inline]
            fn try_from(
                s: &str,
            ) -> ::core::result::Result<
                DiagramViewSegment,
                <Self as ::core::convert::TryFrom<&str>>::Error,
            > {
                ::core::str::FromStr::from_str(s)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Display for DiagramViewSegment {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match *self {
                    DiagramViewSegment::Black { ref scale } => {
                        ::core::fmt::Display::fmt("Black", f)
                    }
                    DiagramViewSegment::StationBetween { ref segment } => {
                        ::core::fmt::Display::fmt("StationBetween", f)
                    }
                }
            }
        }
        impl Default for DiagramViewSegment {
            fn default() -> Self {
                Self::Black { scale: 1.0 }
            }
        }
    }
    pub mod id {
        //! Weaverail上の識別子を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
        //! - WeaverailId
        use std::fmt::Display;
        use serde::{Deserialize, Serialize};
        /// Weaverail上の識別子を表す構造体
        #[ts(as = "String")]
        pub struct WeaverailId(pub u32);
        #[automatically_derived]
        impl ::ts_rs::TS for WeaverailId {
            type WithoutGenerics = WeaverailId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <String as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("WeaverailId").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" Weaverail上の識別子を表す構造体"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("WeaverailId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "WeaverailId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <WeaverailId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "WeaverailId",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <String as ::ts_rs::TS>::inline(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "WeaverailId"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <String as ::ts_rs::TS>::visit_dependencies(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for WeaverailId {}
        #[automatically_derived]
        impl ::core::clone::Clone for WeaverailId {
            #[inline]
            fn clone(&self) -> WeaverailId {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for WeaverailId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for WeaverailId {
            #[inline]
            fn eq(&self, other: &WeaverailId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for WeaverailId {
            #[inline]
            fn default() -> WeaverailId {
                WeaverailId(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for WeaverailId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for WeaverailId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for WeaverailId {}
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for WeaverailId {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_newtype_struct(
                        __serializer,
                        "WeaverailId",
                        &self.0,
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for WeaverailId {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<WeaverailId>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = WeaverailId;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "tuple struct WeaverailId",
                            )
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(
                            self,
                            __e: __E,
                        ) -> _serde::__private229::Result<Self::Value, __E::Error>
                        where
                            __E: _serde::Deserializer<'de>,
                        {
                            let __field0: u32 = <u32 as _serde::Deserialize>::deserialize(
                                __e,
                            )?;
                            _serde::__private229::Ok(WeaverailId(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u32,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct WeaverailId with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(WeaverailId(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        "WeaverailId",
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<WeaverailId>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl WeaverailId {
            pub fn new(id: u32) -> Self {
                Self(id)
            }
        }
        impl Display for WeaverailId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{0}", self.0))
            }
        }
        impl std::fmt::Debug for WeaverailId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for WeaverailId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                Some(crate::path::Heddle::Id(*self))
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for WeaverailId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(id)
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
    }
    pub mod line {
        //! Weaverail上の「路線」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
        //! - Line (路線)
        //!   - LineSegment (駅間)
        use std::iter;
        use indexmap::map::Entry;
        use serde::{Deserialize, Serialize};
        use crate::path::Heddle;
        use crate::{
            error::ModelError,
            model::{
                DiagramRoot, ExtensionProperty, PropertiableObject,
                line_segment::{LineSegment, LineSegmentId},
                station::{Station, StationId},
            },
            weaverail_id,
        };
        pub struct LineId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for LineId {
            type WithoutGenerics = LineId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("LineId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("LineId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "LineId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <LineId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "LineId", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "LineId"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<crate::model::id::WeaverailId>();
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for LineId {}
        #[automatically_derived]
        impl ::core::clone::Clone for LineId {
            #[inline]
            fn clone(&self) -> LineId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for LineId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for LineId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for LineId {
            #[inline]
            fn eq(&self, other: &LineId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for LineId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for LineId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for LineId {
            #[inline]
            fn default() -> LineId {
                LineId(::core::default::Default::default())
            }
        }
        impl LineId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "LIN_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for LineId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for LineId {
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
                    if id != "LIN_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for LineId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for LineId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for LineId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for LineId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(LineId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<LineId> for LineId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// 駅間への参照を表す構造体
        pub struct SegmentRef {
            pub segment_id: LineSegmentId,
            pub is_reversed: bool,
        }
        impl crate::model::RnaObject for SegmentRef {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "segment_id" => {
                        Some(&self.segment_id as &dyn crate::model::RnaObject)
                    }
                    "is_reversed" => {
                        Some(&self.is_reversed as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "segment_id" => {
                        Some(&mut self.segment_id as &mut dyn crate::model::RnaObject)
                    }
                    "is_reversed" => {
                        Some(&mut self.is_reversed as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "segment_id" => {
                        self.segment_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "is_reversed" => {
                        self.is_reversed = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("segment_id".to_string()),
                    self.segment_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("is_reversed".to_string()),
                    self.is_reversed.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for SegmentRef {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<SegmentRef> for SegmentRef {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.segment_id.get_stack_memory_size()
                    + self.is_reversed.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.segment_id.get_stack_memory_size()
                    + self.is_reversed.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for SegmentRef {
            type WithoutGenerics = SegmentRef;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("SegmentRef").to_string()
            }
            fn docs() -> Option<String> {
                Some(::ts_rs::format_docs(&[" 駅間への参照を表す構造体"]))
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("SegmentRef").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "SegmentRef",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <SegmentRef as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "SegmentRef",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "segment_id",
                                                    if false { "?" } else { "" },
                                                    <LineSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "is_reversed",
                                                    if false { "?" } else { "" },
                                                    <bool as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "segment_id",
                                                    if false { "?" } else { "" },
                                                    <LineSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "is_reversed",
                                                    if false { "?" } else { "" },
                                                    <bool as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "SegmentRef"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <LineSegmentId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<LineSegmentId>();
                v.visit::<bool>();
                <bool as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SegmentRef {
            #[inline]
            fn clone(&self) -> SegmentRef {
                SegmentRef {
                    segment_id: ::core::clone::Clone::clone(&self.segment_id),
                    is_reversed: ::core::clone::Clone::clone(&self.is_reversed),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SegmentRef {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SegmentRef {
            #[inline]
            fn eq(&self, other: &SegmentRef) -> bool {
                self.is_reversed == other.is_reversed
                    && self.segment_id == other.segment_id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SegmentRef {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SegmentRef",
                    "segment_id",
                    &self.segment_id,
                    "is_reversed",
                    &&self.is_reversed,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for SegmentRef {
            #[inline]
            fn default() -> SegmentRef {
                SegmentRef {
                    segment_id: ::core::default::Default::default(),
                    is_reversed: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for SegmentRef {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "SegmentRef",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "segment_id",
                        &self.segment_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "is_reversed",
                        &self.is_reversed,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for SegmentRef {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "segment_id" => _serde::__private229::Ok(__Field::__field0),
                                "is_reversed" => _serde::__private229::Ok(__Field::__field1),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"segment_id" => _serde::__private229::Ok(__Field::__field0),
                                b"is_reversed" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<SegmentRef>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = SegmentRef;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct SegmentRef",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                LineSegmentId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct SegmentRef with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct SegmentRef with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(SegmentRef {
                                segment_id: __field0,
                                is_reversed: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                LineSegmentId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<bool> = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "segment_id",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                LineSegmentId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "is_reversed",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("segment_id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("is_reversed")?
                                }
                            };
                            _serde::__private229::Ok(SegmentRef {
                                segment_id: __field0,
                                is_reversed: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "segment_id",
                        "is_reversed",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "SegmentRef",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<SegmentRef>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        /// Weaverail上の1つの路線を表す構造体
        pub struct Line {
            /// 識別ID
            pub id: LineId,
            /// 路線名 (例: "神明線")
            pub name: String,
            /// 路線に所属する駅間リスト
            pub segments: Vec<SegmentRef>,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for Line {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "name" => Some(&self.name as &dyn crate::model::RnaObject),
                    "segments" => Some(&self.segments as &dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "name" => Some(&mut self.name as &mut dyn crate::model::RnaObject),
                    "segments" => {
                        Some(&mut self.segments as &mut dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "name" => {
                        self.name = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "segments" => {
                        self.segments = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("name".to_string()),
                    self.name.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("segments".to_string()),
                    self.segments.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for Line {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<Line> for Line {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.segments.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.segments.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for Line {
            type WithoutGenerics = Line;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("Line").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" Weaverail上の1つの路線を表す構造体"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("Line").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "Line",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <Line as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "Line", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <LineId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 路線名 (例: \"神明線\")"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[" 路線に所属する駅間リスト"],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "segments",
                                                    if false { "?" } else { "" },
                                                    <Vec<SegmentRef> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <LineId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 路線名 (例: \"神明線\")"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[" 路線に所属する駅間リスト"],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "segments",
                                                    if false { "?" } else { "" },
                                                    <Vec<SegmentRef> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "Line"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<String>();
                <String as ::ts_rs::TS>::visit_generics(v);
                v.visit::<LineId>();
                <LineId as ::ts_rs::TS>::visit_generics(v);
                <Vec<SegmentRef> as ::ts_rs::TS>::visit_generics(v);
                v.visit::<ExtensionProperty>();
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<Vec<SegmentRef>>();
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Line {
            #[inline]
            fn clone(&self) -> Line {
                Line {
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    segments: ::core::clone::Clone::clone(&self.segments),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Line {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Line {
            #[inline]
            fn eq(&self, other: &Line) -> bool {
                self.id == other.id && self.name == other.name
                    && self.segments == other.segments
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Line {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Line",
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "segments",
                    &self.segments,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Line {
            #[inline]
            fn default() -> Line {
                Line {
                    id: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    segments: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Line {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Line",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "segments",
                        &self.segments,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Line {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "name" => _serde::__private229::Ok(__Field::__field1),
                                "segments" => _serde::__private229::Ok(__Field::__field2),
                                "properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"name" => _serde::__private229::Ok(__Field::__field1),
                                b"segments" => _serde::__private229::Ok(__Field::__field2),
                                b"properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<Line>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Line;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct Line",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                LineId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Line with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Line with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Vec<SegmentRef>,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Line with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Line with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(Line {
                                id: __field0,
                                name: __field1,
                                segments: __field2,
                                properties: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<LineId> = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<String> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<
                                Vec<SegmentRef>,
                            > = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<LineId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "segments",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<SegmentRef>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("name")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("segments")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(Line {
                                id: __field0,
                                name: __field1,
                                segments: __field2,
                                properties: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "segments",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Line",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<Line>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl Line {
            pub fn new(id: LineId, name: &str, stations: &[SegmentRef]) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    segments: stations.into(),
                    ..Default::default()
                }
            }
            /// 駅間リストを取得する関数
            /// 計算量は `O(segments.len())`
            pub fn segments<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<Vec<&'a LineSegment>, ModelError> {
                self.segments
                    .iter()
                    .map(|id| {
                        root.segments
                            .get(&id.segment_id)
                            .ok_or(ModelError::ObjectNotFound)
                    })
                    .collect()
            }
            /// 最初の駅間を取得する関数
            pub fn first_segment<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<Option<(&'a LineSegment, bool)>, ModelError> {
                if let Some(segment_id) = self.segments.first() {
                    if let Some(segment) = root.segments.get(&segment_id.segment_id) {
                        Ok(Some((segment, segment_id.is_reversed)))
                    } else {
                        Err(ModelError::ObjectNotFound)
                    }
                } else {
                    Ok(None)
                }
            }
            /// 最後の駅間を取得する関数
            pub fn last_segment<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<Option<(&'a LineSegment, bool)>, ModelError> {
                if let Some(segment_id) = self.segments.last() {
                    if let Some(segment) = root.segments.get(&segment_id.segment_id) {
                        Ok(Some((segment, segment_id.is_reversed)))
                    } else {
                        Err(ModelError::ObjectNotFound)
                    }
                } else {
                    Ok(None)
                }
            }
            /// 先頭の駅IDを返す関数
            pub fn first_station_id(
                &self,
                root: &DiagramRoot,
            ) -> Result<Option<StationId>, ModelError> {
                let segment = self.first_segment(root)?;
                if let Some(segment) = segment {
                    if segment.1 {
                        Ok(Some(segment.0.end_station))
                    } else {
                        Ok(Some(segment.0.start_station))
                    }
                } else {
                    Ok(None)
                }
            }
            /// 末尾の駅IDを返す関数
            pub fn last_station_id(
                &self,
                root: &DiagramRoot,
            ) -> Result<Option<StationId>, ModelError> {
                let segment = self.last_segment(root)?;
                if let Some(segment) = segment {
                    if segment.1 {
                        Ok(Some(segment.0.start_station))
                    } else {
                        Ok(Some(segment.0.end_station))
                    }
                } else {
                    Ok(None)
                }
            }
        }
        impl PropertiableObject for Line {
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
        impl DiagramRoot {
            /// 路線を追加する関数
            /// 計算オーダは`O(1)`
            /// 既に同一IDの路線が存在している場合はエラーを返す
            pub fn add_line(&mut self, line: Line) -> Result<(), ModelError> {
                match self.lines.entry(line.id) {
                    Entry::Vacant(entry) => {
                        entry.insert(line);
                        Ok(())
                    }
                    Entry::Occupied(_) => Err(ModelError::DuplicateKey),
                }
            }
            /// 路線を削除する関数
            /// 計算オーダは`O(1)`
            /// 指定IDの路線が存在しない場合はエラーを返す
            pub fn delete_line(&mut self, line_id: LineId) -> Result<Line, ModelError> {
                self.lines.shift_remove(&line_id).ok_or(ModelError::ObjectNotFound)
            }
            /// 路線に所属する駅を取得する関数
            pub fn get_line_stations(
                &self,
                line: &Line,
            ) -> Result<Vec<&Station>, ModelError> {
                if line.segments.is_empty() {
                    return Ok(Vec::new());
                }
                let first_segment = self
                    .segments
                    .get(
                        &line
                            .segments
                            .first()
                            .ok_or(ModelError::ObjectNotFound)?
                            .segment_id,
                    )
                    .ok_or(ModelError::ObjectNotFound)?;
                let start_id = first_segment.start_station;
                let end_ids: Result<Vec<_>, ModelError> = line
                    .segments
                    .iter()
                    .map(|segment_id| {
                        let segment = self
                            .segments
                            .get(&segment_id.segment_id)
                            .ok_or(ModelError::ObjectNotFound)?;
                        Ok(segment.end_station)
                    })
                    .collect();
                let end_ids = end_ids?;
                let result: Result<Vec<_>, _> = iter::once(start_id)
                    .chain(end_ids)
                    .map(|station_id| {
                        self.stations.get(&station_id).ok_or(ModelError::ObjectNotFound)
                    })
                    .collect();
                result
            }
            /// SegmentIdから駅間を取得する関数
            pub fn get_segment(
                &self,
                segment_id: LineSegmentId,
            ) -> Option<&LineSegment> {
                self.segments.values().find(|segment| segment.id == segment_id)
            }
            /// 路線の末尾に駅間を追加する関数
            pub fn push_back_line_segment(
                &mut self,
                line_id: LineId,
                segment_id: LineSegmentId,
                is_reversed: bool,
            ) -> Result<(), ModelError> {
                let (start_station_id, end_station_id) = {
                    let segment = self
                        .segments
                        .get(&segment_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    (segment.start_station, segment.end_station)
                };
                let line: &Line = self
                    .lines
                    .get(&line_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                let last_station = line.last_station_id(self)?;
                let line: &mut Line = self
                    .lines
                    .get_mut(&line_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                if let Some(last_station) = last_station {
                    let is_valid = if is_reversed {
                        end_station_id == last_station
                    } else {
                        start_station_id == last_station
                    };
                    if !is_valid {
                        return Err(ModelError::Error);
                    }
                }
                line.segments
                    .push(SegmentRef {
                        segment_id,
                        is_reversed,
                    });
                Ok(())
            }
            /// 路線の先頭に駅間を追加する関数
            pub fn push_front_line_segment(
                &mut self,
                line_id: LineId,
                segment_id: LineSegmentId,
                is_reversed: bool,
            ) -> Result<(), ModelError> {
                let (start_station_id, end_station_id) = {
                    let segment = self
                        .segments
                        .get(&segment_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    (segment.start_station, segment.end_station)
                };
                let line: &Line = self
                    .lines
                    .get(&line_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                let first_station = line.first_station_id(self)?;
                let line: &mut Line = self
                    .lines
                    .get_mut(&line_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                if let Some(last_station) = first_station {
                    let is_valid = if is_reversed {
                        start_station_id == last_station
                    } else {
                        end_station_id == last_station
                    };
                    if !is_valid {
                        return Err(ModelError::Error);
                    }
                }
                line.segments
                    .insert(
                        0,
                        SegmentRef {
                            segment_id,
                            is_reversed,
                        },
                    );
                Ok(())
            }
            /// 路線の末尾の駅間を削除する関数
            pub fn pop_back_line_segment(
                &mut self,
                line_id: LineId,
            ) -> Result<SegmentRef, ModelError> {
                let line = self
                    .lines
                    .get_mut(&line_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                line.segments.pop().ok_or(ModelError::Empty)
            }
            /// 路線の先頭の駅間を削除する関数
            pub fn pop_front_line_segment(
                &mut self,
                line_id: LineId,
            ) -> Result<SegmentRef, ModelError> {
                let line = self
                    .lines
                    .get_mut(&line_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                if line.segments.is_empty() {
                    return Err(ModelError::Empty);
                }
                Ok(line.segments.remove(0))
            }
            /// 駅間を、開始/終了駅名から検索する関数
            pub fn find_segment_by_name(
                &self,
                start_station_name: &str,
                end_station_name: &str,
            ) -> Result<&SegmentRef, ModelError> {
                let start_station = self
                    .find_station_by_name(start_station_name)
                    .ok_or(ModelError::ObjectNotFound)?
                    .id;
                let end_station = self
                    .find_station_by_name(end_station_name)
                    .ok_or(ModelError::ObjectNotFound)?
                    .id;
                let mut reversed_segment = None;
                for segment_ref in self.lines.values().flat_map(|line| &line.segments) {
                    let segment = self
                        .segments
                        .get(&segment_ref.segment_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    if segment.start_station == start_station
                        && segment.end_station == end_station
                    {
                        reversed_segment = Some(segment_ref);
                        break;
                    }
                }
                let mut forward_segment = None;
                for segment_ref in self.lines.values().flat_map(|line| &line.segments) {
                    let segment = self
                        .segments
                        .get(&segment_ref.segment_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    if segment.start_station == end_station
                        && segment.end_station == start_station
                    {
                        forward_segment = Some(segment_ref);
                        break;
                    }
                }
                if let Some(forward_segment) = forward_segment {
                    Ok(forward_segment)
                } else {
                    Ok(reversed_segment.ok_or(ModelError::ObjectNotFound)?)
                }
            }
            /// 路線データが正常な値であるかを検証する
            pub fn validate_line(&self, line_id: LineId) -> Result<(), ModelError> {
                let line = self.lines.get(&line_id).ok_or(ModelError::ObjectNotFound)?;
                for seg in &line.segments {
                    let _ = self
                        .get_segment(seg.segment_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                }
                Ok(())
            }
        }
    }
    pub mod line_segment {
        use indexmap::map::Entry;
        use serde::{Deserialize, Serialize};
        use crate::path::Heddle;
        use crate::{
            error::ModelError,
            model::{
                DiagramRoot, ExtensionProperty, PropertiableObject,
                station::{Station, StationId},
            },
            weaverail_id,
        };
        pub struct LineSegmentId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for LineSegmentId {
            type WithoutGenerics = LineSegmentId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("LineSegmentId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("LineSegmentId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "LineSegmentId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <LineSegmentId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "LineSegmentId",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "LineSegmentId"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<crate::model::id::WeaverailId>();
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for LineSegmentId {}
        #[automatically_derived]
        impl ::core::clone::Clone for LineSegmentId {
            #[inline]
            fn clone(&self) -> LineSegmentId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for LineSegmentId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for LineSegmentId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for LineSegmentId {
            #[inline]
            fn eq(&self, other: &LineSegmentId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for LineSegmentId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for LineSegmentId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for LineSegmentId {
            #[inline]
            fn default() -> LineSegmentId {
                LineSegmentId(::core::default::Default::default())
            }
        }
        impl LineSegmentId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "SGM_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for LineSegmentId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for LineSegmentId {
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
                    if id != "SGM_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for LineSegmentId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for LineSegmentId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for LineSegmentId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for LineSegmentId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(LineSegmentId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<LineSegmentId> for LineSegmentId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// Weaverail上の1つの路線に属する駅間を表す構造体
        pub struct LineSegment {
            /// 識別ID
            pub id: LineSegmentId,
            /// 開始駅ID
            pub start_station: StationId,
            /// 終了駅ID
            pub end_station: StationId,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for LineSegment {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "start_station" => {
                        Some(&self.start_station as &dyn crate::model::RnaObject)
                    }
                    "end_station" => {
                        Some(&self.end_station as &dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "start_station" => {
                        Some(&mut self.start_station as &mut dyn crate::model::RnaObject)
                    }
                    "end_station" => {
                        Some(&mut self.end_station as &mut dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "start_station" => {
                        self.start_station = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "end_station" => {
                        self.end_station = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("start_station".to_string()),
                    self.start_station.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("end_station".to_string()),
                    self.end_station.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for LineSegment {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<LineSegment> for LineSegment {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.start_station.get_stack_memory_size()
                    + self.end_station.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.start_station.get_stack_memory_size()
                    + self.end_station.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for LineSegment {
            type WithoutGenerics = LineSegment;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("LineSegment").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[
                            " Weaverail上の1つの路線に属する駅間を表す構造体",
                        ],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("LineSegment").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "LineSegment",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <LineSegment as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "LineSegment",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <LineSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 開始駅ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "start_station",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 終了駅ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "end_station",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <LineSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 開始駅ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "start_station",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 終了駅ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "end_station",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "LineSegment"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<LineSegmentId>();
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<StationId>();
                <LineSegmentId as ::ts_rs::TS>::visit_generics(v);
                <StationId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<ExtensionProperty>();
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LineSegment {
            #[inline]
            fn clone(&self) -> LineSegment {
                LineSegment {
                    id: ::core::clone::Clone::clone(&self.id),
                    start_station: ::core::clone::Clone::clone(&self.start_station),
                    end_station: ::core::clone::Clone::clone(&self.end_station),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for LineSegment {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for LineSegment {
            #[inline]
            fn eq(&self, other: &LineSegment) -> bool {
                self.id == other.id && self.start_station == other.start_station
                    && self.end_station == other.end_station
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LineSegment {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "LineSegment",
                    "id",
                    &self.id,
                    "start_station",
                    &self.start_station,
                    "end_station",
                    &self.end_station,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for LineSegment {
            #[inline]
            fn default() -> LineSegment {
                LineSegment {
                    id: ::core::default::Default::default(),
                    start_station: ::core::default::Default::default(),
                    end_station: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for LineSegment {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "LineSegment",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "start_station",
                        &self.start_station,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "end_station",
                        &self.end_station,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for LineSegment {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "start_station" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                "end_station" => _serde::__private229::Ok(__Field::__field2),
                                "properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"start_station" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                b"end_station" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                b"properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<LineSegment>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = LineSegment;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct LineSegment",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                LineSegmentId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct LineSegment with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                StationId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct LineSegment with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                StationId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct LineSegment with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct LineSegment with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(LineSegment {
                                id: __field0,
                                start_station: __field1,
                                end_station: __field2,
                                properties: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                LineSegmentId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<StationId> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<StationId> = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                LineSegmentId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "start_station",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<StationId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "end_station",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<StationId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("start_station")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("end_station")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(LineSegment {
                                id: __field0,
                                start_station: __field1,
                                end_station: __field2,
                                properties: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "start_station",
                        "end_station",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "LineSegment",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<LineSegment>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl LineSegment {
            pub fn new(
                id: LineSegmentId,
                start_station: StationId,
                end_station: StationId,
            ) -> Self {
                Self {
                    id,
                    start_station,
                    end_station,
                    ..Default::default()
                }
            }
            /// 開始駅を取得する関数
            /// 計算量は `O(1)`
            pub fn start_station<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a Station, ModelError> {
                root.stations.get(&self.start_station).ok_or(ModelError::ObjectNotFound)
            }
            /// 終了駅を取得する関数
            /// 計算量は `O(1)`
            pub fn end_station<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a Station, ModelError> {
                root.stations.get(&self.end_station).ok_or(ModelError::ObjectNotFound)
            }
            /// 駅間が指定駅を参照しているか
            pub fn contains_station(&self, station_id: StationId) -> bool {
                self.start_station == station_id || self.end_station == station_id
            }
        }
        impl DiagramRoot {
            /// 駅間を追加する関数
            /// 計算オーダは `O(1)`
            /// 既に同一IDの駅間が存在している場合はエラーを返す
            pub fn add_segment(
                &mut self,
                segment: LineSegment,
            ) -> Result<(), ModelError> {
                match self.segments.entry(segment.id) {
                    Entry::Vacant(entry) => {
                        entry.insert(segment);
                        Ok(())
                    }
                    Entry::Occupied(_) => Err(ModelError::DuplicateKey),
                }
            }
            /// 駅間を削除する関数
            /// 計算オーダは `O(segments.len + template_trains.len)`
            /// 指定IDの駅が存在しない場合はエラーを返す
            /// 路線から参照されている場合はエラーを返す
            /// テンプレート列車から参照されている場合はエラーを返す
            pub fn delete_segment(
                &mut self,
                segment_id: LineSegmentId,
            ) -> Result<LineSegment, ModelError> {
                if self
                    .lines
                    .values()
                    .any(|line| {
                        line.segments.iter().any(|seg| seg.segment_id == segment_id)
                    })
                {
                    return Err(ModelError::ExternalReferenced);
                }
                if self
                    .template_trains
                    .values()
                    .any(|train| train.contains_segment(segment_id))
                {
                    return Err(ModelError::ExternalReferenced);
                }
                self.segments.shift_remove(&segment_id).ok_or(ModelError::ObjectNotFound)
            }
            /// 駅間データが正常な値であるかを検証する
            pub fn validate_segment(
                &self,
                segment_id: LineSegmentId,
            ) -> Result<(), ModelError> {
                let segment = self
                    .get_segment(segment_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                let _ = segment.start_station(self)?;
                let _ = segment.end_station(self)?;
                Ok(())
            }
        }
        impl PropertiableObject for LineSegment {
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
    }
    pub mod segment_train_order {
        use serde::{Deserialize, Serialize};
        use crate::model::{
            ExtensionProperty, PropertiableObject, line_segment::LineSegmentId,
            train::TrainId,
        };
        use crate::path::Heddle;
        /// 駅間での列車の順序を表す
        pub struct SegmentTrainOrder {
            pub segment_id: LineSegmentId,
            pub is_reversed: bool,
            pub order: Vec<TrainId>,
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for SegmentTrainOrder {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "segment_id" => {
                        Some(&self.segment_id as &dyn crate::model::RnaObject)
                    }
                    "is_reversed" => {
                        Some(&self.is_reversed as &dyn crate::model::RnaObject)
                    }
                    "order" => Some(&self.order as &dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "segment_id" => {
                        Some(&mut self.segment_id as &mut dyn crate::model::RnaObject)
                    }
                    "is_reversed" => {
                        Some(&mut self.is_reversed as &mut dyn crate::model::RnaObject)
                    }
                    "order" => Some(&mut self.order as &mut dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "segment_id" => {
                        self.segment_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "is_reversed" => {
                        self.is_reversed = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "order" => {
                        self.order = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("segment_id".to_string()),
                    self.segment_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("is_reversed".to_string()),
                    self.is_reversed.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("order".to_string()),
                    self.order.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for SegmentTrainOrder {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<SegmentTrainOrder> for SegmentTrainOrder {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.segment_id.get_stack_memory_size()
                    + self.is_reversed.get_stack_memory_size()
                    + self.order.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.segment_id.get_stack_memory_size()
                    + self.is_reversed.get_stack_memory_size()
                    + self.order.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for SegmentTrainOrder {
            type WithoutGenerics = SegmentTrainOrder;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("SegmentTrainOrder").to_string()
            }
            fn docs() -> Option<String> {
                Some(::ts_rs::format_docs(&[" 駅間での列車の順序を表す"]))
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("SegmentTrainOrder").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "SegmentTrainOrder",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <SegmentTrainOrder as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "SegmentTrainOrder",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "segment_id",
                                                    if false { "?" } else { "" },
                                                    <LineSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "is_reversed",
                                                    if false { "?" } else { "" },
                                                    <bool as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "order",
                                                    if false { "?" } else { "" },
                                                    <Vec<TrainId> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "segment_id",
                                                    if false { "?" } else { "" },
                                                    <LineSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "is_reversed",
                                                    if false { "?" } else { "" },
                                                    <bool as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "order",
                                                    if false { "?" } else { "" },
                                                    <Vec<TrainId> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "SegmentTrainOrder"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <LineSegmentId as ::ts_rs::TS>::visit_generics(v);
                <Vec<TrainId> as ::ts_rs::TS>::visit_generics(v);
                v.visit::<LineSegmentId>();
                v.visit::<Vec<TrainId>>();
                <bool as ::ts_rs::TS>::visit_generics(v);
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<bool>();
                v.visit::<ExtensionProperty>();
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SegmentTrainOrder {
            #[inline]
            fn clone(&self) -> SegmentTrainOrder {
                SegmentTrainOrder {
                    segment_id: ::core::clone::Clone::clone(&self.segment_id),
                    is_reversed: ::core::clone::Clone::clone(&self.is_reversed),
                    order: ::core::clone::Clone::clone(&self.order),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SegmentTrainOrder {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SegmentTrainOrder {
            #[inline]
            fn eq(&self, other: &SegmentTrainOrder) -> bool {
                self.is_reversed == other.is_reversed
                    && self.segment_id == other.segment_id && self.order == other.order
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SegmentTrainOrder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "SegmentTrainOrder",
                    "segment_id",
                    &self.segment_id,
                    "is_reversed",
                    &self.is_reversed,
                    "order",
                    &self.order,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for SegmentTrainOrder {
            #[inline]
            fn default() -> SegmentTrainOrder {
                SegmentTrainOrder {
                    segment_id: ::core::default::Default::default(),
                    is_reversed: ::core::default::Default::default(),
                    order: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for SegmentTrainOrder {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "SegmentTrainOrder",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "segment_id",
                        &self.segment_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "is_reversed",
                        &self.is_reversed,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "order",
                        &self.order,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for SegmentTrainOrder {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "segment_id" => _serde::__private229::Ok(__Field::__field0),
                                "is_reversed" => _serde::__private229::Ok(__Field::__field1),
                                "order" => _serde::__private229::Ok(__Field::__field2),
                                "properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"segment_id" => _serde::__private229::Ok(__Field::__field0),
                                b"is_reversed" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                b"order" => _serde::__private229::Ok(__Field::__field2),
                                b"properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<SegmentTrainOrder>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = SegmentTrainOrder;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct SegmentTrainOrder",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                LineSegmentId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct SegmentTrainOrder with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct SegmentTrainOrder with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Vec<TrainId>,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct SegmentTrainOrder with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct SegmentTrainOrder with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(SegmentTrainOrder {
                                segment_id: __field0,
                                is_reversed: __field1,
                                order: __field2,
                                properties: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                LineSegmentId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<bool> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<
                                Vec<TrainId>,
                            > = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "segment_id",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                LineSegmentId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "is_reversed",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("order"),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<TrainId>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("segment_id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("is_reversed")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("order")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(SegmentTrainOrder {
                                segment_id: __field0,
                                is_reversed: __field1,
                                order: __field2,
                                properties: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "segment_id",
                        "is_reversed",
                        "order",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "SegmentTrainOrder",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<
                                SegmentTrainOrder,
                            >,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl PropertiableObject for SegmentTrainOrder {
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
    }
    pub mod station {
        //! Weaverail上の「駅」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
        //! - Station (駅)
        //!   - Track (列車番線)
        use crate::path::Heddle;
        use crate::{
            error::ModelError,
            model::{DiagramRoot, ExtensionProperty, PropertiableObject},
            weaverail_id,
        };
        use indexmap::map::Entry;
        use serde::{Deserialize, Serialize};
        pub struct StationId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for StationId {
            type WithoutGenerics = StationId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("StationId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("StationId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "StationId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <StationId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "StationId", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "StationId"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<crate::model::id::WeaverailId>();
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for StationId {}
        #[automatically_derived]
        impl ::core::clone::Clone for StationId {
            #[inline]
            fn clone(&self) -> StationId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for StationId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for StationId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for StationId {
            #[inline]
            fn eq(&self, other: &StationId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for StationId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for StationId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for StationId {
            #[inline]
            fn default() -> StationId {
                StationId(::core::default::Default::default())
            }
        }
        impl StationId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "STA_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for StationId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for StationId {
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
                    if id != "STA_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for StationId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for StationId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for StationId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for StationId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(StationId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<StationId> for StationId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// Weaverail上の1つの駅を表す構造体
        pub struct Station {
            /// 識別ID
            pub id: StationId,
            /// 正式駅名 (例: "梅田")
            pub name: String,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for Station {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "name" => Some(&self.name as &dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "name" => Some(&mut self.name as &mut dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "name" => {
                        self.name = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("name".to_string()),
                    self.name.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for Station {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<Station> for Station {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for Station {
            type WithoutGenerics = Station;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("Station").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" Weaverail上の1つの駅を表す構造体"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("Station").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "Station",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <Station as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "Station", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 正式駅名 (例: \"梅田\")"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 正式駅名 (例: \"梅田\")"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "Station"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<StationId>();
                v.visit::<ExtensionProperty>();
                <String as ::ts_rs::TS>::visit_generics(v);
                <StationId as ::ts_rs::TS>::visit_generics(v);
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<String>();
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Station {
            #[inline]
            fn clone(&self) -> Station {
                Station {
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Station {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Station {
            #[inline]
            fn eq(&self, other: &Station) -> bool {
                self.id == other.id && self.name == other.name
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Station {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Station",
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Station {
            #[inline]
            fn default() -> Station {
                Station {
                    id: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Station {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Station",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Station {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "name" => _serde::__private229::Ok(__Field::__field1),
                                "properties" => _serde::__private229::Ok(__Field::__field2),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"name" => _serde::__private229::Ok(__Field::__field1),
                                b"properties" => _serde::__private229::Ok(__Field::__field2),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<Station>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Station;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct Station",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                StationId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Station with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Station with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Station with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(Station {
                                id: __field0,
                                name: __field1,
                                properties: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<StationId> = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<String> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<StationId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("name")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(Station {
                                id: __field0,
                                name: __field1,
                                properties: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Station",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<Station>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl Station {
            pub fn new(id: StationId, name: &str) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    ..Default::default()
                }
            }
        }
        impl DiagramRoot {
            /// 駅を追加する関数
            /// 計算オーダは `O(1)`
            /// 既に同一IDの駅が存在している場合はエラーを返す
            pub fn add_station(&mut self, station: Station) -> Result<(), ModelError> {
                match self.stations.entry(station.id) {
                    Entry::Vacant(entry) => {
                        entry.insert(station);
                        Ok(())
                    }
                    Entry::Occupied(_) => Err(ModelError::DuplicateKey),
                }
            }
            /// 駅を削除する関数
            /// 計算オーダは `O(segments.len + track.len)`
            /// 指定IDの駅が存在しない場合はエラーを返す
            /// 駅間から参照されている場合はエラーを返す
            /// 番線から参照されている場合はエラーを返す
            pub fn delete_station(
                &mut self,
                station_id: StationId,
            ) -> Result<Station, ModelError> {
                if self
                    .segments
                    .values()
                    .any(|segment| segment.contains_station(station_id))
                {
                    return Err(ModelError::ExternalReferenced);
                }
                if self.tracks.values().any(|track| track.station_id == station_id) {
                    return Err(ModelError::ExternalReferenced);
                }
                self.stations.shift_remove(&station_id).ok_or(ModelError::ObjectNotFound)
            }
            /// 駅名から駅を検索する関数
            /// 見つからない場合は None を返す
            pub fn find_station_by_name(&self, station_name: &str) -> Option<&Station> {
                self.stations.values().find(|station| station.name == station_name)
            }
            /// 駅データが正常な値であるかを検証する
            pub fn validate_station(
                &self,
                station_id: StationId,
            ) -> Result<(), ModelError> {
                let _ = self
                    .stations
                    .get(&station_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                Ok(())
            }
        }
        impl PropertiableObject for Station {
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
    }
    pub mod template_train {
        //! Weaverail上の「テンプレート列車」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
        //! - TemplateTrain (テンプレート列車)
        //!   - TemplateTrainSegment (テンプレート列車の駅間情報)
        //!   - TemplateTrainStation (テンプレート列車の駅情報)
        //!     - StopType (停車種別)
        use indexmap::map::Entry;
        use serde::{Deserialize, Serialize};
        use crate::path::Heddle;
        use crate::{
            error::ModelError,
            model::{
                DiagramRoot, ExtensionProperty, PropertiableObject,
                line_segment::{LineSegment, LineSegmentId},
                station::{Station, StationId},
                time::Time, track::{Track, TrackId},
                train_type::{TrainType, TrainTypeId},
            },
            weaverail_id,
        };
        pub struct TemplateTrainId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for TemplateTrainId {
            type WithoutGenerics = TemplateTrainId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TemplateTrainId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TemplateTrainId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TemplateTrainId",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "TemplateTrainId"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<crate::model::id::WeaverailId>();
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for TemplateTrainId {}
        #[automatically_derived]
        impl ::core::clone::Clone for TemplateTrainId {
            #[inline]
            fn clone(&self) -> TemplateTrainId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TemplateTrainId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TemplateTrainId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TemplateTrainId {
            #[inline]
            fn eq(&self, other: &TemplateTrainId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for TemplateTrainId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for TemplateTrainId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TemplateTrainId {
            #[inline]
            fn default() -> TemplateTrainId {
                TemplateTrainId(::core::default::Default::default())
            }
        }
        impl TemplateTrainId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "TTR_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for TemplateTrainId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for TemplateTrainId {
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
                    if id != "TTR_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for TemplateTrainId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for TemplateTrainId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for TemplateTrainId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for TemplateTrainId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(TemplateTrainId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<TemplateTrainId> for TemplateTrainId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// Weaverail上の1つのテンプレート列車を表す構造体
        pub struct TemplateTrain {
            /// 識別ID
            pub id: TemplateTrainId,
            /// テンプレート列車名 (例: "本線下り普通列車")
            pub name: String,
            /// 列車種別ID
            pub train_type_id: TrainTypeId,
            /// 開始駅情報
            pub start_station: TemplateTrainStation,
            /// 駅間/駅情報の一覧
            pub segments: Vec<TemplateTrainSection>,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for TemplateTrain {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "name" => Some(&self.name as &dyn crate::model::RnaObject),
                    "train_type_id" => {
                        Some(&self.train_type_id as &dyn crate::model::RnaObject)
                    }
                    "start_station" => {
                        Some(&self.start_station as &dyn crate::model::RnaObject)
                    }
                    "segments" => Some(&self.segments as &dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "name" => Some(&mut self.name as &mut dyn crate::model::RnaObject),
                    "train_type_id" => {
                        Some(&mut self.train_type_id as &mut dyn crate::model::RnaObject)
                    }
                    "start_station" => {
                        Some(&mut self.start_station as &mut dyn crate::model::RnaObject)
                    }
                    "segments" => {
                        Some(&mut self.segments as &mut dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "name" => {
                        self.name = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "train_type_id" => {
                        self.train_type_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "start_station" => {
                        self.start_station = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "segments" => {
                        self.segments = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("name".to_string()),
                    self.name.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("train_type_id".to_string()),
                    self.train_type_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("start_station".to_string()),
                    self.start_station.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("segments".to_string()),
                    self.segments.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for TemplateTrain {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<TemplateTrain> for TemplateTrain {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.train_type_id.get_stack_memory_size()
                    + self.start_station.get_stack_memory_size()
                    + self.segments.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.train_type_id.get_stack_memory_size()
                    + self.start_station.get_stack_memory_size()
                    + self.segments.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for TemplateTrain {
            type WithoutGenerics = TemplateTrain;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrain").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[
                            " Weaverail上の1つのテンプレート列車を表す構造体",
                        ],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrain").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TemplateTrain",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TemplateTrain as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TemplateTrain",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[
                                                                        " テンプレート列車名 (例: \"本線下り普通列車\")",
                                                                    ],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 列車種別ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "train_type_id",
                                                    if false { "?" } else { "" },
                                                    <TrainTypeId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 開始駅情報"]),
                                                            ),
                                                        )
                                                    }),
                                                    "start_station",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainStation as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 駅間/駅情報の一覧"]),
                                                            ),
                                                        )
                                                    }),
                                                    "segments",
                                                    if false { "?" } else { "" },
                                                    <Vec<TemplateTrainSection> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[
                                                                        " テンプレート列車名 (例: \"本線下り普通列車\")",
                                                                    ],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 列車種別ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "train_type_id",
                                                    if false { "?" } else { "" },
                                                    <TrainTypeId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 開始駅情報"]),
                                                            ),
                                                        )
                                                    }),
                                                    "start_station",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainStation as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 駅間/駅情報の一覧"]),
                                                            ),
                                                        )
                                                    }),
                                                    "segments",
                                                    if false { "?" } else { "" },
                                                    <Vec<TemplateTrainSection> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "TemplateTrain"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <TemplateTrainId as ::ts_rs::TS>::visit_generics(v);
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                <Vec<TemplateTrainSection> as ::ts_rs::TS>::visit_generics(v);
                v.visit::<TrainTypeId>();
                <String as ::ts_rs::TS>::visit_generics(v);
                v.visit::<TemplateTrainStation>();
                v.visit::<ExtensionProperty>();
                <TemplateTrainStation as ::ts_rs::TS>::visit_generics(v);
                v.visit::<TemplateTrainId>();
                v.visit::<String>();
                v.visit::<Vec<TemplateTrainSection>>();
                <TrainTypeId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TemplateTrain {
            #[inline]
            fn clone(&self) -> TemplateTrain {
                TemplateTrain {
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    train_type_id: ::core::clone::Clone::clone(&self.train_type_id),
                    start_station: ::core::clone::Clone::clone(&self.start_station),
                    segments: ::core::clone::Clone::clone(&self.segments),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TemplateTrain {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TemplateTrain {
            #[inline]
            fn eq(&self, other: &TemplateTrain) -> bool {
                self.id == other.id && self.name == other.name
                    && self.train_type_id == other.train_type_id
                    && self.start_station == other.start_station
                    && self.segments == other.segments
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TemplateTrain {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "name",
                    "train_type_id",
                    "start_station",
                    "segments",
                    "properties",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.name,
                    &self.train_type_id,
                    &self.start_station,
                    &self.segments,
                    &&self.properties,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "TemplateTrain",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TemplateTrain {
            #[inline]
            fn default() -> TemplateTrain {
                TemplateTrain {
                    id: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    train_type_id: ::core::default::Default::default(),
                    start_station: ::core::default::Default::default(),
                    segments: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for TemplateTrain {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "TemplateTrain",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "train_type_id",
                        &self.train_type_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "start_station",
                        &self.start_station,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "segments",
                        &self.segments,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for TemplateTrain {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                4u64 => _serde::__private229::Ok(__Field::__field4),
                                5u64 => _serde::__private229::Ok(__Field::__field5),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "name" => _serde::__private229::Ok(__Field::__field1),
                                "train_type_id" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                "start_station" => {
                                    _serde::__private229::Ok(__Field::__field3)
                                }
                                "segments" => _serde::__private229::Ok(__Field::__field4),
                                "properties" => _serde::__private229::Ok(__Field::__field5),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"name" => _serde::__private229::Ok(__Field::__field1),
                                b"train_type_id" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                b"start_station" => {
                                    _serde::__private229::Ok(__Field::__field3)
                                }
                                b"segments" => _serde::__private229::Ok(__Field::__field4),
                                b"properties" => _serde::__private229::Ok(__Field::__field5),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<TemplateTrain>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = TemplateTrain;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct TemplateTrain",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                TemplateTrainId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TemplateTrain with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct TemplateTrain with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                TrainTypeId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct TemplateTrain with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                TemplateTrainStation,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct TemplateTrain with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Vec<TemplateTrainSection>,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct TemplateTrain with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct TemplateTrain with 6 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(TemplateTrain {
                                id: __field0,
                                name: __field1,
                                train_type_id: __field2,
                                start_station: __field3,
                                segments: __field4,
                                properties: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                TemplateTrainId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<String> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<
                                TrainTypeId,
                            > = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<
                                TemplateTrainStation,
                            > = _serde::__private229::None;
                            let mut __field4: _serde::__private229::Option<
                                Vec<TemplateTrainSection>,
                            > = _serde::__private229::None;
                            let mut __field5: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TemplateTrainId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "train_type_id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TrainTypeId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "start_station",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TemplateTrainStation,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private229::Option::is_some(&__field4) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "segments",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<TemplateTrainSection>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private229::Option::is_some(&__field5) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("name")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("train_type_id")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("start_station")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private229::Some(__field4) => __field4,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("segments")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private229::Some(__field5) => __field5,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(TemplateTrain {
                                id: __field0,
                                name: __field1,
                                train_type_id: __field2,
                                start_station: __field3,
                                segments: __field4,
                                properties: __field5,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "train_type_id",
                        "start_station",
                        "segments",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "TemplateTrain",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<TemplateTrain>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl TemplateTrain {
            /// 列車種別を取得する関数
            /// 計算量は `O(1)`
            pub fn train_type<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a TrainType, ModelError> {
                root.train_types
                    .get(&self.train_type_id)
                    .ok_or(ModelError::ObjectNotFound)
            }
            /// テンプレート列車が指定駅間を参照しているか
            pub fn contains_segment(&self, segment_id: LineSegmentId) -> bool {
                self.segments
                    .iter()
                    .any(|section| section.segment.segment_id == segment_id)
            }
            /// 全ての駅を取得する関数
            pub fn get_stations(&self) -> Vec<&TemplateTrainStation> {
                std::iter::once(&self.start_station)
                    .chain(self.segments.iter().map(|section| &section.station))
                    .collect()
            }
            /// 指定区間の全ての駅を取得する関数
            pub fn get_filtered_stations(
                &self,
                start_station_id: StationId,
                end_station_id: StationId,
            ) -> Result<Vec<&TemplateTrainStation>, ModelError> {
                let (start_station, segments) = self
                    .get_filtered_segment(start_station_id, end_station_id)?;
                Ok(
                    std::iter::once(start_station)
                        .chain(segments.iter().map(|section| &section.station))
                        .collect(),
                )
            }
            /// 指定の駅が含まれているか
            pub fn contains_station(&self, station_id: StationId) -> bool {
                let stations = self.get_stations();
                stations.iter().any(|station| station.station_id == station_id)
            }
            /// 指定の駅が含まれているか
            pub fn contains_track(&self, track_id: TrackId) -> bool {
                let stations = self.get_stations();
                stations.iter().any(|station| station.track_id == track_id)
            }
            /// 指定の駅が何番目にあるか
            pub fn get_station_index(
                &self,
                station_id: StationId,
            ) -> Result<usize, ModelError> {
                let stations = self.get_stations();
                stations
                    .iter()
                    .position(|station| station.station_id == station_id)
                    .ok_or(ModelError::ObjectNotFound)
            }
            /// 指定の駅間を抽出して返す関数
            pub fn get_filtered_segment(
                &self,
                start_station_id: StationId,
                end_station_id: StationId,
            ) -> Result<
                (&TemplateTrainStation, Vec<&TemplateTrainSection>),
                ModelError,
            > {
                if !self.contains_station(start_station_id)
                    || !self.contains_station(end_station_id)
                {
                    ::core::panicking::panic("internal error: entered unreachable code");
                }
                let first_index = self.get_station_index(start_station_id)?;
                let end_index = self.get_station_index(end_station_id)?;
                let first_station = if first_index == 0 {
                    &self.start_station
                } else {
                    &self
                        .segments
                        .get(first_index - 1)
                        .ok_or(ModelError::ObjectNotFound)?
                        .station
                };
                let mut segments: Vec<_> = Vec::new();
                for i in (first_index)..end_index {
                    let section = self
                        .segments
                        .get(i)
                        .ok_or(ModelError::ObjectNotFound)?;
                    segments.push(section);
                }
                Ok((first_station, segments))
            }
            /// 指定の駅間を抽出してイテレータを返す関数
            pub fn get_filtered_segment_iter(
                &self,
                start_station_id: StationId,
                end_station_id: StationId,
            ) -> Result<
                Vec<
                    (&TemplateTrainStation, &TemplateTrainSegment, &TemplateTrainStation),
                >,
                ModelError,
            > {
                let segments = self
                    .get_filtered_segment(start_station_id, end_station_id)?;
                let mut result = Vec::new();
                let get_station_by_index = |index: isize| {
                    if index < 0 {
                        Ok(segments.0)
                    } else {
                        Ok(
                            &segments
                                .1
                                .get(index as usize)
                                .ok_or(ModelError::ObjectNotFound)?
                                .station,
                        )
                    }
                };
                for i in 0..segments.1.len() {
                    let start = get_station_by_index(i as isize - 1)?;
                    let segment = &segments
                        .1
                        .get(i)
                        .ok_or(ModelError::ObjectNotFound)?
                        .segment;
                    let end = &segments
                        .1
                        .get(i)
                        .ok_or(ModelError::ObjectNotFound)?
                        .station;
                    if segment.is_reversed {
                        result.push((end, segment, start));
                    } else {
                        result.push((start, segment, end));
                    }
                }
                Ok(result)
            }
            /// 先頭の駅を返す関数
            pub fn first_station(&self) -> Result<&TemplateTrainStation, ModelError> {
                Ok(&self.start_station)
            }
            /// 末尾の駅を返す関数
            pub fn last_station(&self) -> Result<&TemplateTrainStation, ModelError> {
                if self.segments.is_empty() {
                    Ok(&self.start_station)
                } else {
                    Ok(&self.segments.last().ok_or(ModelError::Empty)?.station)
                }
            }
        }
        impl PropertiableObject for TemplateTrain {
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
        impl DiagramRoot {
            /// テンプレート列車を追加する関数
            /// 既に同一IDのテンプレート列車が存在している場合はエラーを返す
            pub fn add_template_train(
                &mut self,
                template_train: TemplateTrain,
            ) -> Result<(), ModelError> {
                match self.template_trains.entry(template_train.id) {
                    Entry::Vacant(entry) => {
                        entry.insert(template_train);
                        Ok(())
                    }
                    Entry::Occupied(_) => Err(ModelError::DuplicateKey),
                }
            }
            /// テンプレート列車を削除する関数
            /// 計算オーダは `O(template_trains.len)`
            /// 指定IDのテンプレート列車が存在しない場合はエラーを返す
            /// 列車から参照されている場合はエラーを返す
            pub fn delete_template_train(
                &mut self,
                template_train_id: TemplateTrainId,
            ) -> Result<TemplateTrain, ModelError> {
                if self
                    .trains
                    .values()
                    .any(|train| train.contain_template_train(template_train_id))
                {
                    return Err(ModelError::ExternalReferenced);
                }
                self.template_trains
                    .shift_remove(&template_train_id)
                    .ok_or(ModelError::ObjectNotFound)
            }
            /// テンプレート列車名からテンプレート列車を検索する関数
            /// 見つからない場合は None を返す
            pub fn find_template_train_by_name(
                &self,
                template_train_name: &str,
            ) -> Option<&TemplateTrain> {
                self.template_trains
                    .values()
                    .find(|template_train| template_train.name == template_train_name)
            }
            /// テンプレート列車の末尾に駅間を追加する関数
            pub fn push_back_template_segment(
                &mut self,
                template_train_id: TemplateTrainId,
                template_segment: TemplateTrainSegment,
                template_station: TemplateTrainStation,
            ) -> Result<(), ModelError> {
                let (start_station_id, end_station_id) = {
                    let segment = template_segment.segment(self)?;
                    (segment.start_station, segment.end_station)
                };
                let template_train = self
                    .template_trains
                    .get_mut(&template_train_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                let last_station = template_train.last_station()?.station_id;
                let is_valid_station = if template_segment.is_reversed {
                    end_station_id == last_station
                } else {
                    start_station_id == last_station
                };
                if !is_valid_station {
                    return Err(ModelError::Error);
                }
                template_train
                    .segments
                    .push(TemplateTrainSection {
                        segment: template_segment,
                        station: template_station,
                    });
                Ok(())
            }
            /// テンプレート列車の先頭に駅間を追加する関数
            pub fn push_front_template_segment(
                &mut self,
                template_train_id: TemplateTrainId,
                template_segment: TemplateTrainSegment,
                template_station: TemplateTrainStation,
            ) -> Result<(), ModelError> {
                let (start_station_id, end_station_id) = {
                    let segment = template_segment.segment(self)?;
                    (segment.start_station, segment.end_station)
                };
                let template_train = self
                    .template_trains
                    .get_mut(&template_train_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                let first_station = template_train.first_station()?.station_id;
                let is_valid_station = if template_segment.is_reversed {
                    start_station_id == first_station
                } else {
                    end_station_id == first_station
                };
                if !is_valid_station {
                    return Err(ModelError::Error);
                }
                template_train
                    .segments
                    .push(TemplateTrainSection {
                        segment: template_segment,
                        station: template_station,
                    });
                Ok(())
            }
            /// テンプレート列車の末尾の駅間を削除する関数
            /// 列車からの参照がある場合はエラーを返す
            pub fn pop_back_template_segment(
                &mut self,
                template_train_id: TemplateTrainId,
            ) -> Result<(), ModelError> {
                let template_train = self
                    .template_trains
                    .get_mut(&template_train_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                let last = template_train.last_station()?.station_id;
                let is_referenced = self
                    .trains
                    .values()
                    .filter(|train| train.contain_template_train(template_train.id))
                    .any(|train| {
                        train
                            .template_segments
                            .iter()
                            .any(|seg| {
                                seg.start_station_id == last || seg.end_station_id == last
                            })
                    });
                if is_referenced {
                    return Err(ModelError::ExternalReferenced);
                }
                template_train.segments.pop();
                Ok(())
            }
            /// テンプレート列車の先頭の駅間を削除する関数
            /// 列車からの参照がある場合はエラーを返す
            pub fn pop_front_template_segment(
                &mut self,
                template_train_id: TemplateTrainId,
            ) -> Result<(), ModelError> {
                let template_train = self
                    .template_trains
                    .get_mut(&template_train_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                let first = template_train.first_station()?.station_id;
                let is_referenced = self
                    .trains
                    .values()
                    .filter(|train| train.contain_template_train(template_train.id))
                    .any(|train| {
                        train
                            .template_segments
                            .iter()
                            .any(|seg| {
                                seg.start_station_id == first || seg.end_station_id == first
                            })
                    });
                if is_referenced {
                    return Err(ModelError::ExternalReferenced);
                }
                template_train.segments.remove(0);
                Ok(())
            }
            pub fn get_template_segments(
                &self,
                id: LineSegmentId,
            ) -> Vec<&TemplateTrainSegment> {
                let mut result = ::alloc::vec::Vec::new();
                for train in self.template_trains.values() {
                    for section in &train.segments {
                        if section.segment.segment_id == id {
                            result.push(&section.segment)
                        }
                    }
                }
                result
            }
            /// テンプレート列車データが正常な値であるかを検証する
            pub fn validate_template_train(
                &self,
                template_train_id: TemplateTrainId,
            ) -> Result<(), ModelError> {
                let template_train = self
                    .template_trains
                    .get(&template_train_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                for section in template_train.segments.as_slice() {
                    let station = &section.station;
                    let segment = &section.segment;
                    let _ = self
                        .stations
                        .get(&station.station_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    let _ = self
                        .tracks
                        .get(&station.track_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    let _ = self
                        .segments
                        .get(&segment.segment_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                }
                {
                    let station = &template_train.start_station;
                    let _ = self
                        .stations
                        .get(&station.station_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    let _ = self
                        .tracks
                        .get(&station.track_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                }
                Ok(())
            }
        }
        pub struct TemplateTrainSection {
            pub segment: TemplateTrainSegment,
            pub station: TemplateTrainStation,
        }
        impl crate::model::RnaObject for TemplateTrainSection {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "segment" => Some(&self.segment as &dyn crate::model::RnaObject),
                    "station" => Some(&self.station as &dyn crate::model::RnaObject),
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "segment" => {
                        Some(&mut self.segment as &mut dyn crate::model::RnaObject)
                    }
                    "station" => {
                        Some(&mut self.station as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "segment" => {
                        self.segment = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "station" => {
                        self.station = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("segment".to_string()),
                    self.segment.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("station".to_string()),
                    self.station.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for TemplateTrainSection {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<TemplateTrainSection>
        for TemplateTrainSection {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.segment.get_stack_memory_size()
                    + self.station.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.segment.get_stack_memory_size()
                    + self.station.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for TemplateTrainSection {
            type WithoutGenerics = TemplateTrainSection;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainSection").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainSection").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TemplateTrainSection",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TemplateTrainSection as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TemplateTrainSection",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "segment",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainSegment as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "station",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainStation as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "segment",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainSegment as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    "",
                                                    "station",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainStation as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "TemplateTrainSection"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <TemplateTrainSegment as ::ts_rs::TS>::visit_generics(v);
                v.visit::<TemplateTrainSegment>();
                <TemplateTrainStation as ::ts_rs::TS>::visit_generics(v);
                v.visit::<TemplateTrainStation>();
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TemplateTrainSection {
            #[inline]
            fn clone(&self) -> TemplateTrainSection {
                TemplateTrainSection {
                    segment: ::core::clone::Clone::clone(&self.segment),
                    station: ::core::clone::Clone::clone(&self.station),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TemplateTrainSection {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TemplateTrainSection {
            #[inline]
            fn eq(&self, other: &TemplateTrainSection) -> bool {
                self.segment == other.segment && self.station == other.station
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TemplateTrainSection {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "TemplateTrainSection",
                    "segment",
                    &self.segment,
                    "station",
                    &&self.station,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TemplateTrainSection {
            #[inline]
            fn default() -> TemplateTrainSection {
                TemplateTrainSection {
                    segment: ::core::default::Default::default(),
                    station: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for TemplateTrainSection {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "TemplateTrainSection",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "segment",
                        &self.segment,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "station",
                        &self.station,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for TemplateTrainSection {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "segment" => _serde::__private229::Ok(__Field::__field0),
                                "station" => _serde::__private229::Ok(__Field::__field1),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"segment" => _serde::__private229::Ok(__Field::__field0),
                                b"station" => _serde::__private229::Ok(__Field::__field1),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<TemplateTrainSection>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = TemplateTrainSection;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct TemplateTrainSection",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                TemplateTrainSegment,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TemplateTrainSection with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                TemplateTrainStation,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct TemplateTrainSection with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(TemplateTrainSection {
                                segment: __field0,
                                station: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                TemplateTrainSegment,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<
                                TemplateTrainStation,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "segment",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TemplateTrainSegment,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "station",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TemplateTrainStation,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("segment")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("station")?
                                }
                            };
                            _serde::__private229::Ok(TemplateTrainSection {
                                segment: __field0,
                                station: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["segment", "station"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "TemplateTrainSection",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<
                                TemplateTrainSection,
                            >,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct TemplateTrainSegmentId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for TemplateTrainSegmentId {
            type WithoutGenerics = TemplateTrainSegmentId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainSegmentId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainSegmentId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TemplateTrainSegmentId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TemplateTrainSegmentId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TemplateTrainSegmentId",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "TemplateTrainSegmentId"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<crate::model::id::WeaverailId>();
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for TemplateTrainSegmentId {}
        #[automatically_derived]
        impl ::core::clone::Clone for TemplateTrainSegmentId {
            #[inline]
            fn clone(&self) -> TemplateTrainSegmentId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TemplateTrainSegmentId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TemplateTrainSegmentId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TemplateTrainSegmentId {
            #[inline]
            fn eq(&self, other: &TemplateTrainSegmentId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for TemplateTrainSegmentId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for TemplateTrainSegmentId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TemplateTrainSegmentId {
            #[inline]
            fn default() -> TemplateTrainSegmentId {
                TemplateTrainSegmentId(::core::default::Default::default())
            }
        }
        impl TemplateTrainSegmentId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "TSG_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for TemplateTrainSegmentId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for TemplateTrainSegmentId {
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
                    if id != "TSG_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for TemplateTrainSegmentId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for TemplateTrainSegmentId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for TemplateTrainSegmentId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for TemplateTrainSegmentId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(TemplateTrainSegmentId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<TemplateTrainSegmentId>
        for TemplateTrainSegmentId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// Weaverail上のテンプレート列車の駅間情報を表す構造体
        pub struct TemplateTrainSegment {
            /// 識別ID
            pub id: TemplateTrainSegmentId,
            /// 駅間ID
            pub segment_id: LineSegmentId,
            /// 駅間が反転しているか
            pub is_reversed: bool,
            /// 基準運転時分
            pub running_time: Time,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for TemplateTrainSegment {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "segment_id" => {
                        Some(&self.segment_id as &dyn crate::model::RnaObject)
                    }
                    "is_reversed" => {
                        Some(&self.is_reversed as &dyn crate::model::RnaObject)
                    }
                    "running_time" => {
                        Some(&self.running_time as &dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "segment_id" => {
                        Some(&mut self.segment_id as &mut dyn crate::model::RnaObject)
                    }
                    "is_reversed" => {
                        Some(&mut self.is_reversed as &mut dyn crate::model::RnaObject)
                    }
                    "running_time" => {
                        Some(&mut self.running_time as &mut dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "segment_id" => {
                        self.segment_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "is_reversed" => {
                        self.is_reversed = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "running_time" => {
                        self.running_time = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("segment_id".to_string()),
                    self.segment_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("is_reversed".to_string()),
                    self.is_reversed.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("running_time".to_string()),
                    self.running_time.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for TemplateTrainSegment {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<TemplateTrainSegment>
        for TemplateTrainSegment {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.segment_id.get_stack_memory_size()
                    + self.is_reversed.get_stack_memory_size()
                    + self.running_time.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.segment_id.get_stack_memory_size()
                    + self.is_reversed.get_stack_memory_size()
                    + self.running_time.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for TemplateTrainSegment {
            type WithoutGenerics = TemplateTrainSegment;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainSegment").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[
                            " Weaverail上のテンプレート列車の駅間情報を表す構造体",
                        ],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainSegment").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TemplateTrainSegment",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TemplateTrainSegment as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TemplateTrainSegment",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 駅間ID"])),
                                                        )
                                                    }),
                                                    "segment_id",
                                                    if false { "?" } else { "" },
                                                    <LineSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 駅間が反転しているか"]),
                                                            ),
                                                        )
                                                    }),
                                                    "is_reversed",
                                                    if false { "?" } else { "" },
                                                    <bool as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 基準運転時分"]),
                                                            ),
                                                        )
                                                    }),
                                                    "running_time",
                                                    if false { "?" } else { "" },
                                                    <Time as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 駅間ID"])),
                                                        )
                                                    }),
                                                    "segment_id",
                                                    if false { "?" } else { "" },
                                                    <LineSegmentId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 駅間が反転しているか"]),
                                                            ),
                                                        )
                                                    }),
                                                    "is_reversed",
                                                    if false { "?" } else { "" },
                                                    <bool as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 基準運転時分"]),
                                                            ),
                                                        )
                                                    }),
                                                    "running_time",
                                                    if false { "?" } else { "" },
                                                    <Time as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "TemplateTrainSegment"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <TemplateTrainSegmentId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<LineSegmentId>();
                <bool as ::ts_rs::TS>::visit_generics(v);
                v.visit::<Time>();
                <LineSegmentId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<bool>();
                <Time as ::ts_rs::TS>::visit_generics(v);
                v.visit::<ExtensionProperty>();
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<TemplateTrainSegmentId>();
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TemplateTrainSegment {
            #[inline]
            fn clone(&self) -> TemplateTrainSegment {
                TemplateTrainSegment {
                    id: ::core::clone::Clone::clone(&self.id),
                    segment_id: ::core::clone::Clone::clone(&self.segment_id),
                    is_reversed: ::core::clone::Clone::clone(&self.is_reversed),
                    running_time: ::core::clone::Clone::clone(&self.running_time),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TemplateTrainSegment {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TemplateTrainSegment {
            #[inline]
            fn eq(&self, other: &TemplateTrainSegment) -> bool {
                self.is_reversed == other.is_reversed && self.id == other.id
                    && self.segment_id == other.segment_id
                    && self.running_time == other.running_time
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TemplateTrainSegment {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "TemplateTrainSegment",
                    "id",
                    &self.id,
                    "segment_id",
                    &self.segment_id,
                    "is_reversed",
                    &self.is_reversed,
                    "running_time",
                    &self.running_time,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TemplateTrainSegment {
            #[inline]
            fn default() -> TemplateTrainSegment {
                TemplateTrainSegment {
                    id: ::core::default::Default::default(),
                    segment_id: ::core::default::Default::default(),
                    is_reversed: ::core::default::Default::default(),
                    running_time: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for TemplateTrainSegment {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "TemplateTrainSegment",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "segment_id",
                        &self.segment_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "is_reversed",
                        &self.is_reversed,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "running_time",
                        &self.running_time,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for TemplateTrainSegment {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                4u64 => _serde::__private229::Ok(__Field::__field4),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "segment_id" => _serde::__private229::Ok(__Field::__field1),
                                "is_reversed" => _serde::__private229::Ok(__Field::__field2),
                                "running_time" => {
                                    _serde::__private229::Ok(__Field::__field3)
                                }
                                "properties" => _serde::__private229::Ok(__Field::__field4),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"segment_id" => _serde::__private229::Ok(__Field::__field1),
                                b"is_reversed" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                b"running_time" => {
                                    _serde::__private229::Ok(__Field::__field3)
                                }
                                b"properties" => _serde::__private229::Ok(__Field::__field4),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<TemplateTrainSegment>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = TemplateTrainSegment;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct TemplateTrainSegment",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                TemplateTrainSegmentId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TemplateTrainSegment with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                LineSegmentId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct TemplateTrainSegment with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct TemplateTrainSegment with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Time,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct TemplateTrainSegment with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct TemplateTrainSegment with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(TemplateTrainSegment {
                                id: __field0,
                                segment_id: __field1,
                                is_reversed: __field2,
                                running_time: __field3,
                                properties: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                TemplateTrainSegmentId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<
                                LineSegmentId,
                            > = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<bool> = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<Time> = _serde::__private229::None;
                            let mut __field4: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TemplateTrainSegmentId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "segment_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                LineSegmentId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "is_reversed",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "running_time",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<Time>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private229::Option::is_some(&__field4) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("segment_id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("is_reversed")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("running_time")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private229::Some(__field4) => __field4,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(TemplateTrainSegment {
                                id: __field0,
                                segment_id: __field1,
                                is_reversed: __field2,
                                running_time: __field3,
                                properties: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "segment_id",
                        "is_reversed",
                        "running_time",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "TemplateTrainSegment",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<
                                TemplateTrainSegment,
                            >,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl TemplateTrainSegment {
            /// 駅間を取得する関数
            /// 計算量は `O(1)`
            pub fn segment<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a LineSegment, ModelError> {
                root.segments.get(&self.segment_id).ok_or(ModelError::ObjectNotFound)
            }
            /// 駅間を取得する関数
            /// 計算量は `O(1)`
            pub fn segment_with_reversed<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<(&'a LineSegment, bool), ModelError> {
                let result = root
                    .segments
                    .get(&self.segment_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                Ok((result, self.is_reversed))
            }
        }
        impl PropertiableObject for TemplateTrainSegment {
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
        pub struct TemplateTrainStationId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for TemplateTrainStationId {
            type WithoutGenerics = TemplateTrainStationId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainStationId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainStationId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TemplateTrainStationId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TemplateTrainStationId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TemplateTrainStationId",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "TemplateTrainStationId"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<crate::model::id::WeaverailId>();
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for TemplateTrainStationId {}
        #[automatically_derived]
        impl ::core::clone::Clone for TemplateTrainStationId {
            #[inline]
            fn clone(&self) -> TemplateTrainStationId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TemplateTrainStationId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TemplateTrainStationId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TemplateTrainStationId {
            #[inline]
            fn eq(&self, other: &TemplateTrainStationId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for TemplateTrainStationId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for TemplateTrainStationId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TemplateTrainStationId {
            #[inline]
            fn default() -> TemplateTrainStationId {
                TemplateTrainStationId(::core::default::Default::default())
            }
        }
        impl TemplateTrainStationId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "TST_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for TemplateTrainStationId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for TemplateTrainStationId {
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
                    if id != "TST_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for TemplateTrainStationId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for TemplateTrainStationId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for TemplateTrainStationId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for TemplateTrainStationId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(TemplateTrainStationId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<TemplateTrainStationId>
        for TemplateTrainStationId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// Weaverail上のテンプレート列車の駅情報を表す構造体
        pub struct TemplateTrainStation {
            /// 識別ID
            pub id: TemplateTrainStationId,
            /// 駅ID
            pub station_id: StationId,
            /// 駅到着番線ID
            pub track_id: TrackId,
            /// 停車時間
            pub stop_time: StopType,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for TemplateTrainStation {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "station_id" => {
                        Some(&self.station_id as &dyn crate::model::RnaObject)
                    }
                    "track_id" => Some(&self.track_id as &dyn crate::model::RnaObject),
                    "stop_time" => Some(&self.stop_time as &dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "station_id" => {
                        Some(&mut self.station_id as &mut dyn crate::model::RnaObject)
                    }
                    "track_id" => {
                        Some(&mut self.track_id as &mut dyn crate::model::RnaObject)
                    }
                    "stop_time" => {
                        Some(&mut self.stop_time as &mut dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "station_id" => {
                        self.station_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "track_id" => {
                        self.track_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "stop_time" => {
                        self.stop_time = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("station_id".to_string()),
                    self.station_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("track_id".to_string()),
                    self.track_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("stop_time".to_string()),
                    self.stop_time.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for TemplateTrainStation {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<TemplateTrainStation>
        for TemplateTrainStation {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.station_id.get_stack_memory_size()
                    + self.track_id.get_stack_memory_size()
                    + self.stop_time.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.station_id.get_stack_memory_size()
                    + self.track_id.get_stack_memory_size()
                    + self.stop_time.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for TemplateTrainStation {
            type WithoutGenerics = TemplateTrainStation;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainStation").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[
                            " Weaverail上のテンプレート列車の駅情報を表す構造体",
                        ],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TemplateTrainStation").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TemplateTrainStation",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TemplateTrainStation as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TemplateTrainStation",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainStationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 駅ID"])),
                                                        )
                                                    }),
                                                    "station_id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 駅到着番線ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "track_id",
                                                    if false { "?" } else { "" },
                                                    <TrackId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 停車時間"]),
                                                            ),
                                                        )
                                                    }),
                                                    "stop_time",
                                                    if false { "?" } else { "" },
                                                    <StopType as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainStationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 駅ID"])),
                                                        )
                                                    }),
                                                    "station_id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 駅到着番線ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "track_id",
                                                    if false { "?" } else { "" },
                                                    <TrackId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 停車時間"]),
                                                            ),
                                                        )
                                                    }),
                                                    "stop_time",
                                                    if false { "?" } else { "" },
                                                    <StopType as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "TemplateTrainStation"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <StationId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<StopType>();
                v.visit::<TemplateTrainStationId>();
                <TemplateTrainStationId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<TrackId>();
                <TrackId as ::ts_rs::TS>::visit_generics(v);
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<StationId>();
                <StopType as ::ts_rs::TS>::visit_generics(v);
                v.visit::<ExtensionProperty>();
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TemplateTrainStation {
            #[inline]
            fn clone(&self) -> TemplateTrainStation {
                TemplateTrainStation {
                    id: ::core::clone::Clone::clone(&self.id),
                    station_id: ::core::clone::Clone::clone(&self.station_id),
                    track_id: ::core::clone::Clone::clone(&self.track_id),
                    stop_time: ::core::clone::Clone::clone(&self.stop_time),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TemplateTrainStation {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TemplateTrainStation {
            #[inline]
            fn eq(&self, other: &TemplateTrainStation) -> bool {
                self.id == other.id && self.station_id == other.station_id
                    && self.track_id == other.track_id
                    && self.stop_time == other.stop_time
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TemplateTrainStation {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "TemplateTrainStation",
                    "id",
                    &self.id,
                    "station_id",
                    &self.station_id,
                    "track_id",
                    &self.track_id,
                    "stop_time",
                    &self.stop_time,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TemplateTrainStation {
            #[inline]
            fn default() -> TemplateTrainStation {
                TemplateTrainStation {
                    id: ::core::default::Default::default(),
                    station_id: ::core::default::Default::default(),
                    track_id: ::core::default::Default::default(),
                    stop_time: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for TemplateTrainStation {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "TemplateTrainStation",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "station_id",
                        &self.station_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "track_id",
                        &self.track_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "stop_time",
                        &self.stop_time,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for TemplateTrainStation {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                4u64 => _serde::__private229::Ok(__Field::__field4),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "station_id" => _serde::__private229::Ok(__Field::__field1),
                                "track_id" => _serde::__private229::Ok(__Field::__field2),
                                "stop_time" => _serde::__private229::Ok(__Field::__field3),
                                "properties" => _serde::__private229::Ok(__Field::__field4),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"station_id" => _serde::__private229::Ok(__Field::__field1),
                                b"track_id" => _serde::__private229::Ok(__Field::__field2),
                                b"stop_time" => _serde::__private229::Ok(__Field::__field3),
                                b"properties" => _serde::__private229::Ok(__Field::__field4),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<TemplateTrainStation>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = TemplateTrainStation;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct TemplateTrainStation",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                TemplateTrainStationId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TemplateTrainStation with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                StationId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct TemplateTrainStation with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                TrackId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct TemplateTrainStation with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                StopType,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct TemplateTrainStation with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct TemplateTrainStation with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(TemplateTrainStation {
                                id: __field0,
                                station_id: __field1,
                                track_id: __field2,
                                stop_time: __field3,
                                properties: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                TemplateTrainStationId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<StationId> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<TrackId> = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<StopType> = _serde::__private229::None;
                            let mut __field4: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TemplateTrainStationId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "station_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<StationId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "track_id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<TrackId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "stop_time",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<StopType>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private229::Option::is_some(&__field4) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("station_id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("track_id")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("stop_time")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private229::Some(__field4) => __field4,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(TemplateTrainStation {
                                id: __field0,
                                station_id: __field1,
                                track_id: __field2,
                                stop_time: __field3,
                                properties: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "station_id",
                        "track_id",
                        "stop_time",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "TemplateTrainStation",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<
                                TemplateTrainStation,
                            >,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl TemplateTrainStation {
            /// 駅を取得する関数
            /// 計算量は `O(1)`
            pub fn station<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a Station, ModelError> {
                root.stations.get(&self.station_id).ok_or(ModelError::ObjectNotFound)
            }
            /// 番線を取得する関数
            /// 計算量は `O(1)`
            pub fn track<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a Track, ModelError> {
                root.tracks.get(&self.track_id).ok_or(ModelError::ObjectNotFound)
            }
        }
        impl PropertiableObject for TemplateTrainStation {
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
        /// テンプレート列車の停車種別を表す列挙体
        pub enum StopType {
            /// 停車（停車時分）
            Stop(Time),
            /// 通過
            Pass,
        }
        impl crate::model::RnaObject for StopType {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match self {
                    Self::Stop(f0) => {
                        match key {
                            "0" => Some(f0 as &dyn crate::model::RnaObject),
                            _ => None,
                        }
                    }
                    Self::Pass => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match self {
                    Self::Stop(f0) => {
                        match key {
                            "0" => Some(f0 as &mut dyn crate::model::RnaObject),
                            _ => None,
                        }
                    }
                    Self::Pass => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match self {
                    Self::Stop(f0) => {
                        match key {
                            "0" => {
                                *f0 = value
                                    .try_into()
                                    .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                                Ok(())
                            }
                            _ => {
                                Err(crate::model::RnaError::FieldNotFound(key.to_string()))
                            }
                        }
                    }
                    Self::Pass => {
                        Err(crate::model::RnaError::FieldNotFound(key.to_string()))
                    }
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for StopType {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<StopType> for StopType {
            fn get_stack_memory_size(&self) -> usize {
                match self {
                    Self::Stop(f0) => 0 + f0.get_stack_memory_size(),
                    Self::Pass => 0,
                }
            }
            fn get_heap_memory_size(&self) -> usize {
                match self {
                    Self::Stop(f0) => 0 + f0.get_heap_memory_size(),
                    Self::Pass => 0,
                }
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for StopType {
            type WithoutGenerics = StopType;
            type OptionInnerType = Self;
            const IS_ENUM: bool = true;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("StopType").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" テンプレート列車の停車種別を表す列挙体"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("StopType").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "StopType",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <StopType as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "StopType", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                [
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ \"{0}\": {1} }}",
                                "Stop",
                                <Time as ::ts_rs::TS>::name(cfg),
                            ),
                        )
                    }),
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("\"{0}\"", "Pass"))
                    }),
                ]
                    .join(" | ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "({0})",
                            [
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "{{ \"{0}\": {1} }}",
                                            "Stop",
                                            <Time as ::ts_rs::TS>::name(cfg),
                                        ),
                                    )
                                }),
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(format_args!("\"{0}\"", "Pass"))
                                }),
                            ]
                                .join(" | "),
                        ),
                    )
                })
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "StopType"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<Time>();
                <Time as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for StopType {
            #[inline]
            fn clone(&self) -> StopType {
                match self {
                    StopType::Stop(__self_0) => {
                        StopType::Stop(::core::clone::Clone::clone(__self_0))
                    }
                    StopType::Pass => StopType::Pass,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for StopType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for StopType {
            #[inline]
            fn eq(&self, other: &StopType) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (StopType::Stop(__self_0), StopType::Stop(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for StopType {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    StopType::Stop(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Stop",
                            &__self_0,
                        )
                    }
                    StopType::Pass => ::core::fmt::Formatter::write_str(f, "Pass"),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for StopType {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        StopType::Stop(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "StopType",
                                0u32,
                                "Stop",
                                __field0,
                            )
                        }
                        StopType::Pass => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "StopType",
                                1u32,
                                "Pass",
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for StopType {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private229::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Stop" => _serde::__private229::Ok(__Field::__field0),
                                "Pass" => _serde::__private229::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private229::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Stop" => _serde::__private229::Ok(__Field::__field0),
                                b"Pass" => _serde::__private229::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private229::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private229::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<StopType>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = StopType;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "enum StopType",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data) {
                                _serde::__private229::Ok((__Field::__field0, __variant)) => {
                                    _serde::__private229::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Time,
                                        >(__variant),
                                        StopType::Stop,
                                    )
                                }
                                _serde::__private229::Ok(
                                    (__Field::__field1, __variant),
                                ) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private229::Ok(StopType::Pass)
                                }
                                _serde::__private229::Err(__err) => {
                                    _serde::__private229::Err(__err)
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Stop", "Pass"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "StopType",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<StopType>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl ::core::str::FromStr for StopType {
            type Err = ::strum::ParseError;
            #[inline]
            fn from_str(
                s: &str,
            ) -> ::core::result::Result<StopType, <Self as ::core::str::FromStr>::Err> {
                ::core::result::Result::Ok(
                    match s {
                        "Stop" => StopType::Stop(Default::default()),
                        "Pass" => StopType::Pass,
                        _ => {
                            return ::core::result::Result::Err(
                                ::strum::ParseError::VariantNotFound,
                            );
                        }
                    },
                )
            }
        }
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl ::core::convert::TryFrom<&str> for StopType {
            type Error = ::strum::ParseError;
            #[inline]
            fn try_from(
                s: &str,
            ) -> ::core::result::Result<
                StopType,
                <Self as ::core::convert::TryFrom<&str>>::Error,
            > {
                ::core::str::FromStr::from_str(s)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Display for StopType {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match *self {
                    StopType::Stop(ref field0) => ::core::fmt::Display::fmt("Stop", f),
                    StopType::Pass => ::core::fmt::Display::fmt("Pass", f),
                }
            }
        }
        impl Default for StopType {
            fn default() -> Self {
                Self::Stop(Time::new(0, 0, 30))
            }
        }
    }
    pub mod time {
        use std::ops::{Add, AddAssign};
        use serde::{Deserialize, Serialize};
        /// 時刻
        pub struct Time(u32);
        #[automatically_derived]
        impl ::ts_rs::TS for Time {
            type WithoutGenerics = Time;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <u32 as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("Time").to_string()
            }
            fn docs() -> Option<String> {
                Some(::ts_rs::format_docs(&[" 時刻"]))
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("Time").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "Time",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <Time as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "Time", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <u32 as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "Time"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <u32 as ::ts_rs::TS>::visit_generics(v);
                v.visit::<u32>();
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Time {}
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for Time {}
        #[automatically_derived]
        impl ::core::clone::Clone for Time {
            #[inline]
            fn clone(&self) -> Time {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Time {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Time {
            #[inline]
            fn eq(&self, other: &Time) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Time {
            #[inline]
            fn default() -> Time {
                Time(::core::default::Default::default())
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Time {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_newtype_struct(
                        __serializer,
                        "Time",
                        &self.0,
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Time {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<Time>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Time;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "tuple struct Time",
                            )
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(
                            self,
                            __e: __E,
                        ) -> _serde::__private229::Result<Self::Value, __E::Error>
                        where
                            __E: _serde::Deserializer<'de>,
                        {
                            let __field0: u32 = <u32 as _serde::Deserialize>::deserialize(
                                __e,
                            )?;
                            _serde::__private229::Ok(Time(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u32,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct Time with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(Time(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        "Time",
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<Time>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::cmp::Eq for Time {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Time {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Time,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ord::cmp(self, other))
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Time {
            #[inline]
            fn cmp(&self, other: &Time) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Time {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        impl Time {
            pub const fn new(hour: u32, minute: u32, second: u32) -> Self {
                Self(hour * 60 * 60 + minute * 60 + second)
            }
            pub const fn new_from_total_second(second: u32) -> Self {
                Self(second)
            }
            /// 0時0分からの累計秒
            pub fn total_second(&self) -> u32 {
                self.0
            }
            /// 時
            pub fn get_hour(&self) -> u32 {
                self.0 / 60 / 60 % 24
            }
            /// 分
            pub fn get_minute(&self) -> u32 {
                self.0 / 60 % 60
            }
            /// 秒
            pub fn get_second(&self) -> u32 {
                self.0 % 60
            }
        }
        impl std::fmt::Display for Time {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_fmt(
                    format_args!(
                        "{0:0>2}:{1:0>2}:{2:0>2}",
                        self.get_hour(),
                        self.get_minute(),
                        self.get_second(),
                    ),
                )?;
                Ok(())
            }
        }
        impl std::fmt::Debug for Time {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_fmt(
                    format_args!(
                        "{0:0>2}:{1:0>2}:{2:0>2}",
                        self.get_hour(),
                        self.get_minute(),
                        self.get_second(),
                    ),
                )?;
                Ok(())
            }
        }
        impl AddAssign for Time {
            fn add_assign(&mut self, rhs: Self) {
                self.0 = self.total_second() + rhs.total_second();
            }
        }
        impl Add for Time {
            type Output = Time;
            fn add(self, rhs: Self) -> Self::Output {
                Self::new_from_total_second(self.total_second() + rhs.total_second())
            }
        }
    }
    pub mod timetable {
        //! Weaverail上の「時刻表」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
        //! - Timetable (時刻表)
        use indexmap::{IndexMap, map::Entry};
        use serde::{Deserialize, Serialize};
        use crate::path::Heddle;
        use crate::{
            command::CommandError, error::ModelError,
            model::{
                DiagramRoot, ExtensionProperty, PropertiableObject,
                line_segment::LineSegmentId, segment_train_order::SegmentTrainOrder,
            },
            weaverail_id,
        };
        pub struct TimetableId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for TimetableId {
            type WithoutGenerics = TimetableId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TimetableId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TimetableId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TimetableId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TimetableId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TimetableId",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "TimetableId"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<crate::model::id::WeaverailId>();
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for TimetableId {}
        #[automatically_derived]
        impl ::core::clone::Clone for TimetableId {
            #[inline]
            fn clone(&self) -> TimetableId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TimetableId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TimetableId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TimetableId {
            #[inline]
            fn eq(&self, other: &TimetableId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for TimetableId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for TimetableId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TimetableId {
            #[inline]
            fn default() -> TimetableId {
                TimetableId(::core::default::Default::default())
            }
        }
        impl TimetableId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "TBL_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for TimetableId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for TimetableId {
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
                    if id != "TBL_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for TimetableId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for TimetableId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for TimetableId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for TimetableId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(TimetableId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<TimetableId> for TimetableId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// Weaverail上の1つの時刻表を表す構造体
        pub struct Timetable {
            /// 識別ID
            pub id: TimetableId,
            /// 時刻表名
            pub name: String,
            /// 駅間の列車順序 (順行 / 逆行を表す)
            pub segment_train_orders: IndexMap<LineSegmentId, SegmentTrainOrders>,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for Timetable {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "name" => Some(&self.name as &dyn crate::model::RnaObject),
                    "segment_train_orders" => {
                        Some(&self.segment_train_orders as &dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "name" => Some(&mut self.name as &mut dyn crate::model::RnaObject),
                    "segment_train_orders" => {
                        Some(
                            &mut self.segment_train_orders
                                as &mut dyn crate::model::RnaObject,
                        )
                    }
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "name" => {
                        self.name = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "segment_train_orders" => {
                        self.segment_train_orders = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("name".to_string()),
                    self.name.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("segment_train_orders".to_string()),
                    self.segment_train_orders.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for Timetable {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<Timetable> for Timetable {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.segment_train_orders.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.segment_train_orders.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for Timetable {
            type WithoutGenerics = Timetable;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("Timetable").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" Weaverail上の1つの時刻表を表す構造体"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("Timetable").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "Timetable",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <Timetable as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "Timetable", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TimetableId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 時刻表名"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[" 駅間の列車順序 (順行 / 逆行を表す)"],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "segment_train_orders",
                                                    if false { "?" } else { "" },
                                                    <IndexMap<
                                                        LineSegmentId,
                                                        SegmentTrainOrders,
                                                    > as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TimetableId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 時刻表名"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[" 駅間の列車順序 (順行 / 逆行を表す)"],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "segment_train_orders",
                                                    if false { "?" } else { "" },
                                                    <IndexMap<
                                                        LineSegmentId,
                                                        SegmentTrainOrders,
                                                    > as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "Timetable"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<ExtensionProperty>();
                v.visit::<IndexMap<LineSegmentId, SegmentTrainOrders>>();
                v.visit::<TimetableId>();
                <String as ::ts_rs::TS>::visit_generics(v);
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                <IndexMap<
                    LineSegmentId,
                    SegmentTrainOrders,
                > as ::ts_rs::TS>::visit_generics(v);
                <TimetableId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<String>();
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Timetable {
            #[inline]
            fn clone(&self) -> Timetable {
                Timetable {
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    segment_train_orders: ::core::clone::Clone::clone(
                        &self.segment_train_orders,
                    ),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Timetable {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Timetable {
            #[inline]
            fn eq(&self, other: &Timetable) -> bool {
                self.id == other.id && self.name == other.name
                    && self.segment_train_orders == other.segment_train_orders
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Timetable {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Timetable",
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "segment_train_orders",
                    &self.segment_train_orders,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Timetable {
            #[inline]
            fn default() -> Timetable {
                Timetable {
                    id: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    segment_train_orders: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Timetable {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Timetable",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "segment_train_orders",
                        &self.segment_train_orders,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Timetable {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "name" => _serde::__private229::Ok(__Field::__field1),
                                "segment_train_orders" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                "properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"name" => _serde::__private229::Ok(__Field::__field1),
                                b"segment_train_orders" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                b"properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<Timetable>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Timetable;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct Timetable",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                TimetableId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Timetable with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Timetable with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                IndexMap<LineSegmentId, SegmentTrainOrders>,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Timetable with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Timetable with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(Timetable {
                                id: __field0,
                                name: __field1,
                                segment_train_orders: __field2,
                                properties: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                TimetableId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<String> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<
                                IndexMap<LineSegmentId, SegmentTrainOrders>,
                            > = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TimetableId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "segment_train_orders",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                IndexMap<LineSegmentId, SegmentTrainOrders>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("name")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field(
                                        "segment_train_orders",
                                    )?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(Timetable {
                                id: __field0,
                                name: __field1,
                                segment_train_orders: __field2,
                                properties: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "segment_train_orders",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Timetable",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<Timetable>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl Timetable {
            pub fn new(id: TimetableId, name: &str) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    ..Default::default()
                }
            }
        }
        impl DiagramRoot {
            /// 時刻表を追加する関数
            /// 既に同一IDの時刻表が存在している場合はエラーを返す
            pub fn add_timetable(
                &mut self,
                timetable: Timetable,
            ) -> Result<(), CommandError> {
                match self.timetables.entry(timetable.id) {
                    Entry::Vacant(entry) => {
                        entry.insert(timetable);
                        Ok(())
                    }
                    Entry::Occupied(_) => Err(CommandError::DuplicateKey),
                }
            }
            /// 時刻表を削除する関数
            /// 指定IDの時刻表が存在しない場合はエラーを返す
            pub fn delete_timetable(
                &mut self,
                timetable_id: TimetableId,
            ) -> Result<Timetable, CommandError> {
                self.timetables
                    .shift_remove(&timetable_id)
                    .ok_or(CommandError::TargetObjectNotFound)
            }
            /// 時刻表データが正常な値であるかを検証する
            pub fn validate_timetable(
                &self,
                timetable_id: TimetableId,
            ) -> Result<(), ModelError> {
                let timetable = self
                    .timetables
                    .get(&timetable_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                for order in timetable.segment_train_orders.values() {
                    let _ = self
                        .segments
                        .get(&order.prograde.segment_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    let _ = self
                        .segments
                        .get(&order.retrograde.segment_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    for forward in order.prograde.order.iter() {
                        let _ = self
                            .trains
                            .get(forward)
                            .ok_or(ModelError::ObjectNotFound)?;
                    }
                    for forward in order.retrograde.order.iter() {
                        let _ = self
                            .trains
                            .get(forward)
                            .ok_or(ModelError::ObjectNotFound)?;
                    }
                }
                Ok(())
            }
        }
        impl PropertiableObject for Timetable {
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
        pub struct SegmentTrainOrders {
            /// 順行列車
            pub prograde: SegmentTrainOrder,
            /// 逆行列車
            pub retrograde: SegmentTrainOrder,
        }
        impl crate::model::RnaObject for SegmentTrainOrders {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "prograde" => Some(&self.prograde as &dyn crate::model::RnaObject),
                    "retrograde" => {
                        Some(&self.retrograde as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "prograde" => {
                        Some(&mut self.prograde as &mut dyn crate::model::RnaObject)
                    }
                    "retrograde" => {
                        Some(&mut self.retrograde as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "prograde" => {
                        self.prograde = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "retrograde" => {
                        self.retrograde = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("prograde".to_string()),
                    self.prograde.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("retrograde".to_string()),
                    self.retrograde.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for SegmentTrainOrders {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<SegmentTrainOrders> for SegmentTrainOrders {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.prograde.get_stack_memory_size()
                    + self.retrograde.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.prograde.get_stack_memory_size()
                    + self.retrograde.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for SegmentTrainOrders {
            type WithoutGenerics = SegmentTrainOrders;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("SegmentTrainOrders").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("SegmentTrainOrders").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "SegmentTrainOrders",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <SegmentTrainOrders as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "SegmentTrainOrders",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 順行列車"]),
                                                            ),
                                                        )
                                                    }),
                                                    "prograde",
                                                    if false { "?" } else { "" },
                                                    <SegmentTrainOrder as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 逆行列車"]),
                                                            ),
                                                        )
                                                    }),
                                                    "retrograde",
                                                    if false { "?" } else { "" },
                                                    <SegmentTrainOrder as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 順行列車"]),
                                                            ),
                                                        )
                                                    }),
                                                    "prograde",
                                                    if false { "?" } else { "" },
                                                    <SegmentTrainOrder as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 逆行列車"]),
                                                            ),
                                                        )
                                                    }),
                                                    "retrograde",
                                                    if false { "?" } else { "" },
                                                    <SegmentTrainOrder as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "SegmentTrainOrders"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<SegmentTrainOrder>();
                <SegmentTrainOrder as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SegmentTrainOrders {
            #[inline]
            fn clone(&self) -> SegmentTrainOrders {
                SegmentTrainOrders {
                    prograde: ::core::clone::Clone::clone(&self.prograde),
                    retrograde: ::core::clone::Clone::clone(&self.retrograde),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SegmentTrainOrders {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SegmentTrainOrders {
            #[inline]
            fn eq(&self, other: &SegmentTrainOrders) -> bool {
                self.prograde == other.prograde && self.retrograde == other.retrograde
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SegmentTrainOrders {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SegmentTrainOrders",
                    "prograde",
                    &self.prograde,
                    "retrograde",
                    &&self.retrograde,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for SegmentTrainOrders {
            #[inline]
            fn default() -> SegmentTrainOrders {
                SegmentTrainOrders {
                    prograde: ::core::default::Default::default(),
                    retrograde: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for SegmentTrainOrders {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "SegmentTrainOrders",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "prograde",
                        &self.prograde,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "retrograde",
                        &self.retrograde,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for SegmentTrainOrders {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "prograde" => _serde::__private229::Ok(__Field::__field0),
                                "retrograde" => _serde::__private229::Ok(__Field::__field1),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"prograde" => _serde::__private229::Ok(__Field::__field0),
                                b"retrograde" => _serde::__private229::Ok(__Field::__field1),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<SegmentTrainOrders>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = SegmentTrainOrders;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct SegmentTrainOrders",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                SegmentTrainOrder,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct SegmentTrainOrders with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                SegmentTrainOrder,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct SegmentTrainOrders with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(SegmentTrainOrders {
                                prograde: __field0,
                                retrograde: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                SegmentTrainOrder,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<
                                SegmentTrainOrder,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "prograde",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                SegmentTrainOrder,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "retrograde",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                SegmentTrainOrder,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("prograde")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("retrograde")?
                                }
                            };
                            _serde::__private229::Ok(SegmentTrainOrders {
                                prograde: __field0,
                                retrograde: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["prograde", "retrograde"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "SegmentTrainOrders",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<
                                SegmentTrainOrders,
                            >,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub mod track {
        //! Weaverail上の「駅」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
        //! - Station (駅)
        //!   - Track (列車番線)
        use crate::path::Heddle;
        use crate::{
            error::ModelError,
            model::{
                DiagramRoot, ExtensionProperty, PropertiableObject,
                station::{Station, StationId},
            },
            weaverail_id,
        };
        use indexmap::map::Entry;
        use serde::{Deserialize, Serialize};
        pub struct TrackId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for TrackId {
            type WithoutGenerics = TrackId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TrackId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TrackId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TrackId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TrackId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "TrackId", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "TrackId"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<crate::model::id::WeaverailId>();
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for TrackId {}
        #[automatically_derived]
        impl ::core::clone::Clone for TrackId {
            #[inline]
            fn clone(&self) -> TrackId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TrackId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TrackId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TrackId {
            #[inline]
            fn eq(&self, other: &TrackId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for TrackId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for TrackId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TrackId {
            #[inline]
            fn default() -> TrackId {
                TrackId(::core::default::Default::default())
            }
        }
        impl TrackId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "TRC_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for TrackId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for TrackId {
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
                    if id != "TRC_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for TrackId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for TrackId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for TrackId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for TrackId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(TrackId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<TrackId> for TrackId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// Weaverail上の駅に存在している1つの列車番線を表す構造体
        pub struct Track {
            /// 識別ID
            pub id: TrackId,
            /// 駅ID
            pub station_id: StationId,
            /// 番線名 (例: "1番線")
            pub name: String,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for Track {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "station_id" => {
                        Some(&self.station_id as &dyn crate::model::RnaObject)
                    }
                    "name" => Some(&self.name as &dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "station_id" => {
                        Some(&mut self.station_id as &mut dyn crate::model::RnaObject)
                    }
                    "name" => Some(&mut self.name as &mut dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "station_id" => {
                        self.station_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "name" => {
                        self.name = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("station_id".to_string()),
                    self.station_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("name".to_string()),
                    self.name.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for Track {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<Track> for Track {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.station_id.get_stack_memory_size()
                    + self.name.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.station_id.get_stack_memory_size()
                    + self.name.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for Track {
            type WithoutGenerics = Track;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("Track").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[
                            " Weaverail上の駅に存在している1つの列車番線を表す構造体",
                        ],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("Track").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "Track",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <Track as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "Track", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TrackId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 駅ID"])),
                                                        )
                                                    }),
                                                    "station_id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 番線名 (例: \"1番線\")"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TrackId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 駅ID"])),
                                                        )
                                                    }),
                                                    "station_id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 番線名 (例: \"1番線\")"]),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "Track"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<StationId>();
                v.visit::<String>();
                <StationId as ::ts_rs::TS>::visit_generics(v);
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<ExtensionProperty>();
                v.visit::<TrackId>();
                <TrackId as ::ts_rs::TS>::visit_generics(v);
                <String as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Track {
            #[inline]
            fn clone(&self) -> Track {
                Track {
                    id: ::core::clone::Clone::clone(&self.id),
                    station_id: ::core::clone::Clone::clone(&self.station_id),
                    name: ::core::clone::Clone::clone(&self.name),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Track {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Track {
            #[inline]
            fn eq(&self, other: &Track) -> bool {
                self.id == other.id && self.station_id == other.station_id
                    && self.name == other.name && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Track {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Track",
                    "id",
                    &self.id,
                    "station_id",
                    &self.station_id,
                    "name",
                    &self.name,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Track {
            #[inline]
            fn default() -> Track {
                Track {
                    id: ::core::default::Default::default(),
                    station_id: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Track {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Track",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "station_id",
                        &self.station_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Track {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "station_id" => _serde::__private229::Ok(__Field::__field1),
                                "name" => _serde::__private229::Ok(__Field::__field2),
                                "properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"station_id" => _serde::__private229::Ok(__Field::__field1),
                                b"name" => _serde::__private229::Ok(__Field::__field2),
                                b"properties" => _serde::__private229::Ok(__Field::__field3),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<Track>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Track;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct Track",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                TrackId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Track with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                StationId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Track with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Track with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Track with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(Track {
                                id: __field0,
                                station_id: __field1,
                                name: __field2,
                                properties: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<TrackId> = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<StationId> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<String> = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<TrackId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "station_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<StationId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("station_id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("name")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(Track {
                                id: __field0,
                                station_id: __field1,
                                name: __field2,
                                properties: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "station_id",
                        "name",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Track",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<Track>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl Track {
            pub fn new(id: TrackId, station_id: StationId, name: &str) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    station_id,
                    ..Default::default()
                }
            }
            /// 所属駅を取得する関数
            /// 計算量は `O(1)`
            pub fn station<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a Station, ModelError> {
                root.stations.get(&self.station_id).ok_or(ModelError::ObjectNotFound)
            }
        }
        impl DiagramRoot {
            /// 列車番線を追加する関数
            /// 計算オーダは `O(1)`
            /// 既に同一IDの番線が存在している場合はエラーを返す
            pub fn add_track(&mut self, track: Track) -> Result<(), ModelError> {
                match self.tracks.entry(track.id) {
                    Entry::Vacant(entry) => {
                        entry.insert(track);
                        Ok(())
                    }
                    Entry::Occupied(_) => Err(ModelError::DuplicateKey),
                }
            }
            /// 番線を削除する関数
            /// 計算オーダは `O(track.len)`
            /// 指定IDの番線が存在しない場合はエラーを返す
            /// テンプレート列車から参照されている場合はエラーを返す
            pub fn delete_track(
                &mut self,
                track_id: TrackId,
            ) -> Result<Track, ModelError> {
                if self
                    .template_trains
                    .values()
                    .any(|train| train.contains_track(track_id))
                {
                    return Err(ModelError::ExternalReferenced);
                }
                self.tracks.shift_remove(&track_id).ok_or(ModelError::ObjectNotFound)
            }
            /// 番線データが正常な値であるかを検証する
            pub fn validate_track(&self, track_id: TrackId) -> Result<(), ModelError> {
                let track = self
                    .tracks
                    .get(&track_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                let _ = self
                    .stations
                    .get(&track.station_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                Ok(())
            }
        }
        impl PropertiableObject for Track {
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
    }
    pub mod train {
        //! Weaverail上の「列車」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
        //! - Train (列車)
        //!   - TemplateSegment (テンプレート列車への部分参照)
        use indexmap::map::Entry;
        use serde::{Deserialize, Serialize};
        use crate::path::Heddle;
        use crate::{
            error::ModelError,
            model::{
                DiagramRoot, ExtensionProperty, PropertiableObject,
                station::{Station, StationId},
                template_train::{TemplateTrain, TemplateTrainId, TemplateTrainSegment},
                time::Time, timetable::{Timetable, TimetableId},
            },
            weaverail_id,
        };
        pub struct TrainId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for TrainId {
            type WithoutGenerics = TrainId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TrainId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TrainId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TrainId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TrainId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "TrainId", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "TrainId"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<crate::model::id::WeaverailId>();
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for TrainId {}
        #[automatically_derived]
        impl ::core::clone::Clone for TrainId {
            #[inline]
            fn clone(&self) -> TrainId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TrainId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TrainId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TrainId {
            #[inline]
            fn eq(&self, other: &TrainId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for TrainId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for TrainId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TrainId {
            #[inline]
            fn default() -> TrainId {
                TrainId(::core::default::Default::default())
            }
        }
        impl TrainId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "TRA_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for TrainId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for TrainId {
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
                    if id != "TRA_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for TrainId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for TrainId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for TrainId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for TrainId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(TrainId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<TrainId> for TrainId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// Weaverail上の1つの「列車」を表す
        pub struct Train {
            /// 識別ID
            pub id: TrainId,
            /// 時刻表ID
            pub timetable_id: TimetableId,
            /// テンプレート列車ID
            pub template_segments: Vec<TemplateSegment>,
            /// 開始駅の出発時刻
            pub start_departure_time: Time,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for Train {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "timetable_id" => {
                        Some(&self.timetable_id as &dyn crate::model::RnaObject)
                    }
                    "template_segments" => {
                        Some(&self.template_segments as &dyn crate::model::RnaObject)
                    }
                    "start_departure_time" => {
                        Some(&self.start_departure_time as &dyn crate::model::RnaObject)
                    }
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "timetable_id" => {
                        Some(&mut self.timetable_id as &mut dyn crate::model::RnaObject)
                    }
                    "template_segments" => {
                        Some(
                            &mut self.template_segments
                                as &mut dyn crate::model::RnaObject,
                        )
                    }
                    "start_departure_time" => {
                        Some(
                            &mut self.start_departure_time
                                as &mut dyn crate::model::RnaObject,
                        )
                    }
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "timetable_id" => {
                        self.timetable_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "template_segments" => {
                        self.template_segments = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "start_departure_time" => {
                        self.start_departure_time = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("timetable_id".to_string()),
                    self.timetable_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("template_segments".to_string()),
                    self.template_segments.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("start_departure_time".to_string()),
                    self.start_departure_time.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for Train {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<Train> for Train {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.timetable_id.get_stack_memory_size()
                    + self.template_segments.get_stack_memory_size()
                    + self.start_departure_time.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size()
                    + self.timetable_id.get_stack_memory_size()
                    + self.template_segments.get_stack_memory_size()
                    + self.start_departure_time.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for Train {
            type WithoutGenerics = Train;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("Train").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" Weaverail上の1つの「列車」を表す"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("Train").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "Train",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <Train as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "Train", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TrainId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 時刻表ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "timetable_id",
                                                    if false { "?" } else { "" },
                                                    <TimetableId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" テンプレート列車ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "template_segments",
                                                    if false { "?" } else { "" },
                                                    <Vec<TemplateSegment> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 開始駅の出発時刻"]),
                                                            ),
                                                        )
                                                    }),
                                                    "start_departure_time",
                                                    if false { "?" } else { "" },
                                                    <Time as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TrainId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 時刻表ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "timetable_id",
                                                    if false { "?" } else { "" },
                                                    <TimetableId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" テンプレート列車ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "template_segments",
                                                    if false { "?" } else { "" },
                                                    <Vec<TemplateSegment> as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 開始駅の出発時刻"]),
                                                            ),
                                                        )
                                                    }),
                                                    "start_departure_time",
                                                    if false { "?" } else { "" },
                                                    <Time as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "Train"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<ExtensionProperty>();
                v.visit::<Time>();
                <Vec<TemplateSegment> as ::ts_rs::TS>::visit_generics(v);
                v.visit::<TrainId>();
                <TrainId as ::ts_rs::TS>::visit_generics(v);
                <TimetableId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<Vec<TemplateSegment>>();
                v.visit::<TimetableId>();
                <Time as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Train {
            #[inline]
            fn clone(&self) -> Train {
                Train {
                    id: ::core::clone::Clone::clone(&self.id),
                    timetable_id: ::core::clone::Clone::clone(&self.timetable_id),
                    template_segments: ::core::clone::Clone::clone(
                        &self.template_segments,
                    ),
                    start_departure_time: ::core::clone::Clone::clone(
                        &self.start_departure_time,
                    ),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Train {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Train {
            #[inline]
            fn eq(&self, other: &Train) -> bool {
                self.id == other.id && self.timetable_id == other.timetable_id
                    && self.template_segments == other.template_segments
                    && self.start_departure_time == other.start_departure_time
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Train {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Train",
                    "id",
                    &self.id,
                    "timetable_id",
                    &self.timetable_id,
                    "template_segments",
                    &self.template_segments,
                    "start_departure_time",
                    &self.start_departure_time,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Train {
            #[inline]
            fn default() -> Train {
                Train {
                    id: ::core::default::Default::default(),
                    timetable_id: ::core::default::Default::default(),
                    template_segments: ::core::default::Default::default(),
                    start_departure_time: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Train {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Train",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "timetable_id",
                        &self.timetable_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "template_segments",
                        &self.template_segments,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "start_departure_time",
                        &self.start_departure_time,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Train {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                3u64 => _serde::__private229::Ok(__Field::__field3),
                                4u64 => _serde::__private229::Ok(__Field::__field4),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "timetable_id" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                "template_segments" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                "start_departure_time" => {
                                    _serde::__private229::Ok(__Field::__field3)
                                }
                                "properties" => _serde::__private229::Ok(__Field::__field4),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"timetable_id" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                b"template_segments" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                b"start_departure_time" => {
                                    _serde::__private229::Ok(__Field::__field3)
                                }
                                b"properties" => _serde::__private229::Ok(__Field::__field4),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<Train>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Train;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct Train",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                TrainId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Train with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                TimetableId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Train with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Vec<TemplateSegment>,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Train with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Time,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Train with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Train with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(Train {
                                id: __field0,
                                timetable_id: __field1,
                                template_segments: __field2,
                                start_departure_time: __field3,
                                properties: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<TrainId> = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<
                                TimetableId,
                            > = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<
                                Vec<TemplateSegment>,
                            > = _serde::__private229::None;
                            let mut __field3: _serde::__private229::Option<Time> = _serde::__private229::None;
                            let mut __field4: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<TrainId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "timetable_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TimetableId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "template_segments",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<TemplateSegment>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private229::Option::is_some(&__field3) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "start_departure_time",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<Time>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private229::Option::is_some(&__field4) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("timetable_id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field(
                                        "template_segments",
                                    )?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private229::Some(__field3) => __field3,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field(
                                        "start_departure_time",
                                    )?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private229::Some(__field4) => __field4,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(Train {
                                id: __field0,
                                timetable_id: __field1,
                                template_segments: __field2,
                                start_departure_time: __field3,
                                properties: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "timetable_id",
                        "template_segments",
                        "start_departure_time",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Train",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<Train>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl Train {
            pub fn new(id: TrainId, timetable_id: TimetableId) -> Self {
                Self {
                    id,
                    timetable_id,
                    ..Default::default()
                }
            }
            /// 時刻表を取得する関数
            /// 計算量は `O(1)`
            pub fn timetable<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a Timetable, ModelError> {
                root.timetables.get(&self.timetable_id).ok_or(ModelError::ObjectNotFound)
            }
            /// 列車に指定のテンプレート列車が含まれているか
            pub fn contain_template_train(
                &self,
                template_train_id: TemplateTrainId,
            ) -> bool {
                self.template_segments
                    .iter()
                    .any(|seg| seg.template_train_id == template_train_id)
            }
        }
        impl DiagramRoot {
            /// 列車を追加する関数
            /// 計算オーダは `O(1)`
            /// 既に同一IDの列車が存在している場合はエラーを返す
            pub fn add_train(&mut self, train: Train) -> Result<(), ModelError> {
                match self.trains.entry(train.id) {
                    Entry::Vacant(entry) => {
                        entry.insert(train);
                        Ok(())
                    }
                    Entry::Occupied(_) => Err(ModelError::DuplicateKey),
                }
            }
            /// 列車を削除する関数
            /// 計算オーダは `O(1)`
            /// 指定IDの列車が存在しない場合はエラーを返す
            pub fn delete_train(
                &mut self,
                train_id: TrainId,
            ) -> Result<Train, ModelError> {
                for timetable in self.timetables.values_mut() {
                    for order in timetable.segment_train_orders.values_mut() {
                        if let Some(index) = order
                            .prograde
                            .order
                            .iter()
                            .position(|v| *v == train_id)
                        {
                            order.prograde.order.remove(index);
                        }
                        if let Some(index) = order
                            .retrograde
                            .order
                            .iter()
                            .position(|v| *v == train_id)
                        {
                            order.retrograde.order.remove(index);
                        }
                    }
                }
                self.trains.shift_remove(&train_id).ok_or(ModelError::ObjectNotFound)
            }
            /// 列車の経由する駅を列挙する関数
            pub fn get_train_stations(
                &self,
                train: &Train,
            ) -> Result<Vec<StationId>, ModelError> {
                let mut result = Vec::new();
                for segment in &train.template_segments {
                    let template_train = self
                        .template_trains
                        .get(&segment.template_train_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    let stations: Vec<&super::template_train::TemplateTrainStation> = template_train
                        .get_filtered_stations(
                            segment.start_station_id,
                            segment.end_station_id,
                        )?;
                    if result.is_empty() {
                        result.extend(stations.iter().map(|sta| sta.station_id));
                    } else {
                        result.extend(stations[1..].iter().map(|sta| sta.station_id));
                    }
                }
                Ok(result)
            }
            /// 列車の経由する駅間を列挙する関数
            pub fn get_train_segment(
                &self,
                train: &Train,
            ) -> Result<Vec<TemplateTrainSegment>, ModelError> {
                let mut result = Vec::new();
                for segment in &train.template_segments {
                    let template_train = self
                        .template_trains
                        .get(&segment.template_train_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                    let segments = template_train
                        .get_filtered_segment(
                            segment.start_station_id,
                            segment.end_station_id,
                        )?;
                    result
                        .extend(
                            segments.1.iter().map(|section| section.segment.clone()),
                        );
                }
                Ok(result)
            }
            /// 列車種別データが正常な値であるかを検証する
            pub fn validate_train(&self, train_id: TrainId) -> Result<(), ModelError> {
                let train = self
                    .trains
                    .get(&train_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                for seg in train.template_segments.iter() {
                    let _ = self
                        .template_trains
                        .get(&seg.template_train_id)
                        .ok_or(ModelError::ObjectNotFound)?;
                }
                Ok(())
            }
        }
        impl PropertiableObject for Train {
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
        /// Weaverail上のテンプレート列車への部分参照
        pub struct TemplateSegment {
            /// テンプレート列車ID
            pub template_train_id: TemplateTrainId,
            /// 開始駅ID
            pub start_station_id: StationId,
            /// 終了駅ID
            pub end_station_id: StationId,
        }
        impl crate::model::RnaObject for TemplateSegment {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "template_train_id" => {
                        Some(&self.template_train_id as &dyn crate::model::RnaObject)
                    }
                    "start_station_id" => {
                        Some(&self.start_station_id as &dyn crate::model::RnaObject)
                    }
                    "end_station_id" => {
                        Some(&self.end_station_id as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "template_train_id" => {
                        Some(
                            &mut self.template_train_id
                                as &mut dyn crate::model::RnaObject,
                        )
                    }
                    "start_station_id" => {
                        Some(
                            &mut self.start_station_id
                                as &mut dyn crate::model::RnaObject,
                        )
                    }
                    "end_station_id" => {
                        Some(
                            &mut self.end_station_id as &mut dyn crate::model::RnaObject,
                        )
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "template_train_id" => {
                        self.template_train_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "start_station_id" => {
                        self.start_station_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "end_station_id" => {
                        self.end_station_id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("template_train_id".to_string()),
                    self.template_train_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("start_station_id".to_string()),
                    self.start_station_id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("end_station_id".to_string()),
                    self.end_station_id.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for TemplateSegment {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<TemplateSegment> for TemplateSegment {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.template_train_id.get_stack_memory_size()
                    + self.start_station_id.get_stack_memory_size()
                    + self.end_station_id.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.template_train_id.get_stack_memory_size()
                    + self.start_station_id.get_stack_memory_size()
                    + self.end_station_id.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for TemplateSegment {
            type WithoutGenerics = TemplateSegment;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TemplateSegment").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" Weaverail上のテンプレート列車への部分参照"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TemplateSegment").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TemplateSegment",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TemplateSegment as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TemplateSegment",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" テンプレート列車ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "template_train_id",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 開始駅ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "start_station_id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 終了駅ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "end_station_id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" テンプレート列車ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "template_train_id",
                                                    if false { "?" } else { "" },
                                                    <TemplateTrainId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 開始駅ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "start_station_id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 終了駅ID"]),
                                                            ),
                                                        )
                                                    }),
                                                    "end_station_id",
                                                    if false { "?" } else { "" },
                                                    <StationId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}.ts", "TemplateSegment"),
                            )
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <TemplateTrainId as ::ts_rs::TS>::visit_generics(v);
                v.visit::<StationId>();
                v.visit::<TemplateTrainId>();
                <StationId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TemplateSegment {
            #[inline]
            fn clone(&self) -> TemplateSegment {
                TemplateSegment {
                    template_train_id: ::core::clone::Clone::clone(
                        &self.template_train_id,
                    ),
                    start_station_id: ::core::clone::Clone::clone(
                        &self.start_station_id,
                    ),
                    end_station_id: ::core::clone::Clone::clone(&self.end_station_id),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TemplateSegment {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TemplateSegment {
            #[inline]
            fn eq(&self, other: &TemplateSegment) -> bool {
                self.template_train_id == other.template_train_id
                    && self.start_station_id == other.start_station_id
                    && self.end_station_id == other.end_station_id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TemplateSegment {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "TemplateSegment",
                    "template_train_id",
                    &self.template_train_id,
                    "start_station_id",
                    &self.start_station_id,
                    "end_station_id",
                    &&self.end_station_id,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TemplateSegment {
            #[inline]
            fn default() -> TemplateSegment {
                TemplateSegment {
                    template_train_id: ::core::default::Default::default(),
                    start_station_id: ::core::default::Default::default(),
                    end_station_id: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for TemplateSegment {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "TemplateSegment",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "template_train_id",
                        &self.template_train_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "start_station_id",
                        &self.start_station_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "end_station_id",
                        &self.end_station_id,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for TemplateSegment {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "template_train_id" => {
                                    _serde::__private229::Ok(__Field::__field0)
                                }
                                "start_station_id" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                "end_station_id" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"template_train_id" => {
                                    _serde::__private229::Ok(__Field::__field0)
                                }
                                b"start_station_id" => {
                                    _serde::__private229::Ok(__Field::__field1)
                                }
                                b"end_station_id" => {
                                    _serde::__private229::Ok(__Field::__field2)
                                }
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<TemplateSegment>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = TemplateSegment;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct TemplateSegment",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                TemplateTrainId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TemplateSegment with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                StationId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct TemplateSegment with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                StationId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct TemplateSegment with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(TemplateSegment {
                                template_train_id: __field0,
                                start_station_id: __field1,
                                end_station_id: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                TemplateTrainId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<StationId> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<StationId> = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "template_train_id",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TemplateTrainId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "start_station_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<StationId>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "end_station_id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<StationId>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field(
                                        "template_train_id",
                                    )?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("start_station_id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("end_station_id")?
                                }
                            };
                            _serde::__private229::Ok(TemplateSegment {
                                template_train_id: __field0,
                                start_station_id: __field1,
                                end_station_id: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "template_train_id",
                        "start_station_id",
                        "end_station_id",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "TemplateSegment",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<TemplateSegment>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl TemplateSegment {
            /// テンプレート列車を取得する関数
            /// 計算量は `O(1)`
            pub fn template_train<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a TemplateTrain, ModelError> {
                root.template_trains
                    .get(&self.template_train_id)
                    .ok_or(ModelError::ObjectNotFound)
            }
            /// 開始駅を取得する関数
            /// 計算量は `O(1)`
            pub fn start_station<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a Station, ModelError> {
                root.stations
                    .get(&self.start_station_id)
                    .ok_or(ModelError::ObjectNotFound)
            }
            /// 終了駅を取得する関数
            /// 計算量は `O(1)`
            pub fn end_station<'a>(
                &self,
                root: &'a DiagramRoot,
            ) -> Result<&'a Station, ModelError> {
                root.stations.get(&self.end_station_id).ok_or(ModelError::ObjectNotFound)
            }
        }
    }
    pub mod train_type {
        //! Weaverail上の「列車種別」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
        //! - TrainType (列車種別)
        use indexmap::map::Entry;
        use serde::{Deserialize, Serialize};
        use crate::path::Heddle;
        use crate::{
            error::ModelError,
            model::{DiagramRoot, ExtensionProperty, PropertiableObject},
            weaverail_id,
        };
        pub struct TrainTypeId(pub crate::model::id::WeaverailId);
        #[automatically_derived]
        impl ::ts_rs::TS for TrainTypeId {
            type WithoutGenerics = TrainTypeId;
            type OptionInnerType = Self;
            const IS_ENUM: bool = <crate::model::id::WeaverailId as ::ts_rs::TS>::IS_ENUM;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TrainTypeId").to_string()
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TrainTypeId").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TrainTypeId",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TrainTypeId as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0}{1} = {2};",
                            "TrainTypeId",
                            generics,
                            inline,
                        ),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                <crate::model::id::WeaverailId as ::ts_rs::TS>::name(cfg)
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0} cannot be flattened",
                            <Self as ::ts_rs::TS>::name(cfg),
                        ),
                    );
                }
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "TrainTypeId"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                v.visit::<crate::model::id::WeaverailId>();
                <crate::model::id::WeaverailId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for TrainTypeId {}
        #[automatically_derived]
        impl ::core::clone::Clone for TrainTypeId {
            #[inline]
            fn clone(&self) -> TrainTypeId {
                let _: ::core::clone::AssertParamIsClone<crate::model::id::WeaverailId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TrainTypeId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TrainTypeId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TrainTypeId {
            #[inline]
            fn eq(&self, other: &TrainTypeId) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for TrainTypeId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<crate::model::id::WeaverailId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for TrainTypeId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TrainTypeId {
            #[inline]
            fn default() -> TrainTypeId {
                TrainTypeId(::core::default::Default::default())
            }
        }
        impl TrainTypeId {
            pub fn new(id: crate::model::id::WeaverailId) -> Self {
                Self(id)
            }
            pub fn to_string(&self) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}{1}", "TYP_", self.0.to_string()),
                    )
                })
            }
        }
        impl Serialize for TrainTypeId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for TrainTypeId {
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
                    if id != "TYP_" {
                        return Err(serde::de::Error::custom("type is invalid"));
                    }
                    Ok(Self(crate::model::id::WeaverailId(number)))
                } else {
                    return Err(serde::de::Error::custom("type is invalid"));
                }
            }
        }
        impl std::fmt::Display for TrainTypeId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl std::fmt::Debug for TrainTypeId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }
        impl crate::model::RnaObject for TrainTypeId {
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                self.0.to_heddle()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl TryFrom<crate::path::Heddle> for TrainTypeId {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                if let crate::path::Heddle::Id(id) = value {
                    Ok(TrainTypeId(id))
                } else {
                    Err(crate::model::RnaError::TypeMismatch)
                }
            }
        }
        impl crate::primitives::TotalSizable<TrainTypeId> for TrainTypeId {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
        /// Weaverail上の1つの「列車種別」を表す構造体
        pub struct TrainType {
            /// 識別ID
            pub id: TrainTypeId,
            /// 列車種別名 (例: "普通列車")
            pub name: String,
            /// 拡張プロパティ
            pub properties: ExtensionProperty,
        }
        impl crate::model::RnaObject for TrainType {
            fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&self.id as &dyn crate::model::RnaObject),
                    "name" => Some(&self.name as &dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&self.properties as &dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_get_mut(
                &mut self,
                key: &str,
            ) -> Option<&mut dyn crate::model::RnaObject> {
                match key {
                    "id" => Some(&mut self.id as &mut dyn crate::model::RnaObject),
                    "name" => Some(&mut self.name as &mut dyn crate::model::RnaObject),
                    "properties" => {
                        Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                    }
                    _ => None,
                }
            }
            fn rna_set(
                &mut self,
                key: &str,
                value: crate::path::Heddle,
            ) -> Result<(), crate::model::RnaError> {
                match key {
                    "id" => {
                        self.id = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "name" => {
                        self.name = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    "properties" => {
                        self.properties = value
                            .try_into()
                            .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                        Ok(())
                    }
                    _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                }
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn to_heddle(&self) -> Option<crate::path::Heddle> {
                let mut obj = ::indexmap::IndexMap::new();
                obj.insert(
                    crate::path::Heddle::String("id".to_string()),
                    self.id.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("name".to_string()),
                    self.name.to_heddle()?,
                );
                obj.insert(
                    crate::path::Heddle::String("properties".to_string()),
                    self.properties.to_heddle()?,
                );
                Some(crate::path::Heddle::Compound(obj))
            }
        }
        impl TryFrom<crate::path::Heddle> for TrainType {
            type Error = crate::model::RnaError;
            fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
            }
        }
        impl crate::primitives::TotalSizable<TrainType> for TrainType {
            fn get_stack_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
            fn get_heap_memory_size(&self) -> usize {
                0 + self.id.get_stack_memory_size() + self.name.get_stack_memory_size()
                    + self.properties.get_stack_memory_size()
            }
        }
        #[automatically_derived]
        impl ::ts_rs::TS for TrainType {
            type WithoutGenerics = TrainType;
            type OptionInnerType = Self;
            const IS_ENUM: bool = false;
            fn ident(cfg: &::ts_rs::Config) -> String {
                ("TrainType").to_string()
            }
            fn docs() -> Option<String> {
                Some(
                    ::ts_rs::format_docs(
                        &[" Weaverail上の1つの「列車種別」を表す構造体"],
                    ),
                )
            }
            fn name(cfg: &::ts_rs::Config) -> String {
                ("TrainType").to_string()
            }
            fn decl_concrete(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "type {0} = {1};",
                            "TrainType",
                            <Self as ::ts_rs::TS>::inline(cfg),
                        ),
                    )
                })
            }
            fn decl(cfg: &::ts_rs::Config) -> String {
                let inline = <TrainType as ::ts_rs::TS>::inline(cfg);
                let generics = "";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("type {0}{1} = {2};", "TrainType", generics, inline),
                    )
                })
            }
            fn inline(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TrainTypeId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[" 列車種別名 (例: \"普通列車\")"],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn inline_flattened(cfg: &::ts_rs::Config) -> String {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{{ {0} }}",
                                <[String]>::join(
                                    &[
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!("\n{0}", ::ts_rs::format_docs(&[" 識別ID"])),
                                                        )
                                                    }),
                                                    "id",
                                                    if false { "?" } else { "" },
                                                    <TrainTypeId as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(
                                                                    &[" 列車種別名 (例: \"普通列車\")"],
                                                                ),
                                                            ),
                                                        )
                                                    }),
                                                    "name",
                                                    if false { "?" } else { "" },
                                                    <String as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "{0}{1}{2}: {3},",
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}",
                                                                ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                            ),
                                                        )
                                                    }),
                                                    "properties",
                                                    if false { "?" } else { "" },
                                                    <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                                ),
                                            )
                                        }),
                                    ],
                                    " ",
                                ),
                            ),
                        )
                    })
                    .replace(" } & { ", " ")
            }
            fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {}
            fn output_path() -> Option<std::path::PathBuf> {
                Some(
                    std::path::PathBuf::from(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}.ts", "TrainType"))
                        }),
                    ),
                )
            }
            fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
            where
                Self: 'static,
            {
                <String as ::ts_rs::TS>::visit_generics(v);
                v.visit::<TrainTypeId>();
                v.visit::<ExtensionProperty>();
                <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
                v.visit::<String>();
                <TrainTypeId as ::ts_rs::TS>::visit_generics(v);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TrainType {
            #[inline]
            fn clone(&self) -> TrainType {
                TrainType {
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    properties: ::core::clone::Clone::clone(&self.properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TrainType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TrainType {
            #[inline]
            fn eq(&self, other: &TrainType) -> bool {
                self.id == other.id && self.name == other.name
                    && self.properties == other.properties
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TrainType {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "TrainType",
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "properties",
                    &&self.properties,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TrainType {
            #[inline]
            fn default() -> TrainType {
                TrainType {
                    id: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    properties: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for TrainType {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "TrainType",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "properties",
                        &self.properties,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for TrainType {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private229::Ok(__Field::__field0),
                                1u64 => _serde::__private229::Ok(__Field::__field1),
                                2u64 => _serde::__private229::Ok(__Field::__field2),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private229::Ok(__Field::__field0),
                                "name" => _serde::__private229::Ok(__Field::__field1),
                                "properties" => _serde::__private229::Ok(__Field::__field2),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private229::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private229::Ok(__Field::__field0),
                                b"name" => _serde::__private229::Ok(__Field::__field1),
                                b"properties" => _serde::__private229::Ok(__Field::__field2),
                                _ => _serde::__private229::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private229::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private229::PhantomData<TrainType>,
                        lifetime: _serde::__private229::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = TrainType;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private229::Formatter,
                        ) -> _serde::__private229::fmt::Result {
                            _serde::__private229::Formatter::write_str(
                                __formatter,
                                "struct TrainType",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                TrainTypeId,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TrainType with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct TrainType with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                ExtensionProperty,
                            >(&mut __seq)? {
                                _serde::__private229::Some(__value) => __value,
                                _serde::__private229::None => {
                                    return _serde::__private229::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct TrainType with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private229::Ok(TrainType {
                                id: __field0,
                                name: __field1,
                                properties: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private229::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private229::Option<
                                TrainTypeId,
                            > = _serde::__private229::None;
                            let mut __field1: _serde::__private229::Option<String> = _serde::__private229::None;
                            let mut __field2: _serde::__private229::Option<
                                ExtensionProperty,
                            > = _serde::__private229::None;
                            while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private229::Option::is_some(&__field0) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                TrainTypeId,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private229::Option::is_some(&__field1) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private229::Option::is_some(&__field2) {
                                            return _serde::__private229::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "properties",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private229::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ExtensionProperty,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private229::Some(__field0) => __field0,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private229::Some(__field1) => __field1,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("name")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private229::Some(__field2) => __field2,
                                _serde::__private229::None => {
                                    _serde::__private229::de::missing_field("properties")?
                                }
                            };
                            _serde::__private229::Ok(TrainType {
                                id: __field0,
                                name: __field1,
                                properties: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "properties",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "TrainType",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private229::PhantomData::<TrainType>,
                            lifetime: _serde::__private229::PhantomData,
                        },
                    )
                }
            }
        };
        impl TrainType {
            pub fn new(id: TrainTypeId, name: &str) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    ..Default::default()
                }
            }
        }
        impl DiagramRoot {
            /// 列車種別を追加する関数
            /// 既に同一IDの列車種別が存在している場合はエラーを返す
            pub fn add_train_type(
                &mut self,
                train_type: TrainType,
            ) -> Result<(), ModelError> {
                match self.train_types.entry(train_type.id) {
                    Entry::Vacant(entry) => {
                        entry.insert(train_type);
                        Ok(())
                    }
                    Entry::Occupied(_) => Err(ModelError::DuplicateKey),
                }
            }
            /// 列車種別を削除する関数
            /// 計算オーダは `O(template_trains.len)`
            /// 指定IDの列車種別が存在しない場合はエラーを返す
            /// テンプレート列車から讃匠されている場合はエラーを返す
            pub fn delete_train_type(
                &mut self,
                train_type_id: TrainTypeId,
            ) -> Result<TrainType, ModelError> {
                if self
                    .template_trains
                    .values()
                    .any(|train| train.train_type_id == train_type_id)
                {
                    return Err(ModelError::ExternalReferenced);
                }
                self.train_types
                    .shift_remove(&train_type_id)
                    .ok_or(ModelError::ObjectNotFound)
            }
            /// 列車種別名から列車種別を検索する関数
            /// 見つからない場合は None を返す
            pub fn find_train_type_by_name(
                &self,
                train_type_name: &str,
            ) -> Option<&TrainType> {
                self.train_types
                    .values()
                    .find(|train_type| train_type.name == train_type_name)
            }
            /// 列車種別データが正常な値であるかを検証する
            pub fn validate_train_type(
                &self,
                train_type_id: TrainTypeId,
            ) -> Result<(), ModelError> {
                let _ = self
                    .train_types
                    .get(&train_type_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                Ok(())
            }
        }
        impl PropertiableObject for TrainType {
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
    }
    use deepsize::DeepSizeOf;
    use indexmap::IndexMap;
    use serde::{Deserialize, Serialize};
    use crate::{error::ModelError, id_issuer::IdIssuer, path::Heddle};
    pub use station::*;
    pub use track::*;
    pub use line::*;
    pub use line_segment::*;
    pub use train_type::*;
    pub use template_train::*;
    pub use timetable::*;
    pub use segment_train_order::*;
    pub use train::*;
    pub use time::Time;
    pub use diagram_view_settings::*;
    /// ユーザ定義で拡張が行える拡張プロパティを表す構造体
    pub struct ExtensionProperty(IndexMap<String, Heddle>);
    #[automatically_derived]
    impl ::ts_rs::TS for ExtensionProperty {
        type WithoutGenerics = ExtensionProperty;
        type OptionInnerType = Self;
        const IS_ENUM: bool = <IndexMap<String, Heddle> as ::ts_rs::TS>::IS_ENUM;
        fn ident(cfg: &::ts_rs::Config) -> String {
            ("ExtensionProperty").to_string()
        }
        fn docs() -> Option<String> {
            Some(
                ::ts_rs::format_docs(
                    &[
                        " ユーザ定義で拡張が行える拡張プロパティを表す構造体",
                    ],
                ),
            )
        }
        fn name(cfg: &::ts_rs::Config) -> String {
            ("ExtensionProperty").to_string()
        }
        fn decl_concrete(cfg: &::ts_rs::Config) -> String {
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "type {0} = {1};",
                        "ExtensionProperty",
                        <Self as ::ts_rs::TS>::inline(cfg),
                    ),
                )
            })
        }
        fn decl(cfg: &::ts_rs::Config) -> String {
            let inline = <ExtensionProperty as ::ts_rs::TS>::inline(cfg);
            let generics = "";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "type {0}{1} = {2};",
                        "ExtensionProperty",
                        generics,
                        inline,
                    ),
                )
            })
        }
        fn inline(cfg: &::ts_rs::Config) -> String {
            <IndexMap<String, Heddle> as ::ts_rs::TS>::name(cfg)
        }
        fn inline_flattened(cfg: &::ts_rs::Config) -> String {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "{0} cannot be flattened",
                        <Self as ::ts_rs::TS>::name(cfg),
                    ),
                );
            }
        }
        fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
        where
            Self: 'static,
        {}
        fn output_path() -> Option<std::path::PathBuf> {
            Some(
                std::path::PathBuf::from(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}.ts", "ExtensionProperty"))
                    }),
                ),
            )
        }
        fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
        where
            Self: 'static,
        {
            v.visit::<IndexMap<String, Heddle>>();
            <IndexMap<String, Heddle> as ::ts_rs::TS>::visit_generics(v);
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ExtensionProperty {
        #[inline]
        fn clone(&self) -> ExtensionProperty {
            ExtensionProperty(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ExtensionProperty {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ExtensionProperty {
        #[inline]
        fn eq(&self, other: &ExtensionProperty) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ExtensionProperty {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "ExtensionProperty",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for ExtensionProperty {
        #[inline]
        fn default() -> ExtensionProperty {
            ExtensionProperty(::core::default::Default::default())
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ExtensionProperty {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private229::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "ExtensionProperty",
                    &self.0,
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ExtensionProperty {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private229::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private229::PhantomData<ExtensionProperty>,
                    lifetime: _serde::__private229::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ExtensionProperty;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private229::Formatter,
                    ) -> _serde::__private229::fmt::Result {
                        _serde::__private229::Formatter::write_str(
                            __formatter,
                            "tuple struct ExtensionProperty",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private229::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: IndexMap<String, Heddle> = <IndexMap<
                            String,
                            Heddle,
                        > as _serde::Deserialize>::deserialize(__e)?;
                        _serde::__private229::Ok(ExtensionProperty(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private229::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<String, Heddle>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct ExtensionProperty with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private229::Ok(ExtensionProperty(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "ExtensionProperty",
                    __Visitor {
                        marker: _serde::__private229::PhantomData::<ExtensionProperty>,
                        lifetime: _serde::__private229::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::model::RnaObject for ExtensionProperty {
        fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
            match key {
                "0" => Some(&self.0 as &dyn crate::model::RnaObject),
                _ => None,
            }
        }
        fn rna_get_mut(
            &mut self,
            key: &str,
        ) -> Option<&mut dyn crate::model::RnaObject> {
            match key {
                "0" => Some(&mut self.0 as &mut dyn crate::model::RnaObject),
                _ => None,
            }
        }
        fn rna_set(
            &mut self,
            key: &str,
            value: crate::path::Heddle,
        ) -> Result<(), crate::model::RnaError> {
            match key {
                "0" => {
                    self.0 = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
            }
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn to_heddle(&self) -> Option<crate::path::Heddle> {
            let mut obj = ::indexmap::IndexMap::new();
            obj.insert(
                crate::path::Heddle::String("0".to_string()),
                self.0.to_heddle()?,
            );
            Some(crate::path::Heddle::Compound(obj))
        }
    }
    impl TryFrom<crate::path::Heddle> for ExtensionProperty {
        type Error = crate::model::RnaError;
        fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
            value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
        }
    }
    impl crate::primitives::TotalSizable<ExtensionProperty> for ExtensionProperty {
        fn get_stack_memory_size(&self) -> usize {
            0
        }
        fn get_heap_memory_size(&self) -> usize {
            0
        }
    }
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
    impl crate::model::RnaObject for DiagramRoot {
        fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
            match key {
                "stations" => Some(&self.stations as &dyn crate::model::RnaObject),
                "tracks" => Some(&self.tracks as &dyn crate::model::RnaObject),
                "segments" => Some(&self.segments as &dyn crate::model::RnaObject),
                "lines" => Some(&self.lines as &dyn crate::model::RnaObject),
                "train_types" => Some(&self.train_types as &dyn crate::model::RnaObject),
                "template_trains" => {
                    Some(&self.template_trains as &dyn crate::model::RnaObject)
                }
                "timetables" => Some(&self.timetables as &dyn crate::model::RnaObject),
                "trains" => Some(&self.trains as &dyn crate::model::RnaObject),
                "diagram_view_settings" => {
                    Some(&self.diagram_view_settings as &dyn crate::model::RnaObject)
                }
                "properties" => Some(&self.properties as &dyn crate::model::RnaObject),
                "id_issuer" => Some(&self.id_issuer as &dyn crate::model::RnaObject),
                "version" => Some(&self.version as &dyn crate::model::RnaObject),
                _ => None,
            }
        }
        fn rna_get_mut(
            &mut self,
            key: &str,
        ) -> Option<&mut dyn crate::model::RnaObject> {
            match key {
                "stations" => {
                    Some(&mut self.stations as &mut dyn crate::model::RnaObject)
                }
                "tracks" => Some(&mut self.tracks as &mut dyn crate::model::RnaObject),
                "segments" => {
                    Some(&mut self.segments as &mut dyn crate::model::RnaObject)
                }
                "lines" => Some(&mut self.lines as &mut dyn crate::model::RnaObject),
                "train_types" => {
                    Some(&mut self.train_types as &mut dyn crate::model::RnaObject)
                }
                "template_trains" => {
                    Some(&mut self.template_trains as &mut dyn crate::model::RnaObject)
                }
                "timetables" => {
                    Some(&mut self.timetables as &mut dyn crate::model::RnaObject)
                }
                "trains" => Some(&mut self.trains as &mut dyn crate::model::RnaObject),
                "diagram_view_settings" => {
                    Some(
                        &mut self.diagram_view_settings
                            as &mut dyn crate::model::RnaObject,
                    )
                }
                "properties" => {
                    Some(&mut self.properties as &mut dyn crate::model::RnaObject)
                }
                "id_issuer" => {
                    Some(&mut self.id_issuer as &mut dyn crate::model::RnaObject)
                }
                "version" => Some(&mut self.version as &mut dyn crate::model::RnaObject),
                _ => None,
            }
        }
        fn rna_set(
            &mut self,
            key: &str,
            value: crate::path::Heddle,
        ) -> Result<(), crate::model::RnaError> {
            match key {
                "stations" => {
                    self.stations = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "tracks" => {
                    self.tracks = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "segments" => {
                    self.segments = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "lines" => {
                    self.lines = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "train_types" => {
                    self.train_types = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "template_trains" => {
                    self.template_trains = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "timetables" => {
                    self.timetables = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "trains" => {
                    self.trains = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "diagram_view_settings" => {
                    self.diagram_view_settings = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "properties" => {
                    self.properties = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "id_issuer" => {
                    self.id_issuer = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                "version" => {
                    self.version = value
                        .try_into()
                        .map_err(|_| crate::model::RnaError::TypeMismatch)?;
                    Ok(())
                }
                _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
            }
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn to_heddle(&self) -> Option<crate::path::Heddle> {
            let mut obj = ::indexmap::IndexMap::new();
            obj.insert(
                crate::path::Heddle::String("stations".to_string()),
                self.stations.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("tracks".to_string()),
                self.tracks.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("segments".to_string()),
                self.segments.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("lines".to_string()),
                self.lines.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("train_types".to_string()),
                self.train_types.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("template_trains".to_string()),
                self.template_trains.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("timetables".to_string()),
                self.timetables.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("trains".to_string()),
                self.trains.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("diagram_view_settings".to_string()),
                self.diagram_view_settings.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("properties".to_string()),
                self.properties.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("id_issuer".to_string()),
                self.id_issuer.to_heddle()?,
            );
            obj.insert(
                crate::path::Heddle::String("version".to_string()),
                self.version.to_heddle()?,
            );
            Some(crate::path::Heddle::Compound(obj))
        }
    }
    impl TryFrom<crate::path::Heddle> for DiagramRoot {
        type Error = crate::model::RnaError;
        fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
            value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
        }
    }
    impl crate::primitives::TotalSizable<DiagramRoot> for DiagramRoot {
        fn get_stack_memory_size(&self) -> usize {
            0 + self.stations.get_stack_memory_size()
                + self.tracks.get_stack_memory_size()
                + self.segments.get_stack_memory_size()
                + self.lines.get_stack_memory_size()
                + self.train_types.get_stack_memory_size()
                + self.template_trains.get_stack_memory_size()
                + self.timetables.get_stack_memory_size()
                + self.trains.get_stack_memory_size()
                + self.diagram_view_settings.get_stack_memory_size()
                + self.properties.get_stack_memory_size()
                + self.id_issuer.get_stack_memory_size()
                + self.version.get_stack_memory_size()
        }
        fn get_heap_memory_size(&self) -> usize {
            0 + self.stations.get_stack_memory_size()
                + self.tracks.get_stack_memory_size()
                + self.segments.get_stack_memory_size()
                + self.lines.get_stack_memory_size()
                + self.train_types.get_stack_memory_size()
                + self.template_trains.get_stack_memory_size()
                + self.timetables.get_stack_memory_size()
                + self.trains.get_stack_memory_size()
                + self.diagram_view_settings.get_stack_memory_size()
                + self.properties.get_stack_memory_size()
                + self.id_issuer.get_stack_memory_size()
                + self.version.get_stack_memory_size()
        }
    }
    #[automatically_derived]
    impl ::ts_rs::TS for DiagramRoot {
        type WithoutGenerics = DiagramRoot;
        type OptionInnerType = Self;
        const IS_ENUM: bool = false;
        fn ident(cfg: &::ts_rs::Config) -> String {
            ("DiagramRoot").to_string()
        }
        fn docs() -> Option<String> {
            Some(
                ::ts_rs::format_docs(
                    &[
                        " ダイヤグラムプロジェクトファイルを表す構造体",
                    ],
                ),
            )
        }
        fn name(cfg: &::ts_rs::Config) -> String {
            ("DiagramRoot").to_string()
        }
        fn decl_concrete(cfg: &::ts_rs::Config) -> String {
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "type {0} = {1};",
                        "DiagramRoot",
                        <Self as ::ts_rs::TS>::inline(cfg),
                    ),
                )
            })
        }
        fn decl(cfg: &::ts_rs::Config) -> String {
            let inline = <DiagramRoot as ::ts_rs::TS>::inline(cfg);
            let generics = "";
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("type {0}{1} = {2};", "DiagramRoot", generics, inline),
                )
            })
        }
        fn inline(cfg: &::ts_rs::Config) -> String {
            ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "{{ {0} }}",
                            <[String]>::join(
                                &[
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 駅の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "stations",
                                                if false { "?" } else { "" },
                                                <IndexMap<StationId, Station> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 番線の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "tracks",
                                                if false { "?" } else { "" },
                                                <IndexMap<TrackId, Track> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 駅間の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "segments",
                                                if false { "?" } else { "" },
                                                <IndexMap<
                                                    LineSegmentId,
                                                    LineSegment,
                                                > as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 路線の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "lines",
                                                if false { "?" } else { "" },
                                                <IndexMap<LineId, Line> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 列車種別の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "train_types",
                                                if false { "?" } else { "" },
                                                <IndexMap<TrainTypeId, TrainType> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(
                                                                &[" テンプレート列車の集合"],
                                                            ),
                                                        ),
                                                    )
                                                }),
                                                "template_trains",
                                                if false { "?" } else { "" },
                                                <IndexMap<
                                                    TemplateTrainId,
                                                    TemplateTrain,
                                                > as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 時刻表の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "timetables",
                                                if false { "?" } else { "" },
                                                <IndexMap<TimetableId, Timetable> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 列車の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "trains",
                                                if false { "?" } else { "" },
                                                <IndexMap<TrainId, Train> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(
                                                                &[" ダイヤグラムの表示設定の集合"],
                                                            ),
                                                        ),
                                                    )
                                                }),
                                                "diagram_view_settings",
                                                if false { "?" } else { "" },
                                                <IndexMap<
                                                    DiagramViewSettingsId,
                                                    DiagramViewSettings,
                                                > as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                        ),
                                                    )
                                                }),
                                                "properties",
                                                if false { "?" } else { "" },
                                                <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!("\n{0}", ::ts_rs::format_docs(&[" ID発行"])),
                                                    )
                                                }),
                                                "id_issuer",
                                                if false { "?" } else { "" },
                                                <IdIssuer as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" バージョン"]),
                                                        ),
                                                    )
                                                }),
                                                "version",
                                                if false { "?" } else { "" },
                                                <u32 as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                ],
                                " ",
                            ),
                        ),
                    )
                })
                .replace(" } & { ", " ")
        }
        fn inline_flattened(cfg: &::ts_rs::Config) -> String {
            ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "{{ {0} }}",
                            <[String]>::join(
                                &[
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 駅の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "stations",
                                                if false { "?" } else { "" },
                                                <IndexMap<StationId, Station> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 番線の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "tracks",
                                                if false { "?" } else { "" },
                                                <IndexMap<TrackId, Track> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 駅間の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "segments",
                                                if false { "?" } else { "" },
                                                <IndexMap<
                                                    LineSegmentId,
                                                    LineSegment,
                                                > as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 路線の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "lines",
                                                if false { "?" } else { "" },
                                                <IndexMap<LineId, Line> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 列車種別の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "train_types",
                                                if false { "?" } else { "" },
                                                <IndexMap<TrainTypeId, TrainType> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(
                                                                &[" テンプレート列車の集合"],
                                                            ),
                                                        ),
                                                    )
                                                }),
                                                "template_trains",
                                                if false { "?" } else { "" },
                                                <IndexMap<
                                                    TemplateTrainId,
                                                    TemplateTrain,
                                                > as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 時刻表の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "timetables",
                                                if false { "?" } else { "" },
                                                <IndexMap<TimetableId, Timetable> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 列車の集合"]),
                                                        ),
                                                    )
                                                }),
                                                "trains",
                                                if false { "?" } else { "" },
                                                <IndexMap<TrainId, Train> as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(
                                                                &[" ダイヤグラムの表示設定の集合"],
                                                            ),
                                                        ),
                                                    )
                                                }),
                                                "diagram_view_settings",
                                                if false { "?" } else { "" },
                                                <IndexMap<
                                                    DiagramViewSettingsId,
                                                    DiagramViewSettings,
                                                > as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" 拡張プロパティ"]),
                                                        ),
                                                    )
                                                }),
                                                "properties",
                                                if false { "?" } else { "" },
                                                <ExtensionProperty as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!("\n{0}", ::ts_rs::format_docs(&[" ID発行"])),
                                                    )
                                                }),
                                                "id_issuer",
                                                if false { "?" } else { "" },
                                                <IdIssuer as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0}{1}{2}: {3},",
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!(
                                                            "\n{0}",
                                                            ::ts_rs::format_docs(&[" バージョン"]),
                                                        ),
                                                    )
                                                }),
                                                "version",
                                                if false { "?" } else { "" },
                                                <u32 as ::ts_rs::TS>::name(cfg),
                                            ),
                                        )
                                    }),
                                ],
                                " ",
                            ),
                        ),
                    )
                })
                .replace(" } & { ", " ")
        }
        fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
        where
            Self: 'static,
        {}
        fn output_path() -> Option<std::path::PathBuf> {
            Some(
                std::path::PathBuf::from(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}.ts", "DiagramRoot"))
                    }),
                ),
            )
        }
        fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
        where
            Self: 'static,
        {
            <IndexMap<StationId, Station> as ::ts_rs::TS>::visit_generics(v);
            v.visit::<IndexMap<TrainId, Train>>();
            v.visit::<IndexMap<DiagramViewSettingsId, DiagramViewSettings>>();
            v.visit::<IndexMap<LineId, Line>>();
            <IndexMap<TemplateTrainId, TemplateTrain> as ::ts_rs::TS>::visit_generics(v);
            <IndexMap<
                DiagramViewSettingsId,
                DiagramViewSettings,
            > as ::ts_rs::TS>::visit_generics(v);
            v.visit::<ExtensionProperty>();
            <IdIssuer as ::ts_rs::TS>::visit_generics(v);
            <u32 as ::ts_rs::TS>::visit_generics(v);
            <IndexMap<TrainTypeId, TrainType> as ::ts_rs::TS>::visit_generics(v);
            <IndexMap<TrackId, Track> as ::ts_rs::TS>::visit_generics(v);
            <IndexMap<LineId, Line> as ::ts_rs::TS>::visit_generics(v);
            v.visit::<IndexMap<TimetableId, Timetable>>();
            v.visit::<IndexMap<TrackId, Track>>();
            v.visit::<IdIssuer>();
            <IndexMap<TimetableId, Timetable> as ::ts_rs::TS>::visit_generics(v);
            v.visit::<IndexMap<StationId, Station>>();
            <ExtensionProperty as ::ts_rs::TS>::visit_generics(v);
            <IndexMap<LineSegmentId, LineSegment> as ::ts_rs::TS>::visit_generics(v);
            v.visit::<IndexMap<TrainTypeId, TrainType>>();
            v.visit::<IndexMap<TemplateTrainId, TemplateTrain>>();
            <IndexMap<TrainId, Train> as ::ts_rs::TS>::visit_generics(v);
            v.visit::<IndexMap<LineSegmentId, LineSegment>>();
            v.visit::<u32>();
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DiagramRoot {
        #[inline]
        fn clone(&self) -> DiagramRoot {
            DiagramRoot {
                stations: ::core::clone::Clone::clone(&self.stations),
                tracks: ::core::clone::Clone::clone(&self.tracks),
                segments: ::core::clone::Clone::clone(&self.segments),
                lines: ::core::clone::Clone::clone(&self.lines),
                train_types: ::core::clone::Clone::clone(&self.train_types),
                template_trains: ::core::clone::Clone::clone(&self.template_trains),
                timetables: ::core::clone::Clone::clone(&self.timetables),
                trains: ::core::clone::Clone::clone(&self.trains),
                diagram_view_settings: ::core::clone::Clone::clone(
                    &self.diagram_view_settings,
                ),
                properties: ::core::clone::Clone::clone(&self.properties),
                id_issuer: ::core::clone::Clone::clone(&self.id_issuer),
                version: ::core::clone::Clone::clone(&self.version),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DiagramRoot {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DiagramRoot {
        #[inline]
        fn eq(&self, other: &DiagramRoot) -> bool {
            self.version == other.version && self.stations == other.stations
                && self.tracks == other.tracks && self.segments == other.segments
                && self.lines == other.lines && self.train_types == other.train_types
                && self.template_trains == other.template_trains
                && self.timetables == other.timetables && self.trains == other.trains
                && self.diagram_view_settings == other.diagram_view_settings
                && self.properties == other.properties
                && self.id_issuer == other.id_issuer
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DiagramRoot {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "stations",
                "tracks",
                "segments",
                "lines",
                "train_types",
                "template_trains",
                "timetables",
                "trains",
                "diagram_view_settings",
                "properties",
                "id_issuer",
                "version",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.stations,
                &self.tracks,
                &self.segments,
                &self.lines,
                &self.train_types,
                &self.template_trains,
                &self.timetables,
                &self.trains,
                &self.diagram_view_settings,
                &self.properties,
                &self.id_issuer,
                &&self.version,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "DiagramRoot",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for DiagramRoot {
        #[inline]
        fn default() -> DiagramRoot {
            DiagramRoot {
                stations: ::core::default::Default::default(),
                tracks: ::core::default::Default::default(),
                segments: ::core::default::Default::default(),
                lines: ::core::default::Default::default(),
                train_types: ::core::default::Default::default(),
                template_trains: ::core::default::Default::default(),
                timetables: ::core::default::Default::default(),
                trains: ::core::default::Default::default(),
                diagram_view_settings: ::core::default::Default::default(),
                properties: ::core::default::Default::default(),
                id_issuer: ::core::default::Default::default(),
                version: ::core::default::Default::default(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for DiagramRoot {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private229::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "DiagramRoot",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "stations",
                    &self.stations,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "tracks",
                    &self.tracks,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "segments",
                    &self.segments,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "lines",
                    &self.lines,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "train_types",
                    &self.train_types,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "template_trains",
                    &self.template_trains,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "timetables",
                    &self.timetables,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "trains",
                    &self.trains,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "diagram_view_settings",
                    &self.diagram_view_settings,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "properties",
                    &self.properties,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id_issuer",
                    &self.id_issuer,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for DiagramRoot {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private229::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private229::Formatter,
                    ) -> _serde::__private229::fmt::Result {
                        _serde::__private229::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private229::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private229::Ok(__Field::__field0),
                            1u64 => _serde::__private229::Ok(__Field::__field1),
                            2u64 => _serde::__private229::Ok(__Field::__field2),
                            3u64 => _serde::__private229::Ok(__Field::__field3),
                            4u64 => _serde::__private229::Ok(__Field::__field4),
                            5u64 => _serde::__private229::Ok(__Field::__field5),
                            6u64 => _serde::__private229::Ok(__Field::__field6),
                            7u64 => _serde::__private229::Ok(__Field::__field7),
                            8u64 => _serde::__private229::Ok(__Field::__field8),
                            9u64 => _serde::__private229::Ok(__Field::__field9),
                            10u64 => _serde::__private229::Ok(__Field::__field10),
                            11u64 => _serde::__private229::Ok(__Field::__field11),
                            _ => _serde::__private229::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private229::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "stations" => _serde::__private229::Ok(__Field::__field0),
                            "tracks" => _serde::__private229::Ok(__Field::__field1),
                            "segments" => _serde::__private229::Ok(__Field::__field2),
                            "lines" => _serde::__private229::Ok(__Field::__field3),
                            "train_types" => _serde::__private229::Ok(__Field::__field4),
                            "template_trains" => {
                                _serde::__private229::Ok(__Field::__field5)
                            }
                            "timetables" => _serde::__private229::Ok(__Field::__field6),
                            "trains" => _serde::__private229::Ok(__Field::__field7),
                            "diagram_view_settings" => {
                                _serde::__private229::Ok(__Field::__field8)
                            }
                            "properties" => _serde::__private229::Ok(__Field::__field9),
                            "id_issuer" => _serde::__private229::Ok(__Field::__field10),
                            "version" => _serde::__private229::Ok(__Field::__field11),
                            _ => _serde::__private229::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private229::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"stations" => _serde::__private229::Ok(__Field::__field0),
                            b"tracks" => _serde::__private229::Ok(__Field::__field1),
                            b"segments" => _serde::__private229::Ok(__Field::__field2),
                            b"lines" => _serde::__private229::Ok(__Field::__field3),
                            b"train_types" => _serde::__private229::Ok(__Field::__field4),
                            b"template_trains" => {
                                _serde::__private229::Ok(__Field::__field5)
                            }
                            b"timetables" => _serde::__private229::Ok(__Field::__field6),
                            b"trains" => _serde::__private229::Ok(__Field::__field7),
                            b"diagram_view_settings" => {
                                _serde::__private229::Ok(__Field::__field8)
                            }
                            b"properties" => _serde::__private229::Ok(__Field::__field9),
                            b"id_issuer" => _serde::__private229::Ok(__Field::__field10),
                            b"version" => _serde::__private229::Ok(__Field::__field11),
                            _ => _serde::__private229::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private229::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private229::PhantomData<DiagramRoot>,
                    lifetime: _serde::__private229::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = DiagramRoot;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private229::Formatter,
                    ) -> _serde::__private229::fmt::Result {
                        _serde::__private229::Formatter::write_str(
                            __formatter,
                            "struct DiagramRoot",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private229::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<StationId, Station>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<TrackId, Track>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<LineSegmentId, LineSegment>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<LineId, Line>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<TrainTypeId, TrainType>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<TemplateTrainId, TemplateTrain>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<TimetableId, Timetable>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<TrainId, Train>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field8 = match _serde::de::SeqAccess::next_element::<
                            IndexMap<DiagramViewSettingsId, DiagramViewSettings>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field9 = match _serde::de::SeqAccess::next_element::<
                            ExtensionProperty,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        9usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field10 = match _serde::de::SeqAccess::next_element::<
                            IdIssuer,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        10usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field11 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        11usize,
                                        &"struct DiagramRoot with 12 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private229::Ok(DiagramRoot {
                            stations: __field0,
                            tracks: __field1,
                            segments: __field2,
                            lines: __field3,
                            train_types: __field4,
                            template_trains: __field5,
                            timetables: __field6,
                            trains: __field7,
                            diagram_view_settings: __field8,
                            properties: __field9,
                            id_issuer: __field10,
                            version: __field11,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private229::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private229::Option<
                            IndexMap<StationId, Station>,
                        > = _serde::__private229::None;
                        let mut __field1: _serde::__private229::Option<
                            IndexMap<TrackId, Track>,
                        > = _serde::__private229::None;
                        let mut __field2: _serde::__private229::Option<
                            IndexMap<LineSegmentId, LineSegment>,
                        > = _serde::__private229::None;
                        let mut __field3: _serde::__private229::Option<
                            IndexMap<LineId, Line>,
                        > = _serde::__private229::None;
                        let mut __field4: _serde::__private229::Option<
                            IndexMap<TrainTypeId, TrainType>,
                        > = _serde::__private229::None;
                        let mut __field5: _serde::__private229::Option<
                            IndexMap<TemplateTrainId, TemplateTrain>,
                        > = _serde::__private229::None;
                        let mut __field6: _serde::__private229::Option<
                            IndexMap<TimetableId, Timetable>,
                        > = _serde::__private229::None;
                        let mut __field7: _serde::__private229::Option<
                            IndexMap<TrainId, Train>,
                        > = _serde::__private229::None;
                        let mut __field8: _serde::__private229::Option<
                            IndexMap<DiagramViewSettingsId, DiagramViewSettings>,
                        > = _serde::__private229::None;
                        let mut __field9: _serde::__private229::Option<
                            ExtensionProperty,
                        > = _serde::__private229::None;
                        let mut __field10: _serde::__private229::Option<IdIssuer> = _serde::__private229::None;
                        let mut __field11: _serde::__private229::Option<u32> = _serde::__private229::None;
                        while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private229::Option::is_some(&__field0) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "stations",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            IndexMap<StationId, Station>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private229::Option::is_some(&__field1) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("tracks"),
                                        );
                                    }
                                    __field1 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            IndexMap<TrackId, Track>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private229::Option::is_some(&__field2) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "segments",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            IndexMap<LineSegmentId, LineSegment>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private229::Option::is_some(&__field3) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("lines"),
                                        );
                                    }
                                    __field3 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            IndexMap<LineId, Line>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private229::Option::is_some(&__field4) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "train_types",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            IndexMap<TrainTypeId, TrainType>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private229::Option::is_some(&__field5) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "template_trains",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            IndexMap<TemplateTrainId, TemplateTrain>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private229::Option::is_some(&__field6) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "timetables",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            IndexMap<TimetableId, Timetable>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private229::Option::is_some(&__field7) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("trains"),
                                        );
                                    }
                                    __field7 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            IndexMap<TrainId, Train>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private229::Option::is_some(&__field8) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "diagram_view_settings",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            IndexMap<DiagramViewSettingsId, DiagramViewSettings>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private229::Option::is_some(&__field9) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "properties",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ExtensionProperty,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::__private229::Option::is_some(&__field10) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id_issuer",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<IdIssuer>(&mut __map)?,
                                    );
                                }
                                __Field::__field11 => {
                                    if _serde::__private229::Option::is_some(&__field11) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field11 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private229::Some(__field0) => __field0,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("stations")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private229::Some(__field1) => __field1,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("tracks")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private229::Some(__field2) => __field2,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("segments")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private229::Some(__field3) => __field3,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("lines")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private229::Some(__field4) => __field4,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("train_types")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private229::Some(__field5) => __field5,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("template_trains")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private229::Some(__field6) => __field6,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("timetables")?
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private229::Some(__field7) => __field7,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("trains")?
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private229::Some(__field8) => __field8,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field(
                                    "diagram_view_settings",
                                )?
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::__private229::Some(__field9) => __field9,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("properties")?
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::__private229::Some(__field10) => __field10,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("id_issuer")?
                            }
                        };
                        let __field11 = match __field11 {
                            _serde::__private229::Some(__field11) => __field11,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("version")?
                            }
                        };
                        _serde::__private229::Ok(DiagramRoot {
                            stations: __field0,
                            tracks: __field1,
                            segments: __field2,
                            lines: __field3,
                            train_types: __field4,
                            template_trains: __field5,
                            timetables: __field6,
                            trains: __field7,
                            diagram_view_settings: __field8,
                            properties: __field9,
                            id_issuer: __field10,
                            version: __field11,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "stations",
                    "tracks",
                    "segments",
                    "lines",
                    "train_types",
                    "template_trains",
                    "timetables",
                    "trains",
                    "diagram_view_settings",
                    "properties",
                    "id_issuer",
                    "version",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "DiagramRoot",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private229::PhantomData::<DiagramRoot>,
                        lifetime: _serde::__private229::PhantomData,
                    },
                )
            }
        }
    };
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
    pub enum RnaError {
        #[error("フィールド '{0}' が見つかりません")]
        FieldNotFound(String),
        #[error(
            "型の不一致: 値をこのフィールドの型に変換できませんでした"
        )]
        TypeMismatch,
        #[error("このフィールドは読み取り専用です")]
        ReadOnly,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RnaError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                RnaError::FieldNotFound(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FieldNotFound",
                        &__self_0,
                    )
                }
                RnaError::TypeMismatch => {
                    ::core::fmt::Formatter::write_str(f, "TypeMismatch")
                }
                RnaError::ReadOnly => ::core::fmt::Formatter::write_str(f, "ReadOnly"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RnaError {
        #[inline]
        fn clone(&self) -> RnaError {
            match self {
                RnaError::FieldNotFound(__self_0) => {
                    RnaError::FieldNotFound(::core::clone::Clone::clone(__self_0))
                }
                RnaError::TypeMismatch => RnaError::TypeMismatch,
                RnaError::ReadOnly => RnaError::ReadOnly,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RnaError {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RnaError {
        #[inline]
        fn eq(&self, other: &RnaError) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        RnaError::FieldNotFound(__self_0),
                        RnaError::FieldNotFound(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::thiserror::__private19::Error for RnaError {}
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for RnaError {
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            use ::thiserror::__private19::AsDisplay as _;
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                RnaError::FieldNotFound(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "フィールド \'{0}\' が見つかりません",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                RnaError::TypeMismatch {} => {
                    __formatter
                        .write_str(
                            "型の不一致: 値をこのフィールドの型に変換できませんでした",
                        )
                }
                RnaError::ReadOnly {} => {
                    __formatter
                        .write_str("このフィールドは読み取り専用です")
                }
            }
        }
    }
    /// 全ての DNA 構造体・基本型が実装する「動的アクセス窓口」トレイト
    pub trait RnaObject: Any {
        /// 1. フィールドの参照を取得する (Read)
        ///    指定された key (例: "name") に対応するフィールドの &dyn RnaObject を返す
        fn rna_get(&self, _key: &str) -> Option<&dyn RnaObject> {
            None
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
    impl RnaObject for bool {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::Boolean(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for bool {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::Boolean(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for u8 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::U8(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for u8 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::U8(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for i8 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::I8(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for i8 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::I8(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for u16 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::U16(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for u16 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::U16(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for i16 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::I16(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for i16 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::I16(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for u32 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::U32(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for u32 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::U32(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for i32 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::I32(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for i32 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::I32(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for u64 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::U64(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for u64 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::U64(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for i64 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::I64(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for i64 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::I64(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for u128 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::U128(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for u128 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::U128(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for i128 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::I128(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for i128 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::I128(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for f32 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::F32(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for f32 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::F32(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
    impl RnaObject for f64 {
        fn to_heddle(&self) -> Option<Heddle> {
            Some(Heddle::F64(*self))
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl TryFrom<Heddle> for f64 {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::F64(s) => Ok(s),
                _ => Err(crate::model::RnaError::TypeMismatch),
            }
        }
    }
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
            Ok(
                Cell::new(
                    T::try_from(value.clone()).map_err(|_| RnaError::TypeMismatch)?,
                ),
            )
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
                Some(Heddle::Array(value.iter().map(|v| v.clone().unwrap()).collect()))
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
                Heddle::Array(s) => {
                    s.into_iter().map(|v| T::try_from(v)).collect::<Result<Vec<T>, _>>()
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
    impl<
        T: RnaObject + Hash + TryFrom<Heddle> + Eq,
        S: RnaObject + TryFrom<Heddle>,
    > TryFrom<Heddle> for IndexMap<T, S> {
        type Error = crate::model::RnaError;
        fn try_from(value: Heddle) -> Result<Self, Self::Error> {
            match value {
                Heddle::Compound(s) => {
                    let mut map = IndexMap::new();
                    for (key, value) in s.iter() {
                        map.insert(
                            T::try_from(key.clone())
                                .map_err(|_| RnaError::TypeMismatch)?,
                            S::try_from(value.clone())
                                .map_err(|_| RnaError::TypeMismatch)?,
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
    /// 合計メモリ量を取得できる型が実装するトレイト
    pub trait TotalSizable {
        /// オブジェクトの合計メモリ量
        fn total_bytes(&self) -> usize;
    }
}
