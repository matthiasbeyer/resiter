//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform Oks and Errors.
pub trait AndThen<O, E>: Sized {
    /// ```
    /// use resiter::and_then::AndThen;
    /// use std::str::FromStr;
    ///
    /// let mapped: Vec<_> = ["1", "2", "a", "b", "4", "5"]
    ///     .iter()
    ///     .map(|txt| usize::from_str(txt).map_err(|e| (txt, e)))
    ///     .and_then_ok(|i| Ok(2 * i))
    ///     .collect();
    ///
    /// assert_eq!(mapped[0], Ok(2));
    /// assert_eq!(mapped[1], Ok(4));
    /// assert!(mapped[2].is_err());
    /// assert!(mapped[3].is_err());
    /// assert_eq!(mapped[4], Ok(8));
    /// assert_eq!(mapped[5], Ok(10));
    /// ```
    fn and_then_ok<F, O2>(self, _: F) -> AndThenOk<Self, F>
    where
        F: FnMut(O) -> Result<O2, E>;

    /// ```
    /// use resiter::and_then::AndThen;
    /// use std::str::FromStr;
    ///
    /// let mapped: Vec<_> = ["1", "2", "a", "b", "4", "5"]
    ///     .iter()
    ///     .map(|txt| usize::from_str(txt).map_err(|e| (txt, e)))
    ///     .and_then_err(|(txt, e)| if txt == &"a" { Ok(15) } else { Err(e) })
    ///     .collect();
    ///
    /// assert_eq!(mapped[0], Ok(1));
    /// assert_eq!(mapped[1], Ok(2));
    /// assert_eq!(mapped[2], Ok(15));
    /// assert!(mapped[3].is_err());
    /// assert_eq!(mapped[4], Ok(4));
    /// assert_eq!(mapped[5], Ok(5));
    /// ```
    fn and_then_err<F, E2>(self, _: F) -> AndThenErr<Self, F>
    where
        F: FnMut(E) -> Result<O, E2>;
}

impl<I, O, E> AndThen<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    fn and_then_ok<F, O2>(self, f: F) -> AndThenOk<Self, F>
    where
        F: FnMut(O) -> Result<O2, E>,
    {
        AndThenOk { iter: self, f }
    }
    fn and_then_err<F, E2>(self, f: F) -> AndThenErr<Self, F>
    where
        F: FnMut(E) -> Result<O, E2>,
    {
        AndThenErr { iter: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct AndThenOk<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, O2, F> Iterator for AndThenOk<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(O) -> Result<O2, E>,
{
    type Item = Result<O2, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(o)) => Some((self.f)(o)),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct AndThenErr<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, E2, F> Iterator for AndThenErr<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> Result<O, E2>,
{
    type Item = Result<O, E2>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Err(e)) => Some((self.f)(e)),
            Some(Ok(o)) => Some(Ok(o)),
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
