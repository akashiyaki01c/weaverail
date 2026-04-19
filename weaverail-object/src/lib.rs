extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, parse_macro_input};
use quote::quote;

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