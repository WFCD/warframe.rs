use proc_macro2::{
    Span,
    TokenStream,
};
use quote::quote;
use syn::{
    Error,
    Ident,
    LitBool,
    LitStr,
    Meta,
    Token,
    parenthesized,
    parse::{
        Parse,
        ParseStream,
        Parser,
    },
    punctuated::Punctuated,
    spanned::Spanned,
};

use super::return_style::ReturnStyle;

pub struct QueryableParams {
    endpoint: LitStr,
    return_style: ReturnStyle,
}

pub struct QueryableImpl {
    pub impl_endpoint: Option<TokenStream>,
    pub impl_queryable: Option<TokenStream>,
}

fn get_endpoint_impl(struct_name: &Ident, endpoint: &LitStr) -> syn::Result<TokenStream> {
    let endpoint_value = endpoint.value();

    Ok(quote! {
        impl crate::worldstate::models::base::Endpoint for #struct_name {
            fn endpoint(base_url: &str, language: crate::worldstate::language::Language) -> String {
                format!(
                    "{}/pc{}/?language={}",
                    base_url,
                    #endpoint_value,
                    language
                )
            }
        }
    })
}

fn get_queryable_impl(struct_name: &Ident, return_style: ReturnStyle) -> syn::Result<TokenStream> {
    let ret_type = match return_style {
        ReturnStyle::Array => quote! { Vec<#struct_name> },
        ReturnStyle::Object => quote! { #struct_name },
    };

    Ok(quote! {
        impl crate::worldstate::models::base::Queryable for #struct_name {
            type Return = #ret_type;
        }
    })
}

impl QueryableImpl {
    pub fn try_from_queryable_params(
        struct_name: &Ident,
        params: &QueryableParams,
    ) -> syn::Result<Self> {
        Ok(Self {
            impl_endpoint: Some(get_endpoint_impl(struct_name, &params.endpoint)?),
            impl_queryable: Some(get_queryable_impl(struct_name, params.return_style)?),
        })
    }
}

pub struct Args {
    pub queryable_params: Option<QueryableParams>,
    pub timed: syn::LitBool,
    pub expiry_attrs: Option<Vec<Meta>>,
    pub activation_attrs: Option<Vec<Meta>>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut endpoint = None::<LitStr>;
        let mut return_style = None::<ReturnStyle>;
        let mut timed = LitBool::new(false, Span::call_site());
        let mut expiry_attrs = None::<Vec<Meta>>;
        let mut activation_attrs = None::<Vec<Meta>>;

        let model_parser = syn::meta::parser(|meta| match &meta.path {
            path if path.is_ident("endpoint") => {
                let lit: LitStr = meta.value()?.parse()?;
                if !lit.value().starts_with('/') {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "endpoint must start with a `/`",
                    ));
                }
                endpoint = Some(lit);
                Ok(())
            }
            path if path.is_ident("return_style") => {
                return_style = Some(meta.value()?.parse()?);
                Ok(())
            }
            path if path.is_ident("timed") => {
                if let Ok(val) = meta.value() {
                    timed = val.parse()?;
                } else {
                    timed = LitBool::new(true, path.span());
                }
                Ok(())
            }

            path if path.is_ident("expiry") => {
                let content;
                parenthesized!(content in *meta.input);

                expiry_attrs = Some(
                    Punctuated::<Meta, Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .collect(),
                );

                Ok(())
            }

            path if path.is_ident("activation") => {
                let content;
                parenthesized!(content in *meta.input);

                activation_attrs = Some(
                    Punctuated::<Meta, Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .collect(),
                );

                Ok(())
            }

            path => Err(syn::Error::new_spanned(
                path,
                format!("unexpected key: {}", path.get_ident().unwrap()),
            )),
        });

        Parser::parse2(model_parser, input.parse()?)?;

        let queryable_params = match (endpoint, return_style) {
            (Some(endpoint), Some(return_style)) => Some(QueryableParams {
                endpoint,
                return_style,
            }),
            (None, None) => None,
            (_, _) => {
                return Err(Error::new(
                    Span::call_site(),
                    "`endpoint` and `return_style` can only be used together",
                ));
            }
        };

        if !timed.value && (expiry_attrs.is_some() || activation_attrs.is_some()) {
            return Err(Error::new(
                Span::call_site(),
                "`expiry` and `activation` can only be used when `timed` is set to true.",
            ));
        }

        Ok(Self {
            queryable_params,
            timed,
            expiry_attrs,
            activation_attrs,
        })
    }
}
