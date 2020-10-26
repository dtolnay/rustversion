use proc_macro::{Delimiter, Ident, TokenStream, TokenTree};
use std::iter;
use syn::{Error, Result, Token};

#[derive(PartialOrd, PartialEq)]
enum Qualifiers {
    None,
    Async,
    Unsafe,
    Extern,
    Abi,
}

impl Qualifiers {
    fn from_ident(ident: &Ident) -> Self {
        match ident.to_string().as_str() {
            "async" => Qualifiers::Async,
            "unsafe" => Qualifiers::Unsafe,
            "extern" => Qualifiers::Extern,
            _ => Qualifiers::None,
        }
    }
}

pub(crate) fn insert_const(input: TokenStream, const_token: Token![const]) -> Result<TokenStream> {
    let mut out = TokenStream::new();
    let mut stack = vec![input.into_iter()];
    let mut qualifiers = Qualifiers::None;
    let mut pending = Vec::new();

    'outer: while let Some(iter) = stack.last_mut() {
        while let Some(token) = iter.next() {
            match token {
                TokenTree::Group(ref group) if group.delimiter() == Delimiter::None => {
                    stack.push(group.stream().into_iter());
                    continue 'outer;
                }
                TokenTree::Ident(ref ident) if ident.to_string() == "fn" => {
                    let const_ident = Ident::new("const", const_token.span.unwrap());
                    out.extend(iter::once(TokenTree::Ident(const_ident)));
                    out.extend(pending);
                    out.extend(iter::once(token));
                    out.extend(stack.into_iter().rev().flatten());
                    return Ok(out);
                }
                TokenTree::Ident(ref ident) if Qualifiers::from_ident(ident) > qualifiers => {
                    qualifiers = Qualifiers::from_ident(ident);
                    pending.push(token);
                }
                TokenTree::Literal(_) if qualifiers == Qualifiers::Extern => {
                    qualifiers = Qualifiers::Abi;
                    pending.push(token);
                }
                _ => {
                    qualifiers = Qualifiers::None;
                    out.extend(pending.drain(..));
                    out.extend(iter::once(token));
                }
            }
        }
        stack.pop();
    }

    Err(Error::new(const_token.span, "only allowed on a fn item"))
}
