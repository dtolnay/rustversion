use crate::expr::Expr;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;

pub struct Args {
    pub condition: Expr,
    pub then: TokenStream,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let condition: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        if input.is_empty() {
            return Err(input.error("expected one or more attrs"));
        }
        let then: TokenStream = input.parse()?;
        Ok(Args { condition, then })
    }
}
