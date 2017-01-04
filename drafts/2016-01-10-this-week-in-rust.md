Title: This Week in Rust 164
Number: 164
Date: 2017-01-10
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

## Other Weeklies from Rust Community

# Crate of the Week

This week's Crate of the Week is [rocket](https://crates.io/crates/rocket), an experimental web framework (will need a nightly Rust!) with a focus on ease-of-use, expressability and speed. Thanks to Vikrant for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [Diesel: SQLite Getting started/Skeleton app](https://github.com/diesel-rs/diesel/issues/376).
* [easy] [Diesel: Refactorings using macros in type position](https://github.com/diesel-rs/diesel/issues/521).
* [easy] [Diesel: Deny missing docs](https://github.com/diesel-rs/diesel/issues/563).
* [android-rs-glue: Add more arguments and use clap to parse the arguments](https://github.com/tomaka/android-rs-glue/issues/115).
* [tokei: Add package repositories](https://github.com/Aaronepower/tokei/issues/92).
* [RustCrypto/hashes: Missing hash functions](https://github.com/RustCrypto/hashes/issues/1).
* [RustCrypto/block-ciphers: Missing block ciphers](https://github.com/RustCrypto/block-ciphers/issues/1).
* [Funding Redox OS development](https://www.reddit.com/r/rust/comments/5klu34/funding_redox_os_development/).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

109 pull requests were [merged in the last week][merged]. This contains a good number of plugin-breaking changes.

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-12-26..2016-01-02

* [`u128`/`i128` support!](https://github.com/rust-lang/rust/pull/38482) (RFC [#1504](https://github.com/rust-lang/rfcs/blob/master/text/1504-int128.md)), also [vim](https://github.com/rust-lang/rust.vim/pull/133)
* [new `min_atomic_width` target option](https://github.com/rust-lang/rust/pull/38579)
* [`--crate-type=metadata` is now `--emit=metadata`](https://github.com/rust-lang/rust/pull/38571)
* [Rust can now compile to PTX (cuda)](https://github.com/rust-lang/rust/pull/38559)
* [`fastcall` calling convention fixed](https://github.com/rust-lang/rust/pull/38542)
* [`pub(restricted)` visibilities are now handled by `DefId`](https://github.com/rust-lang/rust/pull/38490) (potentially plugin-breaking)
* [rustbuild now build's twice (instead of thrice) by default](https://github.com/rust-lang/rust/pull/38631) (hooray for faster builds!)
* [A new `DroplessArena` for some speedups](https://github.com/rust-lang/rust/pull/38653)
* [Debuggers can now pretty-print unions](https://github.com/rust-lang/rust/pull/38753)
* [Custom derives now can work on structs with macros](https://github.com/rust-lang/rust/pull/38737)
* [sparc64-linux support](https://github.com/rust-lang/rust/pull/38726)
* [`impl TrustedLen for `{`Empty`, `Once`}](https://github.com/rust-lang/rust/pull/38713)

## New Contributors

* Adam Langley
* Christoph Schulz
* Curtis McEnroe
* E. Dunham
* Ian Kerins
* kellerkindt
* Luc Street
* Martijn Vermaat
* Philip Craig

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

* [crates.io] [What categories should be available on crates.io](https://github.com/rust-lang/crates.io/pull/488)?
* [Roadmap for 2017](https://github.com/rust-lang/rfcs/pull/1774).
* [`core::mem::replace_with` for temporarily moving out of ownership](https://github.com/rust-lang/rfcs/pull/1736).
* [Add a 'thread lifetime, which denotes a thread-bounded region](https://github.com/rust-lang/rfcs/pull/1705).
* [Allow `Self` to appear in the where clause of trait impls](https://github.com/rust-lang/rfcs/pull/1647).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).
* [Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).
* [Add Rvalue-static-promotion](https://github.com/rust-lang/rfcs/pull/1414).

## New RFCs

* [Reject crates.io uploads which declare a feature named `no_std`](https://github.com/rust-lang/rfcs/pull/1841).
* [Generators](https://github.com/rust-lang/rfcs/pull/1832).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

Ready for PR:

There's [a lot of them](https://github.com/rust-lang-nursery/fmt-rfcs/issues?q=is%3Aopen+is%3Aissue+label%3Aready-for-PR) right now, contributions here would be very welcome. If you want advice or help getting started, please ping nrc, or any other member of the style team, in #rust-style.

Issues in final comment period:

* [Conventions for Cargo.toml files (FCP)](https://github.com/rust-lang-nursery/fmt-rfcs/pull/41).
* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).

# Upcoming Events


* [1/5. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/236137922/).
* [1/5. Rust DC Hack Session—Part 3](https://www.meetup.com/RustDC/events/236141535/).
* [1/9. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/236209293/)
* [1/11. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/11 Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/12. Rust Utrecht](https://www.meetup.com/Rust-Utrecht/events/235444678/).
* [1/12. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I would argue that if the Rust project would have just one mission statement, it wouldn't be "create a safe systems programming language". It would be "move towards a world where safe systems programming is the norm".

— [GolDDranks in reply to steveklabnik](https://news.ycombinator.com/item?id=13277096).

Thanks to [matematikaadit](https://users.rust-lang.org/users/matematikaadit) for the [suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/338).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
