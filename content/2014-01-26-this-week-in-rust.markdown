Title: This Week in Rust 34
Date: 2014-01-26 19:00
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

<!-- more -->

# What's cooking in master?

74 pull requests were merged this week.

## Breaking Changes

- Struct fields [now inherit the privacy of the struct
itself](https://github.com/mozilla/rust/pull/11777).
- A bug in instantiability checking (determining whether a value of a given
type could ever be constructed) [has been
fixed](https://github.com/mozilla/rust/pull/11661).
- `Zero` and `One` [have been tightened
up](https://github.com/mozilla/rust/pull/11664), and fewer types implement
them now.
- [A lot of things have changed](https://github.com/mozilla/rust/pull/11129)
regarding functions that fail or return Options. Almost everything that used
to fail now returns an Option instead.
- Many incorrect usages of types from `std::libc` [now use Rust
types](https://github.com/mozilla/rust/pull/10943). This might introduce an
extra `as` or few in code using these (relatively obscure) interfaces.
- `invert` on `DoubleEndedIterator` [has been
renamed](https://github.com/mozilla/rust/pull/11686) to `rev`.
- The numeric constants [have been
uppercased](https://github.com/mozilla/rust/pull/11790).
- Some privacy in std and extra [has been shuffled
around](https://github.com/mozilla/rust/pull/11808).


## Other Changes

- Most macros are [no longer injected as a hard-coded
string](https://github.com/mozilla/rust/pull/11774) and in fact live in
libstd.
- Using `-Z no-landing-pads` [actually disables landing pad
generation](https://github.com/mozilla/rust/pull/11653). It was accidentally
broken.
- The FFI documentation [now includes a section on
callbacks](https://github.com/mozilla/rust/pull/11486).
- A lint for unnecessary parens on control flow conditions etc [has been
added](https://github.com/mozilla/rust/pull/11663).
- Support for ARM's thumb instructions [has been
added](https://github.com/mozilla/rust/pull/11700).
- libnative [now implements
timers](https://github.com/mozilla/rust/pull/11294).
- There is now [exponential notation for float
formatting](https://github.com/mozilla/rust/pull/11611).
- `Vec<T>` [has been added](https://github.com/mozilla/rust/pull/11682), which
is intended to eventually replace `~[T]`.
- Borrow checker errors [now have better
spans](https://github.com/mozilla/rust/pull/11718) and should be more
understandable.
- It is now possible to [opt out of using
rpaths](https://github.com/mozilla/rust/pull/11744).
- libnative [now implements
`get_host_addresses`](https://github.com/mozilla/rust/pull/11732).
- A new synchronization primitive, Barrier, [has been
added](https://github.com/mozilla/rust/pull/11725).

## New Contributors

- Andre Arko
- Aydin Kim
- Ben Harris
- Ben Noordhuis
- Chris Wong
- Daniel MacDougall
- Hong Chulju
- Jake Greenfield
- Matthias Einwag
- Philippe Delrieu
- Salem Talha
- Sean Chalmers
- Trent Ogren
- Virgile Andreani
- comex

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-01-21)
discussed a bunch of open pull requests, but nothing too groudbreaking.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

This week, we landed 19 PRs and had a Servo workweek in the Mozilla San
Francisco office from Tuesday through Friday (Monday was a US federal
holiday).


## Notable additions

- Bruno Abinader implemented Document::createHTMLDocument in
[#1523](https://github.com/mozilla/servo/pull/1523).
- Patrick Walton implemented parallel layout for two of our three layout
passes in [#1493](https://github.com/mozilla/servo/pull/1493).
- Patrick Kim fixed computation of image box sizes in
[#1520](https://github.com/mozilla/servo/pull/1520) and text-decoration in
[#1506](https://github.com/mozilla/servo/pull/1506).
- Deokjin Kim landed the `white-space:pre` property in
[#1507](https://github.com/mozilla/servo/pull/1507).
- Aydin Kim and Lars Bergstrom landed fixes for Servo on Android.
- Teodor Szente fixed up a type signature in
[#1541](https://github.com/mozilla/servo/pull/1541).
- Lars Bergstrom changed ref tests so that they now emit a PNG showing the
image-diff between the baseline and incorrect rendering in
[#1544](https://github.com/mozilla/servo/pull/1544).

## New contributors

- Teodor Szente

## Meetings

There was no separate Servo meeting, but during the workweek we discussed
details in our designs for
[layers](https://etherpad.mozilla.org/Servo-workweek-layers), [generated
content](https://etherpad.mozilla.org/Servo-workweek-generated-content),
[tables](https://etherpad.mozilla.org/Servo-workweek-tables),
[HiDPI/pixels](https://etherpad.mozilla.org/Servo-workweek-pixels), and
[DisplayList
creation](https://etherpad.mozilla.org/Servo-workweek-displaylist).

# Announcements, etc

- [Rust contribution
ideas](https://mail.mozilla.org/pipermail/rust-dev/2014-January/008214.html)
- [RustAlgebloat](https://github.com/SiegeLord/RustAlgebloat), a linear
algebra library
- [Table of Potential Closure
Types](http://glaebhoerl.tumblr.com/rust_closure_types)
- [rust-ci
updates](https://mail.mozilla.org/pipermail/rust-dev/2014-January/008088.html),
categorization and documentation uploading
- [Channel API
proposal](http://thread.gmane.org/gmane.comp.lang.rust.devel/7848)

