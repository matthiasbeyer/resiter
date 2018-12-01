//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform Oks and Errors.
pub trait Flatten<O, E>: Sized {
    fn flatten_ok<U, O2>(self) -> FlattenOk<Self, U>
    where
        U: IntoIterator<Item = O2>;
    fn flatten_err<U, E2>(self) -> FlattenErr<Self, U>
    where
        U: IntoIterator<Item = E2>;
}

impl<I, O, E> Flatten<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    fn flatten_ok<U, O2>(self) -> FlattenOk<Self, U>
    where
        U: IntoIterator<Item = O2>,
    {
        FlattenOk {
            frontiter: None,
            iter: self,
        }
    }
    fn flatten_err<U, E2>(self) -> FlattenErr<Self, U>
    where
        U: IntoIterator<Item = E2>,
    {
        FlattenErr {
            frontiter: None,
            iter: self,
        }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FlattenOk<I, U>
where
    U: IntoIterator,
{
    frontiter: Option<<U as IntoIterator>::IntoIter>,
    iter: I,
}

impl<I, E, O2, U> Iterator for FlattenOk<I, U>
where
    I: Iterator<Item = Result<U, E>>,
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
                    self.frontiter = Some(x.into_iter());
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
pub struct FlattenErr<I, U: IntoIterator> {
    frontiter: Option<<U as IntoIterator>::IntoIter>,
    iter: I,
}

impl<I, O, E2, U> Iterator for FlattenErr<I, U>
where
    I: Iterator<Item = Result<O, U>>,
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
                    self.frontiter = Some(e.into_iter());
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
fn test_flatten_ok() {
    use map::Map;

    let mapped: Vec<_> = vec![Ok(1), Ok(2), Err(2), Err(0), Ok(2)]
        .into_iter()
        .map_ok(|i| (0..i))
        .map_err(|i| 0..(i * 2))
        .flatten_ok()
        .flatten_err()
        .collect();

    assert_eq!(
        mapped,
        [
            Ok(0),
            Ok(0),
            Ok(1),
            Err(0),
            Err(1),
            Err(2),
            Err(3),
            Ok(0),
            Ok(1)
        ]
    );
}
