Title: This Week in Rust 49
Date: 2014-05-17 22:30
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

93 pull requests were merged this week, topping the [previous
record](http://blog.octayn.net/blog/2014/03/24/this-week-in-rust-42/) of 91.
Good job everyone, keep up the good work!

## Breaking Changes

- `File` [now has a `stat` method](https://github.com/mozilla/rust/pull/14128)
  that does not use a path, but rather the open file itself (`fstat`). As part
  of this, the `path` field on `FileStat` has been removed.
- Some failure in libcore [has been
  removed](https://github.com/mozilla/rust/pull/14150). `shift_ptr` and
  `pop_ptr` now return an `Option`.
- `TcpStream`s `connect` and `bind` constructors [now take a string and a
  port](https://github.com/mozilla/rust/pull/13919) rather than a `SocketAddr`
  structure. There's some controversy around this change, as seen in the PR
  discussion. The first commit has detailed instructions for porting.
- `num::complex::Cmplx` [has been
  renamed](https://github.com/mozilla/rust/pull/14167) to
  `num::complex::Complex`.
- Process wait timeouts [are now
  implemented](https://github.com/mozilla/rust/pull/13957). See the PR for
  details.
- Our unicode support [has been cleaned
  up](https://github.com/mozilla/rust/pull/14159), and in the process
  `str::Normalizations` has been renamed to `str::Decompositions` to reflect
  what it does.
- `owned` [has moved](https://github.com/mozilla/rust/pull/14184) from libcore
  to libstd, and with it `Box` and `AnyOwnExt`.
- A `read_at_least` method [has been
  added](https://github.com/mozilla/rust/pull/13127) to `Reader`. The `fill`
  and `push_exact` methods have been removed.
- `atomics` [has moved](https://github.com/mozilla/rust/pull/14169) to
  libcore.
- The `android-cross-path` flat to rustc [has been
  removed](https://github.com/mozilla/rust/pull/14179).
- The Process and `dynamic_library` APIs [now use byte
  slices](https://github.com/mozilla/rust/pull/13954) rather than `Path`s or
  strings.
- `run_in_bare_thread` [has been
  removed](https://github.com/mozilla/rust/pull/14200).
- The `bitflags!` generated `from_bits` constructor [is now
  safe](https://github.com/mozilla/rust/pull/14208) and returns an `Option`.
- `from_utf8_owned` [now returns a
  Result](https://github.com/mozilla/rust/pull/14213).
- `std::fmt` [has moved](https://github.com/mozilla/rust/pull/14115) to
  `core::fmt`.
- enum variant names were accidentally leaking into child modules. [This is no
  more](https://github.com/mozilla/rust/pull/14253).

## Other Changes

One PR from last week that apparently slipped through the cracks: [we are
using jemalloc again!](https://github.com/mozilla/rust/pull/14006).
Additionally, some changes around the new [allocator
RFC](https://github.com/rust-lang/rfcs/pull/39) landed.

- String searching [now uses a two-way
  algorithm](https://github.com/mozilla/rust/pull/14135).
- libterm [now supports](https://github.com/mozilla/rust/pull/13401) the win32
  console API.
- The `mangle` method on `HashMap` [has been
  reintroduced](https://github.com/mozilla/rust/pull/14196) in the form of
  `find_with_or_insert_with`
- The shootout-mandelbrot benchmark [has seen a 2x performance
  increase](https://github.com/mozilla/rust/pull/14203).
- The test runner filter [now takes a
  regex](https://github.com/mozilla/rust/pull/13948) rather than a full path.
- The error reporter for unresolved name [hsa been vastly
  improved](https://github.com/mozilla/rust/pull/14086) to also look for
  fields and methods on types, and methods on traits, rather than just local
  variables.
- Blocks [are now allowed](https://github.com/mozilla/rust/pull/14183) in
  constant expressions.
- Windows process spawning and environment variable fetching [is now
  Unicode-aware](https://github.com/mozilla/rust/pull/14075).
- Fallback functions [have been
  added](https://github.com/mozilla/rust/pull/13932) for when a given feature
  isn't available on Windows XP. `rustc` won't run on XP due to LLVM, but
  binaries produced by it should.

## New Contributors

- Alan Williams
- Cameron Zwarich
- Derek Chiang (Enchi Jiang)
- Hanno Braun
- J.C. Moyer
- Piotr Jawniak
- Zooko Wilcox-O'Hearn

# RFCs

- [Multiple trait implementations for a single impl
  block](https://github.com/rust-lang/rfcs/pull/76)
- [Unboxed closures](https://github.com/rust-lang/rfcs/pull/77)
- [Extending safe mutability](https://github.com/rust-lang/rfcs/pull/78)
- [Structs with unspecified layout](https://github.com/rust-lang/rfcs/pull/79)

# Community Updates

The mutpocalypse is nigh. There is almost a full-page of reddit links to
self-posts and RFCs in response to Niko's [Focusing on
Ownership](http://smallcultfollowing.com/babysteps/blog/2014/05/13/focusing-on-ownership/)
post. The situation is pretty ridiculous. I'm going to pretend it doesn't
exist, though.  Peruse reddit if you feel up to reading the dozens of
suggestions.

- [Weekly
  meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-05-13)
- GitHub's Atom editor [has Racer
  support](https://github.com/edubkendo/atom-racer).
- And, [a detailed update on
  Racer](http://phildawes.net/blog/2014/05/10/racer/).
- I've started my internship at Mozilla (working on Rust, of course), and I've
  started updating [my development log](http://rustlog.octayn.net/) again.
- [Seattle meetup
  interest?](https://mail.mozilla.org/pipermail/rust-dev/2014-May/009825.html)
- bindgen [now has a macro](https://github.com/crabtw/rust-bindgen/pull/81)
  which can generate bindings at parse-time.
- [Rust for C++ programmers: part 6, Rc, Gc,
  *](http://featherweightmusings.blogspot.co.nz/2014/05/rust-for-c-programmers-part-6-rc-gc-and.html)

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test cases for the Rust language.

In the last two weeks, we landed 79 PRs.

## Notable additions

- Josh Matthews implemented a [new rooting strategy](https://github.com/mozilla/servo/pull/2101) for JavaScript objects
- Aydin Kim fixed the [Android build](https://github.com/mozilla/servo/pull/2367)
- Glenn Watson fixed a bug with the new rooting strategy [during page interactions](https://github.com/mozilla/servo/pull/2340)
- Brendan Zabarauskascleaned up the way that we [print debug info](https://github.com/mozilla/servo/pull/2336)
- Bryan Bell added [dotted border support](https://github.com/mozilla/servo/pull/2438)
- Cameron Zwarich switched JSRef to more efficiently enforce its [contravariant lifetime](https://github.com/mozilla/servo/pull/2435)
- Guro Bokum converted many of our [RefCells to Cells](https://github.com/mozilla/servo/pull/2390)
- Matt Murphy [converted](https://github.com/mozilla/servo/pull/2317) many of our uses of `~[]` to `Vec`

## New Contributors

- Brendan Zabarauskas (bjz)
- Bryan Bell (bjwbell)
- Cameron Zwarich (zwarich)
- Glenn Watson (gw)
- Guro Bokum (jiojiajiu)
- Matt Murphy (murphm8)

## Meetings and Notes

In the meeting [two weeks
ago](https://github.com/mozilla/servo/wiki/Meeting-2014-05-05), we introduced
a new team member, Cameron Zwarich (zwarich). He is joining us from Apple, and
will be working on cross-language inlining in support of SpiderMonkey, among
other things. In last week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-05-13), we
discussed 32-bit support for Servo, the design of the HTML parser, and
potentially replacing our Azure+Skia graphics stack.
