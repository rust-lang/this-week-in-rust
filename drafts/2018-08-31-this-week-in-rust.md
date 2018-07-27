Title: This Week in Rust 245
Number: 245
Date: 2018-07-31
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

* [Programming Servo: How to match](https://medium.com/programming-servo/programming-servo-how-to-match-b76c43f76fe6)

# Crate of the Week

This week's crate is [rav1e](https://github.com/xiph/rav1e), the fastest and safest AV1 encoder from Xiph.Org Foundation. Thanks to [nasa42](https://users.rust-lang.org/t/crate-of-the-week/2704/419) for suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [How to help test the 2018 edition](https://www.ncameron.org/blog/how-to-help-test-the-2018-edition/).
* [bitwarden_rs: Looking for wannabe rustaceans, that'd like to get their feet wet](https://www.reddit.com/r/rust/comments/90xh79/looking_for_wannabe_rustaceans_thatd_like_to_get/).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

151 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-07-16..2018-07-23

* [Cargo: Import `cargo fix` directly in to Cargo](https://github.com/rust-lang/cargo/pull/5723).
* [Implement existential types](https://github.com/rust-lang/rust/pull/52024).
* [Overhaul exit codes for rustc and rustdoc](https://github.com/rust-lang/rust/pull/52197).
* [rustc: Stabilize `#[wasm_import_module]` as `#[link(...)]`](https://github.com/rust-lang/rust/pull/52445).
* [Stabilize lint handling in rustdoc](https://github.com/rust-lang/rust/pull/52354).
* [Deprecation of `str::slice_unchecked(_mut)`](https://github.com/rust-lang/rust/pull/51807).
* [Lint `async` identifiers in 2018 preparation mode](https://github.com/rust-lang/rust/pull/52375).
* [rustc: Enable `use_extern_macros` in 2018 edition](https://github.com/rust-lang/rust/pull/52472).
* [Implement statfs for dragonfly, freebsd and openbsd](https://github.com/rust-lang/libc/pull/1039).
* [Speed up `SparseBitMatrix` use in `RegionValues`](https://github.com/rust-lang/rust/pull/52250).
* [mem::swap the obvious way for types smaller than the SIMD optimization's block size](https://github.com/rust-lang/rust/pull/52051).
* [Cargo: Don't warn about ignored files in cargo-fix](https://github.com/rust-lang/cargo/pull/5770).
* [rustc: Work around an upstream wasm ThinLTO bug](https://github.com/rust-lang/rust/pull/52506).
* [Allow clippy to be installed with make install](https://github.com/rust-lang/rust/pull/52464).
* [regex: expose lower level search APIs](https://github.com/rust-lang/regex/pull/493).
* [Implement rfc 1789: Conversions from `&mut T` to `&Cell<T>`](https://github.com/rust-lang/rust/pull/50494).

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2289: Associated type bounds of form `MyTrait<AssociatedType: Bounds>`](https://github.com/rust-lang/rfcs/pull/2289).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue: RFC 2103 - attributes for tools](https://github.com/rust-lang/rust/issues/44690).
* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: merge] [Modularize crate-local `#[macro_export] macro_rules`](https://github.com/rust-lang/rust/pull/52234).

## New RFCs

* [Fix the Error trait](https://github.com/rust-lang/rfcs/pull/2504).
* [Make AcqRel universally usable as ordering mode](https://github.com/rust-lang/rfcs/pull/2503).

# Upcoming Events

### Online

* [Jul 31. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Aug  1. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Aug  1. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Aug  8. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Aug  7. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxlbkb/).

### Europe

* [Aug  1. Cologne, DE - Rust Cologne](https://www.meetup.com/RustCologne/events/252432033).
* [Aug  8. Berlin, DE - Binding to Rust from everything](https://www.meetup.com/Rust-Berlin/events/252872742/).
* [Aug  8. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/xkdlvpyxlblb/).

### North America

* [Jul 29. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxkbmc/).
* [Jul 31. Dallas, US - Last Tuesday Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxkbpc/).
* [Aug  1. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxlbcb/).
* [Aug  1. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxlbcb/).
* [Aug  5. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbhb/).
* [Aug  8. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxlblb/).
* [Aug  9. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/252742624).
* [Aug  9. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxlbmb/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).**

### South America

* [Jul 28. Florianópolis, BR - 2º Encontro Rust Floripa](https://www.meetup.com/rustfloripa/events/xvglrpyxkbkb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Commure, Inc. (San Francisco, Boston, Montreal)](https://news.ycombinator.com/item?id=17442861).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I’ve just realized that “guarantees memory safety in the presence of bugs” is a nice way to describe Rust to C/C++ folks

– [matklad](https://internals.rust-lang.org/t/size-hint-correctness-reproducibility-and-documentation/8058/4).

Thanks to [TomP](https://users.rust-lang.org/t/twir-quote-of-the-week/328/545) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
