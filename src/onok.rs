//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct OnOk<I, O, E, F>(I, F)
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(&O);

/// Extension trait for `Iterator<Item = Result<T, E>>` to do something on `Ok(_)`
pub trait OnOkDo<I, O, E, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(&O),
{
    /// Perform a side effect on each Ok value
    ///
    /// ```
    /// use resiter::onok::OnOkDo;
    /// use std::str::FromStr;
    ///
    /// let mut oks = Vec::new();
    /// let _: Vec<Result<usize, ::std::num::ParseIntError>> = ["1", "2", "a", "b", "5"]
    ///     .iter()
    ///     .map(|e| usize::from_str(e))
    ///     .on_ok(|e| oks.push(e.to_owned()))
    ///     .collect();
    ///
    /// assert_eq!(oks, vec![1, 2, 5]);
    /// ```
    fn on_ok(self, _: F) -> OnOk<I, O, E, F>;
}

impl<I, O, E, F> OnOkDo<I, O, E, F> for I
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(&O),
{
    #[inline]
    fn on_ok(self, f: F) -> OnOk<I, O, E, F> {
        OnOk(self, f)
    }
}

impl<I, O, E, F> Iterator for OnOk<I, O, E, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(&O),
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
