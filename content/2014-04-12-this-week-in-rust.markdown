Title: This Week in Rust 45
Date: 2014-04-13 23:06
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

64 pull requests were merged this week.

## Breaking Changes

- `BenchHarness` [has been
  renamed](https://github.com/mozilla/rust/pull/13236) to `Bencher`
- The `push_str` and `push_char` methods on `~str` [have been
  removed](https://github.com/mozilla/rust/pull/13440), and a `StrBuf` type
  added, as an analog to `Vec`.
- Duplicate moves from the variables a `proc` captures [are no longer
  allowed](https://github.com/mozilla/rust/pull/13413).
- `std::libc` [has been extracted](https://github.com/mozilla/rust/pull/13315)
  into its own crate.
- Various bugs in resolve [have been
  fixed](https://github.com/mozilla/rust/pull/13409). The fixes seem
  relatively obscure, but they're well documented if your code breaks.
-  The functions in `flate` now [return Option instead of
   failing](https://github.com/mozilla/rust/pull/13389).

## Other Changes

- `TotalEq` and `TotalOrd` [now
  document](https://github.com/mozilla/rust/pull/13358) exactly what the types
  implementing them must guarantee.
- Some bugs with debuginfo [have been
  fixed](https://github.com/mozilla/rust/pull/13441). In particular, the
  annoying link failure with debuginfo has been fixed.
- Relocation model [is now
  configurable](https://github.com/mozilla/rust/pull/13340) with `-C
  relocation-model`.

Additionally, a lot of cleanup happened. Not much of it sticks out particularly.

## New Contributors

- Boris Egorov
- Jim Radford
- Joseph Crail
- JustAPerson
- Kasey Carrothers
- Kevin Butler
- Manish Goregaokar
- Tobba
- free-Runner

# Weekly Meeting

The weekly meeting was cancelled due to the videoconference system being down
for mitigating the Heartbleed vulnerability, as well as some team members
travelling or otherwise unavailable.

# RFCs

Some new RFCs:

- [Extend nullable pointer optimization to library
  types](https://github.com/rust-lang/rfcs/pull/36)
- [Extended method lookup](https://github.com/rust-lang/rfcs/pull/37)
- [Inherit use](https://github.com/rust-lang/rfcs/pull/38)
- [Allocator trait](https://github.com/rust-lang/rfcs/pull/39)
- [Make libstd a facade](https://github.com/rust-lang/rfcs/pull/40)
- [`Invalid` trait for space optimization of
  enums](https://github.com/rust-lang/rfcs/pull/41)
- [Add a regexp crate to the Rust
  distribution](https://github.com/rust-lang/rfcs/pull/42)

# Project Updates

- [Acronymy](http://www.reddit.com/r/rust/comments/22y6oy/acronymy_a_web_app_written_in_rust/)
has been released. This is a web application (in Rust!) for defining words as
acronyms. It's pretty fun.
- [bitmap](https://github.com/cmr/bitmap-rs) has been released
- [regexp](https://github.com/BurntSushi/regexp/) is a pure-Rust
  implementation of RE2, with wonderful docs and support for statically
  compiling regular expressions.
- [rust-empty](http://www.reddit.com/r/rust/comments/22uirj/rustempty_02_released/)
  has been updated to 0.2.
- [inotify-rs](https://github.com/hannobraun/inotify-rs) has been released,
  bindings to inotify.
- [An unlambda
  interpreter](https://github.com/bwo/unlambda/blob/master/unlambda.rs)
- [RusticMineSweeper](https://github.com/aochagavia/RusticMineSweeper), a
  minesweeper clone.
- [rust-mustache](https://github.com/erickt/rust-mustache/tree/v0.3.0) has
  been updated to 0.3.0.
- [sodiumoxide](https://github.com/dnaq/sodiumoxide), the libsodium bindings,
  have been updated for 0.10.

# Community

- On April 17, there will be an [Introduction to
  Rust](http://www.meetup.com/nyccpp/events/168545012/) by Clark Gaebel in new
  York City, during a C++ meetup.
- [Bay Area
  Rust](https://mail.mozilla.org/pipermail/rust-dev/2014-April/009490.html)'s
  plans for May have been announced.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test cases for the Rust language.

In the last week, we landed 30 PRs.

## Notable additions

- ms2ger cleaned up all of the trailing whitespace that had been nagging down
  our Critic reviews in [#2055](https://github.com/mozilla/servo/pull/2055)
- Jacob Parker added a reftest for `setAttribute`-based restyling in
  [#2062](https://github.com/mozilla/servo/pull/2062)
- Sankha Narayan Guria removed XRay from the script codegen in
  [#2050](https://github.com/mozilla/servo/pull/2050)
- Peiyong Lin moved `namespaceURI` to the `Element` type in
  [#2063](https://github.com/mozilla/servo/pull/2063) and removed all
  remaining `@` boxes in [#2085](https://github.com/mozilla/servo/pull/2085)
- Matt Brubeck fixed bugs related clicking on links in
  [#2068](https://github.com/mozilla/servo/pull/2068) and
  [#2084](https://github.com/mozilla/servo/pull/2084) and
  [#2080](https://github.com/mozilla/servo/pull/2080)
- Hyun June Kim added support for pseudo-elements attached to inline elements
  in [#2071](https://github.com/mozilla/servo/pull/2071)
- Manish Goregaokar cleaned up a whole bunch of warnings left after our last
  Rust update in [#2045](https://github.com/mozilla/servo/pull/2045)
- Lars Bergstrom got Android support working in Servo master in
  [#2070](https://github.com/mozilla/servo/pull/2070)

## New contributors

- Jacob Parker (j3parker)

## Meetings and Notes

In this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-04-07), we went
over our [Q2 roadmap](https://github.com/mozilla/servo/wiki/Roadmap), status
of an Android buildbot, testing, and the ever-present issue of improving our
build system.
