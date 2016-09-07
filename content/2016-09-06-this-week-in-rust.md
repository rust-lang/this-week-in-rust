Title: This Week in Rust 146
Number: 146
Date: 2016-09-06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* [Rust's vision for the 2017 cycle](https://internals.rust-lang.org/t/setting-our-vision-for-the-2017-cycle/3958).
* [One year of Rust](https://mgattozzi.github.io/2016/08/30/1-year-of-rust.html). How Rust and its community has changed in last one year and what we as a community can do differently.
* [The next version of Fedora picks up Rust](http://www.infoworld.com/article/3114475/open-source-tools/the-next-version-of-fedora-picks-up-rust.html).
* [Thoughts on conducting a beginner level Rust workshop](http://pramode.in/2016/09/05/on-teaching-rust-to-beginners/).
* [Wasted](https://llogiq.github.io/2016/08/30/wasted.html). [@llogiq](https://github.com/llogiq) has ideas on reducing network bandwidth when using rustup & Cargo.
* [Porting cargo benchcmp from Python to Rust](https://apanatshka.github.io/compsci/2016/09/04/porting-cargo-benchcmp/).
* [Porting a Raspberry Pi GPIO programming library from Python to Rust](http://pramode.in/2016/08/31/rust-library-for-rpi-gpio-pgming/).
* [Rust merge process for new contributors](https://blog.guillaume-gomez.fr/articles/2016-08-31+Rust+merge+process).

## New Crates & Project Updates

* [libfringe](https://github.com/nathan7/libfringe). A Rust library implementing safe, lightweight context switches, without relying on kernel services.
* [lewton](https://github.com/est31/lewton). Vorbis decoder written in pure Rust.
* [Are we game yet?](http://arewegameyet.com/). The state of game development in Rust.
* [Scaproust](https://github.com/blabaere/scaproust). Scaproust is an implementation of the nanomsg "Scalability Protocols" in the Rust programming language.
* [log_buffer](https://github.com/whitequark/rust-log_buffer). A zero-allocation ring buffer for storing text logs.
* [query_interface](https://github.com/Diggsey/query_interface). Dynamically query a type-erased object for any trait implementation.
* [Plantex](https://github.com/OsnaCS/plantex). Open-world exploration game with procedurally generated content.
* [ref_eq](https://github.com/emosenkis/ref_eq). Determine if two borrowed pointers point to the same thing.
* [releasetag](https://github.com/frehberg/rust-releasetag). Define release-tags for postmortem analysis, extractable from core-files.
* [bytestool](https://github.com/frehberg/rust-bytestool). Compiler plugins, compile time evaluation, eg. `byte_size_of!(b"HELLO")`, `concat_bytes!(b"HEL", b"LO")`.
* [Videos about Rust](https://daily-rust.github.io/2016/09/01/videos.html). Videos about Rust language found in the Web.
* [spectral](https://github.com/cfrancia/spectral). Fluent test assertions.
* [This Week in Rust Docs 20](http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-20).
* [This week in TiKV 2016-09-05](http://www.pingcap.com/tikv/2016/09/05/tikv-weekly/).

# Crate of the Week

You suggested, you voted, and here you have your crate of the week: [accurate](https://github.com/bsteinb/accurate/), a way to do accurate floating point sums. Thanks to [lifthrasir](https://users.rust-lang.org/users/lifthrasiir) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Document what `Default` does for each type in libstd](https://github.com/rust-lang/rust/issues/36265).
* [easy] [rust: Silent overflow on debug beta/nightly](https://github.com/rust-lang/rust/issues/36110).
* [easy] [rust: `mem::replace` and `mem::swap` say they don't copy, but they do](https://github.com/rust-lang/rust/issues/35935).
* [hard] [rust: Support Apple app store bitcode](https://github.com/rust-lang/rust/issues/35968).
* [hard] [rust: Missed opportunities to eliminate bounds checks](https://github.com/rust-lang/rust/issues/35981).
* [easy] [tera: Filters to implement (has examples)](https://github.com/Keats/tera/issues/46).
* [easy] [tokei: Update existing languages with their String litreals](https://github.com/Aaronepower/tokei/issues/52).
* [easy] [tempdir: make directory removal robust on windows](https://github.com/rust-lang-nursery/tempdir/issues/15). This bug lets you
  publish a replacement for the unreliable `std::fs::remove_dir_all` fn.
* [moderate] [rust: Create official .deb packages](https://github.com/rust-lang/rust/issues/28307).
* [easy] [rust-www: Errors displayed after running front-page code look bad](https://github.com/rust-lang/rust-www/issues/490).
  Important bug - first impressions matter.
* [easy] [rust-www: Better front-page example](https://github.com/rust-lang/rust-www/issues/180).
  The front page example on the website isn't so special. Make it shine.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

146 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-08-29..2016-09-05

* [Implement untagged unions](https://github.com/rust-lang/rust/pull/36016) ([RFC #1444](https://github.com/rust-lang/rfcs/pull/1444))
* [Implement custom derive](https://github.com/rust-lang/rust/pull/35957) ([RFC #1681](https://github.com/rust-lang/rust/pull/35957): Macros 1.1)
* [Implement item-like imports](https://github.com/rust-lang/rust/pull/35894) ([RFC #1560](https://github.com/rust-lang/rfcs/pull/1560))
* [Default lifetimes in `static`/`const`s to `'static](https://github.com/rust-lang/rust/pull/35915) ([RFC #1623](https://github.com/rust-lang/rfcs/pull/1623), missing a feature gate for now. Llogiq apologizes profusely)
* [Allow all literals in attributes](https://github.com/rust-lang/rust/pull/35850) ([RFC #1559](https://github.com/rust-lang/rfcs/pull/1559))
* [Unsized tuple warnings are now errors](https://github.com/rust-lang/rust/pull/34982) ([RFC #1592](https://github.com/rust-lang/rfcs/pull/1592) finally in effect)
* [LLVM: Invalidate metadata on SimplifyCFG hoisting](https://github.com/rust-lang/llvm/pull/48) (fixes segfaults)
* [Improved Rust symbol demangling](https://github.com/rust-lang/rust/pull/36059)
* [Fix illegal instruction on overflow in channel cloning](https://github.com/rust-lang/rust/pull/36104)
* [Fix perf regression when working on arrays](https://github.com/rust-lang/rust/pull/36124)
* [Better lifetime error messages with temporary variables](https://github.com/rust-lang/rust/pull/36171)
* [Rust now warns about conflicting `#[repr(..)]`s](https://github.com/rust-lang/rust/pull/34623)
* [Fix `#[derive(..)]` for empty tuple structs/variants](https://github.com/rust-lang/rust/pull/35728)
* [Syntax/HIR: Generics now have their own Span](https://github.com/rust-lang/rust/pull/35591) (plugin-breaking change)
* [Cache projections in trans](https://github.com/rust-lang/rust/pull/35761) (speeds up rustc)
* [Copy-on-Write for incremental compilation caches](https://github.com/rust-lang/rust/pull/35718)
* [`Iterator::`{`min`, `max`}`_by`](https://github.com/rust-lang/rust/pull/35856)
* [`impl Debug for std::path::`{`Components`, `Iter`}](https://github.com/rust-lang/rust/pull/36101)
* [`std::convert` traits implemented for `char`](https://github.com/rust-lang/rust/pull/35755)
* [Condition Variables hardened against time travel](https://github.com/rust-lang/rust/pull/35048) (Remember kids, it's dangerous!)
* [Fix GDB pretty-printing special-cased Rust types](https://github.com/rust-lang/rust/pull/35585)
* [New `rustc --Zsave-analysis-api` option](https://github.com/rust-lang/rust/pull/36132)
* [`cargo --all-features`](https://github.com/rust-lang/cargo/pull/3038) (surprisingly builds with all features enabled)

## New Contributors

* Abhishek Kumar
* Andrea Corradi
* athulappadan
* Eugene R Gonzalez
* Fabian Zaiser
* johnthagen
* Keunhong Lee
* king6cong
* Matt Ickstadt
* philipp
* QuietMisdreavus
* Sebastian Ullrich

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [`mem::discriminant()`](https://github.com/rust-lang/rfcs/pull/1696). Add a function that extracts the discriminant from an enum variant as a comparable, hashable, printable, but (for now) opaque and unorderable type.
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).
* [regex 1.0](https://github.com/rust-lang/rfcs/pull/1620).
* [Saturating and checking integer wrapper types](https://github.com/rust-lang/rfcs/pull/1534).
* [`libstd::sys`, the great `libstd` refactor](https://github.com/rust-lang/rfcs/pull/1502).

## New RFCs

* [Rename the current `?` operator to `?!` to improve code readability and language consistency](https://github.com/rust-lang/rfcs/pull/1737).
* [`core::mem::replace_with` for temporarily moving out of ownership](https://github.com/rust-lang/rfcs/pull/1736).
* [Traits should be aliased the same way types can be aliased with the `type` keyword](https://github.com/rust-lang/rfcs/pull/1733).

# Upcoming Events

* 9/7. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 9/8. Rust release triage at #rust-triage on irc.mozilla.org.
* [9/8. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/232660905/).
* **[9/9. RustConf 2016](http://rustconf.com/)**.
* [9/9. Rust Table of Regulars Darmstadt](https://www.meetup.com/de-DE/Rust-Rhein-Main/events/233544580/).
* [9/9. Tokio Hack Night](https://tokiohacknight.splashthat.com/).
* [9/12. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* 9/14. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [9/14. Rust Boulder/Denver Monthly Meeting](https://www.meetup.com/Rust-Boulder-Denver/events/233463725/).
* **[9/17. Rustfest Europe Conference](http://www.rustfest.eu/)**.
* [9/19. Paris - Rust Paris](https://www.meetup.com/Rust-Paris/events/230111512/).
* [9/20. Rust NYC Meetup](https://www.meetup.com/Rust-NYC/events/233756447/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Research Engineer - Servo at Mozilla](https://careers.mozilla.org/position/gh/267268).
* [Rust engineer at MaidSafe](http://maidsafe.net/careers.html#rust_engineer).
* [Rust developer at ANIXE](http://anixe.pl/rust_dev/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
