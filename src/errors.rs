//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::iter::*;

use util::*;

pub use util::Process as Errors;
// for backward compatibility with previous implementation

/// Extension trait for `Iterator<Item = Result<T, E>>` to get all `E`s
pub trait GetErrors<T, E> : Sized {
    fn errors(self) -> FilterMap<Self, fn(Result<T,E>) -> Option<E>>;
}

impl<T, E, I> GetErrors<T, E> for I
    where I: Iterator<Item = Result<T, E>> + Sized
{
    fn errors(self) -> FilterMap<Self, fn(Result<T,E>) -> Option<E>> {
        self.filter_map(GetErr::get_err)
    }
}

#[test]
fn test_compile() {
    use std::str::FromStr;

    let _ : Result<_, ::std::num::ParseIntError> = ["1", "2", "3", "4", "5"]
        .into_iter()
        .map(|e| usize::from_str(e))
        .errors()
        .process(|e| { println!("Error: {:?}", e); Ok(()) });
}

