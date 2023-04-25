# Changelog

## 0.5.0

* New extensons were added for filtering and mapping over things that can fail
    * `try_map_ok` and `try_map_err`
    * `try_filter_ok` and `try_filter_err`
    * `try_filter_map_ok` and `try_filter_map_err`
* More examples were added
* Doc tests were added

For a detailed changelog, please have a look at the git log.

## 0.4.0

* Add `map_inner_ok_or_else()` which can be used to transform:
    `Iterator<Item = Result<Option<T>, E>> -> Iterator<Item = Result<T, E>>`
* More travis testing jobs were added. Minimal supported Rust version is still
  1.21 though.


## 0.3.0

* Re-export all traits from root module
* The `_x` suffixes were removed
* Several extensions were added:
    * `and_then_ok` and `and_then_err`
    * `and_then_ok` and `and_then_err`
    * `flatten_ok` and `flatten_err`
    * `flat_map_ok` and `flat_map_err`
    * `flatten_ok` and `flatten_err`
    * `flat_map_ok` and `flat_map_err`
    * `filter_map_ok` and `filter_map_err`

Thanks to Dawid, who implemented everything in this release.


## 0.2.0

* 0c07b9c Removed the trait objects. This should result in more efficient code
  as well as more "zero-cost-abstraction"-nessy.
  It also removes static lifetimes.

* 99192b1 Added `map_ok`, `map_err` and `while_ok`

Thanks to [pchampin](https://github.com/pchampin) for these contributions!


## 0.1.0

* Initial release

