//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform Oks and Errors.
pub trait FlatMap<O, E>: Sized {
    fn flat_map_ok<U, F, O2>(self, _: F) -> FlatMapOk<Self, U, F>
    where
        F: FnMut(O) -> U,
        U: IntoIterator<Item = O2>;
    fn flat_map_err<U, F, E2>(self, _: F) -> FlatMapErr<Self, U, F>
    where
        F: FnMut(E) -> U,
        U: IntoIterator<Item = E2>;
}

impl<I, O, E> FlatMap<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    fn flat_map_ok<U, F, O2>(self, f: F) -> FlatMapOk<Self, U, F>
    where
        F: FnMut(O) -> U,
        U: IntoIterator<Item = O2>,
    {
        FlatMapOk {
            frontiter: None,
            iter: self,
            f,
        }
    }
    fn flat_map_err<U, F, E2>(self, f: F) -> FlatMapErr<Self, U, F>
    where
        F: FnMut(E) -> U,
        U: IntoIterator<Item = E2>,
    {
        FlatMapErr {
            frontiter: None,
            iter: self,
            f,
        }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FlatMapOk<I, U, F>
where
    U: IntoIterator,
{
    frontiter: Option<<U as IntoIterator>::IntoIter>,
    iter: I,
    f: F,
}

impl<I, O, E, F, O2, U> Iterator for FlatMapOk<I, U, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(O) -> U,
    U: IntoIterator<Item = O2>,
{
    type Item = Result<O2, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.frontiter {
                if let elt @ Some(_) = inner.next() {
                    return elt.map(Ok);
                }
            }
            match self.iter.next() {
                None => return None,
                Some(Ok(x)) => {
                    self.frontiter = Some((self.f)(x).into_iter());
                }
                Some(Err(e)) => return Some(Err(e)),
            }
        }
    }

    #[inline]
    // TODO: Oh dear, this hint could be much better
    // https://doc.rust-lang.org/src/core/iter/mod.rs.html#2694
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FlatMapErr<I, U: IntoIterator, F> {
    frontiter: Option<<U as IntoIterator>::IntoIter>,
    iter: I,
    f: F,
}

impl<I, O, E, F, E2, U> Iterator for FlatMapErr<I, U, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> U,
    U: IntoIterator<Item = E2>,
{
    type Item = Result<O, E2>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.frontiter {
                if let elt @ Some(_) = inner.next() {
                    return elt.map(Err);
                }
            }
            match self.iter.next() {
                None => return None,
                Some(Err(e)) => {
                    self.frontiter = Some((self.f)(e).into_iter());
                }
                Some(Ok(o)) => return Some(Ok(o)),
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[test]
fn test_flat_map_ok() {
    let mapped: Vec<_> = vec![Ok(1), Ok(2), Err(2), Err(0), Ok(2)]
        .into_iter()
        .flat_map_ok(|i| (0..i))
        .collect();

    assert_eq!(mapped, [Ok(0), Ok(0), Ok(1), Err(2), Err(0), Ok(0), Ok(1)]);
}

#[test]
fn test_flat_map_err() {
    let mapped: Vec<_> = vec![Ok(1), Ok(2), Err(2), Err(0), Ok(2)]
        .into_iter()
        .flat_map_err(|i| 0..(i * 2))
        .collect();

    assert_eq!(
        mapped,
        [Ok(1), Ok(2), Err(0), Err(1), Err(2), Err(3), Ok(2)]
    );
}
