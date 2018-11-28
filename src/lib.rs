//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! resiter
//!
//! This crate helps iterating over `Iterator<Item = Result<O, E>>`.
//! All these things are trivial to build yourself, but why invest the effort if you can use a
//! crate for this?
//!
//! # Contributions welcome
//!
//! If you have _anything_ that might fit the scope of this crate, don't hesitate opening a
//! pull-request for it! This is considered a toolkit and convenience-crate, so everything which
//! might fit its scope should be merged!
//!
//! # Dependencies and Feature-gates
//!
//! If a feature of this crate uses external dependencies, it should be hidden behind a feature
//! gate. The crate itself should be usable without any dependencies besides `std`!
//!
//! # Features
//!
//! Features included in this crate:
//!
//! * Unwrap `Result<O, E>`s inside an Iterator
//! * Select only `Err(_)`s from the Iterator
//! * Select only `Ok(_)`s from the Iterator
//! * Do something in the `Err(_)` case, but don't change the error-object
//! * Do something in the `Ok(_)` case, but don't change the ok-object
//!
//! # Usecase
//!
//! * Consuming iterators until an error occurs
//!
//! ```
//! # fn main() {
//! use std::str::FromStr;
//! use resiter::errors::*;
//!
//! let _ : Option<::std::num::ParseIntError> = ["1", "2", "foo", "4", "5"]
//!     .into_iter()
//!     .map(|e| usize::from_str(e))
//!     .errors()
//!     .next(); // "4" and "5" will never be processed by the iterator
//! # }
//! ```
//!
//! * Consuming iterators and collect all errors
//!
//! ```
//! # fn main() {
//! use std::str::FromStr;
//! use resiter::errors::*;
//!
//! let len = ["1", "2", "foo", "4", "5"]
//!     .into_iter()
//!     .map(|e| usize::from_str(e))
//!     .errors()
//!     .collect::<Vec<::std::num::ParseIntError>>()
//!     .len();
//! assert_eq!(len, 1);
//! # }
//! ```
//!
//! * Consuming iterators and collect all oks
//!
//! ```
//! # fn main() {
//! use std::str::FromStr;
//! use resiter::oks::*;
//!
//! let len = ["1", "2", "foo", "4", "5"]
//!     .into_iter()
//!     .map(|e| usize::from_str(e))
//!     .oks() // Could also be done with .filter_map(Result::ok)
//!     .collect::<Vec<_>>()
//!     .len();
//! assert_eq!(len, 4);
//! # }
//! ```
//!
//! * Printing errors / oks
//!
//! ```
//! # fn main() {
//! use std::str::FromStr;
//! use resiter::oks::*;
//! use resiter::onerr::*;
//! use resiter::onok::*;
//!
//! let len = ["1", "2", "foo", "4", "5"]
//!     .into_iter()
//!     .map(|e| usize::from_str(e))
//!     .on_err(|e| println!("Error happened: {:?}", e)) // ::std::process::exit(1) possible
//!     .on_ok(|o| println!("Parsed : '{}'", o))
//!     .oks()
//!     .collect::<Vec<_>>()
//!     .len();
//! assert_eq!(len, 4);
//! # }
//! ```
//!
//! * Transforming oks
//!
//! ```
//! # fn main() {
//! use std::str::FromStr;
//! use resiter::map_x::*;
//!
//! let doubles = ["1", "2", "foo", "4", "5"]
//!     .into_iter()
//!     .map(|e| usize::from_str(e))
//!     .map_ok(|i| 2*i)
//!     .collect::<Vec<_>>();
//! assert_eq!(doubles[0], Ok(2));
//! assert_eq!(doubles[1], Ok(4));
//! # }
//! ```
//!
//! * Transforming errors
//!
//! ```
//! # fn main() {
//! use std::str::FromStr;
//! use resiter::map_x::*;
//!
//! let doubles = ["1", "2", "foo", "4", "5"]
//!     .into_iter()
//!     .map(|e| usize::from_str(e))
//!     .map_err(|e| format!("{:?}", e))
//!     .collect::<Vec<_>>();
//! assert_eq!(doubles[2], Err("ParseIntError { kind: InvalidDigit }".to_string()));
//! # }
//! ```
//!
//! * Filtering oks (leaving errors as is)
//!
//! ```
//! # fn main() {
//! use std::str::FromStr;
//! use resiter::filter_x::*;
//!
//! let doubles = ["1", "2", "foo", "4", "5"]
//!     .into_iter()
//!     .map(|e| usize::from_str(e))
//!     .filter_ok(|i| i%2 == 0)
//!     .collect::<Vec<_>>();
//! assert_eq!(doubles.len(), 3);
//! assert_eq!(doubles[0], Ok(2));
//! # }
//! ```
//!
//! * Filtering errors (leaving oks as is)
//!
//! ```
//! # fn main() {
//! use std::str::FromStr;
//! use resiter::filter_x::*;
//!
//! let doubles = ["1", "2", "foo", "4", "5"]
//!     .into_iter()
//!     .map(|e| usize::from_str(e))
//!     .filter_err(|_| false) // filter out all errors
//!     .collect::<Vec<_>>();
//! assert_eq!(doubles.len(), 4);
//! assert_eq!(doubles[2], Ok(4));
//! # }
//! ```
//!
//! * Stopping the iteration on the first error
//!
//! ```
//! # fn main() -> () {
//! use std::str::FromStr;
//! use resiter::while_ok::*;
//!
//! let res = ["1", "2", "foo", "4", "5"]
//!     .into_iter()
//!     .map(|e| usize::from_str(e))
//!     .while_ok(|i| {
//!         println!("{} is a usize", i);
//!     });
//! if res.is_err() {
//!     println!("An error occured");
//! }
//! # }
//! ```
//!
//! # License
//!
//! MPL 2.0
//!

pub mod and_then_x;
pub mod errors;
pub mod filter_x;
pub mod map_x;
pub mod oks;
pub mod onerr;
pub mod onok;
pub mod prelude;
pub mod unwrap;
mod util;
pub mod while_ok;
