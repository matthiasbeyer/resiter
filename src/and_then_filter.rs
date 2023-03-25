//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform and map Oks and Errors.
pub trait AndThenFilter<O, E>: Sized {
    /// Equivalent to [Iterator::filter_map] on all `Ok` values.
    /// The filter function can fail with a result and turn an
    /// [Result::Ok] into a [Result::Err]
    ///
    /// ```
    /// use std::str::FromStr;
    /// use resiter::and_then_filter::AndThenFilter;
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
    /// .and_then_filter(|txt| {
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
    fn and_then_filter<F, O2>(self, _: F) -> AndThenFilterOk<Self, F>
    where
        F: FnMut(O) -> Option<Result<O2, E>>;
}

impl<I, O, E> AndThenFilter<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    fn and_then_filter<F, O2>(self, f: F) -> AndThenFilterOk<Self, F>
    where
        F: FnMut(O) -> Option<Result<O2, E>>,
    {
        AndThenFilterOk { iter: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct AndThenFilterOk<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F, O2> Iterator for AndThenFilterOk<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(O) -> Option<Result<O2, E>>,
{
    type Item = Result<O2, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Ok(x)) => match (self.f)(x) {
                    Some(Err(e)) => return Some(Err(e)),
                    Some(Ok(o)) => return Some(Ok(o)),
                    None => continue,
                },
                Some(Err(e)) => return Some(Err(e)),
                None => return None,
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
