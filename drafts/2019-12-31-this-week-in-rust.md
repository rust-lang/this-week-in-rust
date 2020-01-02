Title: This Week in Rust 319
Number: 319
Date: 2019-12-31
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

[Rust in Blockchain #7 – December 2019](https://rustinblockchain.org/2020/01/02/rust-in-blockchain-7-december-2019/). In blockchain and across the Rust ecosystem, of projects quickly picking up stable async/await support and migrating to tokio 0.2.

# Crate of the Week

This week's crate is [cargo-scout](https://github.com/o0Ignition0o/cargo-scout), a cargo subcommand to run clippy on only the changed code in your crate (following git diff).

Thanks to [Philipp Krones](https://users.rust-lang.org/t/crate-of-the-week/2704/694) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [good first issue] [simdjson: flattend json access for the tape](https://github.com/simd-lite/simdjson-rs/issues/91).
* [good first issue] [rsynth: Add support for System Exclusive events with the jack back-end](https://github.com/PieterPenninckx/rsynth/issues/50).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

334 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-12-16..2019-12-23

* [format the world](https://github.com/rust-lang/rust/pull/67540)
* [refactor expr & stmt parsing + improve recovery](https://github.com/rust-lang/rust/pull/66994)
* [add a raw "address of" operator](https://github.com/rust-lang/rust/pull/64588)
* [improve diagnostics for invalid assignment](https://github.com/rust-lang/rust/pull/67538)
* [use structured suggestion for disambiguating method calls](https://github.com/rust-lang/rust/pull/67127)
* [fix too restrictive checks on Drop impls](https://github.com/rust-lang/rust/pull/67059)
* [save LTO import info and check it when trying to reuse build products](https://github.com/rust-lang/rust/pull/67020)
* [merge `ast::Mutability` and `mir::Mutability`](https://github.com/rust-lang/rust/pull/67355)
* [merge `TraitItem` & `ImplItem into `AssocItem`](https://github.com/rust-lang/rust/pull/67131)
* [indicate origin of where type parameter for uninferred types](https://github.com/rust-lang/rust/pull/67285)
* [allocate HIR on an arena 1/4](https://github.com/rust-lang/rust/pull/66931)
* [add simpler entry points to const eval for common usages](https://github.com/rust-lang/rust/pull/66877)
* [chalk: fix coinductive unsoundness](https://github.com/rust-lang/chalk/pull/272)
* [chalk: move ids to type family](https://github.com/rust-lang/chalk/pull/309)
* [const prop should finish propagation into user defined variables](https://github.com/rust-lang/rust/pull/67130)
* [miri: support main functions with Result return type](https://github.com/rust-lang/miri/pull/1125)
* [implement `LineWriter::write_vectored`](https://github.com/rust-lang/rust/pull/67270)
* [add `PartialEq` and `Eq` to `Cursor`](https://github.com/rust-lang/rust/pull/67233)
* [make `ptr::slice_from_raw_parts` a const fn (behind feature flag)](https://github.com/rust-lang/rust/pull/67462)
* [stabilize `std::{rc,sync}::Weak::{weak_count, strong_count}`](https://github.com/rust-lang/rust/pull/65778)
* [stdarch: use more simd_* intrinsics](https://github.com/rust-lang/stdarch/pull/790)
* [futures.rs: make `AtomicWaker::new()` a const fn](https://github.com/rust-lang/futures-rs/pull/2007)
* [cargo: fix overwriting alternate registry token](https://github.com/rust-lang/cargo/pull/7708)
* [rustup: support local toolchain names in the override file](https://github.com/rust-lang/rustup/pull/2141)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Demote Apple 32bit targets to Tier 3](https://github.com/rust-lang/rfcs/pull/2837).
* [disposition: merge] [Announcing the Safe-Transmute Project Group](https://github.com/rust-lang/rfcs/pull/2835).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

*No RFCs are currently in final comment period.*

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Asia Pacific

* [Jan  8. Kuala Lumpur, MY - Rust Meetup January 2019](https://docs.google.com/forms/d/e/1FAIpQLScb1MoYvLE4hfUlUKzg4LJHNI6Abw41hRIQGyBVVIAcwvdGfQ/viewform).

### Europe

* [Jan  8. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgrybccblb/).
* [Jan  9. Lisbon, PT - Rust Lisbon - Live Jan 2020](https://www.meetup.com/Rust-Lisbon/events/266629066/).
* [Jan 10. Darmstadt, DE - Rust Rhein-Main - 1st 2020 Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/267158461/).

### North America

* [Dec 31. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzqbpc/).
* [Jan  1. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpybccbcb/).
* [Jan  7. Denver, CO, US - Rust Boulder/Denver - Rust Meetup: January](https://www.meetup.com/Rust-Boulder-Denver/events/267240914/).
* [Jan  8. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybccblb/).
* [Jan  8. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/qgvxlrybccblb/).
* [Jan  8. Portland, OR, US - PDXRust - C-Side Tourism: Using C libraries from Rust](https://www.meetup.com/PDXRust/events/266938349/).
* [Jan  9. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybccbmb/).
* [Jan  9. San Diego, CA, US - San Diego Rust January 2020 Meetup](https://www.meetup.com/San-Diego-Rust/events/267242856/).
* [Jan  9. Lehi, UT, US - Utah Rust - January 2020 Regular Meetup](https://www.meetup.com/utah-rust/events/265905282/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Unsoundness is what happens when unsafety goes wrong.

– [Alice Ryhl on rust-users](https://users.rust-lang.org/t/learn-rust-the-dangerous-way-the-unsafe-first-tutorial/35806/39)

Thanks to [Daniel H-M](https://users.rust-lang.org/t/twir-quote-of-the-week/328/764) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
