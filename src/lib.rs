//! This crate provides macros for conditional compilation according to rustc
//! compiler version, analogous to [`#[cfg(...)]`][cfg] and
//! [`#[cfg_attr(...)]`][cfg_attr].
//!
//! [cfg]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
//! [cfg_attr]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
//!
//! <br>
//!
//! # Selectors
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::stable]</code></b>
//!   —<br>
//!   True on any stable compiler.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::stable(1.34)]</code></b>
//!   —<br>
//!   True on exactly the specified stable compiler.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::beta]</code></b>
//!   —<br>
//!   True on any beta compiler.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::nightly]</code></b>
//!   —<br>
//!   True on any nightly compiler or dev build.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::nightly(2019-01-01)]</code></b>
//!   —<br>
//!   True on exactly one nightly.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::since(1.34)]</code></b>
//!   —<br>
//!   True on that stable release and any later compiler, including beta and
//!   nightly.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::since(2019-01-01)]</code></b>
//!   —<br>
//!   True on that nightly and all newer ones.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::before(</code></b><i>version or date</i><b><code>)]</code></b>
//!   —<br>
//!   Negative of <i>#[rustc::since(...)]</i>.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::not(</code></b><i>selector</i><b><code>)]</code></b>
//!   —<br>
//!   Negative of any selector; for example <i>#[rustc::not(nightly)]</i>.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::any(</code></b><i>selectors...</i><b><code>)]</code></b>
//!   —<br>
//!   True if any of the comma-separated selectors is true; for example
//!   <i>#[rustc::any(stable, beta)]</i>.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::all(</code></b><i>selectors...</i><b><code>)]</code></b>
//!   —<br>
//!   True if all of the comma-separated selectors are true; for example
//!   <i>#[rustc::all(since(1.31), before(1.34))]</i>.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustc::attr(</code></b><i>selector</i><b><code>, </code></b><i>attribute</i><b><code>)]</code></b>
//!   —<br>
//!   For conditional inclusion of attributes; analogous to
//!   <code>cfg_attr</code>.
//!   </p>
//!
//! <br>
//!
//! # Use cases
//!
//! Providing additional trait impls as types are stabilized in the standard
//! library without breaking compatibility with older compilers; in this case
//! u128 stabilized in [Rust 1.26][u128]:
//!
//! [u128]: https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#128-bit-integers
//!
//! ```
//! # trait MyTrait {}
//! #
//! #[rustc::since(1.26)]
//! impl MyTrait for u128 {
//!     /* ... */
//! }
//! ```
//!
//! Similar but for language features; the #\[must_use\] attribute stabilized in
//! [Rust 1.27][must_use]:
//!
//! [must_use]: https://blog.rust-lang.org/2018/06/21/Rust-1.27.html#[must_use]-on-functions
//!
//! ```
//! # type MyReturn = ();
//! #
//! #[rustc::attr(since(1.27), must_use)]
//! fn f() -> MyReturn {
//!     /* ... */
//! }
//! ```
//!
//! <br>

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
