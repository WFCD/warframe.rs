use std::error::Error;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, ItemFn, Meta};

fn impl_listen(model_ident: Ident, item: ItemFn) -> Result<TokenStream2, Box<dyn Error>> {
    let model_name = model_ident.to_string();

    let original_fn_name = item.sig.ident.clone();

    let mut item_clone = item.clone();
    item_clone.sig.ident = Ident::from_string(&format!("_{}", item.sig.ident)).unwrap();
    let item_sig = Ident::from_string(&format!("_{}", item.sig.ident)).unwrap();

    Ok(quote! {
        async fn #original_fn_name(
            client: &warframe::worldstate::client::Client,
        ) -> Result<(), warframe::worldstate::error::ApiError> {
            use warframe::worldstate::prelude::*;
            use log::debug;
            let mut item = client.fetch::<#model_ident>().await?;

            #item_clone

            loop {
                if item.expiry() <= chrono::offset::Utc::now() {
                    debug!("{} :: Looking for state change from the API", #model_name);
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

                    let new_item = client.fetch::<#model_ident>().await?;

                    if item.expiry() >= new_item.expiry() {
                        continue;
                    } else {
                        // call callback fn
                        #item_sig(&item, &new_item).await;
                        item = new_item;
                    }
                }

                let time_to_sleep = item.expiry() - chrono::offset::Utc::now();

                debug!(
                    "{} :: Sleeping {} seconds",
                    #model_name,
                    time_to_sleep.num_seconds()
                );
                tokio::time::sleep(time_to_sleep.to_std().unwrap()).await;
            }
        }
    })
}

#[proc_macro_attribute]
pub fn listen(args: TokenStream, input: TokenStream) -> TokenStream {
    let model_ident = match parse_macro_input!(args) {
        Meta::Path(path) => path.get_ident().cloned().unwrap(),
        _ => panic!("Expected model type identifier."),
    };

    impl_listen(model_ident, parse_macro_input!(input))
        .unwrap()
        .into()
}

fn impl_listen_any(model_ident: Ident, item: ItemFn) -> Result<TokenStream2, Box<dyn Error>> {
    let model_name = model_ident.to_string();

    let original_fn_name = item.sig.ident.clone();

    let mut item_clone = item.clone();
    item_clone.sig.ident = Ident::from_string(&format!("_{}", item.sig.ident)).unwrap();
    let item_sig = Ident::from_string(&format!("_{}", item.sig.ident)).unwrap();

    Ok(quote! {
        async fn #original_fn_name(
            client: &warframe::worldstate::client::Client,
        ) -> Result<(), warframe::worldstate::error::ApiError> {
            use log::debug;
            use warframe::worldstate::prelude::*;

            #item_clone

            let mut items = client.fetch_arr::<#model_ident>().await?;

            loop {
                debug!("{} :: Fetching for changes", #model_name);
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;

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
                        #item_sig(item).await;
                    }
                    items = new_items;
                }
            }
        }
    })
}

#[proc_macro_attribute]
pub fn listen_any(args: TokenStream, input: TokenStream) -> TokenStream {
    let model_ident = match parse_macro_input!(args) {
        Meta::Path(path) => path.get_ident().cloned().unwrap(),
        _ => panic!("Expected model type identifier."),
    };

    impl_listen_any(model_ident, parse_macro_input!(input))
        .unwrap()
        .into()
}
