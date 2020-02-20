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

- [A Pragmatic Approach To Global State](http://adventures.michaelfbryan.com/posts/pragmatic-global-state/)

# Crate of the Week

This week's crate is [argh](https://github.com/google/argh), a small opinionated argument parsing library for Rust.

Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/718) for the suggestions!

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

261 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-02-03..2020-02-10

* [implement proper C ABI lowering for RISC-V](https://github.com/rust-lang/rust/pull/68452)
* [initial implementation of `#![feature(move_ref_pattern)]`](https://github.com/rust-lang/rust/pull/68376)
* [parser: stop using `BytePos` for computing spans](https://github.com/rust-lang/rust/pull/68845)
* [remove `HashStable` impl for `ast::Lifetime`](https://github.com/rust-lang/rust/pull/68919)
* [replace the leak check with universes, take 2](https://github.com/rust-lang/rust/pull/65232)
* [tweak obligation error output](https://github.com/rust-lang/rust/pull/68377)
* [improve reporting errors and suggestions for trait bounds](https://github.com/rust-lang/rust/pull/67665)
* [implement MIR lowering for or-patterns](https://github.com/rust-lang/rust/pull/67668)
* [improve `merge_from_succ` perf](https://github.com/rust-lang/rust/pull/68790)
* [reduce the number of `RefCell`s in `InferCtxt`](https://github.com/rust-lang/rust/pull/68694)
* [improve performance of coherence checks](https://github.com/rust-lang/rust/pull/68966)
* [speed up the inherent impl overlap check](https://github.com/rust-lang/rust/pull/68911)
* [generator resume arguments](https://github.com/rust-lang/rust/pull/68524)
* [remove some unsound specializations](https://github.com/rust-lang/rust/pull/68358)
* [remove problematic specialization from `RangeInclusive`](https://github.com/rust-lang/rust/pull/68835)
* [mark several functions and methods in `core::cmp` as `#[must_use]`](https://github.com/rust-lang/rust/pull/68946)
* [implement `AsMut<str>` for `String`](https://github.com/rust-lang/rust/pull/68742)
* [fix and test implementation of `BTreeMap::`{`first_entry`, `last_entry`, `pop_first`, `pop_last`}](https://github.com/rust-lang/rust/pull/68834)
* [`BtreeMap::range_search` spruced up](https://github.com/rust-lang/rust/pull/68499)
* [make `num::NonZeroX::new` an unstable const fn](https://github.com/rust-lang/rust/pull/68976)
* [make more arithmetic functions unstably const](https://github.com/rust-lang/rust/pull/68809)
* [remove `Copy` impl from `OnceWith`](https://github.com/rust-lang/rust/pull/68810)
* [derive `Clone + Eq` for `std::string::FromUtf8Error`](https://github.com/rust-lang/rust/pull/68738)
* [futures: add `TryFutureExt::map_ok_or_else` method](https://github.com/rust-lang/futures-rs/pull/2058)
* [cargo: fix `BuildScriptOutput` when a build script is run multiple times](https://github.com/rust-lang/cargo/pull/7857)

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
* [Feb 25. London, GB - Rust LDN Talks @ TrueLayer](https://www.meetup.com/Rust-London-User-Group/events/268354799).
* [Feb 27. Wroclaw, PL - Rust Wrocław Meetup #17](https://www.meetup.com/Rust-Wroclaw/events/268683403).

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

> **Even with just basic optimization, Rust was able to outperform the hyper hand-tuned Go version.** This is a huge testament to how easy it is to write efficient programs with Rust compared to the deep dive we had to do with Go.
>
> [..] After a bit of profiling and performance optimizations, **we were able to beat Go on every single performance metric** . Latency, CPU, and memory were all better in the Rust version.

– [Jesse Howard on the discord blog](https://blog.discordapp.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f)

> The consistency angle really shouldn’t be overlooked. Performance is nice, but slow and consistent can still be planned for much more easily than inconsistent.
>
> That was the big aha moment about Rust for me when I pushed out my first project using the language. Being nervous about it I had added way too much instrumentation so that I could know how every bit of it was responding to real traffic. But as soon as I started seeing the data, I was convinced that my instrumentation code was broken. The graphs I was seeing were just so...boring. Straight lines everywhere, no variation...after 24hrs, the slowest response (not P99...literally P100) was within 75ms of the fastest response.

– [/u/tablair commenting on /r/rust](https://www.reddit.com/r/rust/comments/eytyug/why_discord_is_switching_from_go_to_rust/fgjjpiv/)

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/t/twir-quote-of-the-week/328/811) and [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/809) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
