Title: This Week in Rust 342
Number: 342
Date: 2020-06-09
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

There is no *This Week in Rust* podcast this week, next week's episode will cover both this week and next week.

Check out [this week's *This Week in Rust Podcast*]()

# Updates from Rust Community

## News & Blog Posts

* [Rust cli app integrated with slack](https://bprog.github.io/rust_slack_bot/)

# Crate of the Week

This week's crate is [jql](https://github.com/yamafaktory/jql), a JSON Query Language CLI tool.

Thanks to [Davy Duperron](https://users.rust-lang.org/t/crate-of-the-week/2704/775) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation


Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

442 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-05-25..2020-06-01

* [implement unsafe blocks in unsafe fn](https://github.com/rust-lang/rust/pull/71862) (RFC [#2585](https://rust-lang.github.io/rfcs/2585-unsafe-block-in-unsafe-fn.html))
* [exhaustiveness checking: work around type normalization issues](https://github.com/rust-lang/rust/pull/72506)
* [suggest using `std::mem::drop` function instead of explicit destructor call](https://github.com/rust-lang/rust/pull/72383)
* [add a lint against references to packed fields](https://github.com/rust-lang/rust/pull/72270)
* [avoid setting wrong obligation cause span of associated type mismatch](https://github.com/rust-lang/rust/pull/72807)
* [account for trailing comma when suggesting `where` clauses](https://github.com/rust-lang/rust/pull/72715)
* [fix diagnostics for `@ ..` binding pattern in tuples and tuple structs](https://github.com/rust-lang/rust/pull/72677)
* [chalk: request hidden opaque types lazily](https://github.com/rust-lang/chalk/pull/478)
* [miri: synchronization primitive cleanup](https://github.com/rust-lang/miri/pull/1441)
* [`from_u32_unchecked`: check validity, and fix UB in Wtf8](https://github.com/rust-lang/rust/pull/72683)
* [implement `total_cmp` for `f32`, `f64`](https://github.com/rust-lang/rust/pull/72568)
* [override `Box::<[T]>::clone_from`](https://github.com/rust-lang/rust/pull/72499)
* [add `Extend::`{`extend_one`, `extend_reserve`}](https://github.com/rust-lang/rust/pull/72162)
* [make pointer offset methods/intrinsics const](https://github.com/rust-lang/rust/pull/71500)
* [`impl From<[T; N]> for Box<[T]>`](https://github.com/rust-lang/rust/pull/71095)
* [stabilization of `weak-into-raw`](https://github.com/rust-lang/rust/pull/72288)
* [resolve UB in Arc/Weak interaction, part 2](https://github.com/rust-lang/rust/pull/72533)
* [stabilize `str_strip` feature](https://github.com/rust-lang/rust/pull/72466)
* [`impl Step for char` (make `Range*<char>` iterable)](https://github.com/rust-lang/rust/pull/72413)
* [add `Peekable::next_if`](https://github.com/rust-lang/rust/pull/72310)
* [various minor improvements to `Ipv6Addr::Display`](https://github.com/rust-lang/rust/pull/72407)
* [`SocketAddr` and friends now correctly pad its content](https://github.com/rust-lang/rust/pull/72398)
* [implement PartialOrd and Ord for SocketAddr*](https://github.com/rust-lang/rust/pull/72239)
* [tweak and stabilize `Atomic`N`::fetch_update`](https://github.com/rust-lang/rust/pull/71843)
* [stabilize `Atomic`N`::fetch_`{`min`, `max`}](https://github.com/rust-lang/rust/pull/72324)
* [stdarch: add 64 bit integer AVX512f comparisons and the intrinsics needed to test them](https://github.com/rust-lang/stdarch/pull/856)
* [stdarch: add 64 bit AVX512f le and ge comparisons](https://github.com/rust-lang/stdarch/pull/861)
* [libm: use macros for more division/array checks](https://github.com/rust-lang/libm/pull/244)

## Rust Compiler Performance Triage

This is a new section containing the results of a weekly check on how rustc's
perf has changed.

* [2020-06-02](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-06-02)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Transition to rust-analyzer as our official LSP (Language Server Protocol) implementation](https://github.com/rust-lang/rfcs/pull/2912)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [`impl AsRef<[T]>` for `vec::IntoIter<T>`](https://github.com/rust-lang/rust/pull/72583)
* [disposition: merge] [Add raw_ref macros](https://github.com/rust-lang/rust/pull/72279)
* [disposition: merge] [Tracking issue for `std::io::{BufReader, BufWriter}::capacity`](https://github.com/rust-lang/rust/issues/68833)

## New RFCs

* [add lang-team Major Change Proposals as a "pre-RFC" step](https://github.com/rust-lang/rfcs/pull/2936)
* [Unsafe statics](https://github.com/rust-lang/rfcs/pull/2937)
* [Request for creating pipes with fd other than 0,1,2](https://github.com/rust-lang/rfcs/pull/2939)

# Upcoming Events

### Online
* [June 3. Johannesburg, ZA - Remote - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/270827463/)
* [June 8. Auckland, NZ - Remote - Rust AKL](https://www.meetup.com/rust-akl/events/266876685/)
* [June 9. Seattle, WA - Remote - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybcjbmb/)
* [June 11. San Diego, CA - Remote - San Diego Rust Meetup](https://www.meetup.com/San-Diego-Rust/events/270938860/)

### North America
* [June 3. Indianapolis, IN, US - Indy.rs Meetup](https://www.meetup.com/indyrs/events/dtqwprybcjbfb/)
* [June 11. Columbus, OH, US - Columbus Rust Society Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybcjbpb/)
* [June 11. Lehi, UT, US - Utah Rust - Lightning Talks](https://www.meetup.com/utah-rust/events/269263282/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust enables belligerent refactoring – making dramatic changes and then working with the compiler to bring the project back to a working state.

– [Pankaj Chaudhary on Knoldus Blog](https://blog.knoldus.com/some-extensive-projects-working-with-rust)

Thanks to [Maxim Vorobjov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/880) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/gvwvep/this_week_in_rust_341/)</small>
