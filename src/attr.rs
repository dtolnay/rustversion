use crate::expr::Expr;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;

pub struct Args {
    pub condition: Expr,
    pub then: Then,
}

pub enum Then {
    Const(Token![const]),
    Attribute(TokenStream),
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let condition: Expr = input.parse()?;

        input.parse::<Token![,]>()?;
        if input.is_empty() {
            return Err(input.error("expected one or more attrs"));
        }

        let const_token: Option<Token![const]> = input.parse()?;
        let then = if let Some(const_token) = const_token {
            input.parse::<Option<Token![,]>>()?;
            Then::Const(const_token)
        } else {
            input.parse().map(Then::Attribute)?
        };

        Ok(Args { condition, then })
    }
}
