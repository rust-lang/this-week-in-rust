Title: This Week in Rust 20
Date: 2013-10-19 10:49
Category: This Week in Rust

Welcome to another issue of *This Week in Rust*, a weekly summary of Rust's
progress and happenings in the community.

<!-- more -->

# What's cooking in master?

48 PRs were merged this week.

## Breaking changes

- Slices are now represented as [number of elements, not number of
  bytes](https://github.com/mozilla/rust/pull/9885).
- `fmt!` has been [completely
  removed](https://github.com/mozilla/rust/pull/9919).
- Some fields in `std::comm` [have been made
  private](https://github.com/mozilla/rust/pull/9935).
- `std::sys::refcount` [has been
  moved](https://github.com/mozilla/rust/pull/9922) to
  `std::managed::refcount`.
- [A bunch of
  functions](https://github.com/mozilla/rust/pull/9896/files#diff-69196c6d2488bf8d5b3471084e854407L22)
  have moved from `std::sys` to `std::mem`.
- `once fn` is [now a feature gate](https://github.com/mozilla/rust/pull/9863)
  rather than a `-Z` flag.
- `Path` has been [completely
  rewritten](https://github.com/mozilla/rust/pull/9655).
- `extra::flatpipes` [has been
  removed](https://github.com/mozilla/rust/pull/9886).

## Other changes

- `jemalloc` has been [removed from the
  runtime](https://github.com/mozilla/rust/pull/9933). The [associated
  issue](https://github.com/mozilla/rust/issues/9925) is quite scary.
- `rustdoc` [struct field
  visibility](https://github.com/mozilla/rust/pull/9946) is now corrected (it
  stripped fields where it should not have).
- `rustdoc` also [uses the actual privacy
  rules](https://github.com/mozilla/rust/pull/9941) to strip methods.
- `format!` now gives [much better](https://github.com/mozilla/rust/pull/9932)
  error messages for invalid format strings.
- The `fmt::Default` trait, used for default formatting with `format!`, is
  [now documented](https://github.com/mozilla/rust/pull/9938).
- `include_bin!` has been optimized, and the `k-nucleotides` benchmark now
  compiles [187x faster](https://github.com/mozilla/rust/pull/9851).
- Vectors now have [`starts_with` and `ends_with`
  methods](https://github.com/mozilla/rust/pull/9907), which take slices.
- An `abort` intrinsic [has been
  added](https://github.com/mozilla/rust/pull/9860).
- Vectors now have a [`get_opt`
  method](https://github.com/mozilla/rust/pull/9608).

## New contributors

A new section for new contributors! The new contributors this week are (as
reported by git):

- Chris Sainty
- Eduard Burtescu
- Erik Lyon
- Harry Marr
- Sébastien Chauvel
- Vijay Korapaty
- Ziad Hatahet
- chitra

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-10-15)
discussed removing rusti, changing the attribute syntax, destructors in
statics, and more multi-crate packages with rustpkg.

# Announcements etc

- Rust (and Servo) are participating in the [GNOME Outreach Program for
  Women](https://groups.google.com/forum/#!topic/mozilla.dev.servo/7kX_E0FlfkA).
  A bunch of people have already come into IRC about it.
- <http://exercism.io> [is getting full Rust
  support](https://github.com/kytrinyx/exercism.io/pull/866). This is a very
  cool resource, and could help a lot getting newcomers acclimated.
- [Unified Function/method Call Syntax and further
  simplification](https://mail.mozilla.org/pipermail/rust-dev/2013-October/006034.html).
- [Safe Memory Management in
  Cyclone](http://www.reddit.com/r/rust/comments/1osbq2/safe_manual_memory_management_in_cyclone_research/).
- [Audio](http://opensourcebridge.org/sessions/970) from Tim's talk in June is
  finally available!
- An
  [OSdev](https://github.com/mozilla/rust/wiki/Operating-system-development)
  community has sprung up! The channel is `#rust-osdev`.
- [Should I/O use
  conditions?](http://www.reddit.com/r/rust/comments/1omw47/should_io_use_conditions/).
- [Pointers in Rust: A
  Guide](http://www.reddit.com/r/rust/comments/1opo36/pointers_in_rust_a_guide/).
- I am on a [Rust hiatus](http://cmr.github.io/blog/2013/10/14/rust-hiatus/),
  for the time being. TWiR will still be happening, as you are reading it
  right now.
- [rust-core](https://github.com/thestinger/rust-core) - A stub standard
  library.
