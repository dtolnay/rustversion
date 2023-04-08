#![feature(const_trait_impl)]

#[rustversion::attr(all(), const_trait)]
trait _Trait {
    fn _method() {}
}

#[rustversion::attr(all(), const)]
impl _Trait for () {}
const _: () = <() as _Trait>::_method();

fn main() {}
