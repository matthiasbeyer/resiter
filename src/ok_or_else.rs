//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for doing `Result<Option<T>, E>`  ->  `Result<T, E>`
pub trait ResultOptionExt<T, E, F>
where
    T: Sized,
    E: Sized,
    F: FnOnce() -> E,
{
    fn inner_ok_or_else(self, f: F) -> Result<T, E>;
}

impl<T, E, F> ResultOptionExt<T, E, F> for Result<Option<T>, E>
where
    T: Sized,
    E: Sized,
    F: FnOnce() -> E,
{
    fn inner_ok_or_else(self, f: F) -> Result<T, E> {
        self.and_then(|opt| opt.ok_or_else(f))
    }
}

/// Extension trait for doing
/// `Iterator<Item = Result<Option<T>, E>>`  ->  `Iterator<Item = Result<T, E>>`
pub trait IterInnerOkOrElse<T, E, F>
where
    T: Sized,
    E: Sized,
    Self: Iterator<Item = Result<Option<T>, E>> + Sized,
    F: Fn() -> E,
{
    fn map_inner_ok_or_else(self, f: F) -> IterInnerOkOrElseImpl<Self, T, E, F>;
}

pub struct IterInnerOkOrElseImpl<I, T, E, F>(I, F)
where
    I: Iterator<Item = Result<Option<T>, E>> + Sized,
    T: Sized,
    E: Sized,
    F: Fn() -> E;

impl<I, T, E, F> IterInnerOkOrElse<T, E, F> for I
where
    I: Iterator<Item = Result<Option<T>, E>> + Sized,
    T: Sized,
    E: Sized,
    F: Fn() -> E,
{
    fn map_inner_ok_or_else(self, f: F) -> IterInnerOkOrElseImpl<I, T, E, F> {
        IterInnerOkOrElseImpl(self, f)
    }
}

impl<I, T, E, F> Iterator for IterInnerOkOrElseImpl<I, T, E, F>
where
    I: Iterator<Item = Result<Option<T>, E>> + Sized,
    T: Sized,
    E: Sized,
    F: Fn() -> E,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|e| e.and_then(|opt| opt.ok_or_else(|| (self.1)())))
    }
}

#[test]
fn compile_test_1() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let _: Result<Vec<i32>, &'static str> = v.into_iter()
        .map(Some)
        .map(Ok)
        .map_inner_ok_or_else(|| "error message")
        .collect();
}

#[test]
fn test_unwrap_optional_values() {
    let v: Vec<Option<i32>> = vec![Some(1), Some(2), None, Some(4)];
    let res: Vec<Result<i32, &'static str>> = v.into_iter()
        .map(Ok)
        .map_inner_ok_or_else(|| "error message")
        .collect();

    assert_eq!(res, vec![Ok(1), Ok(2), Err("error message"), Ok(4)])
}

#[test]
fn compile_test_3() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let r: Result<Vec<i32>, &'static str> = v.into_iter()
        .map(|i| if i < 5 { Some(i) } else { None })
        .map(Ok)
        .map_inner_ok_or_else(|| "less than 5 in list")
        .collect();

    assert!(r.is_err());
    assert_eq!(r.unwrap_err(), "less than 5 in list");
}

#[test]
fn compile_test_4() {
    use std::collections::HashMap;

    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let mut h = HashMap::new();
    (0..10).into_iter().for_each(|e| {
        h.insert(e, e);
    });

    let r: Result<Vec<_>, &'static str> = v.into_iter()
        .chain(::std::iter::once(10))
        .map(|e| Ok(h.get(&e)))
        .map_inner_ok_or_else(|| "at least one key missing")
        .collect();

    assert!(r.is_err());
    assert_eq!(r.unwrap_err(), "at least one key missing");
}
