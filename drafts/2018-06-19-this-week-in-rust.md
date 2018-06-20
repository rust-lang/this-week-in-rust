Title: This Week in Rust 239
Number: 239
Date: 2018-06-19
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

* [Divans: Open source compression](https://blogs.dropbox.com/tech/2018/06/building-better-compression-together-with-divans/) in Rust from Dropbox using threads, SIMD with a [WASM demo](https://dropbox.github.io/divans) in the browser.
* [Rust on LPC82x microcontrollers using lpc82x-hal](https://users.rust-lang.org/t/lpc82x-hal-0-2-rust-on-lpc82x-microcontrollers/18144).
* [Building a Diesel project on GitLab CI](https://noyez.gitlab.io/post/2018-06-15-rust-plus-diesel-plus-gitlab/).

# Crate of the Week

This week's crate is [SIMDNoise](https://crates.io/crates/simdnoise), a crate to use modern CPU vector instructions to generate various types of noise really fast. Thanks to [gregwtmtno](https://users.rust-lang.org/u/gregwtmtno) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Sponsor work on Rust](https://aturon.github.io/sponsor/)!
* [wasm-pack has several open good first issues available to new contributors](https://github.com/ashleygwilliams/wasm-pack/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

66 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-06-11..2018-06-18

* [rustc_codegen_llvm: don't treat `i1` as signed, even for `#[repr(i8)]` enums](https://github.com/rust-lang/rust/pull/51594)
* [rustdoc: process cross-crate glob re-exports](https://github.com/rust-lang/rust/pull/51584)
* [Stabilize `#[repr(transparent)]`](https://github.com/rust-lang/rust/pull/51562)
* [libstd: add an RAII utility for `sys_common::mutex::Mutex`](https://github.com/rust-lang/rust/pull/51529)
* [Don't auto-hide inherent impls even if `rustdoc-collapse == true`](https://github.com/rust-lang/rust/pull/51527)
* [Add `Ref`/`RefMut::map_split` method](https://github.com/rust-lang/rust/pull/51466)
* [Improve memoization and refactor NLL type check](https://github.com/rust-lang/rust/pull/51460)
* [refactor: create multiple HIR items for imports](https://github.com/rust-lang/rust/pull/51425)
* [Speed up obligation forest code](https://github.com/rust-lang/rust/pull/51411)
* [`impl Hash for !`](https://github.com/rust-lang/rust/pull/51404)
* [Stabilize `GlobalAlloc` and `#[global_allocator]`](https://github.com/rust-lang/rust/pull/51241)
* [Replace `core::iter::AlwaysOk<T>` by `Result<T, !>`](https://github.com/rust-lang/rust/pull/50941)
* [Stabilize `std::path::Path::ancestors`](https://github.com/rust-lang/rust/pull/50894)
* [Add error message for using >= 65535 hashes for raw string literal escapes](https://github.com/rust-lang/rust/pull/50296)

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

* [disposition: close] [Reserve `throw` and `fail` as keywords in edition 2018](https://github.com/rust-lang/rfcs/pull/2441).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Exhaustive integer matching](https://github.com/rust-lang/rust/pull/50912).
* [disposition: merge] [Stabilize `std::path::Path::ancestors`](https://github.com/rust-lang/rust/pull/50894).
* [disposition: merge] [Add ability to apply custom derive to union types](https://github.com/rust-lang/rust/pull/50383).
* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: merge] [Tracking issue for "macro naming and modularisation" (RFC #1561)](https://github.com/rust-lang/rust/issues/35896).
* [disposition: merge] [Tracking issue for promoting `!` to a type (RFC 1216)](https://github.com/rust-lang/rust/issues/35121).

## New RFCs

* [Add lint warning for inner function marked as `#[test]`](https://github.com/rust-lang/rfcs/pull/2471).

# Upcoming Events

### Online

* [Jun 19. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Jun 20. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jun 20. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jun 27. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).

### Europe

* [Jun 21. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxjbcc/).
* [Jun 27. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/251675898/).
* [Jul 6. Darmstadt, DE - Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/251928672)

### North America

* [Jun 14. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxjbsb/).
* [Jun 14. San Diego, US - San Diego Rust June Meetup](https://www.meetup.com/San-Diego-Rust/events/251001684/).
* [Jun 14. Utah Valley, US - Utah Rust meetup](https://docs.google.com/document/d/1O8S7IEfDw-3jTN74CWCuKYl_UWxTLd6-epz7NOMDYRg).
* [Jun 14. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/250848451).
* [Jun 17. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxjbwb/).
* [Jun 19. Denver, US - Rust Boulder/Denver - June Meetup in Boulder](https://www.meetup.com/Rust-Boulder-Denver/events/250076478/).
* [Jun 19. Kitchener, CA - Graph Mining in Rust & Your Project Demos](https://www.meetup.com/Rust-KW/events/251426929/).
* [Jun 24. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxjbgc/).
* [Jun 26. Dallas, US - Last Tuesday Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxjbjc/).
* [Jun 26. New York City, US - Procedural Macros - parse JSX using nom](https://www.meetup.com/Rust-NYC/events/251490499/).
* [Jun 27. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxjbkc/).
* [Jun 28. San Francisco, US - Rust Bay Area - Zero Knowledge Proof Macros and Cernan (data pipelining)](https://www.meetup.com/Rust-Bay-Area/events/244156617/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).** Registration is now open.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Commure, Inc. San Francisco, US](https://news.ycombinator.com/item?id=16968087).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> In Rust it’s the compiler that complains, with C++ it’s the colleagues

– Michal 'Vorner' Vaner on [gitter](https://gitter.im/rust-lang/rust?at=5b212b1a37a2df7bed398c7c)

(selected by llogiq per one unanimous vote)

[Please submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
