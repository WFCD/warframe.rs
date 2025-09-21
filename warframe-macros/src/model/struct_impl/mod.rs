mod args;
mod return_style;

use std::collections::VecDeque;

use args::{
    Args,
    QueryableImpl,
};
use proc_macro2::{
    Span,
    TokenStream,
};
use quote::quote;
use syn::{
    Error,
    Field,
    FieldMutability,
    Fields,
    FieldsNamed,
    Ident,
    ItemStruct,
    Meta,
    Visibility,
    parse_quote,
    token::Colon,
};

fn append_activation_and_expiry(
    fields: &mut FieldsNamed,
    activation_attrs: Vec<Meta>,
    expiry_attrs: Vec<Meta>,
) {
    let activation = Field {
        ident: Some(Ident::new("activation", Span::call_site())),
        colon_token: Some(Colon::default()),
        mutability: FieldMutability::None,
        ty: syn::parse_str("chrono::DateTime<chrono::Utc>").unwrap(),
        vis: Visibility::Inherited,
        attrs: parse_quote!( #( #[#activation_attrs] )* ),
    };

    let expiry = Field {
        ident: Some(Ident::new("expiry", Span::call_site())),
        colon_token: Some(Colon::default()),
        mutability: FieldMutability::None,
        ty: syn::parse_str("chrono::DateTime<chrono::Utc>").unwrap(),
        vis: Visibility::Inherited,
        attrs: parse_quote!( #( #[#expiry_attrs] )* ),
    };

    fields.named.push(activation);
    fields.named.push(expiry);
}

pub fn parse_struct(args: TokenStream, mut item: ItemStruct) -> syn::Result<TokenStream> {
    let mut vdq = VecDeque::from(item.attrs.clone());
    vdq.push_front(syn::parse_quote!(#[serde(rename_all = "camelCase")]));
    vdq.push_front(syn::parse_quote!(#[derive(Debug, Clone, PartialEq, serde::Deserialize)]));
    item.attrs = vdq.into();

    // panic!("{:?}", item.attrs);

    let struct_name = &item.ident;

    let args: Args = syn::parse2(args)?;

    if args.timed.value {
        let fields = match &mut item.fields {
            Fields::Named(fields_named) => fields_named,
            _ => {
                return Err(Error::new_spanned(
                    &item,
                    "Only Named Fields are supported!",
                ));
            }
        };

        append_activation_and_expiry(
            fields,
            args.activation_attrs.unwrap_or_default(),
            args.expiry_attrs.unwrap_or_default(),
        );
    }

    let timed_event_trait_impl = args.timed.value.then(|| {
        quote! {
            impl crate::worldstate::models::base::TimedEvent for #struct_name {
                #[doc = "Returns the time when this event began"]
                fn activation(&self) -> chrono::DateTime<chrono::Utc> {
                    self.activation
                }
                #[doc = "Returns the time when this event ends"]
                fn expiry(&self) -> chrono::DateTime<chrono::Utc> {
                    self.expiry
                }
            }
        }
    });

    let (endpoint_trait_impl, queryable_trait_impl) = match args.queryable_params {
        None => (None, None),
        Some(params) => QueryableImpl::try_from_queryable_params(struct_name, &params)
            .map(|impls| (impls.impl_endpoint, impls.impl_queryable))?,
    };

    Ok(quote! {
        #item

        #timed_event_trait_impl

        #endpoint_trait_impl

        #queryable_trait_impl
    })
}
