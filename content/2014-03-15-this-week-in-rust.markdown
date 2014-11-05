Title: This Week in Rust 41
Date: 2014-03-15 23:33
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

- Some more string methods [now return
Option](https://github.com/mozilla/rust/pull/12797).
- Matching on `~str` [has been
removed](https://github.com/mozilla/rust/pull/12756).
- IO iterators [now yield
IoResult](https://github.com/mozilla/rust/pull/12414).
- The channel types and constructor [have been
renamed](https://github.com/mozilla/rust/pull/12815). `Chan` is now `Sender`,
`Port` is now `Receiver`, and `Chan::new` is now `std::comm::channel`.
- `std::cmp::{min, max}` [now require
TotalOrd](https://github.com/mozilla/rust/pull/12869). For floats, use
`a.min(b)`.
- The handling of `cfg(not(a, b))` [has changed
slightly](https://github.com/mozilla/rust/pull/12893).
- `libextra` [has finally been
purged](https://github.com/mozilla/rust/pull/12896).

## Other Changes

- Another great PR from the illustrious ktt3ja, the compiler [will now give
suggestions](https://github.com/mozilla/rust/pull/12238) on how to fix
lifetime woes. This is an awesome usability improvement.
- Implementations of `Deref` and `DerefMut` [are now
considered](https://github.com/mozilla/rust/pull/12610) for automatic
dereferencing.
- Partial type hints [are now
implemented](https://github.com/mozilla/rust/pull/12764), but not at the item
level (in function returns etc). The eventual goal is to have
`some_iter.collect::<Vec<_>>()` be possible.
- There is a [new
guide](http://static.rust-lang.org/doc/master/guide-unsafe.html) for "low level and unsafe code".
[PR](https://github.com/mozilla/rust/pull/12887).
- There is now a lint for [uses of
`~[T]`](https://github.com/mozilla/rust/pull/12861). It's very verbose. The
replacement for `~[T]`, as recommended by the lint, is `std::vec_ng::Vec<T>`.
It implements almost everything `~[T]` does.
- `HashMap` [has been
reimplemented](https://github.com/mozilla/rust/pull/12081) to use [Robin Hood
hashing](http://codecapsule.com/2013/11/11/robin-hood-hashing/). It's now much
more faster, and uses less space.
- Support for basic backtracing [has been
reimplemented](https://github.com/mozilla/rust/pull/12602).
- `char` [now has simple case
folding](https://github.com/mozilla/rust/pull/12561). That is, basic
locale-ignorant case conversion.
- Inline assembly [now supports the `+`
modifier](https://github.com/mozilla/rust/pull/12798).
## New Contributors

- Adolfo Ochagavía
- Clark Gaebel
- Peter Marheine
- Piotr Czarnecki
- Piotr Zolnierek
- Robert Gawdzik

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-03-11)
discussed the new hashmap, destructuring the `self` argument, partial type
hints, coercion of returned values, as well as the requirements of unsafe
pointers and the continued existence of `*mut`.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

In the last two weeks, we landed 56 PRs.

## Notable additions

- Patrick Walton made fixes to ACID in
[#1905](https://github.com/mozilla/servo/pull/1905)
- Along with about 20 other PRs, ms2ger brought WindowTimers up to date with
the spec in [#1890](https://github.com/mozilla/servo/pull/1890)
- Pradeep Kumar implemented `position:absolute` in
[#1681](https://github.com/mozilla/servo/pull/1681) and fixed up relative
positioning in [#1808](https://github.com/mozilla/servo/pull/1808)
- Bill Yang fixed up our instructions for installation on Ubuntu Linux in
[#1881](https://github.com/mozilla/servo/pull/1881)
- Bruno Abinader fixed up HTMLCollection in
[#1838](https://github.com/mozilla/servo/pull/1838)
- Isabelle Carter ensured that children of `position:fixed` items make it into
the correct DisplayList in [#1832](https://github.com/mozilla/servo/pull/1832)
- Manish Goregaokar made `getElementsByName` return a `NodeList` in
[#1756](https://github.com/mozilla/servo/pull/1756)
- Keegan McAllister cleaned up our reftest harness and made them test both the
CPU and GPU rendering paths in
[#1804](https://github.com/mozilla/servo/pull/1804)

## New contributors

- Bill Yang (analyst74)
- Manish Goregaokar (manishearth)

## Meetings and Notes

At this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-03-10), we
discussed HTML parsing, embedding, writing modes, the ACID2 burndown, and the
Rust upgrade.

# RFCs

[We have a new RFC
process](https://github.com/rust-lang/rfcs/blob/master/active/0001-rfc-process.md)!
Every week I'll list the new RFCs and RFCs that have been accepted.
Contributing to the discussions around these RFCs is the easiest way to drive
the design of Rust.

As an aside, I will no longer include any RFCs in the "Announcements" section
that are not sent to the RFC repository.

- [Private Fields by Default](https://github.com/rust-lang/rfcs/pull/1)
- [Rearchitecting the attribute-usage lint](https://github.com/rust-lang/rfcs/pull/3)
- [Unified Function Call Syntax](https://github.com/rust-lang/rfcs/pull/4)
- [Virtual Structs](https://github.com/rust-lang/rfcs/pull/5)
- [Add OsUnknown as a target operating system](https://github.com/rust-lang/rfcs/pull/7)
- [Redesign Compiler Intrinsics](https://github.com/rust-lang/rfcs/pull/8)
- ["Fat Objects" for DSTs](https://github.com/rust-lang/rfcs/pull/9), an
alternative to "Virtual Structs".

# Project Updates

[rust-bencode](https://github.com/arjantop/rust-bencode), a pure Rust
implementation of Bencode encoding.
[Bencode](http://en.wikipedia.org/wiki/Bencode) is used primarily in the
BitTorrent protocol. This implementation leverages the `serialize` crate for
automatic Encodable/Decodable implementation as well as custom, more flexible
FromBencode/ToBencode traits.

[Lazily initialized statics v0.2](https://gist.github.com/Kimundi/8782487).
This makes safe use of statics for types which require runtime initialization
almost trivial. It utilizes the recent overloadable deref to do optional
initialization before returning the reference to the wrapped type. It's an
example that the right amount of abstractions, but syntactic (macros) and
semantic (operator overloading) can reduce boilerplate for a more pleasant
experience.

[Racer](https://github.com/phildawes/racer) - code completion for Rust. This
project is in its early stages, but can already do a fair bit of useful
completion. It is not using the parser (libsyntax) or compiler (librustc), so
it will likely always be inaccurate in some respect. Nevertheless, this may
prove to be an incredibly useful tool.

[QuickCheck for Rust](https://github.com/BurntSushi/quickcheck). This has some
seriously great docs. QuickCheck, originally from Haskell, is a way of doing
property-based testing. You describe certain properties your code should have
and it generates random inputs, trying to find inputs that violate those
properties. This is a modern, featureful port of QuickCheck and hopefully gets
even more useful as time goes on.

The [coreutils port](https://github.com/uutils/coreutils) is still alive and
making progress. 32 out of 119 programs are implemented. This is a great
project to contribute to if you want a relatively self-contained,
goal-oriented task. It's great practice both for learning Unix* (some of these
utilities are quirky or not-frequently-used) and Rust.

\* Well, specifically GNU, which as we all know, [ain't
Unix](https://en.wikipedia.org/wiki/GNU).

# Other Announcements

- [Under The Hood Of Mozilla's New Multi-Core Browser And The Open Source
Language That Powers
It](http://www.fastcolabs.com/3027664/under-the-hood-of-mozillas-new-multi-core-browser-and-the-open-source-language-that-powers-i)
- [Bay Area Rust, March 2014
Meetup](http://www.reddit.com/r/rust/comments/20ct5c/march_meetup_live_stream_link_oss_parallel_layout/)
- [Writing an OS in Rust in tiny
steps](http://www.reddit.com/r/rust/comments/20aj03/writing_an_os_in_rust_in_tiny_steps_steps_15/)
- ["Virtual fn" is a bad
idea](http://thread.gmane.org/gmane.comp.lang.rust.devel/8878)
