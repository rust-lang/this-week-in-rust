Title: This Week in Rust 119
Number: 119
Date: 2016-02-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [Scripting in Dyon without garbage collector](http://blog.piston.rs/2016/02/21/scripting-without-garbage-collector/). Dyon is a dynamically typed programming language written in Rust.
* [(Almost) 6 Months of Rust Runtime Performance](https://dikaiosune.github.io/almost-6-months-rust-runtime-perf.html).
* [Embrace the glow cloud](http://www.poumeyrol.fr/2016/02/15/Embrace-the-glow-cloud/). Part #5 of a series about a BigData in Rust experiment.
* [First steps with Rust and Java Native Interface](http://nitschinger.at/First-Steps-with-Rust-and-JNI/).
* [Parallel iterators part 1: Foundations](http://smallcultfollowing.com/babysteps/blog/2016/02/19/parallel-iterators-part-1-foundations/).
* [First steps towards simple and efficient parsers](https://syslog-ng.org/syslog-ng-and-rust/). Making syslog-ng parser plugins with Rust.
* [Are we web yet? is relaunched](http://www.arewewebyet.org/news/2016/02/16/we-are-back-baby/).
* [This week in Servo 51](http://blog.servo.org/2016/02/15/twis-51/).
* [This week in Redox 11](http://www.redox-os.org/news/this-week-in-redox-11/).
* [This week in Amethyst 5](https://thisweekinamethyst.wordpress.com/2016/02/15/twia-5/). Amethyst is a data-oriented game engine written in Rust.

## Notable New Crates & Project Updates

* [termpix](https://github.com/hopey-dishwasher/termpix). Draw images in an ANSI terminal.
* [tarpc](https://github.com/google/tarpc). An RPC framework for Rust with a focus on ease of use.
* [Rust + Haskell experiments](https://github.com/blitzcode/rust-exp) in software rasterization, N-Body simulation and Game of Life.
* [rust-wlc](https://github.com/Immington-Industries/rust-wlc). Rust bindings for wlc, the Wayland compositor library.
* [Afterparty](https://github.com/softprops/afterparty). A library for building Github webhook integrations in Rust.
* [Hubcaps](https://github.com/softprops/hubcaps). A Rust interface for GitHub.

# Updates from Rust Core

107 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-02-15..2016-02-22

## Notable changes

* [regex: Add a lazy DFA](https://github.com/rust-lang-nursery/regex/pull/164).
* [Enable building freestanding sysroot with cargo](https://github.com/rust-lang/rust/pull/31792).
* [Implement cfg-based target-specific dependencies](https://github.com/rust-lang/cargo/pull/2328).
* [[MIR] A pass to type-check MIR](https://github.com/rust-lang/rust/pull/31474).
* [libsyntax: Restrict where non-inline modules can appear](https://github.com/rust-lang/rust/pull/31534).
* [Implement --unpretty mir](https://github.com/rust-lang/rust/pull/31656).
* [std: use more LFS functions on Linux](https://github.com/rust-lang/rust/pull/31668).
* [Use 32-bit mode_t on 64-bit Android devices](https://github.com/rust-lang/libc/pull/192).
* [Make future-compat lint `match_of_unit_variant_via_paren_dotdot` deny by default](https://github.com/rust-lang/rust/pull/31757).
* [Implement read_volatile and write_volatile](https://github.com/rust-lang/rust/pull/31761).
* [libc moved from rust-lang-nursery to rust-lang](https://github.com/rust-lang/rust/pull/31785).
* [`sys::libc` now has preliminary DragonFly support](https://github.com/rust-lang/libc/pull/194).
* [Another rebase on the 3.8 release branch of LLVM](https://github.com/rust-lang/rust/pull/31791).
* [Implement placement-in protocol for `LinkedList`](https://github.com/rust-lang/rust/pull/31696).

## New Contributors

* Chad Shaffer
* Gökhan Karabulut
* Jack O'Connor
* rphmeier
* Vlad Ureche

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1419: Safe memcpy for slices (`[T]::copy_from_slice`)](https://github.com/rust-lang/rfcs/pull/1419).
* [RFC 1467: Stabilize volatile read and write](https://github.com/rust-lang/rfcs/pull/1467).
* [RFC 1443: Extend atomic compare_and_swap](https://github.com/rust-lang/rfcs/pull/1443).
* [RFC 1461: Move some net2 functionality into libstd](https://github.com/rust-lang/rfcs/pull/1461).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add a `let...else` expression, similar to Swift's `guard let...else`](https://github.com/rust-lang/rfcs/pull/1303).
* [Propose a design for _specialization_, which permits multiple `impl` blocks to apply to the same type/trait](https://github.com/rust-lang/rfcs/pull/1210).

## New RFCs

* [`libstd::sys`, the great `libstd` refactor](https://github.com/rust-lang/rfcs/pull/1502).
* [Add support for 128-bit integers](https://github.com/rust-lang/rfcs/pull/1504).
* [Add a generic `Atomic<T>` type](https://github.com/rust-lang/rfcs/pull/1505).
* [Clarify the relationships between various kinds of structs and variants](https://github.com/rust-lang/rfcs/pull/1506).

# Upcoming Events

* [2/24. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [2/25. Tokyo Rust Meetup #3](http://www.meetup.com/Tokyo-Rust-Meetup/events/228425744/).
* [3/1. Rust Detroit presents Hadoop Next Gen: Using Cloudera Kudu and Mozilla Rust to Crunch Big Data](http://www.meetup.com/rust-detroit/events/224586618/).
* [3/2. Rust Amsterdam - Talks and Drinks](http://www.meetup.com/Rust-Amsterdam/events/227827508/).
* [3/2. Cologne: Live Rust-Coding](http://www.meetup.com/de-DE/Rust-Cologne-Bonn/events/229013352/?eventId=229013352&chapter_analytics_code=UA-63812876-1).
* [3/3. Rust São Paulo Meetup](http://www.meetup.com/Rust-Sao-Paulo-Meetup/events/228868463/).
* [3/11. Darmstadt Rust Table of Regulars](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/228665878/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla.
* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# GSoc Project

Hi students! Looking for an awesome summer project in Rust? Look no further! Chris Holcombe from Canonical is an experienced GSoC mentor and has a project to implement CephX protocol decoding. [Check it out here](https://wiki.ubuntu.com/GoogleSoC2016/Ideas#Decode_CephX_Protocol).

# Crate of the Week

This week's Crate of the Week is [Diesel](http://diesel.rs/), a rustic typesafe extensible object-relational mapper and query builder. Just go to their site; the examples speak for themselves.

Thanks to [LilianMoraru](https://users.rust-lang.org/users/LilianMoraru) and [DroidLogician](https://users.rust-lang.org/users/DroidLogician) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

> There is essentially no webpage out there that it cannot get through at multiple hundreds of frames-per-second

— [pcwalton](https://users.rust-lang.org/users/pcwalton) on Servo's astonishing new [WebRender technology](https://air.mozilla.org/bay-area-rust-meetup-february-2016/).

Thanks to [adwhit](https://users.rust-lang.org/users/adwhit) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
