//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

pub trait TryFilter<O, E>: Sized {
    fn try_filter_ok<F>(self, _: F) -> TryFilterOk<Self, F>
    where
        F: FnMut(&O) -> Result<bool, E>;
}

impl<I, O, E> TryFilter<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    /// Extension for `Iterator<Item = Result<O, E>>` to filter the Ok(_) and leaving the Err(_) as
    /// is, but allowing the filter to return a `Result<bool, E>` itself
    fn try_filter_ok<F>(self, f: F) -> TryFilterOk<Self, F>
    where
        F: FnMut(&O) -> Result<bool, E>,
    {
        TryFilterOk { iter: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct TryFilterOk<I, F> {
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
            match self.iter.next() {
                Some(Ok(x)) => match (self.f)(&x) {
                    Ok(true) => return Some(Ok(x)),
                    Ok(false) => continue,
                    Err(e) => return Some(Err(e)),
                },

                other => return other,
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint_sup = self.iter.size_hint().1;
        (0, hint_sup)
    }
}

#[test]
fn test_try_filter_ok() {
    use std::str::FromStr;

    let v = ["1", "2", "a", "4", "5"]
        .iter()
        .map(Ok)
        .try_filter_ok(|e| usize::from_str(e).map(|txt| txt < 3))
        .collect::<Vec<Result<_, _>>>();

    assert_eq!(v.len(), 3);
    assert_eq!(v.iter().filter(|x| x.is_ok()).count(), 2);
    assert_eq!(v.iter().filter(|x| x.is_err()).count(), 1);
}
