Title: This Week in Rust 142
Number: 142
Date: 2016-08-09
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

# Updates from Rust Community

## News & Blog Posts

* [imag usecases](http://beyermatthias.de/blog/2016/08/07/imag-usecases/).
  imag usecases explained.

## New Crates & Project Updates

* [Exar DB](https://github.com/bfil/exar-db). An event store with streaming support.

# Crate of the Week

This week's Crate of the Week is MaidSafe's [lru_time_cache](https://crates.io/crates/lru_time_cache), a simple but complete least-recently-used cache implementation. Thanks [gregwtmtno](https://users.rust-lang.org/users/gregwtmtno) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust book: Script to lint for copypasta'd file paths that should be generic](https://github.com/rust-lang/book/issues/123). Create a little script that will help the next version of the book have fewer errors!
* [easy] [rust-dashboard: Fix '0 days since accident' sign](https://github.com/dikaiosune/rust-dashboard/issues/71).
  Easy bug on important piece of infrastructure that needs more contributors.
* [easy] [rust-dashboard: User-defined date rang](https://github.com/dikaiosune/rust-dashboard/issues/26).
  This is an important enhancement. Requires some web experience.
* [easy] [rustup: Don't capture backtraces without RUST_BACKTRACE=1](https://github.com/rust-lang-nursery/rustup.rs/issues/591#issuecomment-236235677).
  An easy fix, done in two steps, first modifying error-chain, then upgrading it in rustup.
* [easy] [cargo: Warn on duplicate entry points](https://github.com/rust-lang/cargo/issues/2800).
  Simple way to get involved in one of the most important Rust projects.
* [easy] [cargo: Can't specify precise crate version if there are
  multiple versions](https://github.com/rust-lang/cargo/issues/2773).
* [easy] [rust: Make it easier to locate deadlocked
  tests](https://github.com/rust-lang/rust/issues/2873).  A simple
  enhancement to the test runner to notify the user about long-running
  tests.
* [easy] [rustup: Add command to install shell
  completions](https://github.com/rust-lang-nursery/rustup.rs/issues/387#issuecomment-234675568).
* [easy] [rustup: Clean up toolchain directories after
  uninstall](https://github.com/rust-lang-nursery/rustup.rs/issues/596#issuecomment-233716695).
  A simple bug with a clear fix.
* [moderate] [rust: improve error message when resolution via Deref
  actually required
  DerefMut](https://github.com/rust-lang/rust/issues/28419). Good
  first type system bug.
* [hard] [imag: implement bindings to lua/lisp (ketos)/rhai for the filter
  library](https://github.com/matthiasbeyer/imag/issues/245)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

127 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-07-25..2016-08-01

* [Switch to MIR-based translation by default](https://github.com/rust-lang/rust/pull/34096).
* [Reuse `.o` files of unchanged modules](https://github.com/rust-lang/rust/pull/34956) (first step towards incremental compilation)
* [{`vec`, `binary_heap`}::`Drain` is now covariant](https://github.com/rust-lang/rust/pull/34951)
* [`SOCK_CLOEXEC` is now used on Linux](https://github.com/rust-lang/rust/pull/34946) (wasn't before because bug)
* [TT-macro fixup](https://github.com/rust-lang/rust/pull/34908)
* [unstable ABIs now properly feature-gated](https://github.com/rust-lang/rust/pull/34904) (scary change, but crater found zero regressions)
* [`Debug` output now escapes fewer unicode codepoints](https://github.com/rust-lang/rust/pull/34485)
* [`impl DoubleEndedIterator for std::env::args`](https://github.com/rust-lang/rust/pull/33312)
* [ARM personality routine is now Rust](https://github.com/rust-lang/rust/pull/35032)
* [Keep in-transit closure types out of obligation tree](https://github.com/rust-lang/rust/pull/34986)
* [Process `feature` flags only on configured crates](https://github.com/rust-lang/rust/pull/34969)
* [ICE on unresolved imports in patterns fixed](https://github.com/rust-lang/rust/pull/34963)
* [`std::i*:checked_abs()`](https://github.com/rust-lang/rust/pull/35058)
* [ensure absense of single quotes in Windows paths](https://github.com/rust-lang/rust/pull/35117)
* [`MultiSpan` has a bunch of methods back](https://github.com/rust-lang/rust/pull/35094) (erroneously thought to be unused and removed last week, broke clippy)
* [`intravisit::Visitor` now subsumes `IdVisitor`'s functionality](https://github.com/rust-lang/rust/pull/35090)
* [Only export `#[no_mangle]` externals on LTO builds](https://github.com/rust-lang/rust/pull/35069)
* [cargo now has subcommand man pages](https://github.com/rust-lang/cargo/pull/2918)
* [cargo now creates bin crates by default](https://github.com/rust-lang/cargo/pull/2921)

(On a less serious note, the team was [admonished](https://github.com/rust-lang/meeting-minutes/pull/18/files) to avoid time travel)

## New Contributors

* Andrea Pretto
* Jonathan A. Kollasch
* Jonathan Creekmore
* Knight
* mcarton
* Moritz Ulrich
* Panashe M. Fundira
* Rahiel Kasim
* Robert Williamson
* Thomas Garcia
* Vladimir Vukicevic

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1216: Promote `!` to a type](https://github.com/rust-lang/rfcs/pull/1216).
* [RFC 1660: Introduce non-panicking borrow methods on `RefCell<T>`](https://github.com/rust-lang/rfcs/pull/1660).
* [RFC 1653: Add `assert_ne` to compliment `assert_eq`](https://github.com/rust-lang/rfcs/pull/1653).
* [RFC 1504: Add support for 128-bit integers](https://github.com/rust-lang/rfcs/pull/1504).
* [RFC 1548: Add `global_asm!` for module-level inline assembly](https://github.com/rust-lang/rfcs/pull/1548).
* [RFC 1560: Some internal and language-level changes to name resolution](https://github.com/rust-lang/rfcs/pull/1560).
* [Amendment to RFC 1444: Clarify behaviour when writing to a union field that implements Drop](https://github.com/rust-lang/rfcs/pull/1663).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add extra access methods for atomic types](https://github.com/rust-lang/rfcs/pull/1649).
* [Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).
* [Specify Rust compatibility of nursery crates](https://github.com/rust-lang/rfcs/pull/1619).
* [RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [Define a best practices procedure for making bug fixes in the compiler](https://github.com/rust-lang/rfcs/pull/1589).
* [`FusedIterator` marker trait and `iter::Fuse` specialization](https://github.com/rust-lang/rfcs/pull/1581).

## New RFCs

* [Allow deriving `Deref` and `DerefMut`](https://github.com/rust-lang/rfcs/pull/1694).
* [Add a `compile_error!` macro to libstd](https://github.com/rust-lang/rfcs/pull/1695). `compile_error!` will unconditionally cause compilation to fail with the given error message when encountered.
* [`mem::discriminant()`](https://github.com/rust-lang/rfcs/pull/1696). Add a function that extracts the discriminant from an enum variant as a comparable, hashable, printable, but (for now) opaque and unorderable type.

# Upcoming Events

* 8/3. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [8/3. Rust User Group Cologne/Bonn](http://rustaceans.cologne/2016/08/03/one-year-rust-cologne.html).
* 8/4. Rust release triage at #rust-triage on irc.mozilla.org.
* [8/8. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* 8/10. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [8/10. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/events/232581073/).
* [8/11. Columbus Rust Society](http://www.meetup.com/columbus-rs/events/232469955/).
* [8/15. Rust Paris](http://www.meetup.com/Rust-Paris/events/230111511/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
