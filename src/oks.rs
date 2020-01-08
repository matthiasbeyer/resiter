//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use core::iter::*;

use util::*;

pub use util::Process as Oks;
// for backward compatibility with previous implementation

/// Extension trait for `Iterator<Item = Result<T, E>>` to get all `T`s
pub trait GetOks<T, E>: Sized {
    fn oks(self) -> FilterMap<Self, fn(Result<T, E>) -> Option<T>>;
}

impl<T, E, I> GetOks<T, E> for I
where
    I: Iterator<Item = Result<T, E>> + Sized,
{
    fn oks(self) -> FilterMap<Self, fn(Result<T, E>) -> Option<T>> {
        self.filter_map(GetOk::get_ok)
    }
}

#[test]
fn test_compile() {
    use std::str::FromStr;

    let _: Result<_, ::std::num::ParseIntError> = ["1", "2", "3", "4", "5"]
        .into_iter()
        .map(|e| usize::from_str(e))
        .oks()
        .process(|o| {
            println!("Ok: {:?}", o);
            Ok(())
        });
}
