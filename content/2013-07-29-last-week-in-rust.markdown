Title: Last Week in Rust 8
Date: 2013-07-29 06:55
Category: This Week in Rust

Hello and welcome to the eighth issue of *This Week in Rust*. Due to me being
busy and forgetful over the weekend, this is a special issue, *Last Week in
Rust*.

<!-- more -->

# What's cooking on `master`?

Issue churn continues to be negative, -15 this week. A total of 63 PRs were
merged.

## Breaking Changes

There were impressively few breaking changes last week.

- **You now need to pass `--cfg debug` to `rustc` to emit debug logging.**
- [**`mod.rs` is now "blessed".**](https://github.com/mozilla/rust/pull/7926).
  When loading `mod foo;`, rustc will now look for `foo.rs`, then
  `foo/mod.rs`, and will generate an error when both are present.
- [A bunch of `str` functions](https://github.com/mozilla/rust/pull/7996) were
  renamed or shuffled around to be more consistent.
- [`SmallIntSet` was removed](https://github.com/mozilla/rust/pull/7934) in
  favor for the more efficient, equivalent `BitvSet`.
- [`Bitv` and `Bitvset` have switched to external
  iterators](https://github.com/mozilla/rust/pull/7703).
- [`extra::net` and a bunch of other obsolete
  features](https://github.com/mozilla/rust/pull/7883) have been removed.

## Notable library additions, bugfixes, and cleanup

- Various [TCP/UDP additions](https://github.com/mozilla/rust/pull/8040) have
  been made in the new rt.
- Some more [atomic operations](https://github.com/mozilla/rust/pull/8039)
  have been added.
- A [`chain_mut_ref` method](https://github.com/mozilla/rust/pull/7931) was
  added to `Option`.
- [Random access iterators](https://github.com/mozilla/rust/pull/7982) have
  been implemented.
- Some missing [memory orderings on atomic
  types](https://github.com/mozilla/rust/pull/7993) have been added.
- [workcache has seen a bunch of
  attention](https://github.com/mozilla/rust/pull/7885).
- [DList has seen some more cleanup
  too](https://github.com/mozilla/rust/pull/7944).
- [Timers have been added to the new
  rt](https://github.com/mozilla/rust/pull/7916).
- [Vectors now implement `slice_from` and
  `slice_to`](https://github.com/mozilla/rust/pull/7943).

## Notable compiler additions, bugfixes, and cleanup

- [debuginfo for destructured locals and function
  args](https://github.com/mozilla/rust/pull/8045) is now implemented.
- [Raw representations are now
  consolidated](https://github.com/mozilla/rust/pull/7986).
- [Impossible branches on
  constants](https://github.com/mozilla/rust/pull/8041) are now omitted.
- [It is now possible to link against crates with
  `#[no_std]`](https://github.com/mozilla/rust/pull/7924).
- [There is now a warning when matching against
  NaN](https://github.com/mozilla/rust/pull/8029), since it is impossible to
  match against NaN (NaN != NaN).
- A lot of [default method and trait inheritance
  bugs](https://github.com/mozilla/rust/pull/8015) have been fixed.
- [`uint` enum discriminants are now
  allowed](https://github.com/mozilla/rust/pull/8000).
- The [section placement of static and fn items is now
  configurable](https://github.com/mozilla/rust/pull/7958).
- Some [trans naming modernization has
  occured](https://github.com/mozilla/rust/pull/7848).
- Some unnecessary branches and blocks [have been
  removed](https://github.com/mozilla/rust/pull/7941), resulting in a 10%
  speedup of unoptimized rustc.

## Documentation, tools, and other stuff

- [Some benchmarks](https://github.com/mozilla/rust/pull/7912), and [some more
  benchmarks](https://github.com/mozilla/rust/pull/7980).
- Crnobog has [fixed](https://github.com/mozilla/rust/pull/8001)
  [some](https://github.com/mozilla/rust/pull/7979) Windows testsuite issues.
- [`Makefile` dependencies](https://github.com/mozilla/rust/pull/7820) have
  been fixed. `rustc` will never be invoked without its dependencies being
  built.
- [`rust-mode` has been rewritten](https://github.com/mozilla/rust/pull/8031).
- [There are some build system changes surrounding the `--cfg debug`
  changes](https://github.com/mozilla/rust/pull/8020).

# Meetings

The [Tuesday
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-07-23) was
quite productive. A quick summary:

- Graydon wants to investigate using the Memory Pool System as the Rust GC,
  rather than a bespoke one. The [MPS](http://www.ravenbrook.com/project/mps/) is
  a very mature and robust memory management library.
- The buildbots now collect and report some metrics as JSON. Take a poke in
  `http://static.rust-lang.org/build-metrics/<git-sha1>/<builder>/<slave>/<metricsfile>.json`
  if you're interested.
- pcwalton proposes allowing `Self` in impls, like in trait definitions.
- There was some discussion of destructors taking `self` by value.
- There was a proposal to remove `*mut`, but it can be useful. There was no
  consensus.
- There was also some discussion on closures and mutable captures. I don't
  really have enough context to understand the conversation, something to do
  with "thunks".
- Removing `&const` was discussed as well. The "plan is that we add a lint
  flag but document it as a reserved word", as it doesn't really seem to be
  useful.

# Discussion + Blog posts

- [Iterator Blocks for
  Rust](http://michaelwoerister.github.io/2013/07/26/Iterator-Blocks.html)
- [RFC: Removing
  `*T`](http://www.reddit.com/r/rust/comments/1j5vbn/rustdev_rfc_removing_t/)
- [dherman's OSCON
  slides](https://speakerdeck.com/dherman/rust-low-level-programming-without-the-segfaults)
- [Mozilla is hiring a Rust research
  engineer](https://careers.mozilla.org/en-US/position/oKiEXfwn)
- [An alpha release of the MongoDB
  Driver](http://blog.mongodb.org/post/56426792420/introducing-the-mongodb-driver-for-the-rust-programming)
- [A fairly useless benchmark of random number
  generation](https://togototo.wordpress.com/2013/07/23/benchmarking-level-generation-go-rust-haskell-and-d/)

# Projects

- [color-rs: A library that provides types and conversions for working with
  various color formats.](https://github.com/bjz/color-rs)
- [grease-bench: a runtimeless
  benchmarker](https://github.com/Aatch/grease-bench)
- [rustfind, a "jump to definition"
  tool](https://github.com/dobkeratops/rustfind)
- [RustyXML, a pure-Rust XML parser](https://github.com/Florob/RustyXML)
