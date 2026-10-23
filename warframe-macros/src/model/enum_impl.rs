use std::collections::VecDeque;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Expr,
    ExprLit,
    ItemEnum,
    Lit,
    spanned::Spanned,
};

/// Parses and transforms a model enum, adding standard derives and serde deserialization.
pub fn parse_enum(_args: TokenStream, mut item: ItemEnum) -> syn::Result<TokenStream> {
    // this is here to be able to have the derives at the very top
    let mut vdq = VecDeque::from(item.attrs.clone());

    // Only derive Copy when all variants are unit variants (no data-carrying fields)
    let all_unit = item
        .variants
        .iter()
        .all(|v| matches!(v.fields, syn::Fields::Unit));

    if all_unit {
        vdq.push_front(syn::parse_quote!( #[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Copy, Hash, derive_more::Display)] ));
    } else {
        vdq.push_front(syn::parse_quote!( #[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Hash, derive_more::Display)] ));
    }

    // check whether the enum has a discriminant, and maybe implement Deserialize_repr
    match item
        .variants
        .first()
        .ok_or_else(|| syn::Error::new(item.span(), "Needs at least 1 variant"))?
        .discriminant
    {
        Some((
            _,
            Expr::Lit(ExprLit {
                lit: Lit::Int(_), ..
            }),
        )) => {
            // repr(u32) for potentially needed flexibility
            vdq.push_front(syn::parse_quote! {
                #[derive(serde_repr::Deserialize_repr)]
            });
            vdq.push_front(syn::parse_quote! {
                #[repr(u32)]
            });
        }
        _ => vdq.push_front(syn::parse_quote!( #[derive(serde::Deserialize)] )),
    };

    let opposite_trait_impl = match item.variants.len() {
        2 if all_unit => {
            let (a, b) = (
                &item.variants.first().unwrap().ident,
                &item.variants.last().unwrap().ident,
            );

            let type_name = &item.ident;

            Some(quote! {
                impl crate::worldstate::models::base::Opposite for #type_name {
                    fn opposite(&self) -> Self {
                        match self {
                            #type_name::#a => #type_name::#b,
                            #type_name::#b => #type_name::#a,
                        }
                    }
                }
            })
        }
        _ => None,
    };

    item.attrs = vdq.into();

    Ok(quote! {
        #item

        #opposite_trait_impl
    })
}
