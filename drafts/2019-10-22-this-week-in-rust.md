Title: This Week in Rust 309
Number: 309
Date: 2019-10-22
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

* [Programming Servo: shipping message-ports(via a detour into Spectre)](https://medium.com/programming-servo/programming-servo-shipping-message-ports-via-a-detour-into-spectre-c96683ac0b8).

# Crate of the Week

This week, we don't have one, nor two, but *three* crates of the week! There's [Watt](https://github.com/dtolnay/watt), a fast WASM-based proc-macro runtime, [Anyhow](https://github.com/dtolnay/anyhow), yet another error handling crate and [spotify-tui](https://github.com/Rigellute/spotify-tui), a console user interface for Spotify.

Thanks to [Aloso](https://users.rust-lang.org/t/crate-of-the-week/2704/649), [zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/645) and [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/644) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [good first issue] [async-std: Add Stream::throttle](https://github.com/async-rs/async-std/issues/342).
* [good first issue] [async-std: Add Stream::timeout](https://github.com/async-rs/async-std/issues/340).
* [good first issue] [async-std: Add Future::delay](https://github.com/async-rs/async-std/issues/341).
* [async-std: [tracking] streams](https://github.com/async-rs/async-std/issues/129).
* [good first issue] [tracing: core: add `dispatcher::set_default`](https://github.com/tokio-rs/tracing/issues/383).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

302 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-10-07..2019-10-14

* [Add support for `const unsafe? extern fn`](https://github.com/rust-lang/rust/pull/64906)
* [Split non-CAS atomic support off into `target_has_atomic_load_store`](https://github.com/rust-lang/rust/pull/65214)
* [deriving: Avoid dummy Span on an artificial `type_ident` path](https://github.com/rust-lang/rust/pull/65310)
* [Print lifetimes with backticks](https://github.com/rust-lang/rust/pull/65292)
* [Fix suggested bound addition diagnostic](https://github.com/rust-lang/rust/pull/65289)
* [Note when a mutable trait object is needed](https://github.com/rust-lang/rust/pull/65077)
* [Use structured suggestion for removal of `as_str()` call](https://github.com/rust-lang/rust/pull/65194)
* [Fix const generic arguments not displaying in types mismatch diagnostic](https://github.com/rust-lang/rust/pull/65154)
* [Improve message when attempting to instantiate tuple structs with private fields](https://github.com/rust-lang/rust/pull/65153)
* [Suggest dereferencing boolean reference when used in `if` or `while`](https://github.com/rust-lang/rust/pull/65150)
* [When suggesting assoc function with type params, include turbofish](https://github.com/rust-lang/rust/pull/65145)
* [self-profiling: Add events for everything except trait selection](https://github.com/rust-lang/rust/pull/65208)
* [Avoid `SmallVec::collect`](https://github.com/rust-lang/rust/pull/64949)
* [Speed up `TokenStream` concatenation](https://github.com/rust-lang/rust/pull/65198)
* [Implement `Clone::clone_from` for `VecDeque`](https://github.com/rust-lang/rust/pull/65069)
* [Stabilize `slice::repeat`](https://github.com/rust-lang/rust/pull/64877)
* [Stabilize `mem::take`](https://github.com/rust-lang/rust/pull/64716)
* [Implement (`HashMap`) `Entry::insert`](https://github.com/rust-lang/rust/pull/64656)
* [improve performance of signed `saturating_mul`](https://github.com/rust-lang/rust/pull/65312)
* [dist: minimize the `rust-std` component](https://github.com/rust-lang/rust/pull/64823)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2495: Minimum Supported Rust Version](https://github.com/rust-lang/rfcs/pull/2495).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: postpone] [Tighter coupling of Cargo workspaces](https://github.com/rust-lang/rfcs/pull/2315).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize RFC 2451, re-rebalance coherence](https://github.com/rust-lang/rust/issues/63599).
* [disposition: merge] [make is_power_of_two a const function](https://github.com/rust-lang/rust/pull/65092).

## New RFCs

* [Unified coroutines a.k.a. Generator resume arguments](https://github.com/rust-lang/rfcs/pull/2781).
* [impl-only glob imports](https://github.com/rust-lang/rfcs/pull/2782).

# Upcoming Events

### Asia Pacific

* [Oct 19. Chennai, IN - Rust Chennai - Monthly meetup](https://www.meetup.com/mad-rs/events/265677784).
* [Oct 26. Tokyo, JP - Rust.Tokyo 2019](https://rust.tokyo/).
* [Oct 26. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/495062051340992/).

### Europe

* [Oct 17. Barcelona, ES - BcnRust Meetup](https://www.meetup.com/es-ES/BcnRust/events/265509739/).
* [Oct 18. Stuttgart, DE - Rust Meetup Hack and Learn](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/265526369/).
* [Oct 19-23. Rome, IT - Rust+GNOME 2019 Hackfest #6](https://wiki.gnome.org/Hackfests/Rust2019-2#preview).
* [Oct 23. Stockholm, SE - Stockholm Rust - Rust Meetup @Embark Studios](https://www.meetup.com/Stockholm-Rust/events/265322700/).
* [Oct 24. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/265207841).
* [Oct 24. Vienna, AT - Rust Vienna - Rust Townsquare Gathering Oktober](https://www.meetup.com/Rust-Vienna/events/265535638/).
* [Oct 28. Zurich, CH - Rust Zurich - October Meetup: Claudia Saxer – 66 hours of Rust](https://www.meetup.com/Rust-Zurich/events/265507413/).
* [Oct 28. Gouda, NL - Rust Nederland - Rust - Talks & Demos](https://www.meetup.com/Rust-Nederland/events/265656966).
* [Oct 28. London, GB - Rust London User Group - LDN Talks October 2019](https://www.meetup.com/Rust-London-User-Group/events/265590044/).
* [Oct 30. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryznbnc/).

### North America

* [Oct 18 & 19. Dayton, OH, US - Rust Belt Rust](https://www.rust-belt-rust.com/).
* [Oct 23. Portland, OR, US - PDXRust - Hack Night](https://www.meetup.com/PDXRust/events/265043014/).
* [Oct 23. Mesa, AZ, US - Desert Rust - Rust: lightning talks](https://www.meetup.com/Desert-Rustaceans/events/wmmphryznbfc/).
* [Oct 28. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyznblc/).
* [Oct 29. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyznbmc/).
* [Oct 29. Chicago, IL, US - Chicago Rust Meetup - Entity Component Systems: An Intro To The Specs Crate Using Roguelikes](https://www.meetup.com/Chicago-Rust-Meetup/events/265283294).
* [Oct 30. San Francisco, CA, US - Rust in Blockchain Workshop Day (SFBW)](https://www.meetup.com/Rust-in-Blockchain-San-Francisco/events/265362152/)
* [Oct 30. Santa Clara, CA, US - Rust Bay Area - [@ Intel Santa Clara] Security with Rust & SGX + Life of an Async fn](https://www.meetup.com/Rust-Bay-Area/events/265478102).
* [Oct 30. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryznbnc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Blockchain Engineer at Nervos, Hangzhou, CN (Remote available)](https://angel.co/company/nervos-1/jobs/589230-senior-blockchain-engineer).
* [Rust/Core Developer at Parity, Berlin, DE (Remote available)](https://www.parity.io/jobs/#berlin-rust-core-developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> If the Rust community has an ethos, it's that software should have strong static typing, but people should have soft dynamic typing.

– [Kyle Strand on Twitter](https://twitter.com/BatmanAoD/status/1174799660134699008)

Thanks to [Kyle Strand](https://users.rust-lang.org/t/twir-quote-of-the-week/328/710) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
