//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension for `Iterator<Item = Result<O, E>>` to filter the Ok(_) and leaving the Err(_) as
/// is, but allowing the filter to return a `Result<bool, E>` itself
pub trait TryFilter<O, E>: Sized {
    /// Filters every `Ok`-value with a function that can return an Err.
    /// Useful when the filter condition uses functions that can fail.
    ///
    ///```
    /// use resiter::try_filter::TryFilter;
    /// use std::str::FromStr;
    ///
    /// let v = ["1", "2", "4", "a", "5"]
    ///     .iter()
    ///     .map(Ok)
    ///     .try_filter_ok(|e| usize::from_str(e).map(|n| n < 3))
    ///     .collect::<Vec<Result<_, _>>>();
    ///
    /// assert_eq!(v.len(), 3);
    /// assert_eq!(v.iter().filter(|x| x.is_ok()).count(), 2);
    /// assert_eq!(v.iter().filter(|x| x.is_err()).count(), 1);
    ///```
    fn try_filter_ok<F>(self, _: F) -> TryFilterOk<Self, F>
    where
        F: FnMut(&O) -> Result<bool, E>;

    /// Filters every `Err`-value with a function that can return an Err.
    /// Useful when the filter condition uses functions that can fail.
    ///
    /// ```
    /// use resiter::try_filter::TryFilter;
    /// use std::num::ParseIntError;
    /// use std::str::FromStr;
    ///
    /// let v = ["1", "2", "4", "a", "5"]
    ///     .iter()
    ///     .map(|txt| usize::from_str(txt))
    ///     .try_filter_err(|_:&ParseIntError| Ok(false))
    ///     .collect::<Vec<Result<_, _>>>();
    ///
    /// assert_eq!(v.iter().filter(|x| x.is_ok()).count(), 4);
    /// assert_eq!(v.iter().filter(|x| x.is_err()).count(), 0);
    /// ```
    fn try_filter_err<F>(self, _: F) -> TryFilterErr<Self, F>
    where
        F: FnMut(&E) -> Result<bool, E>;
}

impl<I, O, E> TryFilter<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    #[inline]
    fn try_filter_ok<F>(self, f: F) -> TryFilterOk<Self, F>
    where
        F: FnMut(&O) -> Result<bool, E>,
    {
        TryFilterOk { iter: self, f }
    }

    #[inline]
    fn try_filter_err<F>(self, f: F) -> TryFilterErr<Self, F>
    where
        F: FnMut(&E) -> Result<bool, E>,
    {
        TryFilterErr { iter: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct TryFilterOk<I, F> {
    iter: I,
    f: F,
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct TryFilterErr<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F> Iterator for TryFilterOk<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(&O) -> Result<bool, E>,
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.iter.next() {
                Some(Ok(x)) => match (self.f)(&x) {
                    Ok(true) => Some(Ok(x)),
                    Ok(false) => continue,
                    Err(e) => Some(Err(e)),
                },
                other => other,
            };
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint_sup = self.iter.size_hint().1;
        (0, hint_sup)
    }
}

impl<I, O, E, F> Iterator for TryFilterErr<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(&E) -> Result<bool, E>,
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.iter.next() {
                Some(Err(x)) => match (self.f)(&x) {
                    Ok(true) => Some(Err(x)),
                    Ok(false) => continue,
                    Err(e) => Some(Err(e)),
                },
                other => other,
            };
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint_sup = self.iter.size_hint().1;
        (0, hint_sup)
    }
}
