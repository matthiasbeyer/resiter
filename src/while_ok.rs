//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to iter until an error is encountered.
pub trait WhileOk<O, E> {
    /// Perform an on each `Ok` value. Stop on first `Err`
    ///
    /// ```
    /// use resiter::while_ok::WhileOk;
    /// use std::str::FromStr;
    ///
    /// let mut s = 0;
    ///
    /// let res = ["1", "2", "3", "4", "5"]
    ///     .iter()
    ///     .map(|txt| usize::from_str(txt))
    ///     .while_ok(|i| s += i);
    ///
    /// assert_eq!(s, 15);
    /// assert!(res.is_ok());
    /// ```
    /// When every value is `Ok` stop the iteration at the end of the iterator:
    /// ```
    /// use resiter::while_ok::WhileOk;
    /// use std::str::FromStr;
    ///
    /// let mut s = 0;
    ///
    /// let res = ["1", "2", "a", "4", "5"]
    ///     .iter()
    ///     .map(|txt| usize::from_str(txt))
    ///     .while_ok(|i| s += i);
    ///
    /// assert_eq!(s, 3);
    /// assert!(res.is_err());
    /// ```
    fn while_ok<F>(self, _: F) -> Result<(), E>
    where
        F: FnMut(O);
}

impl<I, O, E> WhileOk<O, E> for I
where
    I: Iterator<Item = Result<O, E>>,
{
    #[inline]
    fn while_ok<F>(self, mut f: F) -> Result<(), E>
    where
        F: FnMut(O),
    {
        for res in self {
            f(res?);
        }
        Ok(())
    }
}
