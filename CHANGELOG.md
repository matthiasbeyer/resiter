# Changelog

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

