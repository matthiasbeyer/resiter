# resiter

A collection of helpful iterator adaptors for iterating over `Result<T, E>`.

# Contributions welcome!

Contributions are welcome here! Even if you contribution depends on another
crate (for example you want to add some helpers for `Iterator<Item = Result<T,
E>> where E: error_chain::ChainedError` - feel free to do so!

Just make sure that features that depend on external crates are hidden behind
feature gates!

# License

MPL 2.0

