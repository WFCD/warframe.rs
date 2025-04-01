use syn::{
    Expr,
    parse::{
        self,
        Parse,
    },
};

#[derive(Debug, Clone, Copy)]
pub enum ReturnStyle {
    Object,
    Array,
}

impl Parse for ReturnStyle {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<Expr>()?;
        let path = match path {
            Expr::Path(path) => path,
            _ => return Err(syn::Error::new_spanned(path, "expected path")),
        };

        let ident = path
            .path
            .get_ident()
            .ok_or_else(|| syn::Error::new_spanned(path.clone(), "expected ident"))?;

        match ident.to_string().as_str() {
            "Object" => Ok(Self::Object),
            "Array" => Ok(Self::Array),
            _ => Err(syn::Error::new_spanned(
                ident,
                "expected `Object` or `Array`",
            )),
        }
    }
}
