Title: This Week in Rust 24
Date: 2013-11-23 17:58
Category: This Week in Rust

Welcome to another issue of *This Week in Rust*!

<!-- more -->

# What's cooking on master?

47 PRs were merged this week.

## Breaking Changes

- Non-ASCII identifiers are [feature
  gated](https://github.com/mozilla/rust/pull/10605), due to open questions
  about how it should be done. They aren't being removed, just deferred to
  post-1.0.
- Some more closure reform has gone through. `~fn` has been removed, [use
  `proc` instead](https://github.com/mozilla/rust/pull/10561). `&fn` is [also
  short for this world](https://github.com/mozilla/rust/pull/10568).
- `std::util::NonCopyable` had its dummy constructor
  [removed](https://github.com/mozilla/rust/pull/10575).
- All of the functions in rustrt [have been prefixed with
  `rust_`](https://github.com/mozilla/rust/pull/10440).
- Items inside functions [no longer accept privacy
  modifiers](https://github.com/mozilla/rust/pull/10443), since it is
  meaningless.
- Reachable `extern fn`s are [no longer marked
  internal](https://github.com/mozilla/rust/pull/10539).

## Other changes

- A `Gc<T>` stub [has been added](https://github.com/mozilla/rust/pull/10576).
  New code should use this instead of `@T`.
- `..` is now [the pattern to use](https://github.com/mozilla/rust/pull/10366)
  when ignoring multiple elements in a pattern.
- Stepping through `if` in a debugger [should be much smoother
  now](https://github.com/mozilla/rust/pull/10552).
- `rustpkg -O` [will now actually
  optimize](https://github.com/mozilla/rust/pull/10526) the package it is
  building.
- Did you know we can make tags files for the Rust codebase? `make
  TAGS.{vi,emacs}`! It no longer generates tags for [all of LLVM and
  libuv](https://github.com/mozilla/rust/pull/10508), too.
- The pidigits benchmark [has been
  resurrected](https://github.com/mozilla/rust/pull/10555).
- A native mutex type [now
  exists](https://github.com/mozilla/rust/pull/10479), and it doesn't depend
  on C++.
- Some more derived methods are [marked for
  inlining](https://github.com/mozilla/rust/pull/10557).
- A bunch more [native file IO](https://github.com/mozilla/rust/pull/10495)
  was implemented, too.
- We now have support for the [win64 calling
  convention](https://github.com/mozilla/rust/pull/10527). Additionally, the
  ABI of `extern fn`s is now actually used.
- The tutorial [now covers alternatives to
  ownership](https://github.com/mozilla/rust/pull/10589) much better.
- A lint for unsafe blocks [has been
  added](https://github.com/mozilla/rust/pull/10599).
- A minor bug with privacy [has been
  fixed](https://github.com/mozilla/rust/pull/10583).

## New Contributors

Our first-time contributors this week are:

- Dave Hodder
- Ian Daniher
- Isaac Dupree
- g3xzh

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-11-19)
discussed static linking, changing how `let _ = foo` is handled, task joining,
autoderef, and user-defined vector types.

# Announcements, etc

- A [new
  tutorial](http://adridu59.github.io/rust-tuts/) project
  has sprung up.

# Servo

Thanks to Lars Bergstrom for This Week in Servo! Servo is Rust's sister
project, and I think it's nice to be able to see a highlevel of its progress.
Maybe TWiS will graduate to a separate blog someday.

## This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language. Starting this week, TWiR will include a status
update from Servo.

There were 14 landed PRs this week.

### Notable additions

- Patrick Walton rewrote flow construction so that we can make it incremental
  and parallelizable in https://github.com/mozilla/servo/pull/1271
- Ryan Choi added support for transparant PNG files in
  https://github.com/mozilla/servo/pull/1288
- Junyoung Cho added the ACID2 test file, which is one of the big focuses of
  the team over the coming months, in
  https://github.com/mozilla/servo/pull/1287
- In his first Servo PR, Adenilson Cavalcanti contributed a help message for
  the binary in https://github.com/mozilla/servo/pull/1277

### Meetings

Having spent the prior week at a workweek with Samsung in Korea (where there
were several presentations on both Rust and Servo!
https://github.com/mozilla/servo/wiki/Videos-and-presentations ), the meeting
was pretty light this week. It mainly [covered](
https://github.com/mozilla/servo/wiki/Meeting-2013-11-18 ) getting more formal
tracking of the work required for ACID2 and prioritizing Windows support.
