//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to filter one kind of result (and leaving the other as is)
pub trait Filter<O, E>: Sized {
    fn filter_ok<F>(self, F) -> FilterOk<Self, F>
    where
        F: FnMut(&O) -> bool;
    fn filter_err<F>(self, F) -> FilterErr<Self, F>
    where
        F: FnMut(&E) -> bool;

    fn filter_ok_and_then<F>(self, F) -> FilterOkAndThenImpl<Self, F>
    where
        F: FnMut(&O) -> Result<bool, E>;
}

impl<I, O, E> Filter<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    fn filter_ok<F>(self, f: F) -> FilterOk<Self, F>
    where
        F: FnMut(&O) -> bool,
    {
        FilterOk { iter: self, f }
    }
    fn filter_err<F>(self, f: F) -> FilterErr<Self, F>
    where
        F: FnMut(&E) -> bool,
    {
        FilterErr { iter: self, f }
    }

    /// Extension for `Iterator<Item = Result<O, E>>` to filter the Ok(_) and leaving the Err(_) as
    /// is, but allowing the filter to return a `Result<bool, E>` itself
    fn filter_ok_and_then<F>(self, f: F) -> FilterOkAndThenImpl<Self, F>
    where
        F: FnMut(&O) -> Result<bool, E>,
    {
        FilterOkAndThenImpl { iter: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FilterOk<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F> Iterator for FilterOk<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(&O) -> bool,
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Ok(x)) => {
                    if (self.f)(&x) {
                        return Some(Ok(x));
                    }
                }
                other => {
                    return other;
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint_sup = self.iter.size_hint().1;
        (0, hint_sup)
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FilterErr<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F> Iterator for FilterErr<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(&E) -> bool,
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Err(x)) => {
                    if (self.f)(&x) {
                        return Some(Err(x));
                    }
                }
                other => {
                    return other;
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint_sup = self.iter.size_hint().1;
        (0, hint_sup)
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FilterOkAndThenImpl<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F> Iterator for FilterOkAndThenImpl<I, F>
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
fn test_filter_ok() {
    use std::str::FromStr;

    let mapped: Vec<_> = ["1", "2", "a", "4", "5"]
        .into_iter()
        .map(|txt| usize::from_str(txt))
        .filter_ok(|i| i % 2 == 0)
        .collect();

    assert_eq!(mapped.len(), 3);
    assert_eq!(mapped[0], Ok(2));
    assert_eq!(mapped[2], Ok(4));
}

#[test]
fn test_filter_ok_hint() {
    use std::str::FromStr;

    let hint = ["1", "2", "a", "4", "5"]
        .into_iter()
        .map(|txt| usize::from_str(txt))
        .filter_ok(|i| i % 2 == 0)
        .size_hint();

    assert_eq!(hint, (0, Some(5)));
}

#[test]
fn test_filter_err() {
    use std::str::FromStr;

    let mapped: Vec<_> = ["1", "2", "a", "4", "5"]
        .into_iter()
        .map(|txt| usize::from_str(txt))
        .filter_err(|_| false)
        .collect();

    assert_eq!(mapped, vec![Ok(1), Ok(2), Ok(4), Ok(5)]);
}

#[test]
fn test_filter_err_hint() {
    use std::str::FromStr;

    let hint = ["1", "2", "a", "4", "5"]
        .into_iter()
        .map(|txt| usize::from_str(txt))
        .filter_err(|_| false)
        .size_hint();

    assert_eq!(hint, (0, Some(5)));
}

#[test]
fn test_filter_ok_and_then() {
    use std::str::FromStr;

    let v = ["1", "2", "a", "4", "5"]
        .into_iter()
        .map(Ok)
        .filter_ok_and_then(|e| usize::from_str(e).map(|txt| txt < 3))
        .collect::<Vec<Result<_, _>>>();

    assert_eq!(v.len(), 3);
    assert_eq!(v.iter().filter(|x| x.is_ok()).count(), 2);
    assert_eq!(v.iter().filter(|x| x.is_err()).count(), 1);
}
