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

[Optimising PineTime’s Display Driver with Rust and Mynewt](https://medium.com/@ly.lee/optimising-pinetimes-display-driver-with-rust-and-mynewt-3ba269ea2f5c?source=friends_link&sk=4d2cbd2e6cd2343eed62d214814f7b81).
[Rust in Blockchain #7 – December 2019](https://rustinblockchain.org/2020/01/02/rust-in-blockchain-7-december-2019/). In blockchain and across the Rust ecosystem, of projects quickly picking up stable async/await support and migrating to tokio 0.2.
- [The embedded WG's Raspberry Pi OS dev tutorials: Tutorial 13 - Kernel Unit, Integration and Console tests using QEMU](https://github.com/rust-embedded/rust-raspi3-OS-tutorials/tree/master/13_integrated_testing)
- [A Thought Experiment: Using the ECS Pattern Outside of Game Engines](http://adventures.michaelfbryan.com/posts/ecs-outside-of-games/?utm_source=this-week-in-rust&utm_medium=social&utm_campaign=ecs-architecture)

# Crate of the Week

This week's crate is [attohttpc](https://crates.io/crates/attohttpc), a tiny synchronous HTTP client library.

Thanks to [Matěj Laitl](https://users.rust-lang.org/t/crate-of-the-week/2704/696) for the suggestions!

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

184 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-12-23..2019-12-30

* [doc comments: less attribute mimicking](https://github.com/rust-lang/rust/pull/67151)
* [require const stability attributes on intrinsics to be able to use them in constant contexts](https://github.com/rust-lang/rust/pull/67466)
* [stabilize attribute macros on inline modules](https://github.com/rust-lang/rust/pull/64273)
* [normalize `ident`](https://github.com/rust-lang/rust/pull/66670)
* [resolve long compile times when evaluating always valid constants](https://github.com/rust-lang/rust/pull/67667)
* [avoid memory copy logic for zero-size types](https://github.com/rust-lang/rust/pull/67658)
* [ensure that evaluating or validating a constant never reads from a static](https://github.com/rust-lang/rust/pull/67337)
* [tweak errors for missing associated types and type parameters](https://github.com/rust-lang/rust/pull/67268)
* [typeck: note other end-point when checking range pats](https://github.com/rust-lang/rust/pull/67287)
* [refactorings to borrowck region diagnostic reporting](https://github.com/rust-lang/rust/pull/67241)
* [various const eval and pattern matching ICE fixes](https://github.com/rust-lang/rust/pull/67192)
* [fix ICE in mir interpretation](https://github.com/rust-lang/rust/pull/67546)
* [allocate HIR on an arena 2/4 -- Expr & Pat](https://github.com/rust-lang/rust/pull/66936)
* [allocate HIR on an arena 3/4 -- Ty](https://github.com/rust-lang/rust/pull/66942)
* [initial implementation of `#![feature(bindings_after_at)]`](https://github.com/rust-lang/rust/pull/66296)
* [deprecate `Error::description` for real](https://github.com/rust-lang/rust/pull/66919)
* [add `IntoFuture` trait and support for await](https://github.com/rust-lang/rust/pull/65244)
* [do not ICE on lifetime error involving closures](https://github.com/rust-lang/rust/pull/67687)
* [use `NonNull` in `slice::`{`Iter`, `IterMut`}](https://github.com/rust-lang/rust/pull/67588)
* [implement padding for `IpAddr` without heap alloc](https://github.com/rust-lang/rust/pull/67035)
* [stabilize the `matches!` macro](https://github.com/rust-lang/rust/pull/67659)
* [differentiate `todo!` and `unimplemented!`](https://github.com/rust-lang/rust/pull/67445)
* [fix `Instance::resolve()` incorrectly returning specialized instances](https://github.com/rust-lang/rust/pull/67662)
* [prune ill-conceived `BTreeMap::iter_mut` assertion and test its mutability](https://github.com/rust-lang/rust/pull/67459)
* [clean up const-hack PRs now that const if / match exist](https://github.com/rust-lang/rust/pull/67657)
* [hashbrown: implement `drain_filter` for `HashMap`](https://github.com/rust-lang/hashbrown/pull/135)
* [rustdoc: show the actual value of constant values in the documentation](https://github.com/rust-lang/rust/pull/66221)

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
* [Jan 22. Wrocław, PL - Rust Wrocław Meetup #16](https://www.meetup.com/Rust-Wroclaw/events/267514337/).

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

> Rust has multiple *unique* paradigms that don't even exist in other languages, such as lifetimes and compile-time-tracked "exclusive access". But instead of endorsing them from the beginning, as @mbrubeck's [ *Rust: a unique perspective* ](https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html) does, the Rust book tries to show a language that is "like other languages, but with (magical) compile-time checks". When the truth is that Rust's strength lies in non-`unsafe` Rust being **less expressive** than languages like C or C++.
>
> I think that Rust should start with the statement: "Welcome to a language that by being less expressive forces you to use constructs that are **guaranteed at compile-time to be sound**. But don't worry; after some time you will get used to the coding patterns that are allowed, and will then almost not notice the hindered expressiveness, only the enhanced zero-cost safety that will let you **hack without fear**."
>
> * It doesn't sound bad imho, and is at least honest *w.r.t.* the struggles that someone refusing to shift their way of coding / mental coding patterns may encounter.

– [Daniel H-M on rust-users](https://users.rust-lang.org/t/after-a-week-with-rust/35829/27)

Thanks to [Tom Phinney](https://users.rust-lang.org/t/twir-quote-of-the-week/328/768) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
