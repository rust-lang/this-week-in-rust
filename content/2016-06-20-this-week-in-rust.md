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

* [Writing a simple REST app in Rust](https://gsquire.github.io/static/post/rest-in-rust/).
* [Paradigms of Rust for the Go developer](https://medium.com/@deckarep/paradigms-of-rust-for-the-go-developer-210f67cd6a29#.6rw2gwmg1).
* [Using ltrace to debug a memory leak](http://jvns.ca/blog/2016/06/15/using-ltrace-to-debug-a-memory-leak/).
* [Scheduling timers on OS X with Rust and Kqueue](http://nitschinger.at/Scheduling-Timers-on-OS-X-with-Rust-and-Kqueue/).
* [Problem in Rust adoption](https://sanxiyn.blogspot.in/2016/06/problem-in-rust-adoption.html).
* [podcast] [New Rustacean podcast episode 15](http://www.newrustacean.com/show_notes/e015/index.html). `Box`, `String`, `Vec`, `Rc`, and `Arc` have this in common: they're not dumb.

## New Crates & Project Updates

* [cargo-deb](https://github.com/mmstick/cargo-deb). A cargo subcommand that generates Debian packages from information in Cargo.toml.
* [Habitat](https://github.com/habitat-sh/habitat). Build modern applications with built-in automation.
* [Ruru](https://github.com/d-unseductable/ruru). Native Ruby extensions written in Rust.
* [FasterPath](https://github.com/danielpclark/faster_path). Faster Pathname handling for Ruby written in Rust.
* [Polydraw](https://github.com/polydraw/polydraw). 2D graphics engine written in Rust.
* [Lia](https://github.com/willcrichton/lia). A High-Level Language for Rust.
* [json-rust](https://github.com/maciejhirsz/json-rust). JSON implementation in Rust.
* [OxideNES](https://github.com/iamsix/oxidenes). NES emulator written in Rust.
* [jamal](https://github.com/softprops/jamal). Bi-directional interface for transformations between JSON and YAML.
* [atarashii_imap](https://github.com/GildedHonour/atarashii_imap). MAP client in Rust.
* [This week in Servo 67](https://blog.servo.org/2016/06/13/twis-67/).
* [This week in Rust docs 8](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-8).
* [This week in Ruma 2016-06-12](https://www.ruma.io/news/this-week-in-ruma-2016-06-12/).

# Crate of the Week

This week's Crate of the Week is [error-chain](https://crates.io/crates/error-chain) which feels like the missing piece in Rust's `Result`-based error-handling puzzle. Thanks to [KodrAus](https://users.rust-lang.org/users/KodrAus) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).
* [easy] [rust: List all available ABI strings in reference.md](https://github.com/rust-lang/rust/issues/34267).

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

* Esteban Küber
* marudor

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1567: Normalization for long error codes explanations](https://github.com/rust-lang/rfcs/pull/1567).
* [RFC 1590: Add a `lifetime` specifier to `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1590).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Remove the one-type-only restriction on `format_args!` arguments](https://github.com/rust-lang/rfcs/pull/1618).
* [Introduce more conventions around documenting Rust projects](https://github.com/rust-lang/rfcs/pull/1574).
* [Standardise stream wrappers like compression, encryption](https://github.com/rust-lang/rfcs/pull/1568).
* [Add a initial, minimal form of `impl Trait`](https://github.com/rust-lang/rfcs/pull/1522).
* [Change thread local variables to only accept async-signal-safe types](https://github.com/rust-lang/rfcs/pull/1379).
* [Implement new methods for checked and wrapping casts for potentially lossy integer conversions](https://github.com/rust-lang/rfcs/pull/1218).

## New RFCs

* [Add extra access methods for atomic types](https://github.com/rust-lang/rfcs/pull/1649).
* [Add the ability to define closures that are generic over types](https://github.com/rust-lang/rfcs/pull/1650).
* [Extend `Cell` to work with non-`Copy` types](https://github.com/rust-lang/rfcs/pull/1651).
* [Add `assert_ne` to compliment `assert_eq`](https://github.com/rust-lang/rfcs/pull/1653).

# Upcoming Events

* 6/22. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 6/23. Rust release triage at #rust-triage on irc.mozilla.org.
* 6/29. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [6/30. Zurich, Switzerland - Introduction to Rust](http://www.meetup.com/Mozilla-Meetup-Switzerland/events/231268531/).

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

> The Rust standard libs aren't quite batteries included, but they come with a pile of adaptor cables and an optional chemistry lab.

— [Gankro on Twitter](https://twitter.com/Gankro/status/743425058652196865)

Thanks to [llogiq](https://users.rust-lang.org/users/llogiq) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
