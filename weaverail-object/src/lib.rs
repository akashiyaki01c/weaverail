extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(WeaverailDNA)]
pub fn derive_weaverail_dna(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let dna_impl = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                // 名前付きフィールド { name: String, ... } の場合
                Fields::Named(fields) => {
                    let field_iter = fields.named.iter().map(|f| {
                        let field_name = &f.ident; // フィールド名 (name, mileage等)
                        let field_type = &f.ty;    // 型 (String, f64等)

                        // ここで各フィールドに対してやりたい処理を書く
                        quote! {
                            println!("DNA Field: {} (Type: {})", stringify!(#field_name), stringify!(#field_type));
                        }
                    });
                    quote! { #(#field_iter)* }
                }
                _ => quote! {}, // タプル型 struct(u32, u32) 等は一旦無視
            }
        }
        _ => panic!("WeaverailDNA can only be derived for structs"),
    };
    // 4. 最終的に出力するRustコードを組み立てる
    let expanded = quote! {
        impl #name {
            pub fn print_dna_info() {
                println!("--- DNA Information for {} ---", stringify!(#name));
                #dna_impl
            }
        }
    };

    // 5. TokenStreamに変換して返す
    TokenStream::from(expanded)
}

#[proc_macro_derive(RnaObjectable)]
pub fn derive_weaverail_rna(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let expanded = match &input.data {
        // ==========================================
        // 1. Struct (構造体 & タプル構造体) の場合
        // ==========================================
        Data::Struct(data) => {
            let mut get_arms = Vec::new();
            let mut get_mut_arms = Vec::new();
            let mut set_arms = Vec::new();
            let mut to_heddle = Vec::new();
            let mut stack_count_arms = Vec::new();
            let mut heap_count_arms = Vec::new();

            match &data.fields {
                // ① 名前付きフィールド構造体: struct Station { id: String, name: String }
                Fields::Named(fields) => {
                    for f in &fields.named {
                        let field_name = f.ident.as_ref().unwrap();
                        let field_str = field_name.to_string();

                        get_arms.push(quote! {
                            #field_str => Some(&self.#field_name as &dyn crate::model::RnaObject),
                        });

                        get_mut_arms.push(quote! {
                            #field_str => Some(&mut self.#field_name as &mut dyn crate::model::RnaObject),
                        });

                        set_arms.push(quote! {
                            #field_str => {
                                self.#field_name = value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)?;
                                Ok(())
                            }
                        });

                        to_heddle.push(quote! {
                            obj.insert(crate::path::Heddle::String(stringify!(#field_name).to_string()), self.#field_name.to_heddle()?);
                        });

                        stack_count_arms.push(quote! {
                            + self.#field_name.get_stack_memory_size()
                        });
                        heap_count_arms.push(quote! {
                            + self.#field_name.get_heap_memory_size()
                        });
                    }
                }

                // ② タプル構造体: struct StationId(pub String) や struct GeoPoint(f64, f64)
                Fields::Unnamed(fields) => {
                    for (i, _) in fields.unnamed.iter().enumerate() {
                        let index = syn::Index::from(i); // self.0, self.1 用のインデックス表現
                        let idx_str = i.to_string(); // "0", "1" 用の文字列

                        get_arms.push(quote! {
                            #idx_str => Some(&self.#index as &dyn crate::model::RnaObject),
                        });

                        get_mut_arms.push(quote! {
                            #idx_str => Some(&mut self.#index as &mut dyn crate::model::RnaObject),
                        });

                        set_arms.push(quote! {
                            #idx_str => {
                                self.#index = value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)?;
                                Ok(())
                            }
                        });

                        to_heddle.push(quote! {
                            obj.insert(crate::path::Heddle::String(#idx_str.to_string()), self.#index.to_heddle()?);
                        });
                    }
                }

                // ③ ユニット構造体: struct EmptyMarker;
                Fields::Unit => {}
            }

            quote! {
                impl crate::model::RnaObject for #name {
                    fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                        match key {
                            #(#get_arms)*
                            _ => None,
                        }
                    }

                    fn rna_get_mut(&mut self, key: &str) -> Option<&mut dyn crate::model::RnaObject> {
                        match key {
                            #(#get_mut_arms)*
                            _ => None,
                        }
                    }

                    fn rna_set(&mut self, key: &str, value: crate::path::Heddle) -> Result<(), crate::model::RnaError> {
                        match key {
                            #(#set_arms)*
                            _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                        }
                    }

                    fn as_any(&self) -> &dyn std::any::Any {
                        self
                    }

                    fn to_heddle(&self) -> Option<crate::path::Heddle> {
                        let mut obj = ::indexmap::IndexMap::new();
                        #(#to_heddle)*

                        Some(crate::path::Heddle::Compound(obj))
                    }
                }

                impl TryFrom<crate::path::Heddle> for #name {
                    type Error = crate::model::RnaError;

                    fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                        value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
                    }
                }

                impl crate::primitives::TotalSizable<#name> for #name {
                    fn get_stack_memory_size(&self) -> usize {
                        0 #(#stack_count_arms)*
                    }
                    fn get_heap_memory_size(&self) -> usize {
                        0 #(#heap_count_arms)*
                    }
                }
            }
        }

        // ==========================================
        // 2. Enum (内部フィールド深掘り対応) の場合
        // ==========================================
        Data::Enum(data) => {
            let mut get_arms = Vec::new();
            let mut get_mut_arms = Vec::new();
            let mut set_arms = Vec::new();
            let mut stack_count_arms = Vec::new();
            let mut heap_count_arms = Vec::new();

            for variant in &data.variants {
                let v_ident = &variant.ident;

                match &variant.fields {
                    // ① 名前付きフィールドバリアント: Branch { target_station_id, switch_speed_limit }
                    Fields::Named(fields) => {
                        let mut inner_get = Vec::new();
                        let mut inner_get_mut = Vec::new();
                        let mut inner_set = Vec::new();
                        let mut inner_stack_count_arms = Vec::new();
                        let mut inner_heap_count_arms = Vec::new();

                        let field_names: Vec<_> = fields
                            .named
                            .iter()
                            .map(|f| f.ident.as_ref().unwrap())
                            .collect();

                        for field_name in &field_names {
                            let field_str = field_name.to_string();

                            inner_get.push(quote! {
                                #field_str => Some(#field_name as &dyn crate::model::RnaObject),
                            });
                            inner_get_mut.push(quote! {
                                #field_str => Some(#field_name as &mut dyn crate::model::RnaObject),
                            });
                            inner_set.push(quote! {
                                #field_str => {
                                    *#field_name = value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)?;
                                    Ok(())
                                }
                            });
                            inner_stack_count_arms.push(quote! {
                                + #field_name.get_stack_memory_size()
                            });
                            inner_heap_count_arms.push(quote! {
                                + #field_name.get_heap_memory_size()
                            });
                        }

                        get_arms.push(quote! {
                            Self::#v_ident { #(#field_names),* } => match key {
                                #(#inner_get)*
                                _ => None,
                            },
                        });
                        get_mut_arms.push(quote! {
                            Self::#v_ident { #(#field_names),* } => match key {
                                #(#inner_get_mut)*
                                _ => None,
                            },
                        });
                        set_arms.push(quote! {
                            Self::#v_ident { #(#field_names),* } => match key {
                                #(#inner_set)*
                                _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                            },
                        });
                        stack_count_arms.push(quote! {
                            Self::#v_ident { #(#field_names),* } => 0 #(#inner_stack_count_arms)*,
                        });
                        heap_count_arms.push(quote! {
                            Self::#v_ident { #(#field_names),* } => 0 #(#inner_heap_count_arms)*,
                        });
                    }

                    // ② タプル型バリアント: Sub(u32, String) -> インデックス "0", "1" でアクセス
                    Fields::Unnamed(fields) => {
                        let bindings: Vec<_> = (0..fields.unnamed.len())
                            .map(|i| quote::format_ident!("f{}", i))
                            .collect();

                        let mut inner_get = Vec::new();
                        let mut inner_get_mut = Vec::new();
                        let mut inner_set = Vec::new();
                        let mut inner_stack_count_arms = Vec::new();
                        let mut inner_heap_count_arms = Vec::new();

                        for (i, binding) in bindings.iter().enumerate() {
                            let idx_str = i.to_string();

                            inner_get.push(quote! {
                                #idx_str => Some(#binding as &dyn crate::model::RnaObject),
                            });
                            inner_get_mut.push(quote! {
                                #idx_str => Some(#binding as &mut dyn crate::model::RnaObject),
                            });
                            inner_set.push(quote! {
                                #idx_str => {
                                    *#binding = value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)?;
                                    Ok(())
                                }
                            });
                            inner_stack_count_arms.push(quote! {
                                + #binding.get_stack_memory_size()
                            });
                            inner_heap_count_arms.push(quote! {
                                + #binding.get_heap_memory_size()
                            });
                        }

                        get_arms.push(quote! {
                            Self::#v_ident(#(#bindings),*) => match key {
                                #(#inner_get)*
                                _ => None,
                            },
                        });
                        get_mut_arms.push(quote! {
                            Self::#v_ident(#(#bindings),*) => match key {
                                #(#inner_get_mut)*
                                _ => None,
                            },
                        });
                        set_arms.push(quote! {
                            Self::#v_ident(#(#bindings),*) => match key {
                                #(#inner_set)*
                                _ => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                            },
                        });
                        stack_count_arms.push(quote! {
                            Self::#v_ident ( #(#bindings),* ) => 0 #(#inner_stack_count_arms)*,
                        });
                        heap_count_arms.push(quote! {
                            Self::#v_ident ( #(#bindings),* ) => 0 #(#inner_heap_count_arms)*,
                        });
                    }

                    // ③ ユニットバリアント: Main
                    Fields::Unit => {
                        get_arms.push(quote! {
                            Self::#v_ident => None,
                        });
                        get_mut_arms.push(quote! {
                            Self::#v_ident => None,
                        });
                        set_arms.push(quote! {
                            Self::#v_ident => Err(crate::model::RnaError::FieldNotFound(key.to_string())),
                        });
                        stack_count_arms.push(quote! {
                            Self::#v_ident => 0
                        });
                        heap_count_arms.push(quote! {
                            Self::#v_ident => 0
                        });
                    }
                }
            }

            quote! {
                impl crate::model::RnaObject for #name {
                    fn rna_get(&self, key: &str) -> Option<&dyn crate::model::RnaObject> {
                        match self {
                            #(#get_arms)*
                        }
                    }

                    fn rna_get_mut(&mut self, key: &str) -> Option<&mut dyn crate::model::RnaObject> {
                        match self {
                            #(#get_mut_arms)*
                        }
                    }

                    fn rna_set(&mut self, key: &str, value: crate::path::Heddle) -> Result<(), crate::model::RnaError> {
                        match self {
                            #(#set_arms)*
                        }
                    }

                    fn as_any(&self) -> &dyn std::any::Any {
                        self
                    }
                }

                impl TryFrom<crate::path::Heddle> for #name {
                    type Error = crate::model::RnaError;

                    fn try_from(value: crate::path::Heddle) -> Result<Self, Self::Error> {
                        value.try_into().map_err(|_| crate::model::RnaError::TypeMismatch)
                    }
                }

                impl crate::primitives::TotalSizable<#name> for #name {
                    fn get_stack_memory_size(&self) -> usize {
                        match self {
                            #(#stack_count_arms)*
                        }
                    }
                    fn get_heap_memory_size(&self) -> usize {
                        match self {
                            #(#heap_count_arms)*
                        }
                    }
                } 
            }
        }

        _ => panic!("RnaObjectable can only be derived for structs and enums"),
    };

    TokenStream::from(expanded)
}
