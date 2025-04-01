mod model;

use manyhow::manyhow;
use proc_macro::TokenStream;

/// This is a macro to model a worldstate (api) object.
///
/// Note that this macro will automatically put `#[serde(rename_all = "camelCase")]` on your struct.
///
/// Additionally, it will derive `Debug, Clone, PartialEq, PartialOrd, serde::Deserialize` for
/// structs, and `Debug, PartialEq, PartialOrd, Clone, Eq, Copy, Hash, derive_more::Display` for
/// enums.
///
/// # Args
/// - `endpoint = "/endpoint"` the endpoint to request
/// - `return_style = Array/Object` the JSON format returned by the API
/// - `timed (= true/false)` defaults to false. Adds an `activation` / `expiry` field and implements
///   [TimedEvent](../warframe/worldstate/trait.TimedEvent.html)
#[manyhow]
#[proc_macro_attribute]
pub fn model(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    model::expand(args.into(), item.into()).map(Into::into)
}
