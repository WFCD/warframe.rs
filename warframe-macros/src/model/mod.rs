use enum_impl::parse_enum;
use proc_macro2::TokenStream;
use struct_impl::parse_struct;
use syn::{
    Item,
    spanned::Spanned,
};

mod enum_impl;
mod struct_impl;

pub fn expand(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    match syn::parse2::<Item>(item)? {
        Item::Enum(enum_item) => parse_enum(args, enum_item),
        Item::Struct(struct_item) => parse_struct(args, struct_item),
        item => Err(syn::Error::new(
            item.span(),
            "Only structs and enums are supported",
        )),
    }
}
