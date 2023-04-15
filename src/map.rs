//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform Oks and Errors.
pub trait Map<O, E>: Sized {
    /// Map all `Ok` items while leaving `Err` as is
    ///
    /// ```
    /// use resiter::map::Map;
    /// use std::str::FromStr;
    ///
    /// let mapped: Vec<_> = ["1", "2", "a", "4", "5"]
    ///     .iter()
    ///     .map(|txt| usize::from_str(txt))
    ///     .map_ok(|i| 2 * i)
    ///     .collect();
    ///
    /// assert_eq!(mapped[0], Ok(2));
    /// assert_eq!(mapped[1], Ok(4));
    /// assert!(mapped[2].is_err());
    /// assert_eq!(mapped[3], Ok(8));
    /// assert_eq!(mapped[4], Ok(10));
    /// ```
    fn map_ok<F, O2>(self, _: F) -> MapOk<Self, F>
    where
        F: FnMut(O) -> O2;

    /// Map all `Err` items while leaving `Ok` as is
    ///
    /// ```
    /// use resiter::map::Map;
    /// use std::str::FromStr;
    /// let mapped: Vec<_> = ["1", "2", "a", "4", "5"]
    ///     .iter()
    ///     .map(|txt| usize::from_str(txt))
    ///     .map_err(|e| format!("{:?}", e))
    ///     .collect();
    ///
    /// assert_eq!(
    ///     mapped,
    ///     vec![
    ///         Ok(1),
    ///         Ok(2),
    ///         Err("ParseIntError { kind: InvalidDigit }".to_string()),
    ///         Ok(4),
    ///         Ok(5),
    ///     ]
    /// );
    /// ```
    fn map_err<F, E2>(self, _: F) -> MapErr<Self, F>
    where
        F: FnMut(E) -> E2;
}

impl<I, O, E> Map<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    fn map_ok<F, O2>(self, f: F) -> MapOk<Self, F>
    where
        F: FnMut(O) -> O2,
    {
        MapOk { iter: self, f }
    }
    fn map_err<F, E2>(self, f: F) -> MapErr<Self, F>
    where
        F: FnMut(E) -> E2,
    {
        MapErr { iter: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct MapOk<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F, O2> Iterator for MapOk<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(O) -> O2,
{
    type Item = Result<O2, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|r| r.map(&mut self.f))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct MapErr<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F, E2> Iterator for MapErr<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> E2,
{
    type Item = Result<O, E2>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|r| r.map_err(&mut self.f))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_ok_hint() {
        use std::str::FromStr;

        let hint = ["1", "2", "a", "4", "5"]
            .iter()
            .map(|txt| usize::from_str(txt))
            .map_ok(|i| 2 * i)
            .size_hint();

        assert_eq!(hint, (5, Some(5)));
    }

    #[test]
    fn test_map_err_hint() {
        use std::str::FromStr;

        let hint = ["1", "2", "a", "4", "5"]
            .iter()
            .map(|txt| usize::from_str(txt))
            .map_err(|e| format!("{:?}", e))
            .size_hint();

        assert_eq!(hint, (5, Some(5)));
    }
}
