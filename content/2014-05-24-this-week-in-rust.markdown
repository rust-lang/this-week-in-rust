Title: This Week in Rust 50
Date: 2014-05-24 22:30
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

<!-- more -->

# What's cooking on master?

66 pull requests were merged this week.

## Breaking Changes

- `mem::uninit` [has been renamed](https://github.com/mozilla/rust/pull/14392)
  to `uninitialized`, and everything in `mem` is either stable or deprecated
  (to be removed). There are a [bunch of other
  changes](https://github.com/mozilla/rust/pull/14259) to `mem` that go with
  this.
- All uses of `~str` [have been
  removed](https://github.com/mozilla/rust/pull/14310).
- `UnsafeArc` [has been removed](https://github.com/mozilla/rust/pull/14301),
  replaced with `Arc<Unsafe<T>>`.
- `Rng.choose` [now returns an
  `Option`](https://github.com/mozilla/rust/pull/14319).
- `Bitv::init_to_vec` [has been
  removed](https://github.com/mozilla/rust/pull/14295).
- Test shards with the built-in test runner [are now indexed by
  1](https://github.com/mozilla/rust/pull/14286) in the CLI.
- `Result.unwrap_or_handle` [has been
  renamed](https://github.com/mozilla/rust/pull/14294) to `unwrap_or_else` for
  consistency with `Option`.
- [Some miscellaneous renames in libcollections](https://github.com/mozilla/rust/pull/14279).
- rustc is [more strict](https://github.com/mozilla/rust/pull/14251) about
where it will accept a given `mod foo;` statement.

## Other Changes

There were a lot of various bugfixes and documentation additions all around.

- There is now support for [weak failure](https://github.com/mozilla/rust/pull/14293).
- The docs for the cell types [has been massively
  improved](https://github.com/mozilla/rust/pull/14304).
- `bytes!()` [now properly returns a static
  slice](https://github.com/mozilla/rust/pull/14275).
- A liballoc [has materialized](https://github.com/mozilla/rust/pull/14230),
  that contains all allocation support

## New Contributors

- Anton Löfgren
- Jihyeok Seo
- Jonathan Bailey
- Michael Dagitses
- P1start
- TyOverby
- Valerii Hiora

# RFCs

- [Allow the `unsafe` qualifier on struct
  fields](https://github.com/rust-lang/rfcs/pull/80)
- [Guaranteed tail-call
  optimization](https://github.com/rust-lang/rfcs/pull/81)
- [Add syntax to partially destructure `self` in method
  signatures](https://github.com/rust-lang/rfcs/pull/83)
- [Generalize `macro_registar` to
  `plugin_registrar`](https://github.com/rust-lang/rfcs/pull/86)
- [Bounds on trait objects should be separated with
  +](https://github.com/rust-lang/rfcs/pull/87)
- [Macro syntax to count sequence
  repetitions](https://github.com/rust-lang/rfcs/pull/88)
- [Loadable lints](https://github.com/rust-lang/rfcs/pull/89)
- [Lexer syntax simplification](https://github.com/rust-lang/rfcs/pull/90)

# Community Updates

- [Guaranteeing Memory Safety in Rust (a talk by
  Niko)](https://air.mozilla.org/guaranteeing-memory-safety-in-rust/)
- [Approaches to resource
  disposal](http://swiftcoder.wordpress.com/2014/05/23/approaches-to-resource-disposal/)
- [Introducing
  js.rs](http://tombebbington.github.io/blog/2014/05/24/introducing-rust-js/)
- [Practicality with Rust: Setting Up A
  Project](http://hydrocodedesign.com/2014/04/24/practicality-with-rust/)
- [Burn, a programming language implemented in
  Rust](https://github.com/rainbow-alex/burn)
- [Rust for C++ programmers - part 7: data
  types](http://featherweightmusings.blogspot.co.nz/2014/05/rust-for-c-programmers-part-7-data-types.html)
- [cxx2rust: the pains of wrapping C++ in Rust on the example of
  Qt5](http://www.reddit.com/r/rust/comments/269t6i/cxx2rust_the_pains_of_wrapping_c_in_rust_on_the/)
- [Add a new language design
  faq](http://www.reddit.com/r/rust/comments/269zu4/add_a_new_language_design_faq/)
- [rspt: a physically based renderer](https://github.com/mikejsavage/rspt)
