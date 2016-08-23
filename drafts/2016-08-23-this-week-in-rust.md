Title: This Week in Rust 144
Number: 144
Date: 2016-08-23
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

## New Crates & Project Updates

[This Month in Zone of Control (2016.08.22)](https://users.rust-lang.org/t/this-month-in-zone-of-control-2016-08-22/6993)

# Crate of the Week

No crate was selected for this week for lack of votes. Ain't that a pity?

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [moderate] [rust-www: Add a section to the front page with 3 rotating 'friends'](https://github.com/rust-lang/rust-www/issues/477).
  This is an important change to how we present Rust that everybody will see.
* [easy] [rust: Error code list which need to be updated to new format](https://github.com/rust-lang/rust/issues/35233).
* [easy] [servo: Do not define Pipeline::setup_common when on Windows](https://github.com/servo/servo/issues/12856).
* [easy] [rustup: Don't capture backtraces without RUST_BACKTRACE=1](https://github.com/rust-lang-nursery/rustup.rs/issues/591#issuecomment-236235677).
  An easy fix, done in two steps, first modifying error-chain, then upgrading it in rustup.
* [easy] [rustup: Add command to install shell
  completions](https://github.com/rust-lang-nursery/rustup.rs/issues/387#issuecomment-234675568).
* [moderate] [rust: improve error message when resolution via Deref
  actually required
  DerefMut](https://github.com/rust-lang/rust/issues/28419). Good
  first type system bug.
* [hard] [imag: implement bindings to lua/lisp (ketos)/rhai for the filter
  library](https://github.com/matthiasbeyer/imag/issues/245)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

167 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-08-15..2016-08-22

* [1.11 Changelog](https://github.com/rust-lang/rust/pull/35736)
* [1.12 stabilizations](https://github.com/rust-lang/rust/pull/35607)
* [`!` is now a proper type](https://github.com/rust-lang/rust/pull/35162)
* [`std::slice::Iter` now `AsRef`s to the underlying slice](https://github.com/rust-lang/rust/pull/35559)
* [`std::vec::IntoIter::as_`{`slice`, `mut_slice`}](https://github.com/rust-lang/rust/pull/35447)
* [`Debug` for `std::vec::IntoIter](https://github.com/rust-lang/rust/pull/35707)
* [{`Cell`, `RefCell`, `UnsafeCell`}`<T>` now have `From<T>`](https://github.com/rust-lang/rust/pull/35392)
* [`Drain`s now covariant](https://github.com/rust-lang/rust/pull/35354)
* [`vec::IntoIter` now covariant again](https://github.com/rust-lang/rust/pull/35733)
* [Extra methods on atomic types](https://github.com/rust-lang/rust/pull/35719) (implements [RFC #1649](https://github.com/rust-lang/rfcs/pull/1649))
* [`panic::catch_unwind` slightly faster](https://github.com/rust-lang/rust/pull/35444) (but still an antipattern in normal code)
* [incremental compilation now obeys certain commandline args](https://github.com/rust-lang/rust/pull/35340)
* [improved `&`ptr printing](https://github.com/rust-lang/rust/pull/35611)
* [No more RUST_NEW_ERROR_FORMAT](https://github.com/rust-lang/rust/pull/35708) (the new format is now live everywhere)
* [MIR early exit cache invalidation fixed](https://github.com/rust-lang/rust/pull/35751) (one of CS' hard problems, even for Rust)
* [More groundwork for `item_like_import`s](https://github.com/rust-lang/rust/pull/35776) (part of [RFC #1560](https://github.com/rust-lang/rfcs/pull/1560))
* [LLVM backports to fix perf regressions](https://github.com/rust-lang/rust/pull/35740)
* [MIPS-uclibc target added](https://github.com/rust-lang/rust/pull/35734)
* [Cargo now allows `opt-level=`{`s`, `z`} in profiles](https://github.com/rust-lang/cargo/pull/3007)
* [Cargo resolve no longer cares for the package root](https://github.com/rust-lang/cargo/pull/3013)
* [Cargo: OpenSSL is now correctly downloaded from www.openssl.org](https://github.com/rust-lang/cargo/pull/3011) (formerly openssl.org, but they changed it apparently)
* [`rustdoc` now omits the `!` in links to macros](https://github.com/rust-lang/rust/pull/35234)

## New Contributors

* Andrii Dmytrenko
* Cameron Hart
* Cengiz Can
* Chiu-Hsiang Hsu
* Christophe Vu-Brugier
* Clement Miao
* crypto-universe
* Felix Rath
* hank-der-hafenarbeiter
* José manuel Barroso Galindo
* Krzysztof Garczynski
* Luke Hinds
* Marco A L Barbosa
* Mark-Simulacrum
* Matthew Piziak
* Michael Gattozzi
* Patrick McCann
* Pietro Albini
* ShyamSundarB
* srdja
* Stephen Lazaro

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1643: Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [RFC 1607: RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [RFC 1683: Create a team responsible for documentation for the Rust project](https://github.com/rust-lang/rfcs/pull/1683).
* [RFC 1581: `FusedIterator` marker trait and `iter::Fuse` specialization](https://github.com/rust-lang/rfcs/pull/1581).
* [RFC 1649: Add extra access methods for atomic types](https://github.com/rust-lang/rfcs/pull/1649).
* [RFC 1576: Add a `literal` fragment specifier for `macro_rules!` patterns that matches literal constants](https://github.com/rust-lang/rfcs/pull/1576).
* [RFC 1506: Clarify the relationships between various kinds of structs and variants](https://github.com/rust-lang/rfcs/pull/1506).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Allow deriving `Deref` and `DerefMut`](https://github.com/rust-lang/rfcs/pull/1694).
* [Procedural macros 1.1](https://github.com/rust-lang/rfcs/pull/1681).
* [Add "panic-safe" or "total" alternatives to the existing panicking indexing syntax](https://github.com/rust-lang/rfcs/pull/1679).
* [Add `checked_sub()` already known from various primitive types to the `Duration` struct](https://github.com/rust-lang/rfcs/pull/1640).
* [Omit `'static` lifetimes to reference or generics lifetime values in `static` or `const` declarations](https://github.com/rust-lang/rfcs/pull/1623).
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).
* [Specify Rust compatibility of nursery crates](https://github.com/rust-lang/rfcs/pull/1619).
* [Define a best practices procedure for making bug fixes in the compiler](https://github.com/rust-lang/rfcs/pull/1589).
* [Add `parse_generics!` and `parse_where!` macros](https://github.com/rust-lang/rfcs/pull/1583).
* [Support code generators with source maps and multiple source directories](https://github.com/rust-lang/rfcs/pull/1573).
* [Macro naming and modularisation](https://github.com/rust-lang/rfcs/pull/1561).
* [Propose `Interior<T>` data-type, to allow moves out of the dropped value during the drop hook](https://github.com/rust-lang/rfcs/pull/1180).

## New RFCs

* [Use `#[link(kind)]` to fix imports from native libs on Windows](https://github.com/rust-lang/rfcs/pull/1717).
* [Add "meta tags" to Rust documentation conventions and to the rustdoc tool](https://github.com/rust-lang/rfcs/pull/1713).
* [Add 'else match' blocks to if expressions](https://github.com/rust-lang/rfcs/pull/1712).

# Upcoming Events

* [8/17. Boston Rust Meetup: Hack Night](http://www.meetup.com/BostonRust/events/233260730/).
* 8/17. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [8/17. Rust Los Angeles Meetup](https://www.meetup.com/Rust-Los-Angeles/events/232933613/).
* 8/24. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 8/25. Rust release triage at #rust-triage on irc.mozilla.org.
* [8/29. Rust Sthlm: Rust on the Web](http://www.meetup.com/ruststhlm/events/232054490/).
* [8/29. Rust on the web Rust Meetup Stockholm #2](http://www.meetup.com/ruststhlm/events/232054490/).
* [9/9. Rust Table of Regulars Darmstadt](https://www.meetup.com/de-DE/Rust-Rhein-Main/events/233544580/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The best way to learn Rust is to just `try!` and see what works (or is this to just see what works`?` now?)!

— [llogiq on /r/rust](https://www.reddit.com/r/rust/comments/4xuds0/sharing_coloring_books_with_friends_in_rust/d6jecnz).

Thanks to [UtherII](https://users.rust-lang.org/users/utherii) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
