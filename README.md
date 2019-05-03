Compiler version cfg
====================

[![Build Status](https://api.travis-ci.com/dtolnay/select-rustc.svg?branch=master)](https://travis-ci.com/dtolnay/select-rustc)
[![Latest Version](https://img.shields.io/crates/v/select-rustc.svg)](https://crates.io/crates/select-rustc)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/select-rustc)

This crate provides macros for conditional compilation according to rustc
compiler version, analogous to [`#[cfg(...)]`][cfg] and
[`#[cfg_attr(...)]`][cfg_attr].

[cfg]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
[cfg_attr]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute

```toml
[dependencies]
select-rustc = "0.1"
```

<br>

## Selectors

- <b>`#[rustc::stable]`</b>
  —<br>
  True on any stable compiler.

- <b>`#[rustc::stable(1.34)]`</b>
  —<br>
  True on exactly the specified stable compiler.

- <b>`#[rustc::beta]`</b>
  —<br>
  True on any beta compiler.

- <b>`#[rustc::nightly]`</b>
  —<br>
  True on any nightly compiler or dev build.

- <b>`#[rustc::nightly(2019-01-01)]`</b>
  —<br>
  True on exactly one nightly.

- <b>`#[rustc::since(1.34)]`</b>
  —<br>
  True on that stable release and any later compiler, including beta and
  nightly.

- <b>`#[rustc::since(2019-01-01)]`</b>
  —<br>
  True on that nightly and all newer ones.

- <b>`#[rustc::before(`</b><i>version or date</i><b>`)]`</b>
  —<br>
  Negative of *#[rustc::since(...)]*.

- <b>`#[rustc::not(`</b><i>selector</i><b>`)]`</b>
  —<br>
  Negative of any selector; for example *#[rustc::not(nightly)]*.

- <b>`#[rustc::any(`</b><i>selectors...</i><b>`)]`</b>
  —<br>
  True if any of the comma-separated selectors is true; for example
  *#[rustc::any(stable, beta)]*.

- <b>`#[rustc::all(`</b><i>selectors...</i><b>`)]`</b>
  —<br>
  True if all of the comma-separated selectors are true; for example
  *#[rustc::all(since(1.31), before(1.34))]*.

- <b>`#[rustc::attr(`</b><i>selector</i><b>`, `</b><i>attribute</i><b>`)]`</b>
  —<br>
  For conditional inclusion of attributes; analogous to `cfg_attr`.

<br>

## Use cases

Providing additional trait impls as types are stabilized in the standard library
without breaking compatibility with older compilers; in this case Pin\<P\>
stabilized in [Rust 1.33][pin]:

[pin]: https://blog.rust-lang.org/2019/02/28/Rust-1.33.0.html#pinning

```rust
#[rustc::since(1.33)]
use std::pin::Pin;

#[rustc::since(1.33)]
impl<P: MyTrait> MyTrait for Pin<P> {
    /* ... */
}
```

Similar but for language features; the ability to control alignment greater than
1 of packed structs was stabilized in [Rust 1.33][packed].

[packed]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1330-2019-02-28

```rust
#[rustc::attr(before(1.33), repr(packed))]
#[rustc::attr(since(1.33), repr(packed(2)))]
struct Six(i16, i32);

fn main() {
    println!("{}", std::mem::align_of::<Six>());
}
```

Augmenting code with `const` as const impls are stabilized in the standard
library. This use of `const` as an attribute is recognized as a special case by
the rustc::attr macro.

```rust
use std::time::Duration;

#[rustc::attr(since(1.32), const)]
fn duration_as_days(dur: Duration) -> u64 {
    dur.as_secs() / 60 / 60 / 24
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
