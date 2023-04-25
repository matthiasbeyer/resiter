//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Prelude
//!
//! Imports all things publicly, so you can `use resiter::prelude::*;` in your crate.
//!

pub use and_then::*;
pub use errors::*;
pub use filter::*;
pub use filter_map::*;
pub use flat_map::*;
pub use flatten::*;
pub use map::*;
pub use ok_or_else::*;
pub use oks::*;
pub use onerr::*;
pub use onok::*;
pub use try_filter::*;
pub use try_filter_map::*;
pub use try_map::*;
pub use unwrap::*;
pub use while_ok::*;
