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

pub use util::Process as Oks;
// for backward compatibility with previous implementation

/// Extension trait for `Iterator<Item = Result<T, E>>` to get all `T`s
#[allow(clippy::type_complexity)]
pub trait GetOks<T, E>: Sized {
    /// Iterate over every `Ok` while ignoring every `Err`
    ///
    /// ```
    /// use std::str::FromStr;
    /// use resiter::oks::GetOks;
    ///
    /// let res:Vec<usize> = ["1", "2", "3", "a", "4", "5"]
    ///     .iter()
    ///     .map(|e| usize::from_str(e))
    ///     .oks()
    ///     .collect();
    ///
    /// assert_eq!(
    ///     res,
    ///     vec![1,2,3,4,5]
    /// );
    /// ```
    fn oks(self) -> FilterMap<Self, fn(Result<T, E>) -> Option<T>>;
}

impl<T, E, I> GetOks<T, E> for I
where
    I: Iterator<Item = Result<T, E>> + Sized,
{
    #[inline]
    #[allow(clippy::type_complexity)]
    fn oks(self) -> FilterMap<Self, fn(Result<T, E>) -> Option<T>> {
        self.filter_map(GetOk::get_ok)
    }
}
