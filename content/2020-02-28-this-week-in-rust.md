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

* [Sealed Rust update: The plan for safety critical Rust](https://ferrous-systems.com/blog/sealed-rust-the-plan/).
* [Why is Rust the most loved programming language](https://matklad.github.io/2020/02/14/why-rust-is-loved.html)?
* [Rust ghost, signing off](https://quietmisdreavus.net/self/2020/02/17/rust-ghost-signing-off/).
* [I audited 3 different implementation of async RwLock](https://www.reddit.com/r/rust/comments/f4zldz/i_audited_3_different_implementation_of_async/).
* [`deny(warnings)` is actively harmful](https://www.reddit.com/r/rust/comments/f5xpib/psa_denywarnings_is_actively_harmful/).
* [Rust for Java devs](https://leshow.github.io/post/rust_for_java_devs/).
* [Some nuances of undefined behavior in Rust](https://typr124.github.io/UB1).
* [A pragmatic approach to global state](http://adventures.michaelfbryan.com/posts/pragmatic-global-state/)
* [Graphs in Rust: Introducing GraphCore](https://depth-first.com/articles/2020/02/17/graphs-in-rust-introducting-graphcore/).
* [faux - an inside look](https://nrxus.github.io/faux/blog/how-it-works.html)
* [rust-analyzer changelog 12](https://rust-analyzer.github.io/thisweek/2020/02/17/changelog-12.html).
* [IntelliJ Rust changelog 116](https://intellij-rust.github.io/2020/02/18/changelog-116.html).
* [This month in Rust GameDev 6 - January 2020](https://rust-gamedev.github.io/posts/newsletter-006/).

# Crate of the Week

This week's crates are [pointer-utils](https://github.com/CAD97/pointer-utils), a small library for working with pointers, and [jlrs](https://github.com/Taaitaaiger/jlrs), a crate to call [Julia](https://julialang.org) from Rust.

Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/729) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [puzzle] [The battle for Rust knowledge supremacy](https://rustbattle.net/battle/straight-finch-8-e4f4).

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

* [RFC 2857: Rust 2020 roadmap](https://github.com/rust-lang/rfcs/pull/2857).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Make `u8::is_ascii` a stable `const fn`](https://github.com/rust-lang/rust/pull/68984).
* [disposition: merge] [Stabilize assoc_int_consts associated int/float constants](https://github.com/rust-lang/rust/pull/68952).
* [disposition: merge] [Add primitive module to libcore](https://github.com/rust-lang/rust/pull/67637).
* [disposition: merge] [Tracking issue for `#![feature(maybe_uninit_ref)]`](https://github.com/rust-lang/rust/issues/63568).

## New RFCs

* [Add a new attribute, `#[isa]`](https://github.com/rust-lang/rfcs/pull/2867).

# Upcoming Events

### Asia Pacific

* [Feb 24. Sydney, AU - Rust Sydney - Meetup 19](https://www.meetup.com/Rust-Sydney/events/268525192/).
* [Mar  5. Melbourne, AU - Rust Melbourne - Hack Night, Talks, and Networking](https://www.meetup.com/Rust-Melbourne/events/268002615/).

### Europe

* [Feb 21. Stuttgart, DE - Rust Community Stuttgart - Rust Hack and Learn](https://www.meetup.com/Rust-Community-Stuttgart/events/268416708/).
* [Feb 25. London, GB - Rust LDN Talks @ TrueLayer](https://www.meetup.com/Rust-London-User-Group/events/268354799).
* [Feb 25. Göteborg, SE - Rust Gbg — "Beginner Friendly" February 2020](https://www.meetup.com/rustgbg/events/268653522/).
* [Feb 27. Wroclaw, PL - Rust Wrocław Meetup #17](https://www.meetup.com/Rust-Wroclaw/events/268683403).
* [Mar 4. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gztznrybcfbgb/).

### North America

* [Feb 24. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcdbgc/).
* [Feb 25. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybcdbhc/).
* [Feb 26. Portland, OR, US - PDXRust - Hack Night](https://www.meetup.com/PDXRust/events/268266020/).
* [Feb 26. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Feb 26. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscrybcdbjc/).
* [Feb 26. Mesa, AZ, US - Desert Rust - Rust: lightning talks](https://www.meetup.com/Desert-Rustaceans/events/268793593/).
* [Mar 11. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybcfbpb/).
* [Mar  4. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpybcfbgb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Software Engineer, Rust & Linux (Remote) at Red Canary, Denver, CO, US](https://jobs.lever.co/redcanary/d7729b7d-e18d-4fe9-b3f5-fd5f8b947f22).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

This week we have two (related) quotes:

> `Option` is null in different clothes, but the clothes that nulls wear are important.

– [skysch on rust-users](https://users.rust-lang.org/t/how-would-you-do-that-in-rust-versus-java/38187/6)

Thanks to [Cerberuser](https://users.rust-lang.org/t/twir-quote-of-the-week/328/815) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
