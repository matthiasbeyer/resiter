//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform Oks and Errors.
pub trait AndThenX<O, E>: Sized {
    fn and_then_ok<F>(self, F) -> AndThenOk<Self, F>
    where
        F: FnMut(O) -> Result<O, E>;
    fn and_then_err<F>(self, F) -> AndThenErr<Self, F>
    where
        F: FnMut(E) -> Result<O, E>;
}

impl<I, O, E> AndThenX<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    fn and_then_ok<F>(self, f: F) -> AndThenOk<Self, F>
    where
        F: FnMut(O) -> Result<O, E>,
    {
        AndThenOk { iter: self, f }
    }
    fn and_then_err<F>(self, f: F) -> AndThenErr<Self, F>
    where
        F: FnMut(E) -> Result<O, E>,
    {
        AndThenErr { iter: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct AndThenOk<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F> Iterator for AndThenOk<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(O) -> Result<O, E>,
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(o)) => Some((self.f)(o)),
            other => other,
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

impl<I, O, E, F> Iterator for AndThenErr<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> Result<O, E>,
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Err(e)) => Some((self.f)(e)),
            other => other,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[test]
fn test_and_then_ok() {
    use std::str::FromStr;

    let mapped: Vec<_> = ["1", "2", "a", "4", "5"]
        .into_iter()
        .map(|txt| usize::from_str(txt))
        .and_then_ok(|i| Ok(2 * i))
        .and_then_err(|i| Ok(15))
        .collect();

    assert_eq!(mapped[0], Ok(2));
    assert_eq!(mapped[1], Ok(4));
    assert_eq!(mapped[2], Ok(15));
    assert_eq!(mapped[3], Ok(8));
    assert_eq!(mapped[4], Ok(10));
}
