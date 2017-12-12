Title: This Week in Rust 212
Number: 212
Date: 2017-12-12
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

This week's crate is [printpdf](https://crates.io/crates/printpdf), a pure Rust PDF-writing library that already has a lot of features (though I note a lot of
bool-taking methods). Thanks to [Felix Schütt](https://users.rust-lang.org/u/sharazam) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

105 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-12-04..2017-12-11

* [fix bad error message for cannot_reborrow_already_uniquely_borrowed](https://github.com/rust-lang/rust/pull/46572)
* [rustc_mir: promote references of statics from other statics](https://github.com/rust-lang/rust/pull/46524)
* [MIR borrowck: implement union-and-array-compatible semantics](https://github.com/rust-lang/rust/pull/46268)
* [incr.comp.: uncorporate the stable commandline arg hash and SVHs of upstream crates into the SVH](https://github.com/rust-lang/rust/pull/46427)
* [incr.comp.: enable query result caching for many more queries](https://github.com/rust-lang/rust/pull/46556)
* [coherence: fix is_knowable logic](https://github.com/rust-lang/rust/pull/46192)
* [rustc_trans: don't write discriminants for uninhabited variants](https://github.com/rust-lang/rust/pull/46521)
* [rustc: further tweak linkage in ThinLTO](https://github.com/rust-lang/rust/pull/46549)
* [stabilize abi_sysv64](https://github.com/rust-lang/rust/pull/46528)
* [add read, read_string, and write functions to std::fs](https://github.com/rust-lang/rust/pull/45837)
* [add Drop impl for linked_list::DrainFilter](https://github.com/rust-lang/rust/pull/46581)
* [compiletest: account for `ui` reference files when deciding to skip](https://github.com/rust-lang/rust/pull/46533)
* [rustdoc: improve search style](https://github.com/rust-lang/rust/pull/46502)

## New Contributors

* Christian Duerr
* Irina-Gabriela Popa
* Julian Kulesh
* Kenjiro Nakayama
* Kyle Huey
* Nikolay Merinov

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

* [Closures capture disjoint fields](https://github.com/rust-lang/rfcs/pull/2229).
* [Bury `Error::description()`](https://github.com/rust-lang/rfcs/pull/2230).
* [String reflection](https://github.com/rust-lang/rfcs/pull/2233).
* [Add a machine-readable JSON-output mode for Rust's libtest](https://github.com/rust-lang/rfcs/pull/2234).

# Upcoming Events

* [Dec 11. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/svbqbmywqbpb/).
* [Dec 13. Rust Amsterdam: Theme night on Procedural Macros & Custom Derive](https://www.meetup.com/Rust-Amsterdam/events/245075721/)
* [Dec 13. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Dec 13. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Dec 13. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywqbrb/).
* [Dec 14. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Dec 14. Rust DC - Mid-month Rustful: Falcon](https://www.meetup.com/RustDC/events/243672324/).
* [Dec 14. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlywqbsb/).
* [Dec 20. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Dec 20. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Dec 21. Cambridge Rust Meetup #6](https://www.meetup.com/Cambridge-Rust-Meetup/events/mgtcwnywqbcc/).

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
