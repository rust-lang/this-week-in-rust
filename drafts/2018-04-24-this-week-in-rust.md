Title: This Week in Rust 231
Number: 231
Date: 2018-04-24
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

This week's crate is [human-panic](https://crates.io/crates/human-panic), a crate to make Rust's error handling usable to end users. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Clippy](https://github.com/rust-lang-nursery/rust-clippy) has a lot of [good first issues](https://github.com/rust-lang-nursery/rust-clippy/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22). If you are looking for something specific to get started with, here is one: [Split up our UI-tests into smaller parts](https://github.com/rust-lang-nursery/rust-clippy/issues/2038).
* [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide) is a project to write a short guide about how the rust compiler works, and it needs your help. There are some [easier issues](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AEasy), [issues which might require a bit of investigation/code reading](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AMedium), and [issues which probably require some advanced knowledge or a lot of time](https://github.com/rust-lang-nursery/rustc-guide/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3AHard).
* [Help stabilize a subset of Macros 2.0](https://internals.rust-lang.org/t/help-stabilize-a-subset-of-macros-2-0/7252)!
* [good first issue] [distinst: Optimize Partition Moving Algorithm](https://github.com/pop-os/distinst/issues/51). distinst is a distribution installer backend written in Rust.
* [distinst: Reduce LUKS Device Detection Overhead](https://github.com/pop-os/distinst/issues/80).
* [distinst: Use Entire Disk as LUKS / LVM Partition](https://github.com/pop-os/distinst/issues/64).
* [easy] [tokei: Improve tokei's language test coverage](https://github.com/Aaronepower/tokei/issues/63).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

132 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-04-16..2018-04-23

* [stabilize x86/x86_64 SIMD](https://github.com/rust-lang/rust/pull/49664) (WOOT!)
* [wasm: increase default stack size to 1MB](https://github.com/rust-lang/rust/pull/50083)
* [std: minimize size of panicking on wasm](https://github.com/rust-lang/rust/pull/49488)
* [remove 'proc' from the reserved keywords list](https://github.com/rust-lang/rust/pull/49699)
* [proc_macro: stay on the "use the cache" path more](https://github.com/rust-lang/rust/pull/50069)
* [work around LLVM debuginfo problem in librustc_driver](https://github.com/rust-lang/rust/pull/49904)
* [avoid allocating when parsing \u{...} literals](https://github.com/rust-lang/rust/pull/50052)
* [parser: do not override syntactic context for dummy spans](https://github.com/rust-lang/rust/pull/50152)
* [lazily evaluate EvalErrorKind::*.into() calls](https://github.com/rust-lang/rust/pull/50051)
* [change the hashcounts in raw `Lit` variants from `usize` to `u16`](https://github.com/rust-lang/rust/pull/49993)
* [remove HIR inlining](https://github.com/rust-lang/rust/pull/49991)
* [properly handle ranges of signed enums using both extremums](https://github.com/rust-lang/rust/pull/49981)
* [update Rhs on ShlAssign to default to Self](https://github.com/rust-lang/rust/pull/49630)
* [add inherent methods in libcore for `[T]`, `[u8]`, `str`, `f32`, and `f64`](https://github.com/rust-lang/rust/pull/49896)
* [implement size_hint for some iterators](https://github.com/rust-lang/cargo/pull/5272)
* [atomic: remove 'Atomic*' from Debug output](https://github.com/rust-lang/rust/pull/48553)
* [replace {`Alloc`, `GlobalAlloc`}`::oom` with a lang item](https://github.com/rust-lang/rust/pull/50144)
* [stabilize a bunch of minor api additions](https://github.com/rust-lang/rust/pull/50017)
* [rustdoc: UI tests for rustdoc](https://github.com/rust-lang/rust/pull/49542)
* [rustdoc: add doc search aliases](https://github.com/rust-lang/rust/pull/49757)
* [cargo: add new metadata fields](https://github.com/rust-lang/cargo/pull/5386)

## New Contributors

* Alec Mocatta
* Chris Coulson
* Fabio B
* Hero
* Joshua Barretto
* Nikita Popov
* Steven Malis

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Unapprove placement RFCs](https://github.com/rust-lang/rfcs/pull/2387): [1228: Place left arrow syntax (`place <- expr`)](https://github.com/rust-lang/rfcs/blob/master/text/1228-placement-left-arrow.md) and [RFC 809: `box` and placement `in`](https://github.com/rust-lang/rfcs/blob/master/text/0809-box-and-in-for-stdlib.md).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Cargo publish with internal path dependencies](https://github.com/rust-lang/rfcs/pull/2224).

## New RFCs

* [Zero page optimization](https://github.com/rust-lang/rfcs/pull/2400).
* [`mut (x, y, ..)` and `mut [x, y, ..]` pattern shorthand](https://github.com/rust-lang/rfcs/pull/2401).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Apr 19. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxgbzb/).
* [Apr 21. Chennai, IN - Monthly Meetup - April](https://www.meetup.com/mad-rs/events/249535481/).
* [Apr 22. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbdc/).
* [Apr 24. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Apr 24. Dallas, US - Last Tuesday Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxgbgc/).
* [Apr 25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr 25. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Apr 26. New York City, US - Rust NYC (Security)](https://www.meetup.com/Rust-NYC/events/249849155/).
* [Apr 27. Darmstadt, DE - Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/249543182/).
* [Apr 29. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbmc/).
* [May  1. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxhbcb/).
* [May  2. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/249134945/).
* [May  2. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May  2. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxhbdb/).
* [May  2. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxhbdb/).
* [May  2. Indianapolis, US - Indy.rs - Actix Actor Framework](https://www.meetup.com/indyrs/events/cpvshpyxhbdb/).
* [May  3. Utrecht, NL - Rust Workshop](https://www.meetup.com/Rust-Utrecht/events/248995086/).
* **[May 27. Paris, FR - RustFest Paris 2018](https://paris.rustfest.eu/)**.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Real Time Embedded Software Engineer at Callen-Lenz, UK](https://callenlenz.com/contact/careers/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is one of those friends that take some time to get along with, but that you'll finally want to engage with for a long term relationship.

— [Sylvain Wallez](https://bluxte.net/musings/2018/04/10/go-good-bad-ugly/).

Thanks to [u/rushmorem](https://www.reddit.com/r/rust/comments/8bjio2/xpost_from_rprogramming_go_the_good_the_bad_and/dx7u0lu/) and [saethlin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/514) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
