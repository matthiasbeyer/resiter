//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct OnOk<I, O, E, F>(I, F)
where
    I: Iterator<Item = Result<O, E>>,
    F: Fn(&O) -> ();

/// Extension trait for `Iterator<Item = Result<T, E>>` to do something on `Ok(_)`
pub trait OnOkDo<I, O, E, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: Fn(&O) -> (),
{
    fn on_ok(self, _: F) -> OnOk<I, O, E, F>;
}

impl<I, O, E, F> OnOkDo<I, O, E, F> for I
where
    I: Iterator<Item = Result<O, E>>,
    F: Fn(&O) -> (),
{
    fn on_ok(self, f: F) -> OnOk<I, O, E, F> {
        OnOk(self, f)
    }
}

impl<I, O, E, F> Iterator for OnOk<I, O, E, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: Fn(&O) -> (),
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| {
            r.map(|o| {
                (self.1)(&o);
                o
            })
        })
    }
}

#[test]
fn test_compile_1() {
    use std::str::FromStr;

    let _: Vec<Result<usize, ::std::num::ParseIntError>> = ["1", "2", "3", "4", "5"]
        .iter()
        .map(|e| usize::from_str(e))
        .on_ok(|e| println!("Ok: {:?}", e))
        .collect();
}
