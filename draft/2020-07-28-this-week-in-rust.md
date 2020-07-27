Title: This Week in Rust 349
Number: 349
Date: 2020-07-28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/023-twir-348/)

# Updates from Rust Community

## News & Blog Posts

* [Tutorial for Tokio and async Rust](https://tokio.rs/tokio/tutorial)

# Crate of the Week

This week's crate is [pre](https://github.com/aticu/pre), a library for declaring and checking the assurance of precondition, useful for unsafe functions.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/792) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

394 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-07-13..2020-07-20

* [do not try fetching the ancestors of errored trait impls](https://github.com/rust-lang/rust/pull/74516)
* [only skip impls of foreign unstable traits](https://github.com/rust-lang/rust/pull/74534)
* [don't assign `()` to `!` MIR locals](https://github.com/rust-lang/rust/pull/74411)
* [some `Symbol` related improvements](https://github.com/rust-lang/rust/pull/74357)
* [use `ArrayVec` in `SparseBitSet`](https://github.com/rust-lang/rust/pull/74310)
* [change `SymbolName::name` to a `&str`](https://github.com/rust-lang/rust/pull/74214)
* [enforce the static symbol order](https://github.com/rust-lang/rust/pull/74203)
* [reduce the amount of interning and `layout_of` calls in const eval](https://github.com/rust-lang/rust/pull/74202)
* [add `Arguments::as_str()`](https://github.com/rust-lang/rust/pull/74056)
* [`impl Index<RangeFrom> for CStr`](https://github.com/rust-lang/rust/pull/74021)
* [add (unchecked) indexing methods to raw (and NonNull) slices](https://github.com/rust-lang/rust/pull/73986)
* [make some `Option` methods const](https://github.com/rust-lang/rust/pull/73930)
* [use `step_unchecked` more liberally in range iter impls](https://github.com/rust-lang/rust/pull/73490)
* [add `core::task::ready!` macro](https://github.com/rust-lang/rust/pull/70817)
* [backtrace: use noop backends on Miri](https://github.com/rust-lang/backtrace-rs/pull/360)
* [stdarch: update and revamp wasm32 SIMD intrinsics](https://github.com/rust-lang/stdarch/pull/874)
* [stdarch: implement AVX512f floating point comparisons](https://github.com/rust-lang/stdarch/pull/869)
* [stdarch: constify all x86 `rustc_args_required_const` intrinsics](https://github.com/rust-lang/stdarch/pull/876)
* [make `unreachable_unchecked` a const fn](https://github.com/rust-lang/rust/pull/74459)
* [cargo: fix freshness checks for build scripts on renamed dirs](https://github.com/rust-lang/cargo/pull/8497)
* [crates.io: generate API tokens with a secure RNG, store hashed](https://github.com/rust-lang/crates.io/pull/2637)
* [add Ayu theme to rustdoc](https://github.com/rust-lang/rust/pull/71237)
* [clippy: `unnecessary_sort_by`: avoid linting if key borrows](https://github.com/rust-lang/rust-clippy/pull/5756)

## Rust Compiler Performance Triage

* [2020-07-21](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-07-21.md).
  A disastrous week. At least 7 regressions. 3 improvements. Lots of murkiness due to rollups.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: 'C unwind' ABI](https://github.com/rust-lang/rfcs/pull/2945)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize core::future::{pending,ready}](https://github.com/rust-lang/rust/pull/74328)
* [disposition: merge] [Make more primitive integer methods const](https://github.com/rust-lang/rust/pull/73858)
* [disposition: merge] [Derive common traits for panic::Location](https://github.com/rust-lang/rust/pull/73583)

## New RFCs

* [Add `oneof` configuration predicate to support exclusive features](https://github.com/rust-lang/rfcs/pull/2962)
* [RFC: Promote aarch64-unknown-linux-gnu to a Tier-1 Rust target](https://github.com/rust-lang/rfcs/pull/2959)
* [Add Drop::poll_drop_ready for asynchronous destructors](https://github.com/rust-lang/rfcs/pull/2958)
* [Stabilize Cargo's new feature resolver](https://github.com/rust-lang/rfcs/pull/2957)
* [Add the partial-closure-args RFC](https://github.com/rust-lang/rfcs/pull/2956)

# Upcoming Events

### Online
* [July 23. Berlin, DE - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybckbfc/) 
* [July 27 - August 2. Rusty Days Virtual Rust Conference](https://rusty-days.org/)
* [August 6, Rust Meetup Linz](https://www.meetup.com/de-DE/Rust-Linz/events/271857182/)

### North America
* [July 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybckbkc/)
* [July 28. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybckblc/)

### Asia Pacific
* [August 3. Auckland, NZ - Rust ALK - Rust Meetup](https://www.meetup.com/rust-akl/events/266876693/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> `unsafe` Rust is all about flirting with UB but never giving in.

– [Ralf Jung on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/136281-t-lang.2Fwg-unsafe-code-guidelines/topic/Language.20UB.20vs.20library.20UB/near/204212193)

Thanks to [HeroicKatora](https://users.rust-lang.org/t/twir-quote-of-the-week/328/913) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hvjf4i/this_week_in_rust_348/)small>
