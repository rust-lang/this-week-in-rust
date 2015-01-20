Title: This Week in Rust 66
Date: 2015-01-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This Week in Rust is openly developed [on Github](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

119 pull requests were [merged in the last week][code], and 1 [RFC][rfcs].

[code]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-12..2015-01-18
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-12..2015-01-18

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* `AtomicInt` and `AtomicUint` have been [renamed][atomic] to
  `AtomicIsize` and `AtomicUsize` to match their corresponding integer
  types.
* To fix a bug in coherence [builtin traits can only be implemented
  for structs and enums][cohere].

[atomic]: https://github.com/rust-lang/rust/pull/20896
[cohere]: https://github.com/rust-lang/rust/pull/21167

## Other Changes

* Certain long error messages of the form 'expected foo found bar' are
  now [split neatly across multiple lines][multiline]. Examples in the
  PR.
* UFCS method calls can now be [qualified by the trait][ufcs] of the
  method.  This can be used to disambiguate method calls when multiple
  applicable methods are in scope, e.g. `<i32 as Add<_>>::add(1, 2)`
  which is equivalent to `1.add(2)`. [RFC][ufcs-rfc].
* Negative impls are [partially implemented][negimpl], though appear
  to still be special-cased to the `Send` and `Sync`
  traits. [RFC][negimpl-rfc].
* Mutexes on Windows are faster now they are [implemented with Slim
  Reader Writer Locks][mutex].
* The `#[rustc_on_unimplemented]` attribute, requiring the
  'on_unimplemented' feature, lets rustc [display custom error
  messages when a trait is expected to be implemented for a type but
  is not][onun].
* [Preliminary support for PowerPC][powerpc].
* Fatal runtime errors are [now suitably boring][bore].

[multiline]: https://github.com/rust-lang/rust/pull/19870
[mutex]: https://github.com/rust-lang/rust/pull/20367
[onun]: https://github.com/rust-lang/rust/pull/20889
[negimpl]: https://github.com/rust-lang/rust/pull/20972
[negimpl-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md
[powerpc]: https://github.com/rust-lang/rust/pull/20980
[ufcs]: https://github.com/rust-lang/rust/pull/21077
[ufcs-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0132-ufcs.md
[bore]: https://github.com/rust-lang/rust/pull/20944

## New Contributors

* Aidan Hobson Sayers
* Alexander Korolkov
* Andrew Barchuk
* Cam Jackson
* CarVac
* Diggory Blake
* Fenhl
* Greg Chapple
* Jeff Belgum
* Lauri Lehmijoki
* nathan dotz
* Paul Crowley
* Seth Faxon
* Toni Cárdenas
* Travis Watkins

# Approved RFC's

* [517: io and os reform (skeleton)][rfc-517]. Updating the `io` and
  `os` modules is a large task and at the moment the RFC only lays out
  the motivation. It will be updated incrementally in further pull
  requests. [PR][rfc-517-pr]. Many follow-ups have been posted for
  review: [string handling][rfc-517-string],
  [`Reader`/`Writer`][rfc-517-readwrite], [deadlines][rfc-517-dead],
  [`std::env`][rfc-517-env], [`std::process`][rfc-517-process].

[rfc-517]: https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md
[rfc-517-pr]: https://github.com/rust-lang/rfcs/pull/517
[rfc-517-readwrite]: https://github.com/rust-lang/rfcs/pull/576
[rfc-517-string]: https://github.com/rust-lang/rfcs/pull/575
[rfc-517-dead]: https://github.com/rust-lang/rfcs/pull/577
[rfc-517-env]: https://github.com/rust-lang/rfcs/pull/578
[rfc-517-process]: https://github.com/rust-lang/rfcs/pull/579

# New RFC's

* [Use `isz`/`usz` as literal suffix for `isize`/`usize`][rfc-573-pr].
* [Replace `Vec::drain` by a method that accepts a range parameter][rfc-574-pr].
* [Rename (maybe one of) the standard collections for
  consistency][rfc-580-pr]. Proposes several possible renamings.
* [Add `foreach` to iterators][rfc-582-pr]. An internal iterator.
* [Add `fmt` size hints][rfc-583-pr]. Allows buffers to be optimized.
* [Remove `proc` keyword][rfc-584-pr]. Why isn't it gone already?
* [Negative bounds][rfc-586-pr]. Gives flexibility to avoid coherence
  conflicts.
* [Make return types of `Fn` traits associated
  types][rfc-587-pr]. More flexible because it doesn't require generic
  return types to be constrained.
* [Early trait bounds on generic types][rfc-590-pr]. Create a
  convention to always apply trait bounds to wrapper types to improve
  error messages when the bounds on the underlying types or methods
  can't be satisfied.
* [sizeof, alignof, offsetof, typeof][rfc-591-pr].
* [CStr, the dereferenced complement to CString][rfc-592-pr].
* [Forbid identifier `Self` for definitions][rfc-593-pr]. Fixing a
  weird corner case that results from `Self` not being a keyword.
* [Support incremental compilation][rfc-594-pr]. Rearchitect the
  compiler.
* [Output option handling][rfc-595-pr]. Make rustc's `-o`,
  `--out-dir`, and `--emit` options more consistent.

[rfc-573-pr]: https://github.com/rust-lang/rfcs/pull/573
[rfc-574-pr]: https://github.com/rust-lang/rfcs/pull/574
[rfc-580-pr]: https://github.com/rust-lang/rfcs/pull/580
[rfc-582-pr]: https://github.com/rust-lang/rfcs/pull/582
[rfc-583-pr]: https://github.com/rust-lang/rfcs/pull/583
[rfc-584-pr]: https://github.com/rust-lang/rfcs/pull/584
[rfc-586-pr]: https://github.com/rust-lang/rfcs/pull/586
[rfc-587-pr]: https://github.com/rust-lang/rfcs/pull/587
[rfc-590-pr]: https://github.com/rust-lang/rfcs/pull/590
[rfc-591-pr]: https://github.com/rust-lang/rfcs/pull/591
[rfc-592-pr]: https://github.com/rust-lang/rfcs/pull/592
[rfc-593-pr]: https://github.com/rust-lang/rfcs/pull/593
[rfc-594-pr]: https://github.com/rust-lang/rfcs/pull/594
[rfc-595-pr]: https://github.com/rust-lang/rfcs/pull/595

# Community

The [periodic table of Rust types][period] has been updated, and
/r/rust finally hit [8 trillion subscribers][trill].

[period]: http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types/
[trill]: https://www.reddit.com/r/rust/comments/2sn91h/8_trillion_subscribers_we_did_it_reddit/

## From the Team

* [Weekly-meetings/2015-01-13][mtg]. fott; homu; integer overflow;
  I/O; 1.0; comment RFC

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-01-13.md

## Blog Posts

* [Thoughts about Rust from a D programmer][d]. Good
  comparison. [/r/rust][d-r-rust]. [/r/programming][d-r-programming].
* [Recent syntactic changes][syntax]. Nick discusses some of the
  last-minute syntax changes he's recently landed.
* [Little Orphan Impls][orphan]. Niko discusses options for coherence
  rules. [/r/rust][orphan-r-rust].
* [Unboxed Closures and FFI Callbacks][ffi]. How to use closures as
  foreign callbacks.
* [A Quick Comparison of Nim
  vs. Rust][nim]. [/r/rust][nim-r-rust]. [/r/programming][nim-r-programming].
* [Mooney GB: A Gameboy emulator written in Rust][mooneye].
* [Building a HashMap in Rust - Part 1: What's a
  Hashmap?][hashmap]. Gankro on the Rust
  HashMap. [/r/rust][hashmap-r-rust].
* [Worklog 2015-01-17: Rustdoc testing][rustdoc]. Yurume on his new
  patch to test rustdoc output.
* [Solving Project Eueler with Rust][euler]. A walk through of the
  first problem.
* [Getting started contributing to Rust][started]. kmc's slides from
  the Saturday event in SF.
* [Explore ownership in Rust][pwned]. Good entry level discussion of
  ownership. [/r/rust][pwned-r-rust].
* [DTrace on Rust][dtrace].
* [SystemTap on Rust][systemtap]. Not to be outdone.
* [Summary of Korean Rust Meetup #4][korea]. The Korean Rust
  contingent is always up to interesting stuff.

[d]: http://blog.dicebot.lv/2015/01/thoughts-about-rust-from-d-programmer.html
[d-r-rust]: https://www.reddit.com/r/rust/comments/2s7bnt/thoughts_about_rust_from_d_programmer/
[d-r-programming]: https://www.reddit.com/r/programming/comments/2s70mm/thoughts_about_rust_from_a_d_programmer/
[nim-hn]: http://news.ycombinator.com/item?id=8883791
[nim-r-rust]: https://www.reddit.com/r/rust/comments/2sd5rv/a_quick_comparison_of_nim_vs_rust/
[nim-r-programming]: https://www.reddit.com/r/programming/comments/2scodb/a_quick_comparison_of_nim_vs_rust/
[syntax]: http://featherweightmusings.blogspot.co.nz/2015/01/recent-syntactic-changes-to-rust.html
[euler]: http://unlogic.co.uk/2015/01/12/solving-project-euler-with-rust-1/
[dtrace]: https://twitter.com/bcantrill/status/555143487482368000
[mooneye]: http://gekkio.fi/blog/2015-01-13-mooneye-gb-a-gameboy-emulator-written-in-rust.html
[systemtap]: https://gist.github.com/cuviper/08239cbae023411a29d9
[orphan]: http://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/
[orphan-r-rust]: https://www.reddit.com/r/rust/comments/2sfm4a/little_orphan_impls_nikos_blog/
[hashmap]: http://cglab.ca/~abeinges/blah/robinhood-part-1/
[hashmap-r-rust]: https://www.reddit.com/r/rust/comments/2sjayc/building_a_hashmap_in_rust_part_1_whats_a_hashmap/
[ffi]: http://aatch.github.io/blog/2015/01/17/unboxed-closures-and-ffi-callbacks/
[rustdoc]: https://lifthrasiir.github.io/rustlog/worklog-2015-01-17.html
[started]: http://kmcallister.github.io/talks/rust/2015-contributing-to-rust/slides.html
[pwned]: http://nercury.github.io/rust/guide/2015/01/19/ownership.html
[pwned-r-rust]: https://www.reddit.com/r/rust/comments/2sv4uv/explore_ownership_system_in_rust/
[korea]: https://gist.github.com/lifthrasiir/93ba9f6aec7bd2113941

## Videos

* [Servo: Building a Parallel Web Browser][servo]. Jack Moffit at
  linux.conf.au. [HN][servo-hn]. [/r/programming][servo-r-programming].

[servo]: https://youtu.be/7q9vIMXSTzc
[servo-hn]: https://news.ycombinator.com/item?id=8899812
[servo-r-programming]: https://www.reddit.com/r/programming/comments/2lcap0/an_introduction_to_servo/

## Discussions

* [Small string optimization, remove as_mut_vec][small]. The current
  definition of `Vec` doesn't allow the common 'small string'
  optimization. [/r/rust][small-r-rust].
* [Does Rust let you develop faster than C++?][fast]. TL;DR ¯\_(ツ)_/¯
* [Using Rust 1.0 for video game development][games]. Yes, please.
* [Rust support in Visual Studio][vs]. Doesn't exist yet, sorry.
* [Rust Berlin community efforts][berlin]. Berlin needs you!
* [By value operator overloading problems][over]. Implementing math
  operators for non-copyable types means you have to pass them by
  reference, like `&a + &b`, which some consider unsightly.

[games]: https://www.reddit.com/r/rust/comments/2s4kp9/using_rust_10_for_video_game_development/
[vs]: https://www.reddit.com/r/rust/comments/2s5d65/rust_support_in_visual_studio/
[fast]: https://www.reddit.com/r/rust/comments/2sa2qx/does_rust_let_you_develop_faster_than_c/
[berlin]: https://www.reddit.com/r/rust/comments/2sbkuo/rustberlin_community_efforts/
[small]: http://discuss.rust-lang.org/t/small-string-optimization-remove-as-mut-vec/1320
[small-r-rust]: https://www.reddit.com/r/rust/comments/2slcs8/small_string_optimization_remove_as_mut_vec/
[over]: https://www.reddit.com/r/rust/comments/2srz0g/by_value_operator_overloading_problems/

## New Projects

* [launch-code]. Cryptographic auditing of unsafe code. [/r/rust][launch-code-r-rust].
* [rust-cipher]. Encryption with XSalsa20 and BLAKE2B-512 in the
  Encrypt-then-MAC mode.
* [rust-haskell-ffi]. Example of calling Rust from Haskell.
* [dynamodule]. Experiments with dynamic OOP.
* [rdb-rs]. A Redis RDB parser.
* [carboxyl]. Functional reactive programming.
* [Sparkle]. An entity component system. [/r/rust][Sparkle-r-rust].
* [rust-erl-ext]. Erlang 'external term' parser.
* [rs-intrusive]. A system for creating intrusive data structures,
  Rust's kryptonite.
* [handlebars-iron]. Template middleware for the Iron web framework.
* [hematite_server]. Minecraft server clone.

[launch-code]: https://github.com/kmcallister/launch-code
[launch-code-r-rust]: https://www.reddit.com/r/rust/comments/2sc7oq/cryptographic_signatures_for_auditing_unsafe_code/
[rust-cipher]: https://github.com/zenith-nz/rust-cipher
[rust-haskell-ffi]: https://github.com/aisamanra/rust-haskell-ffi
[dynamodule]: https://github.com/kmcallister/dynamodule
[rdb-rs]: http://fnordig.de/2015/01/15/rdb-rs-fast-and-efficient-rdb-parsing-utility/
[carboxyl]: https://github.com/aepsil0n/carboxyl
[Sparkle]: https://github.com/RustSparkle/Sparkle
[Sparkle-r-rust]: https://www.reddit.com/r/rust/comments/2srrx0/another_entity_component_system/
[rust-erl-ext]: https://github.com/seriyps/rust-erl-ext
[rs-intrusive]: https://github.com/aidancully/rs-intrusive
[handlebars-iron]: https://github.com/sunng87/handlebars-iron
[hematite_server]: https://github.com/PistonDevelopers/hematite_server

## Project Updates

* [This Week in Servo 19][twis].
* [Worklog 2015-01-13: Time zones, chrono, and associated
  types][chrono]. Yurume discusses updates to rust-chrono.

[twis]: http://blog.servo.org/2015/01/13/twis-19/
[chrono]: https://lifthrasiir.github.io/rustlog/worklog-2015-01-13.html

## Upcoming Events

* [January 19 - Rust Paris](http://www.meetup.com/Rust-Paris)
* [January 20 - Rust Berlin](http://www.meetup.com/Rust-Berlin/events/219070839/)
* [January 20 - Fast, Safe, and
  Beautiful](http://www.oreilly.com/pub/e/3291). Jim Blandy's webcast
  on Rust for O'Reilly.
* [January 28 - Rust Amsterdam](http://www.meetup.com/Rust-Amsterdam/events/218908906/)
