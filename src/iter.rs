use proc_macro::{token_stream, Delimiter, TokenStream, TokenTree};

pub struct IterImpl {
    stack: Vec<token_stream::IntoIter>,
}

pub fn new(tokens: TokenStream) -> IterImpl {
    IterImpl {
        stack: vec![tokens.into_iter()],
    }
}

impl Iterator for IterImpl {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let top = self.stack.last_mut()?;
            match top.next() {
                None => drop(self.stack.pop()),
                Some(TokenTree::Group(ref group)) if group.delimiter() == Delimiter::None => {
                    self.stack.push(group.stream().into_iter());
                }
                Some(tt) => return Some(tt),
            }
        }
    }
}
