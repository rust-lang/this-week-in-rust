Title: This Week in Rust 420
Number: 420
Date: 2021-12-08
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

### Project/Tooling Updates
* [Updates in IntelliJ Rust for 2021.3](https://blog.jetbrains.com/rust/2021/12/06/updates-in-intellij-rust-for-2021-3/)
* [cloud-hypervisor v20.0](https://github.com/cloud-hypervisor/cloud-hypervisor/releases/tag/v20.0)
* [SixtyFPS (GUI crate): Changelog for 5th of December 2021](https://sixtyfps.io/thisweek/2021-12-06.html)

### Observations/Thoughts

### Rust Walkthroughs
* [Getting Started with Rust on a Raspberry Pi Pico (Part 3)](https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry-a88)

### Miscellaneous

## Crate of the Week

This week's crate is [tap](https://crates.io/crates/tap), a library with extension traits to provide suffix-position pipeline behavior.

Thanks to [David Mason](https://users.rust-lang.org/t/crate-of-the-week/2704/988) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

286 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-11-29..2021-12-06

* [Optimize `rustc_lexer`](https://github.com/rust-lang/rust/pull/91393)
* [Add support for LLVM coverage mapping format versions 5 and 6](https://github.com/rust-lang/rust/pull/91207)
* [Add support for riscv64gc-unknown-freebsd](https://github.com/rust-lang/rust/pull/91284)
* [Fix ICE in `check_must_not_suspend_ty()`](https://github.com/rust-lang/rust/pull/91367)
* [Fix ICE when `yield`ing in function returning `impl Trait`](https://github.com/rust-lang/rust/pull/91488)
* [Don't suggest types whose inner type is erroneous](https://github.com/rust-lang/rust/pull/91450)
* [Only show notable traits if both types are the same](https://github.com/rust-lang/rust/pull/91366)
* [Improve diagnostic for missing half of binary operator in `if` condition](https://github.com/rust-lang/rust/pull/91435)
* [Improve error message for `E0659` if the source is not available](https://github.com/rust-lang/rust/pull/91298)
* [Improve error message for incorrect field accesses through raw pointers](https://github.com/rust-lang/rust/pull/91364)
* [Add `Option::inspect` and `Result::{inspect, inspect_err}`](https://github.com/rust-lang/rust/pull/91346)
* [Add a `try_reduce` method to the `Iterator` trait](https://github.com/rust-lang/rust/pull/87054)
* [Add slice `take` methods](https://github.com/rust-lang/rust/pull/88502)
* [Make `array::`{`try_from_fn`, `try_map`} and `Iterator::try_find` generic over `Try`](https://github.com/rust-lang/rust/pull/91286)
* [Introduce `RawVec::reserve_for_push`](https://github.com/rust-lang/rust/pull/91352)
* [Implement `VecDeque::retain_mut`](https://github.com/rust-lang/rust/pull/91215)
* [libc: Define `max_align_t` for wasi](https://github.com/rust-lang/libc/pull/2577)
* [portable-simd: Generic `core::ops` for `Simd<T, _>`](https://github.com/rust-lang/portable-simd/pull/195)
* [cargo: Stabilize `future-incompat-report`](https://github.com/rust-lang/cargo/pull/10165)
* [cargo: Support abbreviating `--release` as `-r`](https://github.com/rust-lang/cargo/pull/10133)
* [clippy: Consider `NonNull` as a pointer type](https://github.com/rust-lang/rust-clippy/pull/8074)
* [clippy: Escape backslash in `single_char_pattern.rs`](https://github.com/rust-lang/rust-clippy/pull/8067)
* [clippy: Fix `any()` not taking reference in `search_is_some` lint](https://github.com/rust-lang/rust-clippy/pull/7463)
* [clippy: Fix some false negatives for `single_char_pattern`](https://github.com/rust-lang/rust-clippy/pull/8077)
* [clippy: Parenthesize blocks in `needless_bool` suggestion](https://github.com/rust-lang/rust-clippy/pull/8066)
* [clippy: Upgrade `map_flatten` to complexity](https://github.com/rust-lang/rust-clippy/pull/8054)
* [rustfmt: Determine when new comment lines are needed for itemized blocks](https://github.com/rust-lang/rustfmt/pull/5097)

### Rust Compiler Performance Triage

Overall, many changes this week, but overall an improvement on multiple benchmarks over
the week from a number of pull requests dedicated to optimizations of certain
patterns. We are still seeing a large number of spurious changes due to
[rustc-perf#1105](https://github.com/rust-lang/rustc-perf/issues/1105), which
has yet to be addressed.

Triage done by **@simulacrum**.
Revision range: [22c2d9d..1c028783](https://perf.rust-lang.org/?start=22c2d9ddbf356bcdb718e88ca6ee3665e1e42690&end=1c0287830e0fb3c4007afea2819ba03766da6e9c&absolute=false&stat=instructions%3Au)

4 Regressions, 4 Improvements, 9 Mixed; 5 of them in rollups
41 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-11-30.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered final comment period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [rustdoc: accept --out-dir and soft-deprecate --output](https://github.com/rust-lang/rust/issues/91260)
* [disposition: merge] [Tracking Issue for #![feature(available_parallelism)]](https://github.com/rust-lang/rust/issues/74479)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Attributes in function return type position](https://github.com/rust-lang/rfcs/pull/3201)
* [RFC: proc macro include!](https://github.com/rust-lang/rfcs/pull/3200)
* [RFC: Unsafe Lifetime](https://github.com/rust-lang/rfcs/pull/3199)

## Upcoming Events

Rusty Events between 12/01-12/15 🦀

### Online

* [December 3, 2021 | New York City, NY, US | **Getting started in aerospace with Rust 🦀** | NewSpace NYC](https://www.meetup.com/NewSpace-NYC/events/282320805).
* [December 7, 2021 | Buffalo, NY, US | **First Tuesdays** | Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/281833990/)
* [December 8, 2021 | Los Gatos, CA, US | **Book #24 - Rust for Rustaceans - Chapter 1** | Los Gatos Reading Group](https://www.meetup.com/Los-Gatos-Rust-Reading-Group/events/281966245)
* [December 8, 2021 | Los Angeles, CA, US | **Rust Computer Vision Project with Geordon Worley** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/281944671/)
* [December 8, 2021 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282009864)
* [December 10, 2021 | Chicago, IL, US | **8LU Keynote by Eric Smith - Write Your Game in Rust!** | 8th Light University](https://www.meetup.com/8th-light-university/events/282381279)
* [December 14, 2021 | Seattle, WA, US | **Monthly Meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/281875277)

### North America

* [December 8, 2021 | Atlanta, GA, US | **Grab a beer with fellow Rustaceans** | Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccqblb/)

### Europe

* [December 3, 2021 | Moscow, RU | **Rust Con** | RustCon.ru](https://rustcon.ru)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**DeepSource**

* [Software Engineer, Static Analysis - Rust (Bangalore, India)](https://careers.deepsource.io/o/software-engineer-static-analysis-rust)

**Clear**

* [Full Stack Developer (Remote)](https://docs.google.com/document/d/1OuG5Ts_6s4eWO6CXGzcbklOweD7qGnOgADnSoPjEa10/edit)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> v2 of the patch-series "to add support for Rust as a second language to the Linux kernel" was posted to LKML \[..\]
> 
> There have been several improvements to the overall Rust support since RFC and v2 described in the linked mail.

– [Thorsten Leemhuis on twitter](https://twitter.com/kernellogger/status/1467874273582886921?s=20)

llogiq unanimously suggested and voted that this be our quote for this week.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [marriannegoldin](https://github.com/marriannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
