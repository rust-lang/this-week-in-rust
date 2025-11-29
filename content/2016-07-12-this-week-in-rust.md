Title: This Week in Rust 138
Number: 138
Date: 2016-07-12
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

* 🎈🎉 [Announcing Rust 1.10](https://blog.rust-lang.org/2016/07/07/Rust-1.10.html). 🎉🎈
* [Refining Rust's RFCs](https://aturon.github.io/blog/2016/07/05/rfc-refinement/).
* [Translating C to Rust using Corrode (and how you can help)](http://jamey.thesharps.us/2016/07/translating-c-to-rust-and-how-you-can.html).
* [Rust and Rest](http://lucumr.pocoo.org/2016/7/10/rust-rest/). Lessons Learned from talking to Sentry's HTTP API from Rust.
* [Pairing cryptography in Rust](https://z.cash/blog/pairing-cryptography-in-rust.html).
* [Shave some time from your Travis builds](https://llogiq.github.io/2016/07/05/travis.html).
* [Overview of open source game engines in Rust](http://www.shadercat.com/overview-of-open-source-game-engines-in-rust/).
* [Rust & Docker in production @ Coursera](https://building.coursera.org/blog/2016/07/07/rust-docker-in-production-coursera/).
* [Integer 32](http://www.integer32.com/), a Rust consultancy startup by [Carol Nichols](https://github.com/carols10cents) and [Jake Goulding](https://github.com/shepmaster).

## New Crates & Project Updates

* [Dyon 0.8 is released](http://blog.piston.rs/2016/07/11/dyon-0.8/).
* [Corrode](https://github.com/jameysharp/corrode). Automatic semantics-preserving translation from C to Rust.
* [Rustls](https://github.com/ctz/rustls). A new, modern TLS library written in Rust.
* [rulinalg](https://github.com/AtheMathmo/rulinalg). A linear algebra library in Rust designed for machine learning, extracted from [rusty-machine](https://github.com/AtheMathmo/rusty-machine).
* [task-hookrs](https://github.com/matthiasbeyer/task-hookrs). A Rust library for writing taskwarrior hooks and general interfacing with taskwarrior.
* [jsf](https://github.com/flosse/rust-json-file-store/). A simple JSON file store.
* [This week in Rust docs 12](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-12).

# Crate of the Week

*No crate was selected for CotW.*

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [cargo: Warn on duplicate entry points for libs and bins](https://github.com/rust-lang/cargo/issues/2800).
* [easy] [cargo: Can't specify precise crate version if there are multiple versions](https://github.com/rust-lang/cargo/issues/2773).
* [easy] [cargo: Add `--dry-run` to `cargo publish`](https://github.com/rust-lang/cargo/issues/1332).
* [easy] [rust: E0502 not rendered correctly](https://github.com/rust-lang/rust/issues/34716).
* [easy] [rust: Move some tests into run-pass-valgrind](https://github.com/rust-lang/rust/issues/21696).
* [moderate] [rust: Convert compiler-rt builtins to a Rust crate](https://github.com/rust-lang/rust/issues/34400#issuecomment-230059689).
* [moderate] [rust: Teach rustc to print CPU, etc. features](https://github.com/rust-lang/rust/issues/30961#issuecomment-228905399).
* [easy] [rustfmt: Overlong function signatures](https://github.com/rust-lang-nursery/rustfmt/issues/1049).
* [easy] [rustfmt: Overlong impl signatures](https://github.com/rust-lang-nursery/rustfmt/issues/1048).
* [easy] [rust-by-example: Add a Mutex chapter](https://github.com/rust-lang/rust-by-example/issues/105).
* [easy] [rust-by-example: Add an Arc chapter](https://github.com/rust-lang/rust-by-example/issues/104).
* [easy] [imag: Make `imag` forward `--debug` and `--verbose` to subcommands](https://github.com/matthiasbeyer/imag/issues/506).
* [moderate] [imag: Add Iterator-shortcut for `iter.fold(Ok(()), ...)`](https://github.com/matthiasbeyer/imag/issues/499).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

100 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-07-04..2016-07-11

* [Implement workspaces in Cargo](https://github.com/rust-lang/cargo/pull/2759).
* [Drive trans from the output of the translation item collector](https://github.com/rust-lang/rust/pull/33890).
* [std: Stabilize APIs for the 1.11.0 release](https://github.com/rust-lang/rust/pull/34530).
* [Update jemalloc to include a fix for startup issues on OSX 10.12](https://github.com/rust-lang/rust/pull/34719).
* [Cargo: Add support for RUSTDOCFLAGS](https://github.com/rust-lang/cargo/pull/2794).
* [Add x86 intrinsics for bit manipulation (BMI 1.0, BMI 2.0, and TBM)](https://github.com/rust-lang/rust/pull/34412).
* [Added a pretty printer for &mut slices](https://github.com/rust-lang/rust/pull/34550).
* [Use lazy iterator in vec/slice gdb pretty printers](https://github.com/rust-lang/rust/pull/34639).
* [Introducing TokenStreams and TokenSlices for procedural macros](https://github.com/rust-lang/rust/pull/34575).

## New Contributors

* CrLF0710
* Hariharan R
* Ivan Nejgebauer
* Jared Manning
* Kaivo Anastetiks
* Mike Hommey
* Phlogistic Fugu
* Sam Payson
* Ximin Luo

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1327: Dropck Eyepatch](https://github.com/rust-lang/rfcs/pull/1327). Refine the unguarded-escape-hatch from RFC 1238 (nonparametric dropck).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Default and expanded errors for rustc](https://github.com/rust-lang/rfcs/pull/1644).
* [Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [Introduce more conventions around documenting Rust projects](https://github.com/rust-lang/rfcs/pull/1574).
* [Allow all literals in attributes](https://github.com/rust-lang/rfcs/pull/1559).
* [Add `global_asm!` for module-level inline assembly](https://github.com/rust-lang/rfcs/pull/1548).
* [Exclude macros from importing with `#[macro_use(not(...))]`](https://github.com/rust-lang/rfcs/pull/1517).
* [Add space-friendly arguments](https://github.com/rust-lang/rfcs/pull/1509). Add `-C link-arg` and `-C llvm-arg` which allow you to pass along argument with spaces.
* [Add support for 128-bit integers](https://github.com/rust-lang/rfcs/pull/1504).
* [Add a used attribute to prevent symbols from being discarded](https://github.com/rust-lang/rfcs/pull/1459).
* [Add language support for bitfields](https://github.com/rust-lang/rfcs/pull/1449).

## New RFCs

* [Add an `unwrap!` macro](https://github.com/rust-lang/rfcs/pull/1669).
* [Semantic "private in public" enforcement](https://github.com/rust-lang/rfcs/pull/1671). Enforce that public APIs do not expose private definitions at the semantic level, while allowing the use of private aliases and blanket implementations for convenience and automation.
* [Disjointness based on associated types](https://github.com/rust-lang/rfcs/pull/1672). During coherence checking, when determining if the receivers of two impls are disjoint, treat bounds with disjoint associated types as mutually exclusive bounds.

# Upcoming Events

* 7/13. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [7/13. Rust Boulder/Denver - Hello, Rust!](http://www.meetup.com/Rust-Boulder-Denver/events/232328647/).
* 7/14. Rust release triage at #rust-triage on irc.mozilla.org.
* [7/14. Columbus Rust Society: Monthly Meeting](http://www.meetup.com/columbus-rs/events/231678481/).
* [7/18. Rust Paris Meetup #30](http://www.meetup.com/Rust-Paris/events/230111506/).
* 7/20. Rust Community Team Meeting at #rust-community on irc.mozilla.org.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Rust developer at The Blackbird](https://rust.jobboard.io/jobs/394482-rust-developer-at-the-blackbird).
* [Engineering positions at Zcash mention Rust](https://z.cash/blog/hiring.html).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
