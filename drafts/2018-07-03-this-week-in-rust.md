Title: This Week in Rust 241
Number: 241
Date: 2018-07-03
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

This week's crate is [mutagen](https://github.com/llogiq/mutagen), a mutation testing framework for Rust. Thanks to llogiq for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

95 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-06-18..2018-06-25

* [async/await](https://github.com/rust-lang/rust/pull/51580)
* [rename OOM to allocation error](https://github.com/rust-lang/rust/pull/51543)
* [prohibit `global_allocator` in submodules](https://github.com/rust-lang/rust/pull/51335)
* [run some stuff in parallel](https://github.com/rust-lang/rust/pull/51383)
* [reduce number of allocations done by NLL](https://github.com/rust-lang/rust/pull/51617)
* [NLL: walk the MIR only once for the "unused mut" lint](https://github.com/rust-lang/rust/pull/51660)
* [NLL diagnostics: revise `fn check_access_permissions`](https://github.com/rust-lang/rust/pull/51275)
* [improve memoization and refactor NLL type check](https://github.com/rust-lang/rust/pull/51460)
* [add existential type definitions](https://github.com/rust-lang/rust/pull/51414)
* [make more `libsyntax` methods public](https://github.com/rust-lang/rust/pull/51664)
* [add ability to apply custom derive to union types](https://github.com/rust-lang/rust/pull/50383)
* [The Great Generics Generalisation: HIR Edition](https://github.com/rust-lang/rust/pull/48149)
* [fix processing mod with multi-level path on Windows](https://github.com/rust-lang/rust/pull/51278)
* [disable probestack when GCOV profiling is being used](https://github.com/rust-lang/rust/pull/51666)
* [support future deprecation for rustc_deprecated](https://github.com/rust-lang/rust/pull/51681)
* [fix an ICE when matching over const slices](https://github.com/rust-lang/rust/pull/51733)
* [yet another "old borrowck" bug around match default bindings](https://github.com/rust-lang/rust/pull/51686)
* [fix erroneous error note when using field after move](https://github.com/rust-lang/rust/pull/51688)
* [three diagnostics upgrades](https://github.com/rust-lang/rust/pull/51750)
* [various changes to existing diagnostics](https://github.com/rust-lang/rust/pull/51463)
* [don't suggest incorrect syntax](https://github.com/rust-lang/rust/pull/51670)
* [use fstatat64 where available](https://github.com/rust-lang/rust/pull/51785)
* [specialize `StepBy<Range(Inclusive)>`](https://github.com/rust-lang/rust/pull/51601)
* [`impl Hash for !`](https://github.com/rust-lang/rust/pull/51404)
* [stabilize std::path::Path::ancestors](https://github.com/rust-lang/rust/pull/50894)
* [replace tempdir by tempfile](https://github.com/rust-lang/rust/pull/50698)
* [rustdoc: greatly improve tables display](https://github.com/rust-lang/rust/pull/51482)
* [rustbuild: use quiet tests by default](https://github.com/rust-lang/rust/pull/51367)
* [ship LLVM tools with the toolchain](https://github.com/rust-lang/rust/pull/50336)

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

* [disposition: merge] [Allow panicking in constants](https://github.com/rust-lang/rfcs/pull/2345).
* [disposition: merge] [Allow `loop` in constant evaluation](https://github.com/rust-lang/rfcs/pull/2344).
* [disposition: merge] [Introduce `#[do_not_recommend]` to control errors for trait impls](https://github.com/rust-lang/rfcs/pull/2397).
* [disposition: merge] [Tuple struct construction with `Self(v1, v2, ..)`](https://github.com/rust-lang/rfcs/pull/2302).
* [disposition: merge] [Update RFC 0430 to allow underscores between numbers in CamelCase names](https://github.com/rust-lang/rfcs/pull/2478).
* [disposition: merge] [Add `Option::replace` to the core library](https://github.com/rust-lang/rfcs/pull/2296).
* [disposition: close] [Reserve `delegate` as a keyword in edition 2018](https://github.com/rust-lang/rfcs/pull/2429).
* [disposition: close] [Reserve `f(a = b)` in Rust 2018](https://github.com/rust-lang/rfcs/pull/2443).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for "macro naming and modularisation" (RFC #1561)](https://github.com/rust-lang/rust/issues/35896).
* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: merge] [`extern type` cannot support `size_of_val` and `align_of_val`](https://github.com/rust-lang/rust/issues/49708).
* [disposition: merge] [Tracking issue for the to_bytes and from_bytes methods of integers](https://github.com/rust-lang/rust/issues/49792).
* [disposition: merge] [Implement `iter::{Sum<U>, Product<U>}` for `Option<T>`](https://github.com/rust-lang/rust/pull/50884).
* [disposition: merge] [Implement `PartialEq` between `&str` and `OsString`](https://github.com/rust-lang/rust/pull/51178).
* [disposition: merge] [Remove quote_*! macros](https://github.com/rust-lang/rust/pull/51285).
* [disposition: close] [Can't overload indexing for arrays](https://github.com/rust-lang/rust/issues/49786).

## New RFCs

* [Add new channels for long term support (LTS) releases](https://github.com/rust-lang/rfcs/pull/2483).
* [Stabilize the alloc crate](https://github.com/rust-lang/rfcs/pull/2480).
* [Conversions: `FromLossy` and `TryFromLossy` traits](https://github.com/rust-lang/rfcs/pull/2484).

# Upcoming Events

### Online

* [Jul  3. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Jul  4. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jul  4. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jul 11. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).

### Africa

* [Jul  3. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxkbfb/).

### Europe

* [Jul  4. Dresden, DE - Mozilla Community Dresden - Rust Meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/252020329/).
* [Jul  6. Darmstadt, DE - Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/251928672).
* [Jul 11. Zurich, CH - Actix.rs - July Meetup](https://www.meetup.com/Rust-Zurich/events/250386292/).
* [Jul 11. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/xkdlvpyxkbpb/).

### North America

* [Jul  1. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxkbcb/).
* [Jul  4. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxkbgb/).
* [Jul  4. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxkbgb/).
* [Jul  8. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxkblb/).
* [Jul  9. Seattle, US - Monthly Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxkbmb/).
* [Jul 11. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxjbkc/).
* [Jul 12. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxkbqb/).
* [Jul 12. Utah Valley, Utah, US - Utah Rust - Monthly Meeting](https://www.meetup.com/utahrust/events/251816575/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).** Registration is now open.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Software Developer at Nymi, Toronto, CA](https://nymi.com/careers/sr-software).
* [Senior Rust Engineer at Ticketmaster, Arizona, US](https://www.reddit.com/r/rust/comments/8s0tk1/9095_remote_senior_rust_engineer_ticketmaster/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I’m hesitating in cc’ing [the crate author] because I’d rather this be an educational conversation, and not a unsafety witchhunt.

– vitalyd on [rust-users](https://users.rust-lang.org/t/how-not-to-use-unsafe-code/18170/13)

[Please submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
