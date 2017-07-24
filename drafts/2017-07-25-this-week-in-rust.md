Title: This Week in Rust 192
Number: 192
Date: 2017-07-25
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

This week's crate is [extfsm](https://crates.io/crates/extfsm), a crate to help build finite state machines. Thanks to [Tony P.](https://users.rust-lang.org/u/prz) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get in the swing with the libz blitz contest: Win free tickets to RustFest Zürich](http://blog.rustfest.eu/libz-blitz).
* [Rust libz blitz status update 2017/07/14](https://internals.rust-lang.org/t/rust-libz-blitz/5184/120).
* [gcc: Rename crate](https://github.com/alexcrichton/gcc-rs/issues/186).
* [gcc: Extract windows_registry module into its own crate](https://github.com/alexcrichton/gcc-rs/issues/174).
* [gcc: Add panic conditions to various functions](https://github.com/alexcrichton/gcc-rs/issues/181).
* [gcc: Clarify behavior of `expand`](https://github.com/alexcrichton/gcc-rs/issues/187).
* [gcc: Rename `Config` type](https://github.com/alexcrichton/gcc-rs/issues/189).
* [gcc: Add warning control](https://github.com/alexcrichton/gcc-rs/issues/190).
* [gcc: Add try_compile and try_expand](https://github.com/alexcrichton/gcc-rs/issues/192).
* [gcc: Mention in the crate docs that this crate also supports assembly](https://github.com/alexcrichton/gcc-rs/issues/193).
* [gcc: Explain what metadata is emitted by `cargo_metadata`](https://github.com/alexcrichton/gcc-rs/issues/194).
* [gcc: Document the `rayon` feature](https://github.com/alexcrichton/gcc-rs/issues/196).
* [gcc: Backward spans are displayed strangely](https://github.com/rust-lang/rust/issues/42104).
* [rust-cookbook: Add "Extract phone numbers from text" example](https://github.com/brson/rust-cookbook/issues/241).
* [rust-cookbook: Add "Filter a log file by matching multiple regular expressions" example](https://github.com/brson/rust-cookbook/issues/244).
* [rust-cookbook: Add "Extract all unique links from a webpage" example](https://github.com/brson/rust-cookbook/issues/245).
* [rust-cookbook: Use a custom environment variable to set up logging](https://github.com/brson/rust-cookbook/issues/222).
* [rust-cookbook: Add "Obtain backtrace of complex error scenarios" example](https://github.com/brson/rust-cookbook/issues/216).
* [rust-cookbook: Add "Avoid discarding errors during error conversions" example](https://github.com/brson/rust-cookbook/issues/215).
* [semver: Implement serde Serialize/Deserialize for VersionReq](https://github.com/steveklabnik/semver/issues/119).
* [skeptic: Produce better names for test cases](https://github.com/brson/rust-skeptic/issues/38).
* [easy] [skeptic: Allow lines that begin with `#!`](https://github.com/brson/rust-skeptic/issues/39).
* [easy] [rustup: Replace rustup_utils::remove_dir_all with remove_dir_all crate](https://github.com/rust-lang-nursery/rustup.rs/issues/1204).
* [easy] [stdx: Add encoding_rs crate](https://github.com/brson/stdx/issues/51).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

103 pull requests were [merged in the last week][merged]

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-07-10..2017-07-17

* [don't panic, compiler-builtins](https://github.com/rust-lang/rust/pull/43258)
* [thread-local `pub(restricted)`](https://github.com/rust-lang/rust/pull/43185)
* [thread-local try-with](https://github.com/rust-lang/rust/pull/43158)
* [macro parsing improvements](https://github.com/rust-lang/rust/pull/42913) (fixes *a lot* of issues around old macros),
  [also identifiers in patterns no longer cause problems](https://github.com/rust-lang/rust/pull/43224)
* [revert some SIMD annotations causing problems on PowerPC](https://github.com/rust-lang/rust/pull/43159)
* [More Rust/RLS integration](https://github.com/rust-lang/rust/pull/42146)
* [`cargo test` now fails if no tests found](https://github.com/rust-lang/rust/pull/43145)
* [`cargo` conventions around libs / binaries streamlined](https://github.com/rust-lang/cargo/pull/4259) (epic refactor)

## New Contributors

* Luca Barbato
* Lynn
* Sam Cappleman-Lynes
* Valentin Brandl
* William Brown
* Yorwba

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1937: Allow the `?` operator to be used in `main`, and in `#[test]` functions and doctests](https://github.com/rust-lang/rfcs/pull/1937).
* [RFC 1969: Prepublication dependencies for Cargo](https://github.com/rust-lang/rfcs/pull/1969).
* [RFC 1940: Support the `#[must_use]` attribute on arbitrary functions](https://github.com/rust-lang/rfcs/pull/1940).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Put the RFCs repo under license terms](https://github.com/rust-lang/rfcs/pull/2044).
* [disposition: merge] [Tweak object safety rules to allow static dispatch](https://github.com/rust-lang/rfcs/pull/2027).
* [disposition: postpone] [Warn by default when casting a pointer to an integer smaller than usize](https://github.com/rust-lang/rfcs/pull/1782).
* [disposition: merge] [Amend #1440: allow `const` items to contain drop types](https://github.com/rust-lang/rfcs/pull/1817).
* [disposition: merge] [Add `extern type` declarations for declaring types from external libraries which have an unknown size/layout](https://github.com/rust-lang/rfcs/pull/1861).
* [disposition: merge] [Intra Rustdoc Links](https://github.com/rust-lang/rfcs/pull/1946).
* [disposition: postpone] [Allow the usage of `use` inside `impl` blocks and `match` blocks](https://github.com/rust-lang/rfcs/pull/1976).
* [disposition: merge] [Enable nested method calls](https://github.com/rust-lang/rfcs/pull/2025).

## New RFCs

* [Infer function signatures from trait declaration into 'impl's](https://github.com/rust-lang/rfcs/pull/2063).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

Currently being discussed:

* [Define short](https://github.com/rust-lang-nursery/fmt-rfcs/issues/47)
* [Special casing some macros](https://github.com/rust-lang-nursery/fmt-rfcs/issues/86)


# Upcoming Events

* [Jul 20. Rust DC: Lessons learned integrating Rust with Ruby](https://www.meetup.com/RustDC/events/241110467/).
* [Jul 20. Mozilla Community Dresden Meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/241302860/).
* [Jul 24. Triangle Rustaceans - Durham, NC - Hack Night](https://www.meetup.com/triangle-rustaceans/events/241451502/).
* [Jul 26. Ciudad de México, México - Rust Meetup #6](https://www.meetup.com/es-ES/Rust-MX/events/241026269/).
* [Jul 26. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jul 26. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jul 26. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/241512566/).
* [Jul 27. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Aug  2. Rust User Group Cologne - Crate Polishing](http://rust.cologne/2017/08/02/crate-polishing.html).
* [Aug  2. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug  2. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Good farmers use their bare hands, average farmers use a combine harvester.

— [/u/sin2pifx in response](https://www.reddit.com/r/rust/comments/6nqfez/slashdot_reports_on_a_techcrunch_opinion_piece/dkbpulb/) to "Good programmers write C, average programmers write Rust".

Thanks to [Rushmore](https://users.rust-lang.org/t/twir-quote-of-the-week/328/421) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
