Title: This Week in Rust 36
Date: 2014-02-09 22:15
Category: This Week in Rust


Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

There were a surprising number of breaking changes this week. The [libextra
dissolution continues](https://github.com/mozilla/rust/issues/8784). Condition
removal and the IO error overhaul also landed. `rustpkg` was also removed, and
rustc's CLI interface has changed.  Additionally, Rust gained another
full-time developer! A big hello to Nick Cameron, I look forward to seeing his
work.

It was also [discovered](http://octoverse.github.com/) that we closed the
second largest number of issues of any repository on GitHub! We trailed behind
homebrew, which is almost cheating, because they use it to track issues on
over five thousand packages. Good job everyone!

<!-- more -->

# What's cooking on master?

72 pull requests were merged this week.

## Breaking Changes

- Some intrinsics [have been
moved](https://github.com/mozilla/rust/pull/12124) into `std::mem`, in
preparations of the intrinsics no longer being publically exposed.
- A soundness regression [has been
fixed](https://github.com/mozilla/rust/pull/12117).
- `char` [now has range asserts](https://github.com/mozilla/rust/pull/12086).
This may cause invalid uses of `char` (using invalid values) to break.
- The various traits in `std::fmt` [now use
`&self`](https://github.com/mozilla/rust/pull/12066), rather than a separate
parameter with an argument of type `&Self`.
- `@self` and `@Trait` (managed trait objects) [have been
removed](https://github.com/mozilla/rust/pull/12030).
- `rustpkg` [has been removed](https://github.com/mozilla/rust/pull/11987).
- `std::fmt::Default` [has been
renamed](https://github.com/mozilla/rust/pull/11948) to `Show`.
- IO errors [have been completely
overhauled](https://github.com/mozilla/rust/pull/11946), and no longer use
conditions. Alex wrote [an
email](https://mail.mozilla.org/pipermail/rust-dev/2014-February/008505.html)
to the mailing list summarizing the changes. Especially take note of the
`if_ok!` macro.
- Conditions [have been removed
entirely](https://github.com/mozilla/rust/pull/12039). A detailed post-mortem
is available on the pull request.
- `rustc` has seen some work on its interface. The output flags [have been
unified](https://github.com/mozilla/rust/pull/12020), replacing `-c`,
`--emit-llvm`, `-S`, `--dylib`, `--rlib`, etc with two arguments: `--emit` and
`--crate-type`. There is [another PR in the
queue](https://github.com/mozilla/rust/pull/12084) that replaces a bunch of
miscellaneous flags behind `-C`. It will likely land shortly, and is something
to be aware of.
- `NonCopyable` [has been renamed](https://github.com/mozilla/rust/pull/12016)
to `NoPod` to be consistent with the rest of the "marker types".
- `reserve` naming [has been shuffled around
slightly](https://github.com/mozilla/rust/pull/11951). `reserve` becomes
`reserve_exact` and `reserve_at_least` becomes `reserve`.
- `SendStr` [has been removed](https://github.com/mozilla/rust/pull/12098) in
favor of a `MaybeOwned` type.

As part of the libextra dissolution, the following crates have been
introduced:

- [`libcollections`](https://github.com/mozilla/rust/pull/12010), which has
inherited `Bitv`, `BTree`, `Deque`, `DList`, `List`, `LruCache`,
`PriorityQueue`, `RingBuf`, `SmallIntMap`, `TreeMap`, and `TreeSet`.
- [`libgetopts`](https://github.com/mozilla/rust/pull/12007), which has
inherited `extra::getopts`. Additionally, `getopts::groups` is now the *only*
interface. It has been moved up a level, into just `getopts`, and the old
functions have been removed.
- [`libserialize`](https://github.com/mozilla/rust/pull/11984), which has
inherited `extra::serialize` and `extra::ebml`.
- [`libuuid`](https://github.com/mozilla/rust/pull/11912), which has inherited
`extra::uuid`.
- [`libsemver`](https://github.com/mozilla/rust/pull/12012), which has
inherited `extra::semver`.
- [`libterm`](https://github.com/mozilla/rust/pull/11945), which has inherited
`extra::term` and `extra::terminfo`.

All crates are still documented, and there is a list with links at
<http://static.rust-lang.org/doc/master/index.html>.

## Other Changes

- A `black_box` function [has been added to
`extra::test`](https://github.com/mozilla/rust/pull/12105), and the `iter`
method on BenchHarness can now return values. As
[documented](https://github.com/mozilla/rust/pull/12119) ([generated
version](http://static.rust-lang.org/doc/master/guide-testing.html#benchmarks-and-the-optimizer)),
these exist to make sure that LLVM will not optimize out benchmarks.
- A `fourcc!` syntax extension [has been
added](https://github.com/mozilla/rust/pull/12034).
- A `unimplemented!` macro [has been
added](https://github.com/mozilla/rust/pull/12090), used much the same as
`unreachable!()`.
- The [runtime libraries [have gained
examples](https://github.com/mozilla/rust/pull/12073), and
[libgreen](http://static.rust-lang.org/doc/master/green/index.html) now has an
explanation of how it is structured at a high level, specifically where
concurrency and parallelism come from.
- A `from_utf8_lossy` function [has been
added](https://github.com/mozilla/rust/pull/12062) that replaces invalid
codepoints with the unicode replacement character (�).
- Network streams [are now
cloneable](https://github.com/mozilla/rust/pull/11894). This allows for
multiple tasks reading/writing a `TcpStream`.
- A copy-on-write Arc container [has been added to
libextra](https://github.com/mozilla/rust/pull/11230), though it's now in
`libcontainers`.
- SIMD types [are now allowed in
generics](https://github.com/mozilla/rust/pull/11717).
- A new mutex type that plays well with the various runtimes [has been
implemented](https://github.com/mozilla/rust/pull/11866). It is in the same
ballpark as native, pthread mutexes.

## New Contributors

- Arcterus
- Cole Mickens
- Colin Sherratt
- HeroesGrave
- Ivan Enderlin
- James Deng
- João Souls
- Marek Šuppa
- Q.P.Liu
- Yuri Kunde Schlesner

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-02-04)
discussed adding a `libprim`, operator overloading, and the 1.0 goals for
LLVM.

# Meetups

- There will be a meetup [in
Paris](http://www.eventbrite.fr/e/billets-rust-paris-meetup-10528169037), on
February 25, from 18:30 to 23:30.
- [Bay Area Rust](http://www.meetup.com/Rust-Bay-Area/events/156288462/) will
be meeting February 25, at 19:00 in San Francisco. David Renshaw will be
talking about Cap' Proto, Steven Fackler will be talking about exportable
macros, and Kevin Cantu about testing.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary
test cases for the Rust language.

This week, we landed 14 PRs, but there are a substantial number of very
close pending PRs that add major layout features to Servo (e.g., lists
markers, tables, initial pseudo-class and pseudo-element support).

## Notable additions

- Bruno Abinader continued with his great DOM additions in
[#1634](https://github.com/mozilla/servo/pull/1634) and
[#1622](https://github.com/mozilla/servo/pull/1622) and
[#1604](https://github.com/mozilla/servo/pull/1604)
- Patrick Kim fixed borders on inline boxes in
[#1577](https://github.com/mozilla/servo/pull/1577) and landed
`position:relative` support in
[#1613](https://github.com/mozilla/servo/pull/1613)
- Patrick Walton further improved layout performance in
[#1630](https://github.com/mozilla/servo/pull/1630) and
[#1615](https://github.com/mozilla/servo/pull/1615)
- Adrien Bustany fixed `insertBefore` in
[#1621](https://github.com/mozilla/servo/pull/1621)
- Lars Bergstrom landed reftest stabilization fixes in
[#1623](https://github.com/mozilla/servo/pull/1623) - we are almost ready to
gate landing commits on content and ref tests passing on Linux, once we get
more fonts [installed](https://bugzilla.mozilla.org/show_bug.cgi?id=968375)
on our Linux buildbots

## New contributors

- Adrien Bustany (abustany)

## Meetings

In this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-02-03), we mainly
discussed display list construction with respect to layering and stacking
contexts.

Josh Matthews' talk on Servo at FOSDEM is available
[here](http://mirrors.dotsrc.org/fosdem/2014/UD2218A/Saturday/Servo_building_a_parallel_web_browser.webm).

# Announcements, etc

- [Lazily initialized
statics](http://www.reddit.com/r/rust/comments/1wvxcn/lazily_initialized_statics/)
- [Rust gains a new full-time developer, Nick
Cameron](http://www.reddit.com/r/rust/comments/1wypbz/rust_gains_a_new_fulltime_developer_nick_cameron/).
- [Handling I/O
Errors](http://www.reddit.com/r/rust/comments/1wz7ws/handling_io_errors/)
- [Pure-Rust Adler32 and
CRC32](http://www.reddit.com/r/rust/comments/1x0zeu/adler32_and_crc32/)
- [Closures and the borrow
checker](http://www.reddit.com/r/rust/comments/1x1tvo/closures_and_the_borrow_checker/)
- [Pure-Rust LZW Compression
Algorithm](http://www.reddit.com/r/rust/comments/1x3ooy/lzw_compression_algorithm/)
- [Rust by Example: Default arguments,
etc](http://www.reddit.com/r/rust/comments/1x67wq/rust_by_example_default_arguments_named_params/)
- [Standalone rust
app](http://www.reddit.com/r/rust/comments/1xaum0/standalone_rust_app/)
- [IDE support?](http://www.reddit.com/r/rust/comments/1xfjo7/ide_support/)
- [Playing with the new Rust I/O error
handling](http://www.reddit.com/r/rust/comments/1xdlou/playing_with_the_new_rust_io_error_handling/)
- [Another failed attempt at parser
combinators](http://www.reddit.com/r/rust/comments/1xdudv/another_failed_attempt_at_parser_combinators/)
- [A case for reflection in
Rust](http://www.reddit.com/r/rust/comments/1xck76/a_case_for_reflection_in_rust/)
- [A persistent map implementation, like in Clojure and Scala, with
performance
numbers](http://www.reddit.com/r/rust/comments/1xa8uy/a_persistent_map_implementation_like_in_clojure/).
