extern crate proc_macro;

mod attr;
mod bound;
mod date;
mod expr;
mod rustc;
mod time;
mod version;

use crate::expr::Expr;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Result};

#[proc_macro_attribute]
pub fn stable(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("stable", args, input)
}

#[proc_macro_attribute]
pub fn beta(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("beta", args, input)
}

#[proc_macro_attribute]
pub fn nightly(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("nightly", args, input)
}

#[proc_macro_attribute]
pub fn since(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("since", args, input)
}

#[proc_macro_attribute]
pub fn before(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("before", args, input)
}

#[proc_macro_attribute]
pub fn not(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("not", args, input)
}

#[proc_macro_attribute]
pub fn any(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("any", args, input)
}

#[proc_macro_attribute]
pub fn all(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("all", args, input)
}

fn cfg(top: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    match try_cfg(top, args, input) {
        Ok(tokens) => tokens,
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

fn try_cfg(top: &str, args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let args = TokenStream2::from(args);
    let top = Ident::new(top, Span::call_site());

    let mut full_args = quote!(#top);
    if !args.is_empty() {
        full_args.extend(quote!((#args)));
    }

    let expr: Expr = syn::parse2(full_args)?;
    let version = rustc::version()?;

    if expr.eval(version) {
        Ok(input)
    } else {
        Ok(TokenStream::new())
    }
}

#[proc_macro_attribute]
pub fn attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as attr::Args);

    match try_attr(args, input) {
        Ok(tokens) => tokens,
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

fn try_attr(args: attr::Args, input: TokenStream) -> Result<TokenStream> {
    let version = rustc::version()?;

    if args.condition.eval(version) {
        let then = args.then;
        let input = TokenStream2::from(input);
        Ok(TokenStream::from(quote! {
            #[cfg_attr(all(), #then)]
            #input
        }))
    } else {
        Ok(input)
    }
}
