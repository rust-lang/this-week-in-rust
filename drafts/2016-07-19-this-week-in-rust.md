Title: This Week in Rust 139
Number: 139
Date: 2016-07-19
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

* [Rust Project changelog for
  2016-07-15](https://users.rust-lang.org/t/rust-project-changelog-for-2016-07-15/6555/1). Updates
  to rustup, libc, net2, regex, websites.
* [rustup 0.3
  released](https://internals.rust-lang.org/t/beta-testing-rustup-rs/3316/144). Includes
  fixes for downloading old releases, various cleanups, and
  preliminary (non-functional) rustls support.

# Crate of the Week

*No create was selected for CotW.*

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [imag: Make `imag` forward `--debug` and `--verbose` to subcommands](https://github.com/matthiasbeyer/imag/issues/506).
* [moderate] [imag: Add Iterator-shortcut for `iter.fold(Ok(()), ...)`](https://github.com/matthiasbeyer/imag/issues/499).
* [moderate] [rust: Very confusing error on attempt to pass
  `path::Path` by
  value](https://github.com/rust-lang/rust/issues/23286). This is bad
  error message that is hit often. Good bug to get familiar with the
  compiler.
* [easy] [rust: move coerce_match, coerce_calls and related tests into
  run-pass-valgrind](https://github.com/rust-lang/rust/issues/21696). Just
  moving tests around. Easy introduction to the build system.
* [easy] rustbyexample.com is in need of maintainers. Good first tasks
  are [writing Mutex examples](https://github.com/rust-lang/rust-by-example/issues/105)
  and [Arc examples](https://github.com/rust-lang/rust-by-example/issues/104).
* [hard] [rustup: Write a GUI installer for rustup on
  Windows](https://github.com/rust-lang-nursery/rustup.rs/issues/253). This
  is involved but should be fun. It's an integration problem, writing
  a Windows GUI that hooks into the MSI installation system and calls
  into the rustup libraries. Required for rustup 1.0.
* [easy] [cargo: Warn on the duplicate entry points for lib and
  bin](https://github.com/rust-lang/cargo/issues/2800).
* [easy] [cargo: Can't specify precise crate version if there are
  multiple versions](https://github.com/rust-lang/cargo/issues/2773).
* [easy] [error-chain: Display implementation should show the error's
  Display, not just the
  description](https://github.com/brson/error-chain/issues/2). Looks
  like a simple fix.
* [easy] [rust: Parsing inconsistencies (lambda, proc,
  return)](https://github.com/rust-lang/rust/issues/28784). This bug
  identifies some bugs where the rustc parser disagrees with the
  reference parser. Good first bug for someone interested in parsers.

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
