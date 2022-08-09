//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for `Iterator<Item = Result<O, E>>` to selectively transform and map Oks and Errors.
pub trait AndThenFilter<O, E>: Sized {
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

#[test]
fn test_and_then_filter() {
    use std::str::FromStr;

    let filter_mapped: Vec<_> = vec![
        Ok("1"),
        Err("2"),
        Ok("a"),
        Err("4"),
        Ok("5"),
        Err("b"),
        Err("8"),
    ]
    .into_iter()
    .map(|r| r.map_err(String::from))
    .and_then_filter(|txt| {
        let r = usize::from_str(txt).map_err(|e| e.to_string());
        match r {
            Err(e) => Some(Err(e)),
            Ok(u) => {
                if u < 3 {
                    Some(Ok(u))
                } else {
                    None
                }
            }
        }
    })
    .collect();

    assert_eq!(filter_mapped.len(), 6);
    assert_eq!(filter_mapped[0], Ok(1));
    assert_eq!(filter_mapped[1], Err(String::from("2")));

    assert_ne!(filter_mapped[2], Err(String::from("a")));
    assert!(filter_mapped[2].is_err());

    assert_eq!(filter_mapped[3], Err(String::from("4")));
    assert_eq!(filter_mapped[4], Err(String::from("b")));
    assert_eq!(filter_mapped[5], Err(String::from("8")));
}
