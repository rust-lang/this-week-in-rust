Title: This Week in Rust 25
Date: 2013-11-30 15:56
Category: This Week in Rust

Welcome to another issue of *This Week in Rust*, a weekly newsletter
summarizing Rust's progress and community activity. As always, if you have
something you'd like to be featured, just [send me an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion).

Last week was very slow, but this week more than makes up for it I think!
Static linking landed, and there's been some nice cleanup of error messages
and various APIs.

<!-- more -->

# What's cooking on master?

59 PRs were merged this week.

## Breaking Changes

- Names of methods creating iterators have [changed
  drastically](https://github.com/mozilla/rust/pull/10622) to remove the
  `_iter` suffix. This reflects that iterators are a primary focus of APIs.
  The PR description has all of the changes summarized.
- We now have [static linking
  support](https://github.com/mozilla/rust/pull/10528)! It does, however,
  involve some changes with how the `link` attribute works. The error messages
  should guide you to the fixes.
- In preparation for the placement new changes, the `Path` constructor has
  been [renamed from `new` to
  `init`](https://github.com/mozilla/rust/pull/10697),.
- Some overly-permissive borrow checking for `&mut &mut` [has been
  fixed](https://github.com/mozilla/rust/pull/10519). This is fairly obscure,
  most code shouldn't have hit it.
- The parser is [more strict](https://github.com/mozilla/rust/pull/10642)
  about what it accepts as a doc comment. Now, only `///` and `/**`  are
  counted as introducing doc comments (previously, `////` and `/***` would
  also introduce a doc comment).
- `std::{uint, int}::{min, max}` [have been
  removed](https://github.com/mozilla/rust/pull/10719). They were identical to
  the functions in `std::cmp`, so use those instead.
- `extra::json` [has been rid of @
  boxes](https://github.com/mozilla/rust/pull/10727), and now uses idiomatic
  constructor names.
- The `type_id` intrinsic [now uses a language item as its return
  value](https://github.com/mozilla/rust/pull/10722).
- Some [cleanup](https://github.com/mozilla/rust/pull/10662) has been done to
  `std::rt::thread`, which is an interface to native threads (rather than
  tasks).
- `do` blocks are [no longer
  allowed](https://github.com/mozilla/rust/pull/10581) in non-`proc` contexts.
  This means that `do` can not be used with a function whose last argument is
  not a `proc`. A fairly large [thread on the mailing
  list](https://mail.mozilla.org/pipermail/rust-dev/2013-November/006999.html)
  is ongoing about this change.
- `LittleLock` now [uses RAII](https://github.com/mozilla/rust/pull/10660).
- C-like enums are [now represented as an
  integer](https://github.com/mozilla/rust/pull/10652) rather than a struct,
  at the LLVM level. This affects ABI.
- Linked failure [has been
  removed](https://github.com/mozilla/rust/pull/10603) from the runtime.
- `extra::term` [no longer uses `@mut
  Writer`](https://github.com/mozilla/rust/pull/10637), instead taking the
  writer to use by value.

## Other changes

- `RefCell<T>`, previously known as `Mut<T>`, [has finally
  landed](https://github.com/mozilla/rust/pull/10514), for all your dynamic
  borrow checking needs.
- A lint for unknown attributes [has finally been
  added](https://github.com/mozilla/rust/pull/10316). The compiler will now
  warn when it sees an attribute it doesn't recognize.
- A lock-free [Chase-Lev
  deque](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.170.1097&rep=rep1&type=pdf)
  has [been added to the runtime](https://github.com/mozilla/rust/pull/10678).
- The shootout-spectralnorm benchmark [has been
  resurrected](https://github.com/mozilla/rust/pull/10704).
- Unknown feature gates [are now
  linted](https://github.com/mozilla/rust/pull/10680), rather than being a
  hard failure.
- The AST is [now frozen](https://github.com/mozilla/rust/pull/10693)!
- `GenericPort` [now has an
  iterator](https://github.com/mozilla/rust/pull/10688) over the messages it
  receives.
- `NodeId`, `CrateNum`, `Name`, and `Mrk` (types in libsyntax) [have been
  shrunk to 32 bits](https://github.com/mozilla/rust/pull/10670).
- The restriction on macros expanding to a single item [has been
  lifted](https://github.com/mozilla/rust/pull/10649). This obviates the need
  for macros hackily expanding to a module containing the desired items. Do
  note that this only works when the macro is in "item position", or where an
  item is expected. There is an
  [issue](https://github.com/mozilla/rust/issues/10681) open for lifting this
  restriction.
- A `thread_local` attribute [has been
  added](https://github.com/mozilla/rust/pull/10312), which exposes a
  platform's native TLS, a la C11/C++11 `thread_local`.
- Cross compilation to win64 (via mingw-w64) [is now
  supported](https://github.com/mozilla/rust/pull/10578). There were also a
  [bunch of fixes](https://github.com/mozilla/rust/pull/10631) on real win64.
- The parser gives a [better error
  message](https://github.com/mozilla/rust/pull/10641) when it encounters an
  unclosed delimiter it didn't expect.
- There is a [better error
  message](https://github.com/mozilla/rust/pull/10475) when a module name is
  used as a type (the motivating example being `impl SomeTrait for some_module`).
- JSON decoding [now gives better
  errors](https://github.com/mozilla/rust/pull/10625).
- Linker optimizations are [now
  used](https://github.com/mozilla/rust/pull/10620) on Linux.

## New contributors

Our first-time contributors this week are:

- Andreas Ots
- Eric Biggers
- Jannis Harder
- Kiet Tran

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-11-26)
discussed bootstrap times, the `thread_local` PR, mutexes, and the GC. In
particular, it was decided that we should have nightlies.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

This was a short week due to the US Thanksgiving holiday, but there were still
17 landed PRs this week.

## Notable additions
- Hyunjune Kim and Sammy Kim landed some border style extensions in
  [#1276](http://github.com/mozilla/servo/pull/1322).
- Youngmin Yoo, Seonghyun Kim, and Jaemin Moon landed overflow:hidden in
  [#1298](http://github.com/mozilla/servo/pull/1298).
- Isabelle Carter landed changing the unrendered portion of content to use the
  doc's background color in [#1210](http://github.com/mozilla/servo/pull/1210).
- Patrick Walton has been experimenting with some very promising parallel
  layout work, which he described in a
  [message](https://groups.google.com/forum/#!topic/mozilla.dev.servo/1nKmBvFewIs)
  to the mailing list.

## Meetings

This week's [meeting](https://github.com/mozilla/servo/wiki/Meeting-2013-11-25)
covered the poor situation with Nvidia support on Linux, fleshed out the string
interning plan during CSS selector parsing/matching, and went into some Servo
and Rust-related build issues.

# Announcements etc

Do note that all the links are pulled directly from the
[subreddit](https://reddit.com/r/rust).

- [Matrix Multiply Performance in Rust](http://www.reddit.com/r/rust/comments/1rf8rz/matrix_multiply_performance_in_rust/)
- [nrays](https://github.com/sebcrozet/nrays) - a 3d/4d raytracer
- [SprocketNES: Practical Systems Programming in
  Rust](https://air.mozilla.org/sprocketnes-practical-systems-programming-in-rust/).
  Contrary to the title, this is actually a recording of the presentations at
  the Bay Area meetup.
- [uutils](https://github.com/uutils/coreutils) - an attempt at writing
  universal (as in cross-platform) CLI utils in Rust
- [lbac](https://github.com/cmr/lets-build-a-compiler) - a Rust port of Jack
  Crenshaw's "Let's Build a Compiler"
