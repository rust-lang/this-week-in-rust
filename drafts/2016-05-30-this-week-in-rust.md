Title: This Week in Rust 184
Number: 184
Date: 2017-05-30
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

This week's crate is [Oath2](https://crates.io/crates/oath2), a library for two-factor authentication. Thanks to [crypto-universe](https://users.rust-lang.org/u/crypto-universe) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust libz blitz status update 2017-05-19](https://internals.rust-lang.org/t/rust-libz-blitz/5184/34). Contribution opportunities are available.
* The log crate just completed its [evaluation](https://internals.rust-lang.org/t/crate-evaluation-for-2017-05-16-log/5185/50), generating a ton of [easy-tagged issues](https://github.com/rust-lang-nursery/log/issues?utf8=%E2%9C%93&q=is%3Aissue%20is%3Aopen%20label%3A%22help%20wanted%22%20label%3Aeasy).
* [Rust cookbook needs a lot of help](https://github.com/brson/rust-cookbook/issues?q=is%3Aissue+is%3Aopen+label%3Aexample)!
* [Help wanted: Rust for embedded development: Where we are and what’s missing](https://users.rust-lang.org/t/rust-for-embedded-development-where-we-are-and-whats-missing/10861).
* [rust-url is looking for maintainers](https://users.rust-lang.org/t/help-wanted-maintaining-rust-url/10707).
* [easy] [i3status-rust is looking for contributors to make i3 window manager more awesome](https://github.com/XYunknown/i3status-rust)!
* [rust: Teach .pkg, .msi, .exe installers about RLS](https://github.com/rust-lang/rust/issues/42157).
* [rust: Get test suite working with wasm](https://github.com/rust-lang/rust/issues/38800).
* [gutenberg: Make a SyntaxSet pool](https://github.com/Keats/gutenberg/issues/70). Gutenberg is an opinionated static site generator written in Rust.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

94 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-05-22..2017-05-29

* [declarative macros 2.0](https://github.com/rust-lang/rust/pull/40847) (RFC [#1584](https://github.com/rust-lang/rfcs/blob/0f130f5341574a881283fc00b36166ab4109d669/text/1584-macros.md))
* [stabilize `loop_break_value`](https://github.com/rust-lang/rust/pull/42016) (RFC [#1624](https://github.com/rust-lang/rfcs/blob/0f130f5341574a881283fc00b36166ab4109d669/text/1624-loop-break-value.md))
* [stabilize destructor-less unions with `Copy` fields](https://github.com/rust-lang/rust/pull/42068)
* [stabilize library features for 1.18.0](https://github.com/rust-lang/rust/pull/41904)
* [move the mutable parts out of `LintStore`](https://github.com/rust-lang/rust/pull/42052)
* [support Win32 `thiscall` calling convention](https://github.com/rust-lang/rust/pull/42058)
* [correctly count errors](https://github.com/rust-lang/rust/pull/42150)
* [improve help message on erroneous usage of `PartialEq`](https://github.com/rust-lang/rust/pull/41559)
* [fix `missing_docs` lint ICE with nested enums](https://github.com/rust-lang/rust/pull/42262)
* [forbid more parenthesized parameters in primitive types](https://github.com/rust-lang/rust/pull/41856)
* [make assignments to `Copy` union fields safe](https://github.com/rust-lang/rust/pull/42083)
* [type flags (like `is_sized`) are now proper queries](https://github.com/rust-lang/rust/pull/42015)
* [`trace_macros` now shows both macro call & expansion](https://github.com/rust-lang/rust/pull/42103)
* [prevent `StorageLive`/`StorageDead` in statics](https://github.com/rust-lang/rust/pull/42023)
* [translate array drop glue using MIR](https://github.com/rust-lang/rust/pull/41917)
* [`Iterator::step_by(_)` can now be `ExactSizeIterator`](https://github.com/rust-lang/rust/pull/42167)
* [two-field `RangeInclusive`](https://github.com/rust-lang/rust/pull/42134)
* [cargo can now `--exclude` packages when using `--all`](https://github.com/rust-lang/cargo/pull/4031)

## New Contributors

* Anders Papitto
* Daniel Lockyer
* David LeGare
* Ivan Dardi
* Michael Kohl
* Mike Lubinets
* Venkata Giri Reddy

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Amendment to RFC 1192: Make RangeInclusive just a two-field struct](https://github.com/rust-lang/rfcs/pull/1980).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Finalize syntax and parameter scoping for `impl Trait`, while expanding it to arguments](https://github.com/rust-lang/rfcs/pull/1951).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [disposition: close] [Add Cargo post-build scripts](https://github.com/rust-lang/rfcs/pull/1777).

## New RFCs

* [Support of alternate registries in Cargo](https://github.com/rust-lang/rfcs/pull/2006).
* [Match ergonomics using default binding modes](https://github.com/rust-lang/rfcs/pull/2005).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

Issues in final comment period:

* [Combining opening and closing delims](https://github.com/rust-lang-nursery/fmt-rfcs/issues/61)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)

# Upcoming Events

* [May 24. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 24. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 30. Rust Oslo - Fearless programming with Rust - Adventures with Asynchronous I/O](https://www.meetup.com/Rust-Oslo/events/238315636/).
* [May 30. Rust Toronto meetup - Hands-on parsing in Rust](https://www.meetup.com/Rust-Toronto/events/239839632/).
* [May 31. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/239889748/).
* [May 31. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 31. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun  1. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jun  5. Rust Prague Meetup #4](https://www.meetup.com/rust-prague/events/240025447/).
* [Jun  6. Mozilla Rust Roadshow @ GA Boston: Rust! Hack Without Fear](https://generalassemb.ly/education/ga-mozilla-developer-roadshow-presents-rust-hack-without-fear/boston/36069).
* [Jun  7. Rust Cologne - Rust 2nd Anniversary Reloaded](http://rust.cologne/2017/06/07/rust-2nd-aniversary-part-2.html).
* [Jun  7. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/240072184/).
* [Jun  7. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jun  7. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun 8. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/240198831/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at 1aim](https://rustjobs.rs/jobs/22/1aim-gmbh-rust-developer).
* [Rust Developer at Anixe](https://rustjobs.rs/jobs/21/anixe-rust-developer)
* [Rust Legend at Between Lines](https://rustjobs.rs/jobs/20/between-lines-ltd-rust-legend)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
