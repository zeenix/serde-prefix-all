#![deny(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs
)]
#![warn(unreachable_pub)]
#![allow(clippy::result_unit_err, clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, spanned::Spanned, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Lit,
};

/// Prefix all fields of a struct or enum with a given prefix.
///
/// See [crate-level documentation](index.html) for more information.
#[proc_macro_attribute]
pub fn prefix_all(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let prefix = parse_macro_input!(attrs as Lit);

    match try_prefix_all(prefix, input) {
        Ok(token_stream) => token_stream,
        Err(err) => err.into_compile_error().into(),
    }
}

fn try_prefix_all(prefix: Lit, mut input: DeriveInput) -> syn::Result<TokenStream> {
    let prefix = match &prefix {
        Lit::Str(string) => string.value(),
        _ => return Err(Error::new(prefix.span(), "The attribute is not a string")),
    };

    match &mut input.data {
        Data::Enum(item_enum) => handle_enum(item_enum, &prefix[..]),
        Data::Struct(item_struct) => handle_struct(item_struct, &prefix[..]),
        _ => {
            return Err(Error::new(
                prefix.span(),
                "You can't use the macro on this type",
            ))
        }
    };

    Ok(input.to_token_stream().into())
}

fn create_attribute(prefix: &str, field_name: &str) -> Attribute {
    let attr_prefix = format!("{prefix}{field_name}");
    let attr: Attribute = parse_quote! { #[serde(rename = #attr_prefix)] };
    attr
}

fn handle_enum(input: &mut DataEnum, prefix: &str) {
    let variants = &mut input.variants;
    for variant in variants.iter_mut() {
        let field_name = variant.ident.to_string();
        let attr = create_attribute(prefix, &field_name[..]);
        variant.attrs.push(attr);
    }
}

fn handle_struct(input: &mut DataStruct, prefix: &str) {
    let fields = &mut input.fields;
    for field in fields.iter_mut() {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let attr = create_attribute(prefix, &field_name[..]);
        field.attrs.push(attr);
    }
}
