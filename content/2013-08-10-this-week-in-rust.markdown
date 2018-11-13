Title: This Week in Rust 10
Date: 2013-08-10 21:39
Category: This Week in Rust

Hello and welcome to the tenth issue of *This Week in Rust*. This week marks
the enabling of the new runtime written entirely in Rust. A lot happens every
week, so I'm going to start omitting PRs that I deem of lesser importance.
This process is entirely arbitrary, don't feel hurt if I exclude your PR :).

<!-- more -->

# What's cooking on `master`?

Issue churn was -12 this week. A total of 70 PRs were merged.

[The new runtime has been enabled by
default](https://github.com/mozilla/rust/pull/8358). This is the culmination
of a lot of work by brson and the rt interns (toddaaro, bblum, and ecr being
the ones I know of). It's written entirely in Rust, and lives in `std::rt`.
Additionally, the [old C++ runtime has been
removed](https://github.com/mozilla/rust/pull/8387).

## Breaking Changes

- [**Trailing nulls have been removed from all string
  types.**](https://github.com/mozilla/rust/pull/8296). This will break your
  FFI code in subtle and mysterious ways, if you didn't explicitly use the
  `as_c_str` methods. FFI code using the new `str::c_str` code will be more
  robust, as it forbids interior nulls, and ensures that a trailing null
  always exists. The replacement for `str.as_c_str` is
  `str.to_c_str().as_slice()`, from what I can tell.
- [The `priv` keyword is no longer allowed where it has no
  meaning](https://github.com/cmr/rust/commit/e99eff172a11816f335153147dd0800fc4877bee).
- [`iter` and `iter_err` in Result have been replaced with external
  iterators](https://github.com/mozilla/rust/pull/8265).
- [The `get` method of `Option`, `Either`, and `Result` has been removed in
  favor of `unwrap`](https://github.com/mozilla/rust/pull/8288). They both did
  the same thing, which was useless duplication.
- [`std::gc` and `std::stackwalk`](https://github.com/mozilla/rust/pull/8218)
  have been removed, as they are obsolete with the new runtime.
- [The transitionary `foreach` has been
  removed](https://github.com/mozilla/rust/pull/8264).

## Notable library additions, bugfixes, and cleanup

- [Some redundant `Ord` methods were removed from
  impls](https://github.com/mozilla/rust/pull/8357) where the default methods
  sufficed.
- [FromStr for IpAddr and
  SocketAddr](https://github.com/mozilla/rust/pull/8336) is implemented.
- [Work steealing is implemented for the newrt
  scheduler](https://github.com/mozilla/rust/pull/8356).
- [A frequency counting function has been added to
  `extra::stat`](https://github.com/mozilla/rust/pull/8320).
- [Saturating math](https://github.com/mozilla/rust/pull/8323) is now
  implemented. I knew this as "clamping": it is arithmetic that clamps results
  into a specific interval.
- [A hexadecimal encoding module](https://github.com/mozilla/rust/pull/8287)
  has been added to `extra`.
- [`EnumSet` has been moved into `extra`, it previously existed as a utility
  in `rustc`](https://github.com/mozilla/rust/pull/8054)
- [`str::is_utf8` has seen some more
  optimization](https://github.com/mozilla/rust/pull/8237).

## Notable compiler additions, bugfixes, and cleanup

- [Initial support for the new formatting
  code](https://github.com/mozilla/rust/pull/8245) has been added.
- [A `no_main` attribute has been
  added](https://github.com/mozilla/rust/pull/8279), to omit the Rust entry
  point entirely.
- [Vanilla Linux on ARM](https://github.com/mozilla/rust/pull/8220) is now
  supported.
- [Extra copies of rvalues ](https://github.com/mozilla/rust/pull/8262) are no
  longer omitted.
- [Some cross-arch bugs with node hash
  metadata](https://github.com/mozilla/rust/pull/8361) have been fixed.
- [A soundness bug in struct matching has been
  fixed](https://github.com/mozilla/rust/pull/8350).
- [An `option_env!` syntax extension has been
  added](https://github.com/mozilla/rust/pull/8362) for compile-time inclusion
  of environment variables that may or may not be present.
- [`extern mod a = "b/c/d"` has been
  implemented](https://github.com/mozilla/rust/pull/8176), paving the way for
  more rustpkg awesomeness.

# Meetings

The [Tuesday
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-08-06) seems
to have been a bit under-attended. SIMD, ARM, trailing nulls, order of `mod`
and `use`, and the condition system were all briefly discussed..

# Discussion + Blog posts

- ["A Work-stealing Runtime for
  Rust"](https://air.mozilla.org/2013-intern-todd/), toddaaro's intern
  presentation.
- ["Iterator Blocks for Rust - Feature
  Survey"](http://michaelwoerister.github.io/2013/08/10/iterator-blocks-features.html)

# External projects

- [Galvanized: a simple JIT VM written in Rust, using
  LibJIT](http://www.reddit.com/r/rust/comments/1k43px/a_simple_jit_vm_written_using_rust_and_libjit/)
- [Q3 has a new
  logger](https://github.com/Jeaye/q3/commit/f4c82ce9c276327cababdb6650038e2c1d62f2d5).
  I think it's nicer than the built-in one!
- [rust-protobuf: a protobuf implementation generating rust code, written in
  rust](https://github.com/stepancheg/rust-protobuf).
- [Servo: almost passing acid1
  !](https://twitter.com/metajack/status/364571230331875331/photo/1)
