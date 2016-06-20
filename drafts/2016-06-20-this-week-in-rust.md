Title: This Week in Rust 135
Number: 135
Date: 2016-06-20
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

This week's edition was edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts



## New Crates & Project Updates



# Crate of the Week

This week's Crate of the Week is [error-chain](https://crates.io/crates/error-chain) which feels like the missing piece in Rust's `Result`-based error-handling puzzle. Thanks to [KodrAus](https://users.rust-lang.org/users/KodrAus) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

73 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-06-13..2016-06-20

* [The pretty-printer will now `try!` again instead of questioning](https://github.com/rust-lang/rust/pull/34312) (since `?` isn't stable, this eases backporting)
* MIR drop handling got a little easier by [caching in some cases](https://github.com/rust-lang/rust/pull/34307) and [dropping less](https://github.com/rust-lang/rust/pull/34290)
* [`-Z dump_mir` now writes parent- and promoted MIR to different files](https://github.com/rust-lang/rust/pull/34306)
* [Creating a file can now `Err(ERROR_FILE_EXISTS)` on Windows, too](https://github.com/rust-lang/rust/pull/34270)
* [Fixed macro call site spans](https://github.com/rust-lang/rust/pull/33749)
* [Fixed a macro scoping error introduced last week](https://github.com/rust-lang/rust/pull/34239)
* [Process `#[cfg..]` attributes in decorator-generated items](https://github.com/rust-lang/rust/pull/34295)
* [Support nested `#[cfg..]`s](https://github.com/rust-lang/rust/pull/34216)
* [HIR no longer concerned with identifier hygiene](https://github.com/rust-lang/rust/pull/34207) (lint-breaking change)
* [Debuginfo now contains absolute file paths](https://github.com/rust-lang/rust/pull/34187)
* [More helpful errors when mixing up function arguments](https://github.com/rust-lang/rust/pull/34000) (also arbitrary milestone #34000)
* [Map entries can now remove key or both key and value](https://github.com/rust-lang/rust/pull/33300) (and are still underused IMHO)
* [Faster `.zip(_)` via specialization](https://github.com/rust-lang/rust/pull/33090)

## New Contributors

* Andrew Brinker
* Chris Tomlinson
* Hendrik Sollich
* Horace Abenga
* Jacob Clark
* Jakob Demler
* James Alan Preiss
* James Lucas
* Joachim Viide
* Mark Côté
* Mathieu De Coster
* Michael Necio
* Morten H. Solvang
* Wojciech Nawrocki

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

* [Implement new methods for checked and wrapping casts for potentially lossy integer conversions](https://github.com/rust-lang/rfcs/pull/1218).
* [Change thread local variables to only accept async-signal-safe types](https://github.com/rust-lang/rfcs/pull/1379).
* [Add a initial, minimal form of `impl Trait`](https://github.com/rust-lang/rfcs/pull/1522).
* [Normalization for long error codes explanations](https://github.com/rust-lang/rfcs/pull/1567).
* [Standardise stream wrappers like compression, encryption](https://github.com/rust-lang/rfcs/pull/1568).
* [Add a `lifetime` specifier to `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1590).
* [Remove the one-type-only restriction on `format_args!` arguments](https://github.com/rust-lang/rfcs/pull/1618).

## New RFCs

* [Add `CStr::with_ptr` and deprecate `CStr::as_ptr`](https://github.com/rust-lang/rfcs/pull/1642).
* [Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [Default and expanded errors for rustc](https://github.com/rust-lang/rfcs/pull/1644).
* [Add Cortex-M targets to the compiler + binary releases of `core`](https://github.com/rust-lang/rfcs/pull/1645).
* [Add `&move` pointers, the `DerefMove` trait, and the unsafe `DerefPure` traits](https://github.com/rust-lang/rfcs/pull/1646).
* [Allow `Self` to appear in the where clause of trait impls](https://github.com/rust-lang/rfcs/pull/1647).

# Upcoming Events

* [6/14. Eat – Drink – Rust! Downtown Rust Meetup (San Diego)](http://www.meetup.com/San-Diego-Rust/events/231356534/)
* 6/15. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [6/15. Rust Los Angeles Monthly Meetup - Hack Night](http://www.meetup.com/Rust-Los-Angeles/events/231587506/).
* [6/16. London Rust Meetup](http://www.meetup.com/Rust-London-User-Group/events/231332388/).
* [6/17. Rhein-Main Rust Regulars' Table](http://www.meetup.com/Rust-Rhein-Main/events/231344035/).
* 6/22. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 6/23. Rust release triage at #rust-triage on irc.mozilla.org.

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

>> Isn’t rust too difficult to be widely adopted?

> I believe in people.

— [Steve Klabnik on TRPLF](https://users.rust-lang.org/t/isnt-rust-too-difficult-to-be-widely-adopted/6173/2)

Thanks to [Steven Allen](https://users.rust-lang.org/users/stebalien) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
