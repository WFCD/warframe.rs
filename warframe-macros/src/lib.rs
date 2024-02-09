use std::error::Error;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
    ItemFn,
};

fn impl_listen(
    model_ident: Ident,
    state_ident: Option<Ident>,
    item: ItemFn,
) -> Result<TokenStream2, Box<dyn Error>> {
    let model_name = model_ident.to_string();

    let original_fn_name = item.sig.ident.clone();

    let mut item_clone = item.clone();
    item_clone.sig.ident = Ident::from_string(&format!("_{}", item.sig.ident))?;

    let item_sig = Ident::from_string(&format!("_{}", item.sig.ident))?;

    let state: Option<TokenStream2> = state_ident
        .clone()
        .map(|ident| format!("state: &{},", ident).parse().unwrap());

    let state_arg: Option<TokenStream2> = state_ident.map(|_| "&state,".parse().unwrap());

    Ok(quote! {
        async fn #original_fn_name(
            #state
            client: &warframe::worldstate::client::Client,
        ) -> Result<(), warframe::worldstate::error::ApiError> {
            use warframe::worldstate::prelude::*;
            use log::debug;
            let mut item = client.fetch::<#model_ident>().await?;

            #item_clone


            loop {
                if item.expiry() <= warframe::chrono::offset::Utc::now() {
                    debug!("{} :: Looking for state change from the API", #model_name);
                    warframe::tokio::time::sleep(std::time::Duration::from_secs(30)).await;

                    let new_item = client.fetch::<#model_ident>().await?;

                    if item.expiry() >= new_item.expiry() {
                        continue;
                    } else {
                        // call callback fn
                        #item_sig(#state_arg &item, &new_item).await;
                        item = new_item;
                    }
                }

                let time_to_sleep = item.expiry() - warframe::chrono::offset::Utc::now();

                debug!(
                    "{} :: Sleeping {} seconds",
                    #model_name,
                    time_to_sleep.num_seconds()
                );
                warframe::tokio::time::sleep(time_to_sleep.to_std().unwrap()).await;
            }
        }
    })
}

#[proc_macro_attribute]
pub fn listen(args: TokenStream, input: TokenStream) -> TokenStream {
    let ParsedIdents { first, second } = parse_macro_input!(args as ParsedIdents);

    impl_listen(first, second, parse_macro_input!(input))
        .unwrap()
        .into()
}

struct ParsedIdents {
    first: Ident,
    second: Option<Ident>,
}

impl Parse for ParsedIdents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first = input.parse()?;
        let mut second = None;
        if input.peek(Comma) {
            let _: Comma = input.parse()?;
            second = Some(input.parse()?);
        }

        Ok(ParsedIdents { first, second })
    }
}

fn impl_listen_any(
    model_ident: Ident,
    state_ident: Option<Ident>,
    item: ItemFn,
) -> Result<TokenStream2, Box<dyn Error>> {
    let model_name = model_ident.to_string();

    let original_fn_name = item.sig.ident.clone();

    let mut item_clone = item.clone();
    item_clone.sig.ident = Ident::from_string(&format!("_{}", item.sig.ident)).unwrap();
    let item_sig = Ident::from_string(&format!("_{}", item.sig.ident)).unwrap();

    let state: Option<TokenStream2> = state_ident
        .clone()
        .map(|ident| format!("state: &{},", ident).parse().unwrap());

    let state_arg: Option<TokenStream2> = state_ident.map(|_| "&state,".parse().unwrap());

    Ok(quote! {
        async fn #original_fn_name(
            #state
            client: &warframe::worldstate::client::Client,
        ) -> Result<(), warframe::worldstate::error::ApiError> {
            use log::debug;
            use warframe::worldstate::prelude::*;

            #item_clone

            let mut items = client.fetch_arr::<#model_ident>().await?;

            loop {
                debug!("{} :: Fetching for changes", #model_name);
                warframe::tokio::time::sleep(std::time::Duration::from_secs(60)).await;

                let new_items = client.fetch_arr::<#model_ident>().await?;

                let removed_items: Vec<_> = items
                    .iter()
                    .filter(|&item| !new_items.contains(item))
                    .map(warframe::worldstate::models::Change::Removed)
                    .collect();

                let added_items: Vec<_> = new_items
                    .iter()
                    .filter(|&item| !items.contains(item))
                    .map(warframe::worldstate::models::Change::Added)
                    .collect();

                if !removed_items.is_empty() && !added_items.is_empty() {
                    debug!("{} :: Found changes", #model_name);
                    for item in removed_items.into_iter().chain(added_items) {
                        // call callback fn
                        #item_sig(#state_arg item).await;
                    }
                    items = new_items;
                }
            }
        }
    })
}

#[proc_macro_attribute]
pub fn listen_any(args: TokenStream, input: TokenStream) -> TokenStream {
    let ParsedIdents { first, second } = parse_macro_input!(args as ParsedIdents);

    impl_listen_any(first, second, parse_macro_input!(input))
        .unwrap()
        .into()
}
