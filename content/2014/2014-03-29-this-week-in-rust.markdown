Title: This Week in Rust 43
Date: 2014-03-29 21:56
Category: This Week in Rust


Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

The 0.10 release will likely be this week. 3 months from 0.9 is April 3.

<!-- more -->

# What's cooking on master?

63 pull requests were merged this week.

## Breaking Changes

- The `sync` crate [has seen some significant
  cleanup](https://github.com/mozilla/rust/pull/12900). In particular,
  `RWArc<T>` and `MutexArc<T>` have been removed. `Mutex<T>` and `RWLock<T>`
  have been introduced, and can be used as `Arc<Mutex<T>>` and
  `Arc<RWLock<T>>` to replace `RWArc` and `MutexArc`. There are some other
  minor changes, see the pull request.
- Taking a reference to a static whose type contains an `Unsafe<T>` somewhere
  [is now forbidden](https://github.com/mozilla/rust/pull/13083).
- `Extendable` and `FromIterator` [now take their argument
  by-value](https://github.com/mozilla/rust/pull/13039).
- The crate map [has been
  removed](https://github.com/mozilla/rust/pull/13117), and with it a
  significant amount of complexity. As a consequence of its removal,
  `green::start` now takes the event loop it should use as an argument rather
  than looking in the crate map. [The documentation has an
  example](http://static.rust-lang.org/doc/master/green/index.html#starting-with-libgreen)
- The syntax for bounds on generic paths [has
  changed](https://github.com/mozilla/rust/pull/13079) from
  `Foo:Bound<Params>` to `Foo<Params>:Bound`.
- The default `Send` bound on trait objects [has been
  removed](https://github.com/mozilla/rust/pull/13050).
- The `serialize` infrastructure [now supports error reporting via
  `Result`](https://github.com/mozilla/rust/pull/13107).
- `Pod` [has been renamed](https://github.com/mozilla/rust/pull/13160) to
  `Copy`.
- The attribute syntax [has
  changed](https://github.com/mozilla/rust/pull/13162). Rather than saying
  `#[crate_type = "bin"];` (with a significant semicolon), you say
  `#![crate_type = "bin"]` (no semicolon).
- `collections::List` [has been
  removed](https://github.com/mozilla/rust/pull/13183). Use a vector, or
  `collections::DList` if you really want a linked list. Persistent data
  structures will live in a different crate than `collections`.

## Other Changes

- A `fill` method [has been added](https://github.com/mozilla/rust/pull/13049)
  to `Reader`, for reading an exact amount of bytes or erroring.
- As
  [discussed](https://mail.mozilla.org/pipermail/rust-dev/2014-January/007924.html)
  on the mailing list some months ago, [synchronous, bounded
  channels](https://github.com/mozilla/rust/pull/12991) have been added.
- `Share`'s documentation [has been
  expanded](https://github.com/mozilla/rust/pull/13070).
- A straggler from the doc sprint, [a few hundred lines of
  documentation](https://github.com/mozilla/rust/pull/13135) have been added
  to `std`.
- Some handy methods [have been
  added](https://github.com/mozilla/rust/pull/12780) to `Json` to make
  searching and selecting elements from JSON objects easier.
- There's a [new warn-by-default
  lint](https://github.com/mozilla/rust/pull/13108) for deriving traits on
  types which contain raw pointers.
- Many confusing lifetime-related ICE's [have been
  fixed](https://github.com/mozilla/rust/pull/13157). This is good progress
  towards closing [the notorious issue
  5121](https://github.com/mozilla/rust/issues/5121).
- We can now [emit debuginfo for
  `static`s](https://github.com/mozilla/rust/pull/13143). Yay debuginfo!

## New Contributors

- Davis Silverman
- Noam Gagliardi Rabinovich
- Sean McArthur

# Weekly Meeting

The [meeting this
week](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-03-25)
discussed various things: attribute parsing in macros, opt-in built-in traits,
SIMD, allowing square brackets in macro invocation, allowing bounds on type
parameters in structs, private fields by default, the fate of
`collections::List`, and bounds on trait paths.

# RFCs

- [Smaller refcounts](https://github.com/rust-lang/rfcs/pull/23)
- [Deserializing to a stream of tagged
  values](https://github.com/rust-lang/rfcs/pull/22)
- [Allow bounds on all type
  parameters](https://github.com/rust-lang/rfcs/pull/20)

# Project Updates

- [rust-highlight](https://github.com/KokaKiwi/rust-highlight) can be used to
  highlight Rust code in LaTeX, HTML, and JSON.
- [rust-tabular](https://github.com/arjantop/rust-tabular), a library for
  handling delimiter-separated values (commas, tabs, or any other character),
  as well as fixed columns of fixed width.
- [rust-csv](https://github.com/BurntSushi/rust-csv), a CSV library which
  implements `Encoder` and `Decoder` for `serialize` support.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

In the last two weeks, we landed 33 PRs.

## Notable additions

- Lars Bergstrom landed a Rust upgrade, bringing us to late February in
  [#1934](https://github.com/mozilla/servo/pull/1934)
- ms2ger cleaned up some sketchy use of unsafe native pointers, wrapping them
  in a `JS<T>` in [#1915](https://github.com/mozilla/servo/pull/1915)
- Matt Brubeck named the WorkQueue tasks so that we'll know where failure is
  coming from during parallel layout in
  [#1977](https://github.com/mozilla/servo/pull/1977)

## New contributors

- Matt Brubeck (mbrubeck)

## Meetings and Notes

Patrick Walton authored and combined the final set of changes required to get
Servo [passing ACID2](https://twitter.com/pcwalton/status/449299846873108480)!

Matthew Brubeck has joined the Servo team full-time, coming over from the
Firefox Metro project.

In the meeting [two weeks
ago](https://github.com/mozilla/servo/wiki/Meeting-2014-03-17), we discussed
the Rust upgrade, z-index support, and ACID2. In the most recent
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-03-24), we
provided an update on ACID2 and the status of Servo on Android.

# Other Announcements

- [Official Installers and
  Nightlies](https://mail.mozilla.org/pipermail/rust-dev/2014-March/009223.html)
