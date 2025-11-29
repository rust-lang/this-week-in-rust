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

* 🎈🎉 [Announcing Rust 1.11](https://blog.rust-lang.org/2016/08/18/Rust-1.11.html). 🎉🎈
* [A year of Rust and DNS](https://bluejekyll.github.io/blog/rust/dns/2016/08/21/a-year-of-rust-and-dns.html). Benjamin Fry talks about his experience developing [trust-dns
](https://github.com/bluejekyll/trust-dns) - a Rust based DNS server.
* [GC support in Rust: API design](https://manishearth.github.io/blog/2016/08/18/gc-support-in-rust-api-design/).
* [Writing a JPEG decoder in Rust - Part 2: Implementation I](https://mht.technology/post/jpeg-rust-2/).
* [A Tokio echo server in 35 lines](https://pyfisch.org/blog/tokio-echo-server/). Tokio is a network application framework for Rust.
* [Filters everywhere](http://beyermatthias.de/blog/2016/08/22/filters-everywhere/). Short intro to the "filters" crate and remaining issues.
* [🔥 discussion] [The `?` operator will be harmful to Rust](https://internals.rust-lang.org/t/the-operator-will-be-harmful-to-rust/3882).
* [slides] [Rust: Systems programming for everyone](https://pnkfelix.github.io/presentations/qcon-london2016-deploy/qcon-london2016.html).
* [podcast] [New Rustacean bonus episode 6](http://www.newrustacean.com/show_notes/bonus/_6/). Building (and celebrating) all the little, not-so-glorious pieces of the Rust ecosystem.

## New Crates & Project Updates

* [rustup 0.6.0 released](https://internals.rust-lang.org/t/beta-testing-rustup-rs/3316/151) with bug fixes in manifest handling.
* [aelitabot.xyz](https://aelitabot.xyz/). bors-like commit checking for GitHub repositories.
* [GilRs](https://gitlab.com/Arvamer/gilrs). GilRs abstract platform specific APIs to provide unified interfaces for working with gamepads.
* [This week in Servo 75](https://blog.servo.org/2016/08/15/twis-75/).
* [This week in Rust docs 18](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-18).
* [This week in Tock embedded OS 3](http://www.tockos.org/blog/2016/talking-tock-3/).
* [This week in TiKV 2016-08-21](http://www.pingcap.com/tikv/2016/08/22/tikv-weekly/).
* [This week in Ruma 2016-08-21](https://www.ruma.io/news/this-week-in-ruma-2016-08-21/).
* [This month in Zone of Control 2016-08-22](https://users.rust-lang.org/t/this-month-in-zone-of-control-2016-08-22/6993). ZoC is a turn-based hexagonal strategy game written in Rust.

# Crate of the Week

No crate was selected for this week for lack of votes. Ain't that a pity?

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [tokei: Update existing languages with their String literals](https://github.com/Aaronepower/tokei/issues/52).
* [easy] [servo: Change starting line of scripts from 0 to 1](https://github.com/servo/servo/issues/12996).
* [easy] [servo: Remove the mako zip file and use proper python dependencies](https://github.com/servo/servo/issues/12958).
* [less easy] [servo: Create a `./mach test-perf` command](https://github.com/servo/servo/issues/12792).
* [easy] [rust: Error code list which need to be updated to new format](https://github.com/rust-lang/rust/issues/35233).
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
* [hard] [filters: implement `BitAnd`, `BitOr`, `BitXor` and `Not` for all
  implementations of `Filter<N>`](https://github.com/matthiasbeyer/filters/issues/4)

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
* [`Debug` for `std::vec::IntoIter`](https://github.com/rust-lang/rust/pull/35707)
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

* Alexandre Oliveira
* Amit Levy
* clementmiao
* DarkEld3r
* Dustin Bensing
* Erik Uggeldahl
* Jacob
* JessRudder
* Michael Layne
* Nazım Can Altınova
* Neil Williams
* pliniker

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1679: Add "panic-safe" or "total" alternatives to the existing panicking indexing syntax](https://github.com/rust-lang/rfcs/pull/1679).
* [RFC 1640: Add `checked_sub()` already known from various primitive types to the `Duration` struct](https://github.com/rust-lang/rfcs/pull/1640).
* [RFC 1589: Define a best practices procedure for making bug fixes in the compiler](https://github.com/rust-lang/rfcs/pull/1589).
* [RFC 1561: Macro naming and modularisation](https://github.com/rust-lang/rfcs/pull/1561).
* [RFC 1623: Omit `'static` lifetimes to reference or generics lifetime values in `static` or `const` declarations](https://github.com/rust-lang/rfcs/pull/1623).
* [RFC 1681: Procedural macros 1.1](https://github.com/rust-lang/rfcs/pull/1681).

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

* [A process for establishing the Rust roadmap](https://github.com/rust-lang/rfcs/pull/1728).
* [Add two functions, `ptr::read_unaligned` and `ptr::write_unaligned`, which allows reading/writing to an unaligned pointer](https://github.com/rust-lang/rfcs/pull/1725).
* [Enable customizing the linkage of a platform's C runtime](https://github.com/rust-lang/rfcs/pull/1721).
* [Add std::function macro for getting the name of the current function](https://github.com/rust-lang/rfcs/pull/1719).

# Upcoming Events

* 8/24. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 8/25. Rust release triage at #rust-triage on irc.mozilla.org.
* [8/29. Rust Sthlm: Rust on the Web](http://www.meetup.com/ruststhlm/events/232054490/).
* [8/29. Rust on the web Rust Meetup Stockholm #2](http://www.meetup.com/ruststhlm/events/232054490/).
* 8/31. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [9/5. Rust Cologne/Bonn: Compiling Rust to your Browser](http://rustaceans.cologne/2016/09/05/compile-to-js.html).
* [9/5. Zurich / Switzerland: Rust Meetup - Lecture: Rust<T>](http://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/233292936/).
* [9/9. RustConf 2016](http://rustconf.com/).
* [9/9. Rust Table of Regulars Darmstadt](https://www.meetup.com/de-DE/Rust-Rhein-Main/events/233544580/).
* [9/9. Tokio Hack Night](https://tokiohacknight.splashthat.com/).

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

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
