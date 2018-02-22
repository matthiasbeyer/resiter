# resiter

[![Build Status](https://travis-ci.org/matthiasbeyer/resiter.svg?branch=master)](https://travis-ci.org/matthiasbeyer/resiter)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/resiter/badge/pr?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/resiter)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/resiter/badge/issue?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/resiter)
[![license](https://img.shields.io/github/license/matthiasbeyer/resiter.svg?maxAge=2592000?style=flat-square)]()
[![Tokei](https://tokei.rs/b1/github/matthiasbeyer/resiter)](https://github.com/matthiasbeyer/resiter)

A collection of helpful iterator adaptors for iterating over `Result<T, E>`.

# Contributions welcome!

Contributions are welcome here! Even if you contribution depends on another
crate (for example you want to add some helpers for `Iterator<Item = Result<T,
E>> where E: error_chain::ChainedError` - feel free to do so!

Just make sure that features that depend on external crates are hidden behind
feature gates!

# License

MPL 2.0

