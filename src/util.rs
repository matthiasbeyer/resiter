//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

pub trait GetErr<T> {
    fn get_err(self) -> Option<T>;
}

impl<U, T> GetErr<T> for Result<U, T> {
    fn get_err(self) -> Option<T> {
        self.err()
    }
}

pub trait GetOk<T> {
    fn get_ok(self) -> Option<T>;
}

impl<T, E> GetOk<T> for Result<T, E> {
    fn get_ok(self) -> Option<T> {
        self.ok()
    }
}

