Title: This Week in Rust 141
Number: 141
Date: 2016-08-02
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

* [What's coming up in imag (12)](http://beyermatthias.de/blog/2016/07/29/what-s-coming-up-in-imag-12/)
  was published on the 29th of July. imag is a personal information
  management suite for the commandline.
  The project also got [a website](http://imag-pim.org), an
  [IRC channel](irc://irc.freenode.net/#imag) and a
  [mailinglist](http://imag-pim.org/mailinglist/).

## New Crates & Project Updates

* [Zone of Control was ported to GFX](https://github.com/ozkriff/zoc/issues/183).
* [`rustup` 0.5.0 was released](https://internals.rust-lang.org/t/beta-testing-rustup-rs/3316/149).
  This build includes a more compact syntax for `rustup run`, a new
  `rustup man` command, and bug fixes.

# Crate of the Week

This week's Crate of the Week is MaidSafe's [lru_time_cache](https://crates.io/crates/lru_time_cache), a simple but complete least-recently-used cache implementation. Thanks [gregwtmtno](https://users.rust-lang.org/users/gregwtmtno) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

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
* [easy] [imag: Switch to clap in the `imag` binary](https://github.com/matthiasbeyer/imag/issues/566).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

127 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-07-25..2016-08-01

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

* Evgeny Safronov
* Matt Horn

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1559: Allow all literals in attributes](https://github.com/rust-lang/rfcs/pull/1559).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Promote `!` to a type](https://github.com/rust-lang/rfcs/pull/1216)
* [Add language support for bitfields](https://github.com/rust-lang/rfcs/pull/1449).
* [Add support for 128-bit integers](https://github.com/rust-lang/rfcs/pull/1504).
* [Add space-friendly arguments](https://github.com/rust-lang/rfcs/pull/1509). Add `-C link-arg` and `-C llvm-arg` which allow you to pass along argument with spaces.
* [Exclude macros from importing with `#[macro_use(not(...))]`](https://github.com/rust-lang/rfcs/pull/1517).
* [Add `global_asm!` for module-level inline assembly](https://github.com/rust-lang/rfcs/pull/1548).
* [Some internal and language-level changes to name resolution](https://github.com/rust-lang/rfcs/pull/1560).
* [Define a best practices procedure for making bug fixes in the compiler](https://github.com/rust-lang/rfcs/pull/1589).
* [RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).
* [Replace synchronization primitives with those from parking_lot](https://github.com/rust-lang/rfcs/pull/1632).
* [Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [Add `assert_ne` to compliment `assert_eq`](https://github.com/rust-lang/rfcs/pull/1653).
* [Introduce non-panicking borrow methods on `RefCell<T>`](https://github.com/rust-lang/rfcs/pull/1660).
* [Propose asserts](https://github.com/rust-lang/rfcs/pull/1662). This rfc proposes that the following macros be added: `assert_gt`, `assert_lt`, `assert_ge`, and `assert_le`.
* [Clarify behaviour when writing to a union field that implements Drop](https://github.com/rust-lang/rfcs/pull/1663).

## New RFCs

* [Add API documentation front page styleguide](https://github.com/rust-lang/rfcs/pull/1687).
* [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [Add an environment variable to choose between whether to link the static CRT or dynamic CRT](https://github.com/rust-lang/rfcs/pull/1684).
* [Create a team responsible for documentation for the Rust project](https://github.com/rust-lang/rfcs/pull/1683).
* [Propose a shorthand syntax for constructing struct-like values with _named_ fields](https://github.com/rust-lang/rfcs/pull/1682).

# Upcoming Events

* 7/27. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [7/27. Rust Berlin July Meetup](http://www.meetup.com/Rust-Berlin/events/232583152/).
* [7/28. Rust Bay Area: Machine Learning, Bioinformatics, and Embedded OSes](http://www.meetup.com/Rust-Bay-Area/events/232406852/).
* 8/3. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 8/4. Rust release triage at #rust-triage on irc.mozilla.org.
* [8/8. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

<!-- Do not roll over to next week -->
## [PhD Position: Concepts and Methods for securing the Future Energy System's Components and Plants](https://stellen.jobs.kit.edu/cgi-bin/appl/list.pl?tmpl=job_details&job_nr=248AF364-5B71-4112-9FBD-0F144C731087&cat_nr=28FD50F7-057E-4EB7-BF74-E00DD530252E&loc_nr=12B87EE0-C700-11D4-8972-0050BAC69B70)

TLDR: the goal of the thesis is the development of a methodology for using the engineering metadata of the systems to generate models that can be used to check the communication and behaviour of the systems. A prototype of the checking tool will be implemented in Rust on ARM hardware.

# Quote of the Week

> you have a problem. you decide to use Rust. now you have a Rc<RefCell<Box<Problem\>\>\>

[kmc on #rust](https://botbot.me/mozilla/rust/2016-07-25/?msg=70207904&page=14).

Thanks to [Alex Burka](https://users.rust-lang.org/users/durka) for the tip. [Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
