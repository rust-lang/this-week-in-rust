Title: This Week in Rust 15
Date: 2013-09-15 17:59
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*. We're gearing up
for the 0.8 release in 2-3 weeks. It looks like it's going to be a really
solid release. I'll write another `State of Rust`, hopefully before it is
released.

<!-- more -->

# What's cooking in master?

68 PRs were merged this week.

## Breaking changes

- `std::iterator` [has been
  renamed](https://github.com/mozilla/rust/pull/9065) to `std::iter`.
- The `std::num::Primitive` trait is [now
  constrained](https://github.com/mozilla/rust/pull/9051) by the `Clone` and
  `DeepClone` traits, as well as
  [`Orderable`](https://github.com/mozilla/rust/pull/9182).
- Some [more free functions](https://github.com/mozilla/rust/pull/9062) have
  been removed from `std::vec`. `unzip` now takes an iterator, a
  `Permutations` iterator has been added, and some rarely-used, obsolete,
  functions were removed.
- A bunch of changes to `Option` and `Result` [were
  made](https://github.com/mozilla/rust/pull/9115). Specifically, `chain` was
  changed to `and_then` and `unwrap_or_default` to `unwrap_or`.
- rustpkg [builds into
  target-specific](https://github.com/mozilla/rust/pull/9151) subdirectories
  now.

## Additions and fixes

- debuginfo now has [namespace
  support](https://github.com/mozilla/rust/pull/9097). Looking at all the
  various PRs Michael has opened over the summer, it seems DWARF is a very
  flexible, nice debuginfo format, but gdb and LLVM don't support it very
  well.
- Correct `range_step` and `range_step_inclusive` iterators [have been
  added](https://github.com/mozilla/rust/pull/9199). They are correct in cases
  of overflow, and are generic.
- A handy `sleep` function [has been
  added](https://github.com/mozilla/rust/pull/9191) to newrt.
- File IO in newrt [works on
  windows](https://github.com/mozilla/rust/pull/9165) now.
- A bug where nested items in a default method weren't compiled [has been
  fixed](https://github.com/mozilla/rust/pull/9162).
- A rendezvous concurrency structure, much like Ada's, [has been
  added](https://github.com/mozilla/rust/pull/8908).
- Buffered IO wrappers [have been
  added](https://github.com/mozilla/rust/pull/9091).
- nmatsakis landed a PR that [closed 7 issues at
  once](https://github.com/mozilla/rust/pull/9088).
- rustpkg now uses `extra::workcache` [to prevent recompilation of
  already-compiled crates](https://github.com/mozilla/rust/pull/9034).

# Meeting

The [Tuesday
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-09-10)
discussed the github commit policy, implicit copyability, patterns, and the
fate of `&const`.

# Other things

- Eric Reed (ecr)'s intern presentation: [An I/O System for
  Rust](https://air.mozilla.org/intern-presentations-reed/). Unfortunately,
  the audio cuts out.
- [Evict-BT](https://github.com/singingboyo/evict), a git-integrated issue
  tracker.
- [Computer Graphics and Game
  Development](https://github.com/mozilla/rust/wiki/Computer-Graphics-and-Game-Development).
  Also note the `#rust-gamedev` channel.
- [rust-for-real](https://github.com/FlaPer87/rust-for-real), a collection of
  Rust examples to aid in learning. Needs more examples!
