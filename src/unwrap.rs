//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<T, E>>` to unwrap everything.
///
/// Errors can be unwraped as well. If the closure `F` returns `Some(O)`, that `O` will be inserted
/// instead of the `E` into the resulting iterator.
/// If the closure returns `None`, the error will be dropped (equally to
/// `iter.filter_map(Result::ok)`.
///
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct UnwrapWith<I, O, E, F>(I, F)
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> Option<O>;

impl<I, O, E, F> Iterator for UnwrapWith<I, O, E, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> Option<O>,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(o) = self.0.by_ref().next() {
            match o {
                Ok(t) => return Some(t),
                Err(e) => {
                    if let Some(t) = (self.1)(e) {
                        return Some(t);
                    }
                }
            }
        }

        None
    }
}

pub trait UnwrapWithExt<I, O, E, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> Option<O>,
{
    fn unwrap_with(self, _: F) -> UnwrapWith<I, O, E, F>;
}

impl<I, O, E, F> UnwrapWithExt<I, O, E, F> for I
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> Option<O>,
{
    fn unwrap_with(self, f: F) -> UnwrapWith<I, O, E, F> {
        UnwrapWith(self, f)
    }
}

#[test]
fn test_compile_1() {
    use std::str::FromStr;

    let _: Vec<usize> = ["1", "2", "3", "4", "5"]
        .iter()
        .map(|e| usize::from_str(e))
        .unwrap_with(|_| None) // ignore errors
        .collect();
}
