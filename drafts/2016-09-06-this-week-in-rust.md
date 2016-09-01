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

# Crate of the Week

No crate was suggested for this week. So I unilaterally declare [ring](https://crates.io/crates/ring), Brian Smith's Rust crypto implementation, which is finally on crates.io as this week's crate.

Can we have suggestions and votes next week? Pretty please?

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

138 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-08-22..2016-08-29

* [Don't round up DST prefix size to alignment](https://github.com/rust-lang/rust/pull/36027)
* [Stabilize type macros](https://github.com/rust-lang/rust/pull/36014)
* [Fix line numbers in macro expansion](https://github.com/rust-lang/rust/pull/35238)
* [Fixed lifetime rules for `if` conditions](https://github.com/rust-lang/rust/pull/36029)
* [Borrowck no longer hashes types in loan paths](https://github.com/rust-lang/rust/pull/36004)
* [Substs now interleave types and regions](https://github.com/rust-lang/rust/pull/36002)
* [`char::decode_utf8` now yields errors per Unicode](https://github.com/rust-lang/rust/pull/35947)
* [Trans no longer generates `alloca`s for unused locals](https://github.com/rust-lang/rust/pull/35916)
* [Typeck now uses `NoExpectation` to check type of diverging `fn`](https://github.com/rust-lang/rust/pull/35883)
* [Fixed ICE in typeck on missing arg types in impl/trait methods](https://github.com/rust-lang/rust/pull/35877)
* [Corrected memrchr alignment computation](https://github.com/rust-lang/rust/pull/35969) (lead to crashes on non-linux ARMv7 before)
* [Avoid `Vec` growth to 0-terminate `CString`](https://github.com/rust-lang/rust/pull/35871)
* [Hash HIR elements only once at the beginning](https://github.com/rust-lang/rust/pull/35854)
* [Trans: Removed AST backend](https://github.com/rust-lang/rust/pull/35764) (MIR point of no return reached)
* [Improved demangling of Rust symbols](https://github.com/rust-lang/rust/pull/36059)
* [`FusedIterator`](https://github.com/rust-lang/rust/pull/35656) (implements [RFC #1581](https://github.com/rust-lang/rfcs/pull/1581))
* [`impl CoerceUnsized for` {`Cell`, `RefCell`, `UnsafeCell`}`](https://github.com/rust-lang/rust/pull/35627)
* [Transmuting from `fn` item to pointer-sized types is now an error](https://github.com/rust-lang/rust/pull/34923)
* [`cargo metadata` now works with workspaces](https://github.com/rust-lang/cargo/pull/3051)

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
