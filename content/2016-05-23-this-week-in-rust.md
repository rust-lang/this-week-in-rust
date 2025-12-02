Title: This Week in Rust 131
Number: 131
Date: 2016-05-23
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

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts

* 🎈🎉 [One year of Rust](http://blog.rust-lang.org/2016/05/16/rust-at-one-year.html). 🎉🎈
* [Let's build a web server in Rust](https://dfockler.github.io/2016/05/20/web-server.html).
* [Creating a basic webservice in Rust](http://hermanradtke.com/2016/05/16/creating-a-basic-webservice-in-rust.html).
* [Writing GStreamer plugins and elements in Rust](https://coaxion.net/blog/2016/05/writing-gstreamer-plugins-and-elements-in-rust/).
* [Writing a compiler plugin to instrument code](https://llogiq.github.io/2016/05/17/flamer.html).
* [Our polyglot approach: Getting started with Rust](https://tech.zalando.de/blog/getting-started-with-rust/).
* [This week in Rust docs 4](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-4).
* [This week in Parity 1](https://ethcore.github.io/twip/2016-05-17.html).
* [This week in RustFest 1: Introduction](http://www.rustfest.eu/blog/this-week-in-rustfest-1).
* [This week in Ruma 2016-05-22](https://www.ruma.io/news/this-week-in-ruma-2016-05-22/).
* [Plushie Rustacean pattern](http://edunham.net/2016/04/11/plushie_rustacean_pattern.html). Sew your own plush Rustacean toy!

## New Crates & Project Updates

* [Systemd Manager](https://github.com/mmstick/systemd-manager). A systemd service manager written in Rust with the GTK-rs wrapper and direct integration with dbus.
* [FLAME](https://github.com/tyoverby/flame). A flamegraph profiling tool for Rust.
* [Jobsteal](https://github.com/rphmeier/jobsteal). A work-stealing fork-join threadpool written in Rust.
* [pest](https://github.com/dragostis/pest). Simple, efficient parser generator.

# Crate of the Week

This weeks Crate of the Week is [parking_lot](https://crates.io/crates/parking_lot) which gives us synchronization primitives (Mutex, RWLock, CondVar and friends) that are both smaller and faster than the standard library's implementations.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Participate in 2016 State of Rust Survey](http://blog.rust-lang.org/2016/05/09/survey.html).
* [easy] [redox: Participate in Redox Survey](http://goo.gl/forms/RUOfIsxXsv).
* [easy] [clippy: Participate in Rust Clippy Survey](https://docs.google.com/forms/d/1k0wuWgGwDhuUL3q_cONGVxQ6PJSYq5JRZOHKc0itLbg/viewform?c=0&w=1).
* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).
* [medium] [rustup: Make `rustup default x86_64-unknown-linux-gnu` do something smarter](https://github.com/rust-lang-nursery/rustup.rs/issues/411).
* [easy] [servo: Add icon to Servo on Windows](https://github.com/servo/servo/issues/11315).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

117 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-05-16..2016-05-23

* [macro expression spans fixed](https://github.com/rust-lang/rust/pull/33712)
* [`#[rustc_on_unimplemented = _]` now does fuzzy matching](https://github.com/rust-lang/rust/pull/33694)
* [fixes for old error style, improvements for new](https://github.com/rust-lang/rust/pull/33688)
* [more new-style error reporting improvements](https://github.com/rust-lang/rust/pull/33619)
* [Macro expansion diagnostics improvements](https://github.com/rust-lang/rust/pull/33730)
* [`hir::Ident`](https://github.com/rust-lang/rust/pull/33654) and [`ExplicitSelf`](https://github.com/rust-lang/rust/pull/33505) removed (requires lint writers to rustup)
* [new crate type `cdylib`](https://github.com/rust-lang/rust/pull/33553) (for lean libs to embed in C)
* [replace obligation forest with graph](https://github.com/rust-lang/rust/pull/33491)
* [incremental compilation: Track deps across crates](https://github.com/rust-lang/rust/pull/33476)
* [float parsing fixed on x87](https://github.com/rust-lang/rust/pull/33429)
* [`HashMap::new()` speedups via thread-local storage](https://github.com/rust-lang/rust/pull/33318)
* [extended warning for unsoundness with elided associated type lifetimes](https://github.com/rust-lang/rust/pull/33137)
* [`EscapeUnicode::last()` and other specializations](https://github.com/rust-lang/rust/pull/33103)
* [Backport an unsoundness fix in libbacktrace](https://github.com/rust-lang/rust/pull/33729)

## New Contributors

* Daniel Campoverde [alx741]
* mark-summerfield
* Postmodern
* Rémy Rakic
* Robert Habermeier
* Val Vanderschaegen

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

* [Normalization for long error codes explanations](https://github.com/rust-lang/rfcs/pull/1567).
* [Add a `lifetime` specifier to `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1590/files).

## New RFCs

* [regex 1.0](https://github.com/rust-lang/rfcs/pull/1620).
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).
* [Omit `'static` lifetimes to reference or generics lifetime values in `static` or `const` declarations](https://github.com/rust-lang/rfcs/pull/1623).
* [Let a `loop { ... }` expression return a value via `break my_value;`](https://github.com/rust-lang/rfcs/pull/1624).
* [Specify Rust compatibility of nursery crates](https://github.com/rust-lang/rfcs/pull/1619).
* [Remove the one-type-only restriction on `format_args!` arguments](https://github.com/rust-lang/rfcs/pull/1618).

# Upcoming Events

* [5/24. Rust Brisbane Meetup #1](http://www.meetup.com/Rust-Brisbane/events/230676018/).
* [5/24. Rust Barcelona - Cross compiling for ARM linux targets + accessing the GPIO in a Raspberry Pi](http://www.meetup.com/Rust-Barcelona/events/230638451/).
* 5/25. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [5/25. Boston Rust Meetup w/ Steve Klabnik and Andrew Gallant](http://www.meetup.com/BostonRust/events/230419544/).
* [5/26. Tokyo Rust Meetup #5](http://www.meetup.com/Tokyo-Rust-Meetup/events/231018886/).
* [5/26. Rust DC - Inaugural Meetup](http://www.meetup.com/RustDC/events/227138240/).
* 6/1. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [6/1. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [6/6. Cologne / Germany: Rust Anniversary Meetup - Part II](http://www.meetup.com/de-DE/Rust-Cologne-Bonn/events/231135785/).

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

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
