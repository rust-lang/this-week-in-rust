Title: This Week in Rust 234
Number: 234
Date: 2018-05-15
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

This week's crate is [semverver](https://crates.io/crates/askama), a Jinja-like type-safe compiled templating engine. Thanks to [Icefoxen](https://users.rust-lang.org/u/Icefoxen) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Sodium Oxide: Call for maintainers](https://github.com/sodiumoxide/sodiumoxide/issues/203). Sodium Oxide is a fast cryptographic library for Rust (bindings to libsodium).
* [easy] [gfx-rs: Set buffers mutability qualifiers on Metal compute pipelines](https://github.com/gfx-rs/gfx/issues/1999). gfx-rs is a high-performance, bindless graphics API for Rust.
* [easy] [gfx-rs: Set threadGroupSizeIsMultipleOfThreadExecutionWidth on Metal compute pipelines](https://github.com/gfx-rs/gfx/issues/1998).
* [easy] [gfx-rs: Use set_bytes for short temporary data in Metal internal shaders](https://github.com/gfx-rs/gfx/issues/1997).
* [mutagen has some 'good first issues'](https://github.com/llogiq/mutagen/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

153 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-05-07..2018-05-14

* [set PrepareForThinLTO flag when using ThinLTO](https://github.com/rust-lang/rust/pull/50684)
* [typeck: fix ICE with struct update syntax](https://github.com/rust-lang/rust/pull/50643)
* [typeck: save the index of private fields](https://github.com/rust-lang/rust/pull/50693)
* [use `SmallVec` for `DepNodeIndex` within `dep_graph`](https://github.com/rust-lang/rust/pull/50565)
* [inline `Span` methods](https://github.com/rust-lang/rust/pull/50564)
* [don't use Lock for heavily accessed `CrateMetadata::cnum_map`](https://github.com/rust-lang/rust/pull/50532)
* [do not silently truncate offsets for `read_at`/`write_at` on emscripten](https://github.com/rust-lang/rust/pull/50634)
* [fix `panic` for binaries built during tests](https://github.com/rust-lang/cargo/pull/5513)
* [fix volatile_store and nontemporal_store](https://github.com/rust-lang/rust/pull/50648)
* [rustc: leave space for fields of uninhabited types to allow partial initialization](https://github.com/rust-lang/rust/pull/50622)
* [rustc: don't trip an assertion for enums with present but uninhabited variants](https://github.com/rust-lang/rust/pull/50735)
* [rustc: allow an edition's feature on that edition](https://github.com/rust-lang/rust/pull/50663)
* [rustc: include semicolon when removing `extern crate`](https://github.com/rust-lang/rust/pull/50670)
* [improve single-use and zero-use lifetime lints](https://github.com/rust-lang/rust/pull/50440)
* [prevent infinite recursion of modules](https://github.com/rust-lang/rust/pull/50305)
* [fix self referential impl Trait substitutions](https://github.com/rust-lang/rust/pull/50694)
* [macros: Add a 'literal' fragment specifier](https://github.com/rust-lang/rust/pull/49835)
* [rename Pin to PinMut, and some more breaking changes](https://github.com/rust-lang/rust/pull/50497)
* [stabilize macro_lifetime_matcher](https://github.com/rust-lang/rust/pull/50385)
* [don't allocate when creating an empty BTree](https://github.com/rust-lang/rust/pull/50352)
* [only lookup types in one interner](https://github.com/rust-lang/rust/pull/50332)
* [idiom lints for removing `extern crate`](https://github.com/rust-lang/rust/pull/50260)
* [added missing implementation hint](https://github.com/rust-lang/rust/pull/50161)
* [make `String::new()` const](https://github.com/rust-lang/rust/pull/50460)
* [turn `ManuallyDrop::new` into a constant function](https://github.com/rust-lang/rust/pull/50148)
* [std: avoid `ptr::copy` if unnecessary in `vec::Drain`](https://github.com/rust-lang/rust/pull/50575)
* [add fn `into_inner(self) -> (Idx, Idx)` to RangeInclusive](https://github.com/rust-lang/rust/pull/50574)
* [./x.py test should be able to run individual tests](https://github.com/rust-lang/rust/pull/49729)

## New Contributors

* Harm Berntsen
* rleungx
* Samuel Wilson

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2388: Reserve `try` for `try { .. }` block expressions](https://github.com/rust-lang/rfcs/pull/2388).
* [RFC 2230: Bury `Error::description()`](https://github.com/rust-lang/rfcs/pull/2230).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Unreserve `proc`](https://github.com/rust-lang/rfcs/pull/2420).
* [disposition: close] [`Result::pass()`, turning `Result<T,E>` into `Result<U,F>`, if `From` is set up](https://github.com/rust-lang/rfcs/pull/1996).
* [disposition: close] [The ConstDefault trait](https://github.com/rust-lang/rfcs/pull/2204).

## New RFCs

* [Reserve `delegate` as a keyword in edition 2018](https://github.com/rust-lang/rfcs/pull/2429).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [May 10. Redwood City, US - Bay Area - Where "Self-Driving" Database Meets a "Rusty" Distributed Key-Value Store](https://www.meetup.com/Bay-Area-NewSQL-Database-Meetup/events/249676562/).
* [May 10. Arlington, US - Rust DC - Learn+Try: parsing with nom](https://www.meetup.com/RustDC/events/249883820).
* [May 10. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/lcsdqpyxhbnb/).
* [May 10. San Diego, US - San Diego Rust May Meetup](https://www.meetup.com/San-Diego-Rust/events/249783590/).
* [May 13. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbrb/).
* [May 14. Seattle, US - Seattle Rust Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxhbsb/).
* [May 15. Rome, IT - Rust learning and hacking evening #8](https://www.meetup.com/Rust-Roma/events/250581929/).
* [May 16. Orange County, US - Coding Session and Discussion](https://www.meetup.com/oc-rust/events/250342850/).
* [May 16. Denver, US - Rust Boulder/Denver - Rust Denver May Meetup](https://www.meetup.com/Rust-Boulder-Denver/events/249098925/).
* [May 16. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May 16. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/249497881/).
* [May 16. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxhbvb/).
* [May 17. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxhbwb/).
* [May 20. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbbc/).
* [May 22. Paris, FR - Rust Paris meetup #42](https://www.meetup.com/Rust-Paris/events/250587163/).
* [May 22. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [May 23. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May 23. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* **[May 27. Paris, FR - RustFest Paris 2018](https://paris.rustfest.eu/)**.
* [May 30/31. Rust/WASM course around JSConf.EU](https://ti.to/asquera-event-ug/rust-wasm-wwwtf-2018/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Commure, Inc. San Francisco, US](https://news.ycombinator.com/item?id=16968087).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
