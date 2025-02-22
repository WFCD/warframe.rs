mod model;

use manyhow::manyhow;
use proc_macro::TokenStream;

#[manyhow]
#[proc_macro_attribute]
pub fn model(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    model::expand(args.into(), item.into()).map(Into::into)
}
