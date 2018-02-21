//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use util::*;

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Errors<T>(Box<Iterator<Item = T>>);

/// Extension trait for `Iterator<Item = Result<T, E>>` to get all `E`s
pub trait GetErrors<T> {
    fn errors(self) -> Errors<T>;
}

impl<T, U, I> GetErrors<T> for I
    where I: Iterator<Item = Result<U, T>> + 'static,
          T: 'static,
          U: 'static
{
    fn errors(self) -> Errors<T> {
        let bx : Box<Iterator<Item = T>> = Box::new(self.filter_map(GetErr::get_err));
        Errors(bx)
    }
}

impl<T> Iterator for Errors<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<T> Errors<T> {

    /// Process all errors with a lambda
    pub fn process<R: Default, E, F>(self, f: F) -> Result<R, E>
        where F: Fn(T) -> Result<R, E>
    {
        for element in self {
            let _ = f(element)?;
        }
        Ok(R::default())
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

