//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform and map Oks and Errors.
pub trait TryFilterMap<O, E>: Sized {
    /// Equivalent to [Iterator::filter_map] on all `Ok` values.
    /// The filter function can fail with a result and turn an
    /// [Result::Ok] into a [Result::Err]
    ///
    /// ```
    /// use std::str::FromStr;
    /// use resiter::try_filter_map::TryFilterMap;
    ///
    /// let filter_mapped: Vec<_> = vec![
    ///     Ok("1"),
    ///     Err("2".to_owned()),
    ///     Ok("a"), // will become an error
    ///     Err("4".to_owned()),
    ///     Ok("5"), // will be filtered out
    ///     Err("b".to_owned()),
    ///     Err("8".to_owned()),
    /// ]
    /// .into_iter()
    /// .try_filter_map_ok(|txt| {
    ///     match usize::from_str(txt).map_err(|e| e.to_string()) {
    ///         Err(e) => Some(Err(e)),
    ///         Ok(u) => {
    ///             if u < 3 {
    ///                 Some(Ok(u))
    ///             } else {
    ///                 None
    ///             }
    ///         }
    ///     }
    /// })
    /// .collect();
    ///
    /// assert_eq!(
    ///     filter_mapped,
    ///     [
    ///         Ok(1),
    ///         Err("2".to_owned()),
    ///         Err("invalid digit found in string".to_owned()),
    ///         Err("4".to_owned()),
    ///         Err("b".to_owned()),
    ///         Err("8".to_owned())
    ///     ]
    /// );
    /// ```
    fn try_filter_map_ok<F, O2>(self, _: F) -> TryFilterMapOk<Self, F>
    where
        F: FnMut(O) -> Option<Result<O2, E>>;

    /// Equivalent to [Iterator::filter_map] on all `Err` values.
    /// The filter function can fail with a result and turn a
    /// [Result::Err] into a [Result::Ok]
    ///
    /// ```
    /// use std::str::FromStr;
    /// use resiter::try_filter_map::TryFilterMap;
    ///
    /// let filter_mapped: Vec<_> = vec![
    ///     Ok("1".to_owned()),
    ///     Err("2".to_owned()), // will become ok
    ///     Ok("a".to_owned()),
    ///     Err("4".to_owned()), // will be removed
    ///     Ok("5".to_owned()),
    ///     Err("b".to_owned()), // will be an error
    ///     Err("8".to_owned()), // will be removed
    /// ]
    /// .into_iter()
    /// .try_filter_map_err(|txt| {
    ///     match usize::from_str(&txt).map_err(|e| e.to_string()) {
    ///         Err(e) => Some(Err(e)),
    ///         Ok(u) => {
    ///             if u < 3 {
    ///                 Some(Ok(u.to_string()))
    ///             } else {
    ///                 None
    ///             }
    ///         }
    ///     }
    /// })
    /// .collect();
    ///
    /// assert_eq!(
    ///     filter_mapped,
    ///     [
    ///         Ok("1".to_owned()),
    ///         Ok("2".to_owned()),
    ///         Ok("a".to_owned()),
    ///         Ok("5".to_owned()),
    ///         Err("invalid digit found in string".to_owned()),
    ///     ]
    /// );
    /// ```
    fn try_filter_map_err<F>(self, _: F) -> TryFilterMapErr<Self, F>
    where
        F: FnMut(E) -> Option<Result<O, E>>;
}

impl<I, O, E> TryFilterMap<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    fn try_filter_map_ok<F, O2>(self, f: F) -> TryFilterMapOk<Self, F>
    where
        F: FnMut(O) -> Option<Result<O2, E>>,
    {
        TryFilterMapOk { iter: self, f }
    }

    fn try_filter_map_err<F>(self, f: F) -> TryFilterMapErr<Self, F>
    where
        F: FnMut(E) -> Option<Result<O, E>>,
    {
        TryFilterMapErr { iter: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct TryFilterMapOk<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F, O2> Iterator for TryFilterMapOk<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(O) -> Option<Result<O2, E>>,
{
    type Item = Result<O2, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.iter.next() {
                Some(Ok(x)) => match (self.f)(x) {
                    Some(r) => Some(r),
                    None => continue,
                },
                Some(Err(e)) => Some(Err(e)),
                None => None,
            };
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct TryFilterMapErr<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F> Iterator for TryFilterMapErr<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> Option<Result<O, E>>,
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.iter.next() {
                Some(Err(x)) => match (self.f)(x) {
                    Some(r) => Some(r),
                    None => continue,
                },
                v => v,
            };
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
