//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct OnErr<I, O, E, F>(I, F)
    where I: Iterator<Item = Result<O, E>>,
          F: Fn(&E) -> ();

/// Extension trait for `Iterator<Item = Result<T, E>>` to do something on `Err(_)`
pub trait OnErrDo<I, O, E, F>
    where I: Iterator<Item = Result<O, E>>,
          F: Fn(&E) -> ()
{
    fn on_err(self, F) -> OnErr<I, O, E, F>;
}

impl<I, O, E, F> OnErrDo<I, O, E, F> for I
    where I: Iterator<Item = Result<O, E>>,
          F: Fn(&E) -> ()
{
    fn on_err(self, f: F) -> OnErr<I, O, E, F> {
        OnErr(self, f)
    }
}

impl<I, O, E, F> Iterator for OnErr<I, O, E, F>
    where I: Iterator<Item = Result<O, E>>,
          F: Fn(&E) -> ()
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| r.map_err(|e| {(self.1)(&e); e }))
    }
}

#[test]
fn test_compile_1() {
    use std::str::FromStr;

    let _ : Vec<Result<usize, ::std::num::ParseIntError>> = ["1", "2", "3", "4", "5"]
        .into_iter()
        .map(|e| usize::from_str(e))
        .on_err(|e| println!("Error: {:?}", e))
        .collect();
}
