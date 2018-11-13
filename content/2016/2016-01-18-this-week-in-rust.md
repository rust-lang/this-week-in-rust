Title: This Week in Rust 114
Number: 114
Date: 2016-01-18
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

This week's edition was edited by: [nasa42](https://github.com/nasa42), [brson](https://github.com/brson), and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [Guide: Contributing to the Rust compiler](http://gregchapple.com/contributing-to-the-rust-compiler/).
* [A type-safe and zero-allocation library for reading and navigating ELF files](http://www.ncameron.org/blog/a-type-safe-and-zero-allocation-library-for-reading-and-navigating-elf-files/).
* [podcast] [New Rustacean podcast episode 09](http://www.newrustacean.com/show_notes/e009/). Getting into the nitty-gritty with Rust's traits.
* [ArcadeRS 1.12: Brawl, at last](https://jadpole.github.io/arcaders/arcaders-1-12/)! Part of the series [ArcadeRS 1.0: The project](https://jadpole.github.io/arcaders/arcaders-1-0/) - a series whose objective is to explore the Rust programming language and ecosystem through the development of a simple, old-school shooter.
* [Raspberry Pi bare metal programming with Rust](https://blog.thiago.me/raspberry-pi-bare-metal-programming-with-rust/).
* [This week in Servo 47](http://blog.servo.org/2016/01/11/twis-47/).
* [This week in Redox OS 10](http://www.redox-os.org/news/this-week-in-redox-10/).

## Notable New Crates & Project Updates

* [Amethyst](https://github.com/ebkalderon/amethyst). Data-oriented game engine written in Rust.
* [Rust website](https://www.rust-lang.org/) has received some [major updates](https://www.reddit.com/r/rust/comments/40zxey/major_website_updates/).
* [Rust](https://packages.debian.org/stretch/rustc) and [Cargo](https://packages.debian.org/stretch/cargo) are now available in Debian stretch.
* [Rust on Particle: Call for contributors](https://community.particle.io/t/rust-on-particle-call-for-contributors/19090).
* [capnp-rpc-rust rewritten to use async I/O](https://dwrensha.github.io/capnproto-rust/2016/01/11/async-rpc.html).
* [Palette](https://github.com/Ogeon/palette). A Rust library for linear color calculations and conversion.

# Updates from Rust Core

164 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-01-11..2016-01-18

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-tue-jan-05-2016/3052
[subteam]: https://internals.rust-lang.org/t/subteam-reports-2016-01-08/3067

## Notable changes

* [std: Stabilize APIs for the 1.7 release](https://github.com/rust-lang/rust/pull/30943).
* [Refactor and improve: Arena, TypedArena](https://github.com/rust-lang/rust/pull/27807).
* [Let `str::replace` take a pattern](https://github.com/rust-lang/rust/pull/29498).
* [rustc_resolve: Fix bug in duplicate checking for extern crates](https://github.com/rust-lang/rust/pull/30295).
* [Rewrite BTreeMap to use parent pointers](https://github.com/rust-lang/rust/pull/30426).
* [Support generic associated consts](https://github.com/rust-lang/rust/pull/30446).
* [Add an `impl` for `Box<Error>` from String](https://github.com/rust-lang/rust/pull/30509).
* [Introduce "obligation forest" data structure into fulfillment to track backtraces](https://github.com/rust-lang/rust/pull/30533).
* [Remove negate_unsigned feature gate](https://github.com/rust-lang/rust/pull/30538).
* [llvm: Add support for vectorcall (X86_VectorCall) convention](https://github.com/rust-lang/rust/pull/30567).
* [Make coherence more tolerant of error types](https://github.com/rust-lang/rust/pull/30676).
* [Add fast path for ASCII in UTF-8 validation](https://github.com/rust-lang/rust/pull/30740).
* [Downgrade unit struct match via S(..) warnings to errors](https://github.com/rust-lang/rust/pull/30753).
* [Move const block checks before lowering step](https://github.com/rust-lang/rust/pull/30930).
 
## New Contributors

* Anton Blanchard
* Jonas Tepe
* Jörg Krause
* Joshua Olson
* kalita.alexey
* Pierre Krieger
* Sergey Veselkov
* Simon Martin
* Steffen
* tomaka

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1331: `src/grammar` for the canonical grammar of the Rust language](https://github.com/rust-lang/rfcs/pull/1331).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add `[` to the FOLLOW(ty) in macro future-proofing rules](https://github.com/rust-lang/rfcs/pull/1462).
* [Rewrite `for` loop desugaring to use language items](https://github.com/rust-lang/rfcs/pull/1457).
* [Amend 1192 (RangeInclusive) to use an enum](https://github.com/rust-lang/rfcs/pull/1320).
* [Trait-based exception handling](https://github.com/rust-lang/rfcs/pull/243).
* [Improve Cargo target-specific dependencies](https://github.com/rust-lang/rfcs/pull/1361).
* [Add a `IndexAssign` trait that allows overloading "indexed assignment" expressions like `a[b] = c`](https://github.com/rust-lang/rfcs/pull/1129).
* [Allow eliding more type parameters](https://github.com/rust-lang/rfcs/pull/1196).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Add a used attribute to prevent symbols from being discarded](https://github.com/rust-lang/rfcs/pull/1459).
* [Move some net2 functionality into libstd](https://github.com/rust-lang/rfcs/pull/1461).
* [Add `some!` macro for unwrapping Option more safely](https://github.com/rust-lang/rfcs/pull/1465).
* [Stabilize the `volatile_load` and `volatile_store` intrinsics as `ptr::volatile_read` and `ptr::volatile_write`](https://github.com/rust-lang/rfcs/pull/1467).

# Upcoming Events

* [1/19. Rust Hack and Learn Hamburg @ Ponton](http://www.meetup.com/Rust-Meetup-Hamburg/events/227838367/).
* [1/21. SF Bay Area: Rust Concurrency and Parallelism](http://www.meetup.com/Rust-Bay-Area/events/227841778/).
* [1/27. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Rust Engineer](http://maidsafe.net/rust_engineer.html) at MaidSafe.
* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla.
* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [toml](https://github.com/alexcrichton/toml-rs), a crate for all our configuration needs, simple yet effective.

Thanks to [Steven Allen](https://users.rust-lang.org/users/stebalien) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

> Borrow/lifetime errors are usually Rust compiler bugs.
> Typically, I will spend 20 minutes detailing the precise conditions of
> the bug, using language that understates my immense knowledge, while
> demonstrating sympathetic understanding of the pressures placed on a
> Rust compiler developer, who is also probably studying for several exams
> at the moment. The developer reading my bug report may not understand
> this stuff as well as I do, so I will carefully trace the lifetimes of
> each variable, where memory is allocated on the stack vs the heap, which
> struct or function owns a value at any point in time, where borrows
> begin and where they... oh yeah, actually that variable really doesn't
> live long enough.

— [peterjoel on /r/rust](https://www.reddit.com/r/rust/comments/4084yx/my_trick_when_i_get_stuck_as_a_beginner/cysqz3s).

Thanks to [Wa Delma](https://users.rust-lang.org/users/WaDelma) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
