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
* [Rust concurrency patterns: Natural Born Pipelines](https://medium.com/@polyglot_factotum/rust-concurrency-patterns-natural-born-pipelines-4d599e7612fc)

# Crate of the Week

This week's crate is [Taizen](https://github.com/NerdyPepper/taizen), a wikipedia browser for your terminal. Thanks to [nasa42](https://users.rust-lang.org/t/crate-of-the-week/2704/419) for suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [How to help test the 2018 edition](https://www.ncameron.org/blog/how-to-help-test-the-2018-edition/).
* [bitwarden_rs: Looking for wannabe rustaceans, that'd like to get their feet wet](https://www.reddit.com/r/rust/comments/90xh79/looking_for_wannabe_rustaceans_thatd_like_to_get/).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [exercism.io needs Rust mentors](https://users.rust-lang.org/t/exercism-io-needs-mentors/19222)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

158 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-07-23..2018-07-31

* [try to fix an ICE](https://github.com/rust-lang/rust/pull/52673)
* [abort if a promoted fails to be const evaluable and its runtime checks didn't trigger](https://github.com/rust-lang/rust/pull/52571)
* [allow declaring existential types inside blocks](https://github.com/rust-lang/rust/pull/52645)
* [do not overwrite child def-id in place but rather remove/insert](https://github.com/rust-lang/rust/pull/52546)
* [format linker args in a way that works for gcc and ld](https://github.com/rust-lang/rust/pull/52654)
* [rustc: implement tokenization of nested items](https://github.com/rust-lang/rust/pull/52618)
* [buffer NLL errors](https://github.com/rust-lang/rust/pull/52566)
* [don't match on region kinds when reporting NLL errors](https://github.com/rust-lang/rust/pull/52617)
* [NLL: improve the "fully elaborated type" case in region errors](https://github.com/rust-lang/rust/pull/52648)
* [NLL: use better spans in some errors](https://github.com/rust-lang/rust/pull/52678)
* [NLL: make temp for each candidate in `match` arm](https://github.com/rust-lang/rust/pull/52733)
* [NLL: fix some things for bootstrap](https://github.com/rust-lang/rust/pull/52830)
* [suggest underscore when using dashes in crate name](https://github.com/rust-lang/rust/pull/52740)
* [suggest fix when encountering different mutability from impl to trait](https://github.com/rust-lang/rust/pull/52702)
* [do a basic sanity check for all constant values](https://github.com/rust-lang/rust/pull/51361)
* [tweak the raw_identifiers lints in 2018](https://github.com/rust-lang/rust/pull/52722)
* [change ManuallyDrop<T> to a lang item](https://github.com/rust-lang/rust/pull/52711)
* [don't use NonNull::dangling as sentinel value in Rc, Arc](https://github.com/rust-lang/rust/pull/52637)
* [add unaligned volatile intrinsics](https://github.com/rust-lang/rust/pull/52391)
* [`impl PartialEq+Eq for BuildHasherDefault`](https://github.com/rust-lang/rust/pull/52402)
* [`impl Executor for Box<E: Executor>`](https://github.com/rust-lang/rust/pull/52674)
* [`impl std::ops::Try for std::task::Poll`](https://github.com/rust-lang/rust/pull/52721)
* [`impl Send & Sync for JoinHandle`](https://github.com/rust-lang/rust/pull/52759)
* [make `memrchr` use `align_offset`](https://github.com/rust-lang/rust/pull/52744)
* [stablize Redox Unix Sockets](https://github.com/rust-lang/rust/pull/52656)
* [don't `format!()` string literals](https://github.com/rust-lang/rust/pull/52805)
* [`cargo -Zcompile-progress`: use the target name in the progress bar when building a test/binary](https://github.com/rust-lang/cargo/pull/5828)
* [rustdoc: rework how default passes are chosen](https://github.com/rust-lang/rust/pull/52751)

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
* [Aug 10. Frankfurt, DE - Rhein-Main Rust Meetup (with Special Guest)](https://www.meetup.com/Rust-Rhein-Main/events/253311151).

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

> Rust is more restrictive, indeed. But only in the sense that a car with seatbelts is more restrictive than one without: both reach the same top speed, but only one of them will save you in a bad day 😊

– [Felix91gr on rust-users](https://users.rust-lang.org/t/which-language-gives-users-more-control-c-or-rust/19034/8).

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/u/juleskers) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
