//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#[cfg(not(test))]
use core::iter::*;
#[cfg(test)]
use std::iter::*;

use util::*;

pub use util::Process as Errors;
// for backward compatibility with previous implementation

/// Extension trait for `Iterator<Item = Result<T, E>>` to get all `E`s
#[allow(clippy::type_complexity)]
pub trait GetErrors<T, E>: Sized {
    /// Get all errors from this `Iterator`
    ///
    /// ```
    /// use std::str::FromStr;
    /// use resiter::GetErrors;
    ///
    /// let res: Vec<std::num::ParseIntError> = ["1", "2", "a", "4", "b"]
    ///     .iter()
    ///     .map(|e| usize::from_str(e))
    ///     .errors()
    ///     .collect();
    ///
    /// assert_eq!(res.len(), 2);
    /// ```
    fn errors(self) -> FilterMap<Self, fn(Result<T, E>) -> Option<E>>;
}

impl<T, E, I> GetErrors<T, E> for I
where
    I: Iterator<Item = Result<T, E>> + Sized,
{
    #[allow(clippy::type_complexity)]
    fn errors(self) -> FilterMap<Self, fn(Result<T, E>) -> Option<E>> {
        self.filter_map(GetErr::get_err)
    }
}
