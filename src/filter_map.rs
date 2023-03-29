//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform and map Oks and Errors.
pub trait FilterMap<O, E>: Sized {
    fn filter_map_ok<F, O2>(self, _: F) -> FilterMapOk<Self, F>
    where
        F: FnMut(O) -> Option<O2>;
    fn filter_map_err<F, E2>(self, _: F) -> FilterMapErr<Self, F>
    where
        F: FnMut(E) -> Option<E2>;
}

impl<I, O, E> FilterMap<O, E> for I
where
    I: Iterator<Item = Result<O, E>> + Sized,
{
    fn filter_map_ok<F, O2>(self, f: F) -> FilterMapOk<Self, F>
    where
        F: FnMut(O) -> Option<O2>,
    {
        FilterMapOk { iter: self, f }
    }
    fn filter_map_err<F, E2>(self, f: F) -> FilterMapErr<Self, F>
    where
        F: FnMut(E) -> Option<E2>,
    {
        FilterMapErr { iter: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FilterMapOk<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F, O2> Iterator for FilterMapOk<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(O) -> Option<O2>,
{
    type Item = Result<O2, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Ok(x)) => {
                    if let Some(x) = (self.f)(x) {
                        return Some(Ok(x));
                    }
                }
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

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FilterMapErr<I, F> {
    iter: I,
    f: F,
}

impl<I, O, E, F, E2> Iterator for FilterMapErr<I, F>
where
    I: Iterator<Item = Result<O, E>>,
    F: FnMut(E) -> Option<E2>,
{
    type Item = Result<O, E2>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Ok(x)) => return Some(Ok(x)),
                Some(Err(e)) => {
                    if let Some(e) = (self.f)(e) {
                        return Some(Err(e));
                    }
                }
                None => return None,
            }
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[test]
fn test_filter_map_ok() {
    use std::str::FromStr;

    let filter_mapped: Vec<_> = vec![
        Ok("1"),
        Err("2"),
        Ok("a"),
        Err("4"),
        Ok("5"),
        Err("b"),
        Err("8"),
    ].into_iter()
        .filter_map_ok(|txt| usize::from_str(txt).ok())
        .collect();
    assert_eq!(filter_mapped[0], Ok(1));
    assert_eq!(filter_mapped[1], Err("2"));
    assert_eq!(filter_mapped[2], Err("4"));
    assert_eq!(filter_mapped[3], Ok(5));
    assert_eq!(filter_mapped[4], Err("b"));
    assert_eq!(filter_mapped[5], Err("8"));
}

#[test]
fn test_filter_map_err() {
    use std::str::FromStr;

    let filter_mapped: Vec<_> = vec![
        Ok("1"),
        Err("2"),
        Ok("a"),
        Err("4"),
        Ok("5"),
        Err("b"),
        Err("8"),
    ].into_iter()
        .filter_map_err(|txt| usize::from_str(txt).ok())
        .collect();

    assert_eq!(filter_mapped[0], Ok("1"));
    assert_eq!(filter_mapped[1], Err(2));
    assert_eq!(filter_mapped[2], Ok("a"));
    assert_eq!(filter_mapped[3], Err(4));
    assert_eq!(filter_mapped[4], Ok("5"));
    assert_eq!(filter_mapped[5], Err(8));
}

#[test]
fn test_filter_map_ok_hint() {
    use std::str::FromStr;

    let hint = ["1", "2", "a", "4", "5"]
        .iter()
        .map(|txt| usize::from_str(txt))
        .filter_map_ok(|i| Some(2 * i))
        .size_hint();

    assert_eq!(hint, (5, Some(5)));
}

#[test]
fn test_filter_map_err_hint() {
    use std::str::FromStr;

    let hint = ["1", "2", "a", "4", "5"]
        .iter()
        .map(|txt| usize::from_str(txt))
        .filter_map_err(|e| Some(format!("{:?}", e)))
        .size_hint();

    assert_eq!(hint, (5, Some(5)));
}
