Title: This Week in Rust 214
Number: 214
Date: 2017-12-26
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

# Crate of the Week

This week's crate is [cargo-audit](https://crates.io/crates/cargo-audit), a cargo subcommand to look through a crates dependencies for known insecure versions. Thanks to [Danilo](https://users.rust-lang.org/u/dbrgn) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Use cargo-contribute to find issues in your dependencies to contribute to](https://github.com/Xion/cargo-contribute).
* [Rusoto, an AWS SDK, is looking for maintainers](https://github.com/rusoto/rusoto/issues/593).
* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [good first issue] [Criterion.rs: Replace rustc_serialize with Serde](https://github.com/japaric/criterion.rs/issues/91).
* [good first issue] [Criterion.rs: Replace Floaty with num-traits](https://github.com/japaric/criterion.rs/issues/95).
* [medium] [allocators-rs: object-alloc-test: Finish `corruption::mapped::is_mapped_range` on Windows](https://github.com/ezrosent/allocators-rs/issues/137).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

82 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-12-11..2017-12-18

* [Validate miri against the HIR const evaluator](https://github.com/rust-lang/rust/pull/45002)
* [Implement impl Trait lifetime elision](https://github.com/rust-lang/rust/pull/46616)
* [ThinLTO: updates for LLVM 5](https://github.com/rust-lang/rust/pull/46652)
* [incr.comp.: Mark DepKind node as input](https://github.com/rust-lang/rust/pull/46811)
* [syntax: recovery for incorrect associated item paths like `[T; N]::clone`](https://github.com/rust-lang/rust/pull/46788)
* [suggest `..` for erroneous `...` struct field patterns](https://github.com/rust-lang/rust/pull/46763)
* [When attempting to write str with single quote suggest double quotes](https://github.com/rust-lang/rust/pull/46653)
* [fix borrow casts or binary expression suggestions](https://github.com/rust-lang/rust/pull/46761)
* [Fix division-by-zero ICE in -Z perf-stats](https://github.com/rust-lang/rust/pull/46728)
* [Point at var in short lived borrows instead of drop location](https://github.com/rust-lang/rust/pull/46719)
* [Point at whole method call instead of args](https://github.com/rust-lang/rust/pull/46633)
* [Fix visible_parent_map to choose globally minimal paths](https://github.com/rust-lang/rust/pull/46708)
* [Lifetime Resolution for Generic Associated Types](https://github.com/rust-lang/rust/pull/46706)
* [rustc: unpack newtyped of `#[repr(simd)]` vector types](https://github.com/rust-lang/rust/pull/46701)
* [rustc_trans: approximate ABI alignment for padding/union fillers](https://github.com/rust-lang/rust/pull/46623)
* [lossless UNIX OsStr Debug impl](https://github.com/rust-lang/rust/pull/46798)
* [Expose the line and column fields from the proc_macro::LineColumn struct](https://github.com/rust-lang/rust/pull/46690)
* [cargo: Don’t swallow virtual manifest parsing errors](https://github.com/rust-lang/cargo/pull/4828)

## New Contributors

* David Teller
* Felix Schütt
* Nika Layzell
* qres
* varkor

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

* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [Change impls of `PartialEq` and friends in libstd to be more generic](https://github.com/rust-lang/rfcs/pull/2245).
* [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).

# Upcoming Events

* [Dec 21. Cambridge Rust Meetup #6](https://www.meetup.com/Cambridge-Rust-Meetup/events/mgtcwnywqbcc/).
* [Dec 27. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Dec 27. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Dec 28. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jan  2. Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxcbdb/).
* [Jan  3. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxcbfb/).
* [Jan  3. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jan  3. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at ANIXE Wrocław Poland](http://anixe.pl/jobs/rust_dev/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
