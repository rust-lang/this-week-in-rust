Title: This Week in Rust 9
Date: 2013-08-04 18:40
Category: This Week in Rust

Hello and welcome to the ninth issue of *This Week in Rust*. This week brings
the new `for` loop, which is very exciting, as well as a bunch of runtime
changes and cleanup.

<!-- more -->

# What's cooking on `master`?

Issue churn was +4 this week. A total of 63 PRs were merged (again).

## Breaking Changes

- **The `for` loop now uses external iterators.** This means any code written
  to use the old internal iterator protocol will no longer work. See the
  [iterator tutorial](http://static.rust-lang.org/doc/tutorial-container.html)
  for more information on how to use it. Related pull requests:
  [#8141](https://github.com/mozilla/rust/pull/8141),
  [#8184](https://github.com/mozilla/rust/pull/8184),
  [#8190](https://github.com/mozilla/rust/pull/8190),
  [#8244](https://github.com/mozilla/rust/pull/8244). A few uses now
  require `do` rather than `for` because they cannot/have not been
  implemented in terms of external iterators.
- `unsafe` is [no longer allowed](https://github.com/mozilla/rust/pull/8235)
  for functions in `extern` blocks: they are all unsafe.
- The [`extra::dbg` module](https://github.com/mozilla/rust/pull/8175) has
  been removed.
- `uint::range` and all its friends have been replaced with an [external
  iterator](https://github.com/mozilla/rust/pull/8216), that is in the
  prelude. Code like the following now works:
  ```
  for x in range(0, 10) {
	  println(x.to_str());
  }
  ```
- The pipes compiler (the thing driving `proto!`) [has been
  removed](https://github.com/mozilla/rust/pull/8170), as it saw limited
  use, was very old, and was a significant maintenance burden.
- `PortSet` [has been removed](https://github.com/mozilla/rust/pull/8164) from
  std, as the new scheduler does not support it.
- A bunch of old task APIs [have been
  removed](https://github.com/mozilla/rust/pull/8139), also in preparation for
  the new schduler.
- `is_utf8` now [rejects overlong
  encodings](https://github.com/mozilla/rust/pull/8133).
- The iterator adaptors [no longer have the Iterator
  suffix](https://github.com/mozilla/rust/pull/8090), same with [str and vec
  iterators](https://github.com/mozilla/rust/pull/8095) as well.

## newrt changes

A bunch of newrt things landed this week, so it gets its own section.

- Some [bugs preventing the arc and sync tests from
  passing](https://github.com/mozilla/rust/pull/8234) have been fixed.
- The new scheduler now supports [the `SingleThreaded` spawn
  mode](https://github.com/mozilla/rust/pull/8221).
- A bunch of work with task killing [has
  landed](https://github.com/mozilla/rust/pull/8195).
- Some [major TLS changes](https://github.com/mozilla/rust/pull/8116) also
  landed.
- Tasks can [now be named](https://github.com/mozilla/rust/pull/8158).
- [`select` on newrt pipes](https://github.com/mozilla/rust/pull/8008) has
  been implemented.

## Notable library additions, bugfixes, and cleanup

- `Map::contains_key` is [now a default
  method](https://github.com/mozilla/rust/pull/8246) implemented in terms of
  `Map::find`
- A `dynamic_lib` segfault [has been
  fixed](https://github.com/mozilla/rust/pull/8219).
- A keyed `HashMap` constructor is [now
  exposed](https://github.com/mozilla/rust/pull/8186) for runtimeless programs
  that want to use it.
- The `Str` trait now has an [`into_owned`
  method](https://github.com/mozilla/rust/pull/8204) to avoid copies when you
  already have a `~str`.
- A bunch of [SHA1 and SHA2
  cleanup/optimizations](https://github.com/mozilla/rust/pull/8174) landed. I
  hear that the speed is almost optimal, only a few cycles/byte short of
  Intel's optimized implementation.
- Errno coverage has been [significantly expanded for
  Linux](https://github.com/mozilla/rust/pull/8193). I added all of the ones
  that were missing, at least the ones that were present on my system.
- `assert!()` without a message [now does less
  allocation](https://github.com/mozilla/rust/pull/8150).
- '\' is [no longer treated as a path
  separater](https://github.com/mozilla/rust/pull/8138) on POSIX system.
- `getopt`'s `opts_str` [has been corrected to use more than just the first
  element of the vector](https://github.com/mozilla/rust/pull/8135).
- Some more methods [were added](https://github.com/mozilla/rust/pull/8115) in
  `std::num`.
- An iterator over the offsets of each character in a string [was
  added](https://github.com/mozilla/rust/pull/8082).
- A bunch of `RandomAccessIterator` implementations [have been
  added](https://github.com/mozilla/rust/pull/8120).
- `Clone` and `DeepClone` are [now
  implemented](https://github.com/mozilla/rust/pull/8109) for `extern "Rust"
  fn`.

## Notable compiler additions, bugfixes, and cleanup

- A `cfg!` syntax extension [has been
  added](https://github.com/mozilla/rust/pull/8188) for conditionally running
  code based on crate configuration, similar to what `#[cfg]` does for
  conditional compilation. It expands into a true/false constant, so LLVM
  should optimize out the dead branches.
- Some more codegen tests [have been
  added](https://github.com/mozilla/rust/pull/8165).
- `copy` [has been removed as a
  keyword](https://github.com/mozilla/rust/pull/8162).
- Static struct initializers [can now contain
  `..base`](https://github.com/mozilla/rust/pull/8091) for functional update.
- Take glue [has been unified](https://github.com/mozilla/rust/pull/8146) for
  unique pointer type.
- Pointer arithmetic is [now implemented with
  GEP](https://github.com/mozilla/rust/pull/8121) rather than casting to int
  and back to the pointer.
- Some more AST types [were
  renamed](https://github.com/mozilla/rust/pull/8107).
- Cross-crate conditions [now
  work](https://github.com/mozilla/rust/pull/8185).

## Documentation, tools, and other stuff

- LLVM assertions [can now be
  disabled](https://github.com/mozilla/rust/pull/8147) with a configure
  option.
- Benchmarking can [now be
  disabled](https://github.com/mozilla/rust/pull/8111) by passing `NO_BENCH=1`
  to make.
- `NO_REBUILD` [no longer requires a
  re-boostrap](https://github.com/mozilla/rust/pull/8110), which should make
  debug cycles on libstd much shorter.
- `vec` [now has module
  documentation](https://github.com/mozilla/rust/pull/7223).
- rustpkg [now handles tags](https://github.com/mozilla/rust/pull/8032), and
  not just version numbers, in the package ID.

# Meetings

The [Tuesday
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-07-30) this
week was quite meaty. I'm not going to try to summarize it, as it seems no
real decisions were made.

# Discussion + Blog posts

- [Visibility scopes in Rust Debug
  Info](http://michaelwoerister.github.io/2013/08/03/visibility-scopes.html).
- [Architecting Servo: Pipelines and
  Parallelism](https://air.mozilla.org/2013-intern-kuehn/), a talk by Tim
  Kuehn.
- [Runtimeless
  sprocketnes](http://www.reddit.com/r/rust/comments/1jo431/runtimeless_sprocketnes/).
- [Porting machine learning algorithms to
  Rust](http://www.reddit.com/r/rust/comments/1joy7f/porting_machine_learning_algorithms_to_rust/).
- [RFC: Overloadable dereference
  operator](https://mail.mozilla.org/pipermail/rust-dev/2013-July/005039.html).

# External projects

- [RustGnuplot](https://github.com/SiegeLord/RustGnuplot) was updated to
  latest Rust.
- A [protobuf implementation](https://github.com/stepancheg/rust-protobuf) has
  been started.
- [rustsqlite](https://github.com/linuxfood/rustsqlite) has
  been updated to latest Rust.
- A [library for HTML escaping](https://github.com/veddan/rust-htmlescape) has
  been created.
- A [library for procedurally generating
  noise](https://github.com/bjz/noise-rs) has been created.
- A [pure-Rust implementation of
  Keccak](https://github.com/MarkJr94/rust-keccak) has been created.
- [rust-zmq](https://github.com/erickt/rust-zmq) has been updated to latest
  Rust, as well as cleaner error/constant interface.
- [q3](https://github.com/Jeaye/q3) now does multithreaded rendering.
