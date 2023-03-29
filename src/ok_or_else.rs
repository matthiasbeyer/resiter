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
    /// Map option inside an ok result, fail with the else-value if None
    ///
    /// ```
    /// use resiter::ok_or_else::IterInnerOkOrElse;
    ///
    /// let v: Vec<Result<Option<i32>, &'static str>> = vec![
    ///     Ok(Some(1)),
    ///     Err("untouched err"),
    ///     Ok(None),
    ///     Ok(Some(4))];
    ///
    /// let res: Vec<Result<i32, &'static str>> = v.into_iter()
    ///     .map_inner_ok_or_else(|| "error message")
    ///     .collect();
    ///
    /// assert_eq!(
    ///     res,
    ///     vec![
    ///        Ok(1),
    ///        Err("untouched err"),
    ///        Err("error message"),
    ///        Ok(4)])
    /// ```
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
        self.0.next().map(|e| e.inner_ok_or_else(|| self.1()))
    }
}

#[test]
fn test_unwrap_optional_some_inside_result() {
    let res = Ok(Some(1));
    let unwrapped = res.inner_ok_or_else(|| "else this");
    assert_eq!(unwrapped, Ok(1));
}

#[test]
fn test_unwrap_optional_none_inside_result() {
    let res = Ok(None);
    let unwrapped: Result<i32, &'static str> = res.inner_ok_or_else(|| "else this");
    assert_eq!(unwrapped, Err("else this"));
}
