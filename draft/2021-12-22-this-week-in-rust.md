Title: This Week in Rust 422
Number: 422
Date: 2021-12-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Foundation

### Newsletters

### Project/Tooling Updates

* [SixtyFPS (GUI crate): Changelog for 19th of December 2021](https://sixtyfps.io/thisweek/2021-12-20.html)

### Observations/Thoughts

* [Video] [Safe && Portable Data Structure Design (10 minute lightning talk)](https://www.youtube.com/watch?v=1UtklNrB8XA&t=1619s)

### Rust Walkthroughs

- [Cross-compiling Rust Lambdas on macOS without Docker](https://noserve.rs/rust-lambdas-macos/)
- [ZH] [「Pin 三部曲」第二部之 《Rust Pin 进阶](https://folyd.com/blog/rust-pin-advanced/)

### Miscellaneous

## Crate of the Week

This week's crate is [efg](https://crates.io/crates/efg), a proc macro to allow boolean expression like syntax for `#[cfg]`s.

Thanks to [farnbams](https://users.rust-lang.org/t/crate-of-the-week/2704/991) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

315 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-12-06..2021-12-13

* [asm: allow using r9 (ARM) and x18 (AArch64) if they are not reserved by the current target](https://github.com/rust-lang/rust/pull/91643)
* [suggest using a temporary variable to fix borrowck errors](https://github.com/rust-lang/rust/pull/83174)
* [tweak "call this function" suggestion to have smaller span](https://github.com/rust-lang/rust/pull/91503)
* [tweak assoc type obligation spans](https://github.com/rust-lang/rust/pull/91769)
* [better span for unexpected normalization failure in CTFE engine](https://github.com/rust-lang/rust/pull/91815)
* [give more help in the unaligned_references lint](https://github.com/rust-lang/rust/pull/91718)
* [suggest casting between `i`/`u32` and `char`](https://github.com/rust-lang/rust/pull/91245)
* [add a suggestion if `macro_rules` is misspelled](https://github.com/rust-lang/rust/pull/91337)
* [avoid cloning refcounted types during folding](https://github.com/rust-lang/rust/pull/91353)
* [deduplicate projection sub-obligations](https://github.com/rust-lang/rust/pull/90423)
* [do not ICE when suggesting elided lifetimes on non-existent spans](https://github.com/rust-lang/rust/pull/91764)
* [do not add `;` to expected tokens list when it's wrong](https://github.com/rust-lang/rust/pull/91531)
* [do not attempt to suggest help for overly malformed struct/function call](https://github.com/rust-lang/rust/pull/91634)
* [improve 'cannot contain emoji' error](https://github.com/rust-lang/rust/pull/91476)
* [add `spin_loop` hint for RISC-V architecture](https://github.com/rust-lang/rust/pull/91548)
* [override `Iterator::advance`(`_back`)`_by` for `array::IntoIter`](https://github.com/rust-lang/rust/pull/91512)
* [replace dominators algorithm with simple Lengauer-Tarjan](https://github.com/rust-lang/rust/pull/85013)
* [add `<*{const|mut} T>::{to|from}_bits`](https://github.com/rust-lang/rust/pull/91127)
* [add `array::IntoIter::`{`empty`, `from_raw_parts`}](https://github.com/rust-lang/rust/pull/91341)
* [add `rsplit_array` variants to slices and arrays](https://github.com/rust-lang/rust/pull/91515)
* [make `Option::cloned` `const`](https://github.com/rust-lang/rust/pull/90741)
* [make `(*mut T)::write_bytes` `const`](https://github.com/rust-lang/rust/pull/91824)
* [make `Borrow` and `BorrowMut` impls `const`](https://github.com/rust-lang/rust/pull/90270)
* [make `Unique`s methods `const`](https://github.com/rust-lang/rust/pull/91806)
* [make `intrinsics::write_bytes` `const`](https://github.com/rust-lang/rust/pull/90081)
* [implement `TryFrom<&'_ mut [T]>` for `[T; N]`](https://github.com/rust-lang/rust/pull/91086)
* [implement `core::future::join!`](https://github.com/rust-lang/rust/pull/91645)
* [implement concat_bytes!](https://github.com/rust-lang/rust/pull/87599)
* [provide the `ReadBuf` abstraction](https://github.com/rust-lang/rust/pull/81156)
* [stabilise `feature(const_generics_defaults)`](https://github.com/rust-lang/rust/pull/90207)
* [stabilize `ControlFlow::`{`is_break`, `is_continue`}](https://github.com/rust-lang/rust/pull/91091)
* [stabilize `const_cstr_unchecked`](https://github.com/rust-lang/rust/pull/91855)
* [cargo: improve I/O error message for fingerprint of build script](https://github.com/rust-lang/cargo/pull/10191)
* [rustdoc: show type layout for type aliases](https://github.com/rust-lang/rust/pull/91682)
* [clippy: add new lint to warn when `#[must_use]` attribute should be used on a method](https://github.com/rust-lang/rust-clippy/pull/8071)
* [clippy: fix FP on `question_mark` if returned object is not local](https://github.com/rust-lang/rust-clippy/pull/8080)
* [clippy: fix `blocks_in_if_conditions` false positive](https://github.com/rust-lang/rust-clippy/pull/8100)
* [clippy: fix bad suggestion on `option_if_let_else` when there is complex subpat](https://github.com/rust-lang/rust-clippy/pull/8086)
* [clippy: ignore associated types in traits when considering type complexity](https://github.com/rust-lang/rust-clippy/pull/8030)

### Rust Compiler Performance Triage

This week's report started with 6 regressions; after eliminating truly obvious noise, we are left with just 2 minor regressions. Of the cases that regressed, I think the only interesting one is keccak (regressed by 1.73% in PR #91549). But don't be too depressed: keccak was also improved up to 23% by PR #85013 (!); thanks to @**simulacrum** for that PR.

Triage done by **@pnkfelix**.
Revision range: [e2116a..404c847](https://perf.rust-lang.org/?start=e2116acae59654bfab2a9729a024f3e2fd6d4b02&end=404c8471aba60c2d837fa728e7c729a0f52d5830&absolute=false&stat=instructions%3Au)

2 Regressions, 5 Improvements, 3 Mixed; 1 of them in rollups
48 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-12-14.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Amend RFC 2996 to replace Stream with AsyncIterator](https://github.com/rust-lang/rfcs/pull/3208)
* [disposition: merge] [Thread local Cell methods.](https://github.com/rust-lang/rfcs/pull/3184)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Make rustdoc --passes and rustdoc --no-defaults have no effect](https://github.com/rust-lang/rust/issues/91714)
* [disposition: merge] [make Instant::{duration_since, elapsed, sub} saturating and remove workarounds](https://github.com/rust-lang/rust/pull/89926)
* [disposition: close] [Tracking issue for RFC 2115: In-band lifetime bindings](https://github.com/rust-lang/rust/issues/44524)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No new RFCs were published this week.*

## Upcoming Events

Rusty Events between 12/15/2021 - 1/15/2022 🦀

### Online

* [December 15, 2021 | Cardiff, UK | **Rust Book Study Session - Error Handling & Generic Types, Traits, and Lifetimes** | Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/282313169/)
* [December 16, 2021 | Linz, AT | **Rust Meetup Linz - 17th Edition** | Rust Linz](https://www.meetup.com/Rust-Linz/events/282559064/)
* [December 17, 2021 | Various cities, IR | **The Third Rust Iran online meetup** | Rust Iran Meetup](https://rust-meetup.ir)
* [December 18, 2021 | Vancouver, BC, CA | **Your Rust Web Development Toolset** | Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/nwcmpsyccqbtb/)
* [December 21, 2021 | Los Gatos, CA, US | **Book #24 - Rust for Rustaceans - Chapter 1 (session 3)** | Los Gatos Reading Group](https://www.meetup.com/Los-Gatos-Rust-Reading-Group/events/282687733/) | [Alternative Link (South Padre Island, TX Reading Group)](https://www.meetup.com/the-south-padre-island-reading-group/events/282687761/)
* [December 21, 2021 | Washington, DC, US | **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/vdhxgsyccqbcc/)
* [December 23, 2021 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/ttjjqsyccqbfc/)
* [December 28, 2021 | Dallas, TX, US | **Dallas Rust - Last Tuesday** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccqblc/)
* [January 5, 2022 | Indianapolis, IN, US | **Indy.rs - with Social Distancing** | Indy Rust](https://www.meetup.com/indyrs/events/qwtdjsydccbhb/)
* [January 6, 2022 | Nürnberg, DE | **Rust Nürnberg online #8**| Rust Nuremberg](https://www.meetup.com/rust-noris/events/282344613/)
* [January 8, 2022 | Various cities | **Rust GameDev Monthly Meetup** | Rust GameDev](https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com)
* [January 11, 2022 | Seattle, WA, US | **Monthly meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydccbpb/)
* [January 12, 2022 | Boulder, CO, US | **Monthly Meetup** | Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydccbqb/)
* [January 12, 2022 | Los Angeles, CA, US | **Live Coding Session - Mob Programming a Rust Code Kata [Virtual] Jan. 2022** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/282580016/)
* [January 12, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/gjrtqsydccbqb/)


### North America

* [December 16, 2021 | Austin, TX, US | **Rust Lunch** | Rust ATX](https://www.meetup.com/rust-atx/events/282472182)
* [January 12, 2022 | Atlanta, GA, US | **Grab a beer with fellow Rustaceans** | Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsydccbqb/)
* [January 13, 2022 | Columbus, OH, US | **Monthly Meeting** | Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgrydccbrb/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Timescale**
[Senior Rust/C/C++ Engineer, Database Toolkit (Remote (UTC-8 to UTC-5)](https://boards.greenhouse.io/timescale/jobs/5542785002)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

**Parity Technologies**

* [Rust Core Engineer - Solidity Compiler (Solang) (Remote)](https://grnh.se/a5a5c0a33us)
* [Rust Core Engineer - Smart Contract Platform (Remote)](https://grnh.se/cb272e3b3us)
* [Multiple other Rust / Blockchain Engineering positions](https://www.parity.io/jobs)


# Quote of the Week

> This is safer than you may think, because those who need async tend to know it themselves and
> don't ask "should I use async" question. In other words, asking itself is a signal that answer is
> no. MITM proxy case was a rare exception.

– [Seo Sanghyeon on rust-users](https://users.rust-lang.org/t/examples-of-high-performance-rust-multi-thread-network-app-w-o-async/68513/4)

Thanks to [Zeroexcuses](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1146) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
