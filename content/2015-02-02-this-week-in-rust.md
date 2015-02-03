Title: This Week in Rust 68
Date: 2015-02-02
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

86 pull requests were [merged in the last week][merged], and 1 [RFCs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-26..2015-02-01
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-26..2015-02-01

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* `for` loops [now operate on the `IntoIterator` trait][into], which
  eleminates the need to call `.iter()`, etc. to iterate over
  collections. There are some new subtleties to remember though
  regarding what sort of iterators various types yield, in particular
  that `for foo in bar { }` yields values from a move iterator,
  destroying the original collection. [RFC][into-rfc].
* `std::io` was [renamed][oldio] to `std::old_io` in preparation
  for implementing the [I/O overhaul RFC][oldio-rfc].
* The return type of `Fn`, `FnMut`, and `FnOnce` [are associated
  types][fnassoc]. The unsugared forms of these traits are unstable
  so this shouldn't break stable code. [RFC][fnassoc-rfc].
* `FullRange`, the type that represents a slice over a complete
  collection, has been [renamed to `RangeFull`][fullrange], and
  removed from the prelude.
  
[into]: https://github.com/rust-lang/rust/pull/20790
[into-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0235-collections-conventions.md#intoiterator-and-iterable
[fnassoc]: https://github.com/rust-lang/rust/pull/21019
[fnassoc-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0587-fn-return-should-be-an-associated-type.md
[oldio]: https://github.com/rust-lang/rust/pull/21543
[oldio-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md
[fullrange]: https://github.com/rust-lang/rust/pull/21717

## Other Changes

* The standard library now [partipates in feature gating][feat], so
  use of unstable libraries now requires a `#![feature(...)]`
  attribute. The impact of this change is [described on the
  forum][feat-forum]. [RFC][feat-rfc].
* References to types that implement `Deref<U>` now [automatically
  coerce to references][deref] to the dereferenced type `U`, e.g. `&T
  where T: Deref<U>` automatically coerces to `&U`. This should
  eliminate many unsightly uses of `&*`, as when converting from
  references to vectors into references to slices. [RFC][deref-rfc].
* The `private_no_mangle_fns` lint [warns about functions that are
  `#[no_mangle]` but not exported][nomang].

[feat]: https://github.com/rust-lang/rust/pull/21248
[feat-forum]: http://users.rust-lang.org/t/psa-important-info-about-rustcs-new-feature-staging/82/5
[feat-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0507-release-channels.md
[deref]: https://github.com/rust-lang/rust/pull/21351
[deref-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0241-deref-conversions.md
[nomang]: https://github.com/rust-lang/rust/pull/21495

## New Contributors

* Carl Lerche
* Dominik Inführ
* emanueLczirai
* jatinn
* John Hodge
* Loïc Damien
* Luke Steensen
* Nelson Chen
* Orpheus Lummis
* Pyfisch
* Sébastien Marie
* Tyler Thrailkill
* Victory
* Vojtech Kral

# Approved RFC's

* [RFC 517][rfc-517] on I/O has been [updated with material on
  `std::env`][env].

[rfc-517]: https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md
[env]: https://github.com/rust-lang/rfcs/pull/578

# New RFC's

* [Allow closure expressions to expand to a `&` or `&mut`
  temporary][rfc-756-pr]. Make it so you don't have to write `&` or
  `&mut` in front of closure literals in some circumstances.
* [Extensible enums][rfc-757-pr]. Adds a type of enum that can be
  extended to new variants without breaking downstream users by
  disabling exhaustiveness checks.
* [Sound generic drop][rfc-769-pr]. Makes `Drop` safely implementable
  on more types, eliminating `#[unsafe_destructor]`.
* [io error handling design][rfc-770-pr]. Deals with the semantics of
  closing I/O types.
* [std::iter::once][rfc-771-pr]. Adds a fn `once` that returns an
  iterator yielding a single element.
* [Add linear type facility][rfc-776-pr]. Adds a type that requires
  explicit drop.

[rfc-756-pr]: https://github.com/rust-lang/rfcs/pull/756
[rfc-757-pr]: https://github.com/rust-lang/rfcs/pull/757
[rfc-769-pr]: https://github.com/rust-lang/rfcs/pull/769
[rfc-770-pr]: https://github.com/rust-lang/rfcs/pull/770
[rfc-771-pr]: https://github.com/rust-lang/rfcs/pull/771
[rfc-776-pr]: https://github.com/rust-lang/rfcs/pull/776

# Community


## Announcements

* [Welcome to the new Rust forum][users]. There's a new user forum.
* [Weekly-meetings/2015-01-27][mtg]: Slow meeting.

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-01-27.md
[users]: http://users.rust-lang.org/t/welcome-to-the-new-rust-forum/21

## Blog Posts

* [Heartbleed in Rust][heartbleed]. It is possible to write
  Heartbleed-style bugs in Rust, though there is nothing to be alarmed
  about - Rust still rules.
  [HN][heartbleed-hn]. [/r/rust][heartbleed-r-rust]. [/r/programming][heartbleed-r-programming].
* [Some notes on Rust, the language][rtl]. First thread about Rust at
  LtU in a while.
  [HN][rtl-hn]. [/r/rust][rtl-r-rust]. [/r/programming][rtl-r-programming].
* [An Adventure in API Design and Concurrency in Rust][adv].
* [On Rust and Nim][nim]. Similar but
  different. [/r/rust][nim-r-rust]. [/r/programming][nim-r-programming].
* [updated crates.io dependency graph][deps]. The registry is growing
  fast. Thanks /u/corvette.

[heartbleed]: http://www.tedunangst.com/flak/post/heartbleed-in-rust
[heartbleed-hn]: https://news.ycombinator.com/item?id=8983771
[heartbleed-r-rust]: https://www.reddit.com/r/rust/comments/2uii0u/heartbleed_in_rust/
[heartbleed-r-programming]: https://www.reddit.com/r/programming/comments/2uinge/heartbleed_in_rust/
[rtl]: http://lambda-the-ultimate.org/node/5113
[rtl-hn]: http://news.ycombinator.com/item?id=8979620
[rtl-r-programming]: https://www.reddit.com/r/programming/comments/2uedrz/some_notes_on_rust_the_language/
[rtl-r-rust]: https://www.reddit.com/r/rust/comments/2uef23/some_notes_on_rust_the_language/
[nim]: https://andreaferretti.github.io/on-rust-and-nim/
[nim-r-rust]: https://www.reddit.com/r/rust/comments/2u6hvs/on_rust_and_nim/
[nim-r-programming]: https://www.reddit.com/r/programming/comments/2u8tih/on_rust_and_nim/
[deps]: http://froosky.rwell.org/crates-2015-01-31.svg
[adv]: http://damienradtke.com/adventures-in-concurrency-1/

## Discussions

* [Pre-RFC: resolve support for hyphens in crate names][hyph]. People
  love the aesthetics of hyphens-between-words, but hyphens are not
  valid identifiers.
* [Crates.io and namespacing][namespace].
* [Testing Rust's I/O Speed vs. C][iospeed]. Not such a good showing.
* There was bigtime [drama] after strcat stopped [maintaining his Arch
  packages][arch].

[hyph]: http://internals.rust-lang.org/t/pre-rfc-resolve-support-for-hyphens-in-crate-names/1459
[namespace]: https://www.reddit.com/r/rust/comments/2ud8uf/cratesio_and_namespacing/
[drama]: https://www.reddit.com/r/rust/comments/2u7p8e/i_have_archived_the_thread_from_yesterday_please/
[arch]: https://www.reddit.com/r/rust/comments/2tx7vj/psa_thestingers_rust_packages_for_arch_linux_are/
[iospeed]: https://www.reddit.com/r/rust/comments/2u057m/testing_rusts_io_speed_vs_c/

## Videos

* [SHA1 Performance Quest][sha1]. Implementing SHA1 in Rust.
* [Trace Quest 5][tq5]. A raytracer in Rust.

[sha1]: https://www.youtube.com/playlist?list=PLMHbQxe1e9MnDKy7FKXZwMJ6t_RCxpHqD
[tq5]: http://youtu.be/uhYRveqF27U?list=PLMHbQxe1e9MlR80JVZCa0uJf9cz_PxlCY

## New Projects

* [soa]. Struct-of-array types, a sometimes more efficient version of
  `Vec<(A, B)>` from cgaebel. [/r/rust][soa-r-rust].
* [valico]. JSON schema validator, from the [rustless] RESTful
  micro-framework project.
* [FFI-Platypus-Lang-Rust][perl]. Call Rust from Perl.
* [glassful]. kmc strikes again with a syntax extension that translates
  from Rust syntax to OpenGL Shader Language. [/r/rust][glassful-r-rust].
* [secretshare]. Shamir's secret sharing.
* [chip8-rust]. A emulator for the Chip-8 VM used to implement some
  old video games. Uses Piston for graphics.
* [snowflake]. Library for generating process-unique IDs.
* [rust-dev-box]. A Vagrantfile for working on Rust.
* [rust-roller]. A simple console dice-rolling app using the sweet
  [rustbox] termbox clone.

[soa]: https://github.com/cgaebel/soa
[soa-r-rust]: https://www.reddit.com/r/rust/comments/2uiu0e/soa_structofarray_types_in_rust/
[valico]: https://github.com/rustless/valico
[rustless]: https://github.com/rustless
[perl]: https://github.com/plicease/FFI-Platypus-Lang-Rust
[glassful]: https://github.com/kmcallister/glassful
[glassful-r-rust]: https://www.reddit.com/r/rust/comments/2ufeqf/rustlike_syntax_for_opengl_shading_language/
[secretshare]: https://github.com/sellibitze/secretshare
[chip8-rust]: https://github.com/jakerr/chip8-rust
[snowflake]: https://crates.io/crates/snowflake
[rust-dev-box]: https://github.com/estsauver/rust-dev-box
[rust-roller]: https://github.com/freiguy1/rust-roller
[rustbox]: https://github.com/gchp/rustbox

## Project Updates

* [Glium project update][glium]. Glium is a safe wrapper for OpenGL
  that is making a lot of progress.
* [This Week in Servo 21][twis].

[glium]: https://www.reddit.com/r/rust/comments/2ty8ag/glium_project_update/
[twis]: http://blog.servo.org/2015/01/21/twis-21/

## Upcoming Events

* [Feb 3. N Languages in N Months NY][n]. John Barker is giving an
  intro to Rust.
* [Feb 9. Sydney Meetup][syd]. Huon Wilson and Steve Klabnik will be
  in attendance.
* [Feb 9. Seattle Meetup][seattle].

[n]: http://www.meetup.com/N-Languages-in-N-Months-NYC/events/218757056/
[syd]: http://www.meetup.com/Rust-Sydney/events/220100853/
[seattle]: https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg
