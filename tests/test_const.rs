#[rustversion::attr(all(), const)]
fn _basic() {}
const _BASIC: () = _basic();

#[rustversion::attr(all(), const)]
unsafe fn _unsafe() {}
const _UNSAFE: () = unsafe { _unsafe() };

macro_rules! item {
    ($i:item) => {
        #[rustversion::attr(all(), const)]
        $i
    };
}

item!(fn _item() {});
const _ITEM: () = _item();

macro_rules! ident {
    ($fn:ident) => {
        #[rustversion::attr(all(), const)]
        $fn _ident() {}
    };
}

ident!(fn);
const _IDENT: () = _ident();
