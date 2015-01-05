Title: This Week in Rust 64
Date: 2015-01-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This Week in Rust is openly developed [on Github](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

121 pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-29..2015-01-04

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* The prelude exports [fewer traits][prelude] per [RFC
  503][prelude-rfc]. This mostly affects code that implements basic
  traits like operators and iterators, which will now need to import
  those traits directly, but also removes the deprecated `from_str`
  function, which should be replaced by the `parse` method.
* 'Object-safe' traits [can not include static methods][objsafe].
* [Trait objects now implement their trait by
  default][trait-for-trait].
* Fixed-length array syntax is now `[T; n]` and the [old syntax no
  longer works][array]. [RFC][array-rfc].
* Importing the containing module at the same time as items it
  contains is [now done with `self` instead of `mod`][self], as in
  `use foo::{self, bar}`, and the `deriving` attribute is changing to
  `derive`. The old deprecated syntaxes continued to work for about a
  day but [were quickly terminated][oldsyntax]. [RFC][self-rfc] and
  [RFC][deriving-rfc].
* `std::comm` [now lives in `std::sync::mpsc`][comm], has seen
  some minor API changes, and is considered stable.
* `RWLock` and `Mutex` methods [now return `LockResult` or
  `TryLockResult`][lock] so that they can communicate whether the lock
  was poisoned.
* Arguments to macros are now [properly checked for unstable
  features][macrogate]. This only breaks code that was accidentally
  subverting feature-gates.
* `TaskRng` is [now `ThreadRng`][rng].
* The [hashmap iterators have been renamed][hashmapiter] to match
  conventions.
* The in-tree `term` crate is [deprecated][term]. Use Cargo.
* There have been some [minor breaking changes to `std::string`][str]
  during another stabilization pass.
* There have been some [minor breaking changes to `std::sync`][sync]
  during another stabilization pass.
* Using the FFI to sneakily link to LLVM intrinsics [requires the
  `link_llvm_intrinsics` feature gate][intrinsics].
* [Atomic ordering variants are no longer reepxorted][atomic].
* A number `DList` methods are [deprecated, along with some minor
  breaking changes to other collections][coll]. 
* The `BoxAny` trait [is removed][boxany]. Use `Box<Any>`.

[lock]: https://github.com/rust-lang/rust/pull/19661
[prelude]: https://github.com/rust-lang/rust/pull/20157
[prelude-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0503-prelude-stabilization.md
[macrogate]: https://github.com/rust-lang/rust/pull/20190
[rng]: https://github.com/rust-lang/rust/pull/20264
[hashmapiter]: https://github.com/rust-lang/rust/pull/20215
[comm]: https://github.com/rust-lang/rust/pull/20273
[term]: https://github.com/rust-lang/rust/pull/20276
[str]: https://github.com/rust-lang/rust/pull/20306
[sync]: https://github.com/rust-lang/rust/pull/20315
[objsafe]: https://github.com/rust-lang/rust/pull/20325
[intrinsics]: https://github.com/rust-lang/rust/pull/20334
[trait-for-trait]: https://github.com/rust-lang/rust/pull/20341
[atomic]: https://github.com/rust-lang/rust/pull/20348
[coll]: https://github.com/rust-lang/rust/pull/20356
[array]: https://github.com/rust-lang/rust/pull/20387
[array-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0520-new-array-repeat-syntax.md
[boxany]: https://github.com/rust-lang/rust/pull/20420
[self]: https://github.com/rust-lang/rust/pull/20365
[self-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0532-self-in-use.md
[deriving-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0534-deriving2derive.md
[oldsyntax]: https://github.com/rust-lang/rust/pull/20504

## Other Changes

* Rust again has [picks a fallback (either `i32` or `f64`) for
  uninferred numeric types][fb]. [RFC][fb-rfc].
* [Indicating that a type parameter may be unsized][sized] is now done
  with `T: ?Sized` instead of `Sized? T`, and that `Self` may be
  unsized with `trait T for ?Sized` instead of `trait T for
  Sized?. The old syntax still works for the immediate
  future. [RFC][sized-rfc].
* Nullable enum optimizations [have been extended to more types][null]
  so that e.g. `Option<Vec<T>>` and `Option<String>` take up no more
  space than the inner types themselves.
* [Preliminary AArch64 support][arm64] has landed.
* Rust includes a [`rust-gdb`][gdb] script which launches gdb with Rust-specific
  pretty-printers enabled.

[fb]: https://github.com/rust-lang/rust/pull/20189
[fb-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0212-restore-int-fallback.md
[null]: https://github.com/rust-lang/rust/pull/19765
[arm64]: https://github.com/rust-lang/rust/pull/19790
[gdb]: https://github.com/rust-lang/rust/pull/19954
[sized]: https://github.com/rust-lang/rust/issues/19607
[sized-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0490-dst-syntax.md

## New Contributors

* A.J. Gardner
* Andrea Canciani
* arturo
* Ben Foppa
* bombless
* crhino
* dan@daramos.com
* Diego Giagio
* Dirk Gadsden
* Earl St Sauver
* Eric Allen
* John Albietz
* JONNALAGADDA Srinivas
* Joseph Rushton Wakeling
* Rohit Joshi
* Tamir Duberstein
* Timon Rapp
* Titouan Vervack
* YawarRaza7349

# Approved RFC's

- [526: Statically enforce Unicode in `std::fmt`](https://github.com/rust-lang/rfcs/blob/master/text/0526-fmt-text-writer.md)
- [532: Rename the `mod` keyword to `self` inside of imports](https://github.com/rust-lang/rfcs/blob/master/text/0532-self-in-use.md)
- [534: Rename deriving to derive](https://github.com/rust-lang/rfcs/blob/master/text/0534-deriving2derive.md)

# New RFC's

- [Make the implicit type parameter `Self` have no default `Sized` bound](https://github.com/rust-lang/rfcs/pull/546)
- [Integer to Bytes](https://github.com/rust-lang/rfcs/pull/548)

# Community

## From the Team

* [Weekly-meetings/2014-12-30][mtg].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-12-30.md

## Blog Posts

- [My thoughts on Rust in 2015](http://featherweightmusings.blogspot.co.nz/2014/12/my-thoughts-on-rust-in-2015.html)
- [A Rust adventure](http://ck.kennt-wayne.de/2014/dec/a-rust-adventure)
- [Porting a Ray Tracer to Rust, part 1](http://www.willusher.io/2014/12/30/porting-a-ray-tracer-to-rust-part-1/)
- [Dependency injection container - Learning the ropes in Rust](http://nercury.github.io/rust/di/2015/01/02/dependency-injection-learning-rust.html)
- [Make your own programming language](http://blog.ppelgren.se/2015-01-03/DIY-Make-Your-Own-Programming-language/) - Building a programming language in Rust.

## Discussions

- [PSA: Rust installers now come with Cargo and docs](https://www.reddit.com/r/rust/comments/2r92yw/psa_rust_installers_now_come_with_cargo_and_docs/)
- [Is Intel MPX relevant for Rust?](http://www.reddit.com/r/rust/comments/2qlbx1/is_intel_mpx_relevant_for_rust/)
- [Restarting the `int/uint` discussion](http://discuss.rust-lang.org/t/restarting-the-int-uint-discussion/1131)
- [Making safety more explicit](http://www.reddit.com/r/rust/comments/2qr5yf/making_safety_more_explicit/)
- [Erlang vs Rust for High concurrency servers](http://www.reddit.com/r/rust/comments/2qzyfb/erlang_vs_rust_for_high_concurrency_servers/)
- [crates.io dependency graph](https://www.reddit.com/r/rust/comments/2rawmg/cratesio_crate_graph/)
- [Architectural portability as a first-class artifact of the build system](http://discuss.rust-lang.org/t/architectural-portability-as-a-first-class-artifact-of-the-build-and-dependency-systems/1187)
- [Implicit widening, polymorphic indexing, and similar ideas](http://discuss.rust-lang.org/t/implicit-widening-polymorphic-indexing-and-similar-ideas/1141)

## New Projects

- [fern](https://github.com/daboross/fern-rs), a runtime-configurable logging library
- [Recreating F#'s active patterns with macros](http://www.reddit.com/r/rust/comments/2qqlfa/recreating_fs_active_patterns_in_rust_with_macros/)
- [di, a dependency injection container](https://github.com/Nercury/di-rs)
- [event, a fast, ergonomic thread-local event loop built on mio](https://github.com/reem/rust-event)
- [JNI bindings for Rust](https://github.com/larroy/RustJni)
- [A stopwatch library for Rust. Used to time things](https://github.com/ellisonch/rust-stopwatch)
- [TaskGraph, a task library in the spirit of C++'s TPL](https://github.com/tobbebex/TaskGraph)
- [clock_ticks](https://crates.io/crates/clock_ticks) - A simple
  alternative to `precise_time_ns` that doesn't require the
  (frequently broken) `time` crate.


## Project Updates

- [This Week in Servo 17](http://blog.servo.org/2014/12/31/twis-17/)
- [Cross platform watch script for building cargo projects](https://gist.github.com/bvssvni/ac6c92167d260b73aa6e)
- [Update on cargo-emscripten](https://www.reddit.com/r/rust/comments/2raoad/emscripten_experiments_update_early_prototype_for/)

## Upcoming Events

- [Rust Seatle Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg),
January 12th
- [Rust Beijing Meetup](https://www.eventbrite.com/e/rust-meet-up-in-beijing-tickets-14905925023),
January 17th
- [Getting started contributing to Rust](http://www.meetup.com/Rust-Bay-Area/events/203782472/),
January 17th
- The [first Rust-Amsterdam meetup](http://www.meetup.com/Rust-Amsterdam/events/218908906/#event-comments-section),
January 28th. They're [looking for speakers](http://www.reddit.com/r/rust/comments/2qxhp1/first_rustamsterdam_meetup_28th_of_january/)
