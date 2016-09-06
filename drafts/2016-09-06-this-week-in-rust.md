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

## New Crates & Project Updates
* [ref_eq](https://github.com/emosenkis/ref_eq)
* [releasetag](https://github.com/frehberg/rust-releasetag) Define release-tags for postmortem analysis, extractable from core-files.
* [bytestool](https://github.com/frehberg/rust-bytestool) Compiler plugins, compile time evaluation, eg. byte_size_of!(b"HELLO"), concat_bytes!(b"HEL", b"LO")

# Crate of the Week

You suggested, you voted, and here you have your crate of the week: [accurate](https://github.com/bsteinb/accurate/), a way to do accurate floating point sums. Thanks to [lifthrasir](https://users.rust-lang.org/users/lifthrasiir) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [tera: Filters to implement (has examples)](https://github.com/Keats/tera/issues/46).
* [easy] [tokei: Update existing languages with their String litreals](https://github.com/Aaronepower/tokei/issues/52).
* [easy] [rust: Error code list which need to be updated to new format](https://github.com/rust-lang/rust/issues/35233).
* [easy] [rustup: Don't capture backtraces without RUST_BACKTRACE=1](https://github.com/rust-lang-nursery/rustup.rs/issues/591#issuecomment-236235677).
  An easy fix, done in two steps, first modifying error-chain, then upgrading it in rustup.
* [hard] [imag: implement bindings to lua/lisp (ketos)/rhai for the filter
  library](https://github.com/matthiasbeyer/imag/issues/245)
* [hard] [filters: implement `BitAnd`, `BitOr`, `BitXor` and `Not` for all
  implementations of `Filter<N>`](https://github.com/matthiasbeyer/filters/issues/4)
* [easy] [tempdir: make directory removal robust on windows](https://github.com/rust-lang-nursery/tempdir/issues/15). This bug lets you
  publish a replacement for the unreliable `std::fs::remove_dir_all` fn.
* [easy] [rust: Add version info to source tarball](https://github.com/rust-lang/rust/issues/32444).
  Do you love makefiles?
* [moderate] [rust: Create official .deb packages](https://github.com/rust-lang/rust/issues/28307).
* [easy] [rustup: Fix PATH order for proxy binaries](https://github.com/rust-lang-nursery/rustup.rs/issues/475#issuecomment-241792606).
  Simple task for an important tool.
* [moderate] [rustup: Check for unexpected cargo/rustc during install](https://github.com/rust-lang-nursery/rustup.rs/issues/681).
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

* changchun.fan
* Daniele Baracchi
* Mohit Agarwal
* Paul Fanelli
* Shyam Sundar B
* zjhmale

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
* [Eager expansion of macros](https://github.com/rust-lang/rfcs/pull/1628).
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).
* [regex 1.0](https://github.com/rust-lang/rfcs/pull/1620).
* [Allow type aliases in enumeration repr attributes](https://github.com/rust-lang/rfcs/pull/1605).
* [Introduce `dyn` keyword](https://github.com/rust-lang/rfcs/pull/1603).
* [Add a `vis` matcher to `macro_rules!` that matches valid visibility annotations](https://github.com/rust-lang/rfcs/pull/1575).
* [Saturating and checking integer wrapper types](https://github.com/rust-lang/rfcs/pull/1534).
* [`libstd::sys`, the great `libstd` refactor](https://github.com/rust-lang/rfcs/pull/1502).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

* 8/31. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [9/5. Rust Cologne/Bonn: Compiling Rust to your Browser](http://rustaceans.cologne/2016/09/05/compile-to-js.html).
* [9/5. Zurich / Switzerland: Rust Meetup - Lecture: Rust<T>](http://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/233292936/).
* 9/7. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 9/8. Rust release triage at #rust-triage on irc.mozilla.org.
* [9/8. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/232660905/).
* **[9/9. RustConf 2016](http://rustconf.com/)**.
* [9/9. Rust Table of Regulars Darmstadt](https://www.meetup.com/de-DE/Rust-Rhein-Main/events/233544580/).
* [9/9. Tokio Hack Night](https://tokiohacknight.splashthat.com/).
* [9/12. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [9/20. Rust NYC Meetup](https://www.meetup.com/Rust-NYC/events/233756447/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Rust engineer at MaidSafe](http://maidsafe.net/careers.html#rust_engineer).
* [Rust developer at ANIXE](http://anixe.pl/rust_dev/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> My Rust memoirs will be called "Once in 'a lifetime".

— [@Argorak on Twitter](https://twitter.com/Argorak/status/768040922030432256).

Thanks to [Vincent Esche](https://users.rust-lang.org/users/regexident) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
