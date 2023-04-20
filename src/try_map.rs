//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform and map Oks and Errors.
pub trait TryMap<O, E>: Sized {
    /// Equivalent to [Iterator::map] on all `Ok` values.
    /// The map function can fail with a result and turn a
    /// [Result::Ok] into a [Result::Err]
    ///
    /// ```
    /// use std::str::FromStr;
    /// use resiter::try_map::TryMap;
    ///
    /// let mapped: Vec<_> = vec![
    ///     Ok("1"),
    ///     Err("2".to_owned()),
    ///     Ok("a"), // will become an error
    ///     Err("4".to_owned()),
    ///     Ok("5"), // will be too high
    ///     Err("b".to_owned()),
    ///     Err("8".to_owned()),
    /// ]
    /// .into_iter()
    /// .try_map_ok(|txt| {
    ///     let n = usize::from_str(txt).map_err(|e| e.to_string())?;
    ///     if n < 3 {
    ///        Ok(n)
    ///     }
    ///     else {
    ///        Err("Too high".to_string())
    ///     }
    /// })
    /// .collect();
    ///
    /// assert_eq!(
    ///     mapped,
    ///     [
    ///         Ok(1),
    ///         Err("2".to_owned()),
    ///         Err("invalid digit found in string".to_owned()),
    ///         Err("4".to_owned()),
    ///         Err("Too high".to_owned()),
    ///         Err("b".to_owned()),
    ///         Err("8".to_owned())
    ///     ]
    /// );
    /// ```
    fn try_map_ok<F, O2>(self, _: F) -> TryMapOk<Self, F>
    where
        F: FnMut(O) -> Result<O2, E>;

    /// Equivalent to [Iterator::map] on all `Err` values.
    /// The map function can fail with a result and turn a
    /// [Result::Err] into a [Result::Ok] or another [Result::Err]
    /// possibly changing it's error type
    ///
    /// ```
    /// use std::str::FromStr;
    /// use resiter::try_map::TryMap;
    ///
    /// let mapped: Vec<_> = vec![
    ///     Ok(1),
    ///     Err("2".to_owned()), // will become ok
    ///     Ok(3),
    ///     Err("4".to_owned()), // will be "Too high"
    ///     Ok(5),
    ///     Err("b".to_owned()), // will be an error
    ///     Err("8".to_owned()), // will be "Too high"
    /// ]
    /// .into_iter()
    /// .try_map_err(|txt| {
    ///     let n = usize::from_str(&txt).map_err(|e| e.to_string())?;
    ///     if n < 4 {
    ///         Ok(n)
    ///     }
    ///     else {
    ///         Err("Too high".to_owned())
    ///     }
    /// })
    /// .collect();
    ///
    /// assert_eq!(
    ///     mapped,
    ///     [
    ///         Ok(1),
    ///         Ok(2),
    ///         Ok(3),
    ///         Err("Too high".to_owned()),
    ///         Ok(5),
    ///         Err("invalid digit found in string".to_owned()),
    ///         Err("Too high".to_owned()),
    ///     ]
    /// );
    /// ```
    fn try_map_err<F, E2>(self, _: F) -> TryMapErr<Self, F>
    where
        F: FnMut(E) -> Result<O, E2>;
}

impl<I, O, E> TryMap<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    #[inline]
    fn try_map_ok<F, O2>(self, f: F) -> TryMapOk<Self, F>
    where
        F: FnMut(O) -> Result<O2, E>,
    {
        TryMapOk { iter: self, f }
    }

    #[inline]
    fn try_map_err<F, E2>(self, f: F) -> TryMapErr<Self, F>
    where
        F: FnMut(E) -> Result<O, E2>,
    {
        TryMapErr { iter: self, f }
    }
}

pub struct TryMapOk<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F, O2> Iterator for TryMapOk<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(O) -> Result<O2, E>,
{
    type Item = Result<O2, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(x)) => Some((self.f)(x)),
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
pub struct TryMapErr<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, E2, F> Iterator for TryMapErr<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> Result<O, E2>,
{
    type Item = Result<O, E2>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Err(x)) => Some((self.f)(x)),
            Some(Ok(x)) => Some(Ok(x)),
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
