Title: This Week in Rust 18
Date: 2013-10-06 18:25
Category: This Week in Rust

Welcome to another issue of *This Week in Rust*.

<!-- more -->

# What's cooking on master?

Only 63 PRs were merged this week. Currently in the queue is the massive
privacy change, the path rewrite, enum descriminant shrinking, a `rand`
rework, a lock-free scheduler message queue, and libuv signal bindings.

## Breaking Changes

- `float` has been [removed from the
  language](https://github.com/mozilla/rust/pull/9519).
- `loop` has been [removed](https://github.com/mozilla/rust/pull/9665). The
  transition to `continue` is complete.
- A macro ignoring tokens now [throws an
  error](https://github.com/mozilla/rust/pull/9673) rather than silently
  ignoring it.
- `IntConvertible` has been
  [replaced](https://github.com/mozilla/rust/pull/9250) with `ToPrimitive` and
  `FromPrimitive`. This also heralds the ability to derive to/from int methods
  on enums.

## Everything Else

- `rustpkg` can [fetch remote packages
  again](https://github.com/mozilla/rust/pull/9741).
- A bunch of unsafe code surrounding logging [has been
  remove](https://github.com/mozilla/rust/pull/9593).
- A bunch of work was done to make more things immediate. A
  [bugfix](https://github.com/mozilla/rust/pull/9643), [small
  tuples](https://github.com/mozilla/rust/pull/9656), [small
  enums](https://github.com/mozilla/rust/pull/9677), and [unit/C-like
  enums](https://github.com/mozilla/rust/pull/9699).
- A bunch of work went into decreasing our memory usage. The massive spike at
  the beginning [has been fixed](https://github.com/mozilla/rust/pull/9612).
  The tcx (type context, holds the result of *all* compiler analysis) is also
  free'd before translation now, reducing memory usage by a good [500+
  MB](https://github.com/mozilla/rust/pull/9686). Another cache during
  constant evaluation [reduces memory usage by
  200MB](https://github.com/mozilla/rust/pull/9722).
- We're now [bundling our
  dependencies](https://github.com/mozilla/rust/pull/9662) on Windows, making
  it easier to bootstrap and to setup a working Rust environment.
- All of the really old obsolete syntax checkers [have been
  removed](https://github.com/mozilla/rust/pull/9712).
- Hyperlinking between crates is [now
  reimplemented](https://github.com/mozilla/rust/pull/9691) in rustdoc.
- Gearing up for its removal, [all `fmt!` usage in the compiler has been
  removed](https://github.com/mozilla/rust/pull/9599).

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-10-01)
discussed a bunch of proposed changes, including rustpkg submodules, removing
float (which has already happened), raw string literals (which have a patch
almost finished), and the changes to the Option API.

# Announcements

- [ncurses-rs](https://github.com/jeaye/ncurses-rs) - a safe wrapper that
  keeps the same ncurses interface you know and "love", but more rustic.
- [The Rusticon](https://github.com/mozilla/rust/wiki/The%20Rusticon) - A
  living glossary of Rust terms.
- [A production use of
  Rust!](http://www.reddit.com/r/rust/comments/1nqzth/zeromq_helping_us_block_malicious_domains_in_real/)
- [Understanding the Servo
  Strategy](http://www.reddit.com/r/rust/comments/1ntnvf/understanding_the_servo_strategy/)
  \- The Servo presentation given at the various Mozilla Summits
- [Dynamic Typing implemented as a
  library](http://www.reddit.com/r/rust/comments/1nmarr/dynamic_typing_implemented_as_library_code/)
  \- a pretty cool use of our reflection.
- [Running Rust Tests on Travis
  CI](http://www.reddit.com/r/rust/comments/1ni84a/running_rust_tests_on_travis_ci/).
- [Rust bindings for
  FUSE](http://www.reddit.com/r/rust/comments/1ngqgr/rust_bindings_for_fuse_filesystem_in_userspace/)

# Meetups

Haven't had a meetup section in a long time, but two are forming!

- [Francisco Bay Area](http://www.meetup.com/Rust-Bay-Area/)
- [Rust Skåne (Lund, Sweden)](http://www.meetup.com/rust-skane/)
