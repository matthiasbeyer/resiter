//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// Extension trait for doing `Result<Option<T>, E>  ->  Result<T, E>`
pub trait ResultOptionExt<T, E, F>
    where T: Sized,
          E: Sized,
          F: FnOnce() -> E
{
    fn inner_ok_or_else(self, f: F) -> Result<T, E>;
}

impl<T, E, F> ResultOptionExt<T, E, F> for Result<Option<T>, E>
    where T: Sized,
          E: Sized,
          F: FnOnce() -> E
{
    fn inner_ok_or_else(self, f: F) -> Result<T, E> {
        self.and_then(|opt| opt.ok_or_else(f))
    }
}



/// Extension trait for doing
///
/// ```ignore
///     Iterator<Item = Result<Option<T>, E>>  ->  Iterator<Item = Result<T, E>>
/// ```
///
pub trait IterInnerOkOrElse<T, E, F>
    where T: Sized,
          E: Sized,
          Self: Iterator<Item = Result<Option<T>, E>> + Sized,
          F: Fn() -> E,
{
    fn map_inner_ok_or_else(self, f: F) -> IterInnerOkOrElseImpl<Self, T, E, F>;
}

pub struct IterInnerOkOrElseImpl<I, T, E, F>(I, F)
    where I: Iterator<Item = Result<Option<T>, E>> + Sized,
          T: Sized,
          E: Sized,
          F: Fn() -> E;

impl<I, T, E, F> IterInnerOkOrElse<T, E, F> for I
    where I: Iterator<Item = Result<Option<T>, E>> + Sized,
          T: Sized,
          E: Sized,
          F: Fn() -> E,
{
    fn map_inner_ok_or_else(self, f: F) -> IterInnerOkOrElseImpl<I, T, E, F> {
        IterInnerOkOrElseImpl(self, f)
    }
}

impl<I, T, E, F> Iterator for IterInnerOkOrElseImpl<I, T, E, F>
    where I: Iterator<Item = Result<Option<T>, E>> + Sized,
          T: Sized,
          E: Sized,
          F: Fn() -> E,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|e| e.and_then(|opt| opt.ok_or_else(|| (self.1)())))
    }
}

