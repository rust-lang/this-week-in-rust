Title: This Week in Rust 222
Number: 222
Date: 2018-02-20
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

This week sadly had to go without a crate for lack of votes.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [github-rs: Pure Rust bindings to the Github API](https://github.com/mgattozzi/github-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22Help+Wanted%22) needs help with some beginner-friendly issues.
* [gutenberg: Make content::Section hold references](https://github.com/Keats/gutenberg/issues/205). Gutenberg is an opinionated static site generator with everything built-in.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

117 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-05..2018-02-12

* epochs: [rustc](https://github.com/rust-lang/rust/pull/48014) and [cargo](https://github.com/rust-lang/cargo/pull/5011) (RFC [#2052](https://rust-lang.github.io/rfcs/2052-epochs.html))
* [rustc: upgrade to LLVM 6](https://github.com/rust-lang/rust/pull/47828)
* [customizable extended tools](https://github.com/rust-lang/rust/pull/48015)
* [fix ICE for mismatched args on target without span](https://github.com/rust-lang/rust/pull/48047)
* [proc_macro: don't panic parsing `..=`](https://github.com/rust-lang/rust/pull/48036)
* [implement `?` macro repetition](https://github.com/rust-lang/rust/pull/47752)
* [warn about more ignored bounds in type aliases](https://github.com/rust-lang/rust/pull/48020)
* [do not ignore lifetime bounds in Copy impls](https://github.com/rust-lang/rust/pull/47877)
* [add filtering options to `rustc_on_unimplemented`](https://github.com/rust-lang/rust/pull/47613)
* [rustc: Add `#[rustc_args_required_const]`](https://github.com/rust-lang/rust/pull/48018)
* [rustc_mir: insert a dummy access to places being matched on, when building MIR](https://github.com/rust-lang/rust/pull/48092)
* [emit data::Impl in save-analysis](https://github.com/rust-lang/rust/pull/47657)
* [ui tests: diff from old (expected) to new (actual) instead of backwards](https://github.com/rust-lang/rust/pull/47978)
* [NLL: improve `DefiningTy::Const`](https://github.com/rust-lang/rust/pull/47957)
* [NLL: add false edges out of infinite loops](https://github.com/rust-lang/rust/pull/47802)
* [stabilize `use_nested_groups`](https://github.com/rust-lang/rust/pull/47948)
* [implement `TrustedLen` for `Take<Repeat>` and `Take<RangeFrom>`](https://github.com/rust-lang/rust/pull/47944)
* [override `try_(r)fold` for RangeInclusive](https://github.com/rust-lang/rust/pull/48012)
* [add some APIs to ptr::NonNull](https://github.com/rust-lang/rust/pull/47631)
* [add `-Zteach` documentation](https://github.com/rust-lang/rust/pull/47843)
* [update book](https://github.com/rust-lang/rust/pull/47753)
* [fix rustdoc ICE on macros defined within functions](https://github.com/rust-lang/rust/pull/47959)
* [make resolution backtracking smarter](https://github.com/rust-lang/cargo/pull/4834)
* [do not rename packages on `cargo new`](https://github.com/rust-lang/cargo/pull/5013)

## New Contributors

* bobtwinkles
* Martin Algesten
* Peter Hrvola
* Yury Delendik

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1909: Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [RFC 2145: Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).
* [RFC 2116: Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Issues are not feature requests](https://github.com/rust-lang/rfcs/pull/2299).
* [disposition: merge] [Allow trivial constraints to appear in where clauses](https://github.com/rust-lang/rfcs/pull/2056).
* [disposition: merge] [impl-only-use](https://github.com/rust-lang/rfcs/pull/2166). The `use …::{… as …}` syntax can now accept `_` as alias to a trait to only import the implementations of such a trait.
* [disposition: merge] [or-patterns in if / while let expressions](https://github.com/rust-lang/rfcs/pull/2175).
* [disposition: merge] [Formally define repr(u32, i8, etc...) and repr(C) on enums with payloads](https://github.com/rust-lang/rfcs/pull/2195).
* [disposition: merge] [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).
* [disposition: merge] [`?` repetition in macro rules](https://github.com/rust-lang/rfcs/pull/2298).
* [disposition: postpone] [Allow fields in traits that map to lvalues in impl'ing type](https://github.com/rust-lang/rfcs/pull/1546).
* [disposition: postpone] [Fix the handling of uninhabited types in pattern matching](https://github.com/rust-lang/rfcs/pull/1872).
* [disposition: postpone] [Unions 1.2](https://github.com/rust-lang/rfcs/pull/1897).
* [disposition: postpone] [Allow destructuring of structs that implement Drop](https://github.com/rust-lang/rfcs/pull/2061).
* [disposition: postpone] [Adding unsafe modules and unsafe blocks outside functions](https://github.com/rust-lang/rfcs/pull/2148).
* [disposition: close] [Guard Clause Flow Typing](https://github.com/rust-lang/rfcs/pull/2221).
* [disposition: close] [Legal double reference](https://github.com/rust-lang/rfcs/pull/2268).
* [disposition: close] [Add match/in statements](https://github.com/rust-lang/rfcs/pull/2144).

## New RFCs

* [Stable SIMD in Rust](https://github.com/rust-lang/rfcs/pull/2325).
* [Officially adopt Ferris as the mascot for the current epoch](https://github.com/rust-lang/rfcs/pull/2328).
* [Prior art](https://github.com/rust-lang/rfcs/pull/2333). A section to the RFC template where RFC authors may discuss the experience of other programming languages.
* [Amend RFC 0141 Lifetime elision: Mention deduplicated lifetimes](https://github.com/rust-lang/rfcs/pull/2330).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Feb 15. Cambridge, UK - Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/mgtcwnyxdbtb/).
* [Feb 17. Chennai, India - Monthly Meetup - February](https://www.meetup.com/mad-rs/events/247446699/).
* [Feb 18. Rust Dev in Mountain View - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxdbxb/).
* [Feb 21. Vilnius, Lithuania - Rust Meetup #2](https://www.meetup.com/Rust-in-Vilnius/events/244401223/).
* [Feb 21. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb 21. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb 22. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Feb 22. Rust London User Group - LDN Talks: February 2018](https://www.meetup.com/Rust-London-User-Group/events/246860921/).
* [Feb 22. Minneapolis, US - February 2018 Meetup](https://www.meetup.com/RustMN/events/247512052/).
* [Feb 25. Rust Dev in Mountain View - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxdbxb/).
* [Feb 27. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-content)
* [Feb 28. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb 28. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb 28. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
