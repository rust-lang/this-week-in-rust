Title: This Week in Rust 326
Number: 326
Date: 2020-02-18
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

This week's crates are [pointer-utils](https://github.com/CAD97/pointer-utils), a small library for working with pointers, and [jlrs](https://github.com/Taaitaaiger/jlrs), a crate to call [Julia](https://julialang.org) from Rust.

Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/729) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [image: Tracking issue: Converting error representations](https://github.com/image-rs/image/issues/1134).
* [Ferrous Systems and TrueLayer: Rust Training in London, March 2020](https://ferrous-systems.com/blog/training-in-london/).
* [libc crate is looking for maintainers](https://github.com/rust-lang/libc/issues/1657)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

276 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-02-10..2020-02-17

* [enable Control Flow Guard in rustbuild](https://github.com/rust-lang/rust/pull/68824)
* [transition macro_legacy_warnings into a hard error](https://github.com/rust-lang/rust/pull/69129)
* [parse: unify function front matter parsing](https://github.com/rust-lang/rust/pull/69023)
* [fix extra subslice lowering](https://github.com/rust-lang/rust/pull/69128)
* [fix lifetime shadowing check in GATs](https://github.com/rust-lang/rust/pull/68938)
* [record proc macro harness order for use during metadata deserialization](https://github.com/rust-lang/rust/pull/68814)
* [tweak borrow error on `FnMut` when `Fn` is expected](https://github.com/rust-lang/rust/pull/68816)
* [when expecting `BoxFuture` and using `async {}`, suggest `Box::pin`](https://github.com/rust-lang/rust/pull/69082)
* [micro-optimize the heck out of LEB128 reading and writing](https://github.com/rust-lang/rust/pull/69050)
* [traits: preallocate 2 Vecs of known initial size](https://github.com/rust-lang/rust/pull/69022)
* [don't run coherence twice for future-compat lints](https://github.com/rust-lang/rust/pull/69044)
* [correct inference of primitive operand type behind binary operation](https://github.com/rust-lang/rust/pull/68129)
* [support new LLVM pass manager](https://github.com/rust-lang/rust/pull/67954)
* [rustc_session: allow overriding lint level of individual lints from a group](https://github.com/rust-lang/rust/pull/67885)
* [migrate borrowck dataflow impls to new framework](https://github.com/rust-lang/rust/pull/68241)
* [infer regions for opaque types in borrowck](https://github.com/rust-lang/rust/pull/67681)
* [use a `ParamEnvAnd<Predicate>` for caching in `ObligationForest`](https://github.com/rust-lang/rust/pull/68475)
* [add missing `_zeroed` varants to `AllocRef`](https://github.com/rust-lang/rust/pull/69027)
* [make ASCII ctype functions unstably const](https://github.com/rust-lang/rust/pull/68986)
* [speed up `SipHasher128`](https://github.com/rust-lang/rust/pull/68914)
* [miri: fix exact_div](https://github.com/rust-lang/rust/pull/69126)
* [miri: add shim for rename](https://github.com/rust-lang/miri/pull/1158)
* [BTree: lighten the load on Miri](https://github.com/rust-lang/rust/pull/68781)
* [improve `ty.needs_drop`](https://github.com/rust-lang/rust/pull/68679)
* [preparation for allocator aware `Box`](https://github.com/rust-lang/rust/pull/69058)
* [hide niches under `UnsafeCell`](https://github.com/rust-lang/rust/pull/68491)
* [relax bounds on `HashMap`/`HashSet`](https://github.com/rust-lang/rust/pull/67642)
* [improve `char::is_ascii_*` codegen](https://github.com/rust-lang/rust/pull/67585)
* [implement `LowerExp` and `UpperExp` for integers](https://github.com/rust-lang/rust/pull/66721)
* [add `From<Vec<NonZeroU8>>` for `CString`](https://github.com/rust-lang/rust/pull/64069)
* [fix `std::fs::copy` on WASI target](https://github.com/rust-lang/rust/pull/69106)
* [futures: implement fast-path for already-completed shared futures](https://github.com/rust-lang/futures-rs/pull/2074)
* [rustdoc: struct variant field search](https://github.com/rust-lang/rust/pull/68668)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2396: target_feature 1.1](https://github.com/rust-lang/rfcs/pull/2396).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Rust 2020 roadmap](https://github.com/rust-lang/rfcs/pull/2857).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add primitive module to libcore](https://github.com/rust-lang/rust/pull/67637).
* [disposition: merge] [rustc_session: allow overriding lint level of individual lints from a group](https://github.com/rust-lang/rust/pull/67885).
* [disposition: merge] [Add Wake trait for safe construction of Wakers](https://github.com/rust-lang/rust/pull/68700).
* [disposition: merge] [Add Display and Error impls for proc_macro::LexError](https://github.com/rust-lang/rust/pull/68899).
* [disposition: merge] [Stabilize Once::is_completed](https://github.com/rust-lang/rust/pull/68945).
* [disposition: close] [Fix an inconsistency in Linux version of TcpListener::accept](https://github.com/rust-lang/rust/pull/67028).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Asia Pacific

* [Feb 15. Chennai, IN - Rust Chennai - Monthly meetup](https://www.meetup.com/mad-rs/events/268597652).
* [Feb 18. Seoul, KR - Seoul Rust Meetup - Learning Rust #2 - Control flow and pattern matching](https://www.meetup.com/Rust-Seoul-Meetup/events/djkzlrybcdbxb/).
* [Feb 24. Sydney, AU - Rust Sydney - Meetup 19](https://www.meetup.com/Rust-Sydney/events/268525192/).

### Europe

* [Feb 19. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgrybcdbzb/).
* [Feb 20. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/268060855).
* [Feb 21. Stuttgart, DE - Rust Community Stuttgart - Rust Hack and Learn](https://www.meetup.com/Rust-Community-Stuttgart/events/268416708/).

### North America

* [Feb 18. Redmond, WA, US - Seattle Rust Meetup - Monthly meetup in Redmond](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdrybcdbpb/).
* [Feb 19. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcdbzb/).
* [Feb 24. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcdbgc/).
* [Feb 25. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybcdbhc/).
* [Feb 26. Portland, OR, US - PDXRust - Hack Night](https://www.meetup.com/PDXRust/events/268266020/).
* [Feb 26. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Feb 26. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscrybcdbjc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

This week we have two (related) quotes:

> `Option` is null in different clothes, but the clothes that nulls wear are important.

– [skysch on rust-users](https://users.rust-lang.org/t/how-would-you-do-that-in-rust-versus-java/38187/6)

Thanks to [Cerberuser](https://users.rust-lang.org/t/twir-quote-of-the-week/328/815) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
