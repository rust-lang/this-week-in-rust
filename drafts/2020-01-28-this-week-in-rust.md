Title: This Week in Rust 323
Number: 323
Date: 2020-01-28
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

This week's crate is [test-case](https://crates.io/crates/test-case), a framework for parameterized testing.

Thanks to [Synek317](https://users.rust-lang.org/t/crate-of-the-week/2704/712) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for Papers: Rust LATAM Mexico 2020](https://www.reddit.com/r/rust/comments/em0ru8/rust_2020_a_conference_in_latin_america).
* [time: Implement function returning the local UTC offset](https://github.com/time-rs/time/issues/203).
* [rusty-celery: Multiple issues marked "Status: Help Wanted", good place to start for newcomers](https://github.com/rusty-celery/rusty-celery/issues?q=is%3Aissue+is%3Aopen+label%3A%22Status%3A+Help+Wanted%22).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

261 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-01-20..2020-01-27

* [implement `?const` opt-out for trait bounds](https://github.com/rust-lang/rust/pull/68140)
* [unbreak linking with lld 9 on FreeBSD 13.0-CURRENT i386](https://github.com/rust-lang/rust/pull/68361)
* [export weak symbols used by MemorySanitizer](https://github.com/rust-lang/rust/pull/68410)
* [stabilize `#[repr(transparent)]` on `enum`s](https://github.com/rust-lang/rust/pull/68122)
* [fix `#[track_caller]` and function pointers](https://github.com/rust-lang/rust/pull/68302)
* [micro-optimize OutputFilenames](https://github.com/rust-lang/rust/pull/68409)
* [perf: avoid creating a SmallVec if nothing changes during a fold](https://github.com/rust-lang/rust/pull/68031)
* [suggest borrowing `Vec<NonCopy>` in for loop](https://github.com/rust-lang/rust/pull/68424)
* [further improve `impl Trait`/`dyn Trait` suggestions](https://github.com/rust-lang/rust/pull/68522)
* [typeck: simplify the handling of `diverges`](https://github.com/rust-lang/rust/pull/68422)
* [don't discard marker trait impls when inference variables are present](https://github.com/rust-lang/rust/pull/68057)
* [account for non-types in substs for opaque type error messages](https://github.com/rust-lang/rust/pull/68438)
* [avoid declaring a fake dependency edge](https://github.com/rust-lang/rust/pull/68298)
* [render const pointers in MIR more compactly](https://github.com/rust-lang/rust/pull/68516)
* [filter and test predicates using `normalize_and_test_predicates` for const-prop](https://github.com/rust-lang/rust/pull/68297)
* [make pointers to statics internal](https://github.com/rust-lang/rust/pull/68494)
* [avoid overflow in `std::iter::Skip::count`](https://github.com/rust-lang/rust/pull/68469)
* [simplify NodeHeader by avoiding slices in BTreeMaps with shared roots](https://github.com/rust-lang/rust/pull/67686)
* [add leading_ones and trailing_ones methods to the primitive integer types](https://github.com/rust-lang/rust/pull/68165)
* [futures: avoid starvation from FuturesUnordered::poll_next](https://github.com/rust-lang/futures-rs/pull/2049)
* [futures: add StreamExt::scan](https://github.com/rust-lang/futures-rs/pull/2044)
* [stdarch: add Icelake avx512 features](https://github.com/rust-lang/stdarch/pull/838)
* [cargo: store maximum queue length](https://github.com/rust-lang/cargo/pull/7829)
* [cargo: search for root manifest with ephemeral workspaces](https://github.com/rust-lang/cargo/pull/7768)
* [rustdoc: fix handling of compile errors when running `rustdoc --test`](https://github.com/rust-lang/rust/pull/68357)
* [docs.rs: fix various bugs in match_version](https://github.com/rust-lang/docs.rs/pull/565)
* [compiletest: simplify multi-debugger support](https://github.com/rust-lang/rust/pull/68391)

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

* [disposition: merge] [Introduce the ASM project group](https://github.com/rust-lang/rfcs/pull/2836).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize `ptr::slice_from_raw_parts[_mut]`](https://github.com/rust-lang/rust/pull/68234).
* [disposition: merge] [Stabilize `#[repr(transparent)]` on `enum`s in Rust 1.42.0](https://github.com/rust-lang/rust/pull/68122).
* [disposition: merge] [Stabilize the debug_map_key_value feature](https://github.com/rust-lang/rust/pull/68200).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Europe

* [Jan 23. Paris, FR - Rust Paris meetup #49](https://www.meetup.com/Rust-Paris/events/267250053/).
* [Jan 23. Warsaw, PL - Rust Warsaw 3](https://www.meetup.com/Rust-Warsaw/events/267525144/).
* [Feb  2. Brussels, BE - Rust devroom @ FOSDEM](https://fosdem.org/2020/schedule/track/rust/).
* [Jan 22. Hamburg, DE - Rust Hack & Learn January 2020](https://www.meetup.com/Rust-Meetup-Hamburg/events/267692684/).
* [Jan 23. Zagreb, HR - impl Zagreb for Rust - Rust Meetup 2020/01: Uvod u lock-free Rust](https://www.meetup.com/Zagreb-Rust-Meetup/events/267742601).
* [Jan 29. Copenhagen, DK - Copenhagen Rust Hack Night #21](https://cph.rs/).
* [Feb  5. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgrybcdbhb/).

### North America

* [Jan 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybccbkc/).
* [Jan 28. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybccblc/).
* [Jan 29. Chicago, IL, US - Chicago Rust Meetup - Byte Sized Rust - AWS Lambdas & Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/267616019/).
* [Jan 29. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Feb  5. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/qgvxlrybcdbhb/).
* [Feb  5. Atlanta, GA, US - Rust Atlanta - Static Assertions Internals](https://www.meetup.com/Rust-ATL/events/qxqdgrybcdbqb/).
* [Feb  5. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpybcdbhb/).

### South America

* [Jan 23. Buenos Aires, AR - Rust Argentina - Encuentro de Enero](https://www.meetup.com/Rust-Argentina/events/267904544/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

* [Rust Developer for privacy software at Cosmian, Paris (Remote available)](https://cosmian.com/were-hiring-developer-rust-cryptography-m-w-x/)

# Quote of the Week

> Rust is basically Haskell's athletic younger brother. Not as intellectual, but still smart and lifts weights.

– [icefox, Jan 22 in community-Discord #games-and-graphics](https://discordapp.com/channels/273534239310479360/335502453371961344/669636317277192222)

Thanks to [Duane](https://users.rust-lang.org/t/twir-quote-of-the-week/328/801) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
