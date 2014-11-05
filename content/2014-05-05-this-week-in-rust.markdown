Title: This Week in Rust 47
Date: 2014-05-05 14:26
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

45 pull requests were merged this week.

## Breaking Changes

- `~"foo"` and `&"bar"` literals [have been removed from the
  language](https://github.com/mozilla/rust/pull/13877). The second is a noop
  and the first is replaced by `"foo".to_owned()`.
- The various `rev_iter` methods [have been
  removed](https://github.com/mozilla/rust/pull/13648) in favor of making more
  iterators implement `DoubleEndedIterator`. See the commit messages for more
  details.

## Other Changes

- The first part of [opt-in built-in
  traits](https://github.com/rust-lang/rfcs/blob/master/active/0003-opt-in-builtin-traits.md)
  [has landed](https://github.com/mozilla/rust/pull/13868). The built-in
  traits can now be explicitly implemented and derived.
- There is [now a lint](https://github.com/mozilla/rust/pull/13579) for
  negating `uint` values.
- There is [now a `debug_assert`
  macro](https://github.com/mozilla/rust/pull/13789) for assertions which can
  be compiled out.
- There is [now a `bitflags`
  macro](https://github.com/mozilla/rust/pull/13072) for generating a nice
  bitflag API.
- `serialize` [now has a streaming JSON
  parser](https://github.com/mozilla/rust/pull/12740).
- We now use `-ffunction-sections`, `-fdata-sections`, and `--gc-sections`,
  [for a 67% size reduction](https://github.com/mozilla/rust/pull/13833) of
  hello world on Linux.

## New Contributors

- Alexandre Gagnon
- Ali Smesseim
- Emanuel Rylke
- James Laverack
- Justin Noah
- Michael Pratt
- Nicolas Silva
- Noam Yorav-Raphael
- Phil Ruffwind
- Wendell Smith
- m-r-r

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-04-29)
discussed many RFCs.

# RFCs

- [Allow some intrinsics in
  statics](https://github.com/rust-lang/rfcs/pull/54)
- [Low level features](https://github.com/rust-lang/rfcs/pull/55)
- [Static values as generic
  parameters](https://github.com/rust-lang/rfcs/pull/56)
- [Text/Unicode oriented streams](https://github.com/rust-lang/rfcs/pull/57)
- [Rename `&mut` to `&only`, allow `&only` borrowing of non-mut
  slots](https://github.com/rust-lang/rfcs/pull/58)
- [Rename `StrBuf` to `Str` and remove `&str` from the
  language](https://github.com/rust-lang/rfcs/pull/60)
- [Enforce module directory structure more
  strictly](https://github.com/rust-lang/rfcs/pull/63)
- [Longer numeric types (`u8` to `uint8`
  etc)](https://github.com/rust-lang/rfcs/pull/64)
- [Change the syntax of struct literals to use
  `=`](https://github.com/rust-lang/rfcs/pull/65)
- [Better temporary lifetimes](https://github.com/rust-lang/rfcs/pull/66)
- [User friendly input macros](https://github.com/rust-lang/rfcs/pull/67)-

# Community Updates

- [Syntax extensions and regular expressions for
  Rust](http://blog.burntsushi.net/rust-regex-syntax-extensions)
- [What to expect as a C#
  user](http://www.reddit.com/r/rust/comments/244oog/what_to_expect_as_a_c_user/)
- [Ludum Dare 29 Entry: Sea Snake
  Escape](http://www.reddit.com/r/rust/comments/244zrj/ludum_dare_29_in_rust_sea_snake_escape/)
- [LD29 Entry:
  Repercussion](http://www.reddit.com/r/rust/comments/247744/repercussion_ludum_dare_29/)
- [LD29 Entry: Shadows
  Below](http://www.reddit.com/r/rust/comments/249clu/ld29_jam_entry_shadows_below/)
- [Bitfields in
  Rust](http://www.reddit.com/r/rust/comments/244yz6/bitfields_in_rust/)
- [Rust for C++ programmers part
  4](http://featherweightmusings.blogspot.co.nz/2014/04/rust-for-c-programmers-part-4-unique.html)
- [New moderation
  policy](https://mail.mozilla.org/pipermail/rust-dev/2014-April/009704.html)
- [rust-empty 0.3](https://github.com/bvssvni/rust-empty/releases/tag/0.3) has
  been released.
- [RMX, a Rust instrumentation
  tool](https://github.com/Fiedzia/rust-instrumentation)
- [A Game Boy Advance game, written in
  Rust](https://github.com/exoticorn/gba-rust)
- [An interactive compiler](http://rust.godbolt.org/), which lets you
  disassemble Rust code nicely
- [ClearCrypt: A new transport encryption
  library](https://mail.mozilla.org/pipermail/rust-dev/2014-May/009761.html)

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test cases for the Rust language.

In the last week, we landed 28 PRs.

## Notable additions

- Lars Bergstrom landed the Rust upgrade, bringing Servo in line with April
  10th Rust in [#2238](https://github.com/mozilla/servo/pull/2238)
- Patrick Walton re-enabled parallel layout in
  [#2174](https://github.com/mozilla/servo/pull/2174) and added parallel
  display list building in [#2235](https://github.com/mozilla/servo/pull/2235)
- jgraham updated the Web Platform Tests integration to support passing some
  arguments via. env variables in
  [#2245](https://github.com/mozilla/servo/pull/2245)
- ms2ger rewrote the handling of optional arguments in
  `getJSToNativeConversionTemplate` in
  [#2244](https://github.com/mozilla/servo/pull/2244)
- Matt Brubeck cleaned up another dynamic borrow failure leading to pipeline
  problems in [#2252](https://github.com/mozilla/servo/pull/2252)
- Jack Moffitt cleaned up some of our Makefile and autoconf files in
  [#2232](https://github.com/mozilla/servo/pull/2232)
- Gulshan Singh fixed `TimeStamp` to return the actual time in
  [#2275](https://github.com/mozilla/servo/pull/2275)
- Mike Blumenkrantz added the start of embedding support in
  [#2257](https://github.com/mozilla/servo/pull/2257)
- Manish Goregaokar implemented the webidl and basic implementation of XHR in
  [#2292](https://github.com/mozilla/servo/pull/2292)

## New Contributors

- Gulshan Singh (gsingh93)

## Meetings and Notes

In this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-04-28), we
introduced three new team members. Brendan (bjz) is a long-time Rust developer
here on a Mozilla internship. Glenn Watson (gw) is a full-time member of the
Servo team, with an initial focus on platform features. And Manish Goregaokar
will be participating in Google Summer of Code, working on XMLHttpRequest. We
covered the JS rooting changes, timing of the next Rust upgrade, the status of
Web Platform Tests, and the use of prebuilt Rust compiler binaries.
