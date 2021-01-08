Title: This Week in Rust 372
Number: 372
Date: 2020-01-06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official

### Project/Tooling Updates
* [napi 1.0 released](https://napi.rs)
* [Insta got a new website with docs](https://insta.rs/) and a [Visual Studio Code Extension](https://marketplace.visualstudio.com/items?itemName=mitsuhiko.insta)

### Observations/Thoughts
* [Reflecting on developing a database (2020 edition)](https://alex-dukhno.github.io/2020-12-31-Reflecting-on-developing-a-database-(2020-edition)/)
* [bore(1) + nonymous: lessons learned writing a DNS query tool and `#![no_std]` DNS library](https://www.azabani.com/2021/01/03/nonymous-bore.html)

### Rust Walkthroughs
* [Building a runtime reflection system for Rust 🦀️ (Part 3)](https://www.osohq.com/post/runtime-reflection-pt-3)

### Miscellaneous

# Crate of the Week

This week's crate is [nom-supreme](https://crates.io/crates/nom-supreme), a crate of utilities for nom parsers, especially for great parse error handling.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/864) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

322 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-12-28..2021-01-04

* [add edition 2021](https://github.com/rust-lang/rust/pull/79576)
* [sync `rustc_codegen_cranelift`](https://github.com/rust-lang/rust/pull/80408)
* [diag: print enum variant instead of enum type](https://github.com/rust-lang/rust/pull/80613)
* [suggest renaming or escaping when fixing non-snake-case identifiers which would conflict with keywords](https://github.com/rust-lang/rust/pull/80592)
* [support pattern as const parents in `type_of`](https://github.com/rust-lang/rust/pull/80551)
* [parse const generics defaults](https://github.com/rust-lang/rust/pull/80547)
* [miri: make size/align_of_val work for dangling raw ptrs](https://github.com/rust-lang/rust/pull/80491)
* [slightly more typed interface to panic implementation](https://github.com/rust-lang/rust/pull/80260)
* [remove all `doc_comment!{}` hacks by using `#[doc = expr]` where needed](https://github.com/rust-lang/rust/pull/79150)
* [make `copy`(`_nonoverlapping`) const](https://github.com/rust-lang/rust/pull/79684)
* [add `Iterator::intersperse`](https://github.com/rust-lang/rust/pull/79479)
* [add fallible `Box`, `Arc`, and `Rc` allocator APIs](https://github.com/rust-lang/rust/pull/80310)
* [do not create dangling `&T` in `Weak<T>::drop`](https://github.com/rust-lang/rust/pull/80488)
* [de-stabilize unsized raw ptr methods for `Weak`](https://github.com/rust-lang/rust/pull/80422)

## Rust Compiler Performance Triage

* [2020-12-24](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-24.md):
3 Regressions, 5 Improvements
Major event this week is landing PGO for rustc (on x86_64-unknown-linux-gnu). We
expect other platforms to follow but further investigation will be needed,
especially for cross-compiled platforms. We expect to add LLVM PGO as well.

Triage done by @simulacrum.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-24.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [Stabilize split_inclusive](https://github.com/rust-lang/rust/pull/77858)
* [Tracking issue for stable SIMD in Rust](https://github.com/rust-lang/rust/issues/48556)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [January 6, Johannesburg, ZA - Monthly Joburg Rust Chat - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/275424876/)
* [January 6, Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy Rust](https://www.meetup.com/indyrs/events/jhfstrycccbjb/)
* [January 6, Denver, CO, US - Building Malware in Rust TBG by Stu Small - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/275094422/)
* [January 7, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrycccbkb/)
* [January 11, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrycccbqb/)
* [January 12, Saarbücken, Saarland, DE - Meetup: 7u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/275077213/)
* [January 14, San Diego, CA, US - San Diego Rust January 2021 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/275547915/)

### North America
* [January 14, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgrycccbsb/)
* [January 14, Provo, UT, US - The Blue Pill: Rust on Microcontrollers (Jan / Third Round) - Utah Rust](https://www.meetup.com/utah-rust/events/268567961/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Think of "it works" when you have UB like this: You've flipped a coin 1 time and it's come up heads and you've concluded it's never tails.

– @mirashii on the community discord

Thanks to [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/981) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
