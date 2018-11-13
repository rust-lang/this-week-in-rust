Title: This Week in Rust 12
Date: 2013-08-25 12:54
Category: This Week in Rust

Hello and welcome to the 12th issue of *This Week in Rust*. Sorry for the
brevity, though most weeks are probably going to be like this, as I've started
uni and have much less free time than in the summer.

<!-- more -->

# What's cooking in master?

A whopping 71 pull requests were merged this week, and issue churn remains
negative at -32.

## Breaking changes
- [**`yield` is now reserved as a
  keyword**](https://github.com/mozilla/rust/pull/8560), with the hope of
  maybe using it for generators.
- [The type of `extern fn foo` is now `extern "C" fn`, rather than
  `*u8`](https://github.com/mozilla/rust/pull/8666).
- [Some free functions in `extra::json` were turned into associated functions
  on the Json enum](https://github.com/mozilla/rust/pull/8679).
- [`XorShiftRng::new()` now uses a random
  seed](https://github.com/mozilla/rust/pull/8558). This is only breaking if
  you were depending on that constant seed, which you shouldn't have.

## Library changes

- [`extra::getopts` aligns based on codepoint count and not byte
  count](https://github.com/mozilla/rust/pull/8710), as a step towards more
  correct unicode handling.
- [`fprintf` has been added, as well as some `ifmt`
  cleanup](https://github.com/mozilla/rust/pull/8637).
  [The new runtime has seen a lot of
  optimization](https://github.com/mozilla/rust/pull/8740).
- [File IO has been added to the new
  runtime](https://github.com/mozilla/rust/pull/8655).
- [Some parsing errors related to ports have been fixed in
  `extra::url`](https://github.com/mozilla/rust/pull/8616).
- [CharIterator has seen some optimization too, with reverse iterators being
  much closer in performance to forward iterators
  now](https://github.com/mozilla/rust/pull/8590).
- [sysconf names have been added for
  android](https://github.com/mozilla/rust/pull/8602).
- [The new runtime now has threadsafe
  IO](https://github.com/mozilla/rust/pull/8631).
- [A callback optimization has sped up message passing benchmarks to the tune
  of 40%](https://github.com/mozilla/rust/pull/8566).
- [jemalloc is back](https://github.com/mozilla/rust/pull/8584).

## Compiler changes

- [gnueabihf actually uses hard floats
  now](https://github.com/mozilla/rust/pull/8736).
- [Frame pointer elimination is no longer
  disabled](https://github.com/mozilla/rust/pull/8695).
- [Some debuginfo fixes landed](https://github.com/mozilla/rust/pull/8684).
  Supposedly, libstd can now be compiled with `-Z debug-info`. Yay!
- [Stack unwinding on 32-bit windows now
  works](https://github.com/mozilla/rust/pull/8596). This is a major step
  forward for Windows support, I'm very excited to see it land.
- [A handful of default method bugs have been
  fixed](https://github.com/mozilla/rust/pull/8659).
- [Inheriting from kinds now sorta
  works](https://github.com/mozilla/rust/pull/8562), you can do `trait Foo:
  Freeze`, for example.
- [Supertrait methods can now be used from a trait
  object](https://github.com/mozilla/rust/pull/8519).
- The rest of pnkfelix's visitor trait rewrite series landed.
  [2](https://github.com/mozilla/rust/pull/8539),
  [3](https://github.com/mozilla/rust/pull/8619),
  [4](https://github.com/mozilla/rust/pull/8623), and
  [5](https://github.com/mozilla/rust/pull/8638).
- [Foreign function wrappers have been
  removed](https://github.com/mozilla/rust/pull/8535).
- [LLVM has been updated](https://github.com/mozilla/rust/pull/8328).

## Docs etc

- [A new condition tutorial has been
  added](https://github.com/mozilla/rust/pull/8563).
- [Some docs for trait bounds have been
  added](https://github.com/mozilla/rust/pull/8725).

# Meeting

The [Tuesday
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-08-20)
discussed cycle time and how to fix it.

# Projects and discussion

- [gl-rs](https://github.com/bjz/gl-rs) is now ready for use, since the
  foreign function wrappers have been removed!
- [msgpack-rust](https://github.com/omasanori/msgpack-rust) has been created.
  It ties into `extra::serialize`.

- [A template for Arduino Due
  projects](https://github.com/jensnockert/dueboot).
- [Parallel cross-language level generation
  benchmarks](http://www.reddit.com/r/rust/comments/1kxz7y/benchmarks_round_two_parallel_go_rust_d_scala_and/).
- [A Week with
  Rust](http://www.reddit.com/r/rust/comments/1ktjrw/a_week_with_mozillas_rust/).

