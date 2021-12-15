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

This week's crate is [poem-openapi](https://crates.io/crates/poem-openapi), a framework to implement OpenAPI services.

llogiq is very pleased with his suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

244 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-11-22..2021-11-29

* [add codegen option for using LLVM stack smash protection](https://github.com/rust-lang/rust/pull/84197)
* [do not use atomic reads on platforms without atomic support in LLVM](https://github.com/rust-lang/compiler-builtins/pull/442)
* [diagnostic tweaks](https://github.com/rust-lang/rust/pull/85102)
* [chalk: introduce `Folder::Error`](https://github.com/rust-lang/chalk/pull/709)
* [miri: more portable SIMD: rem, shl, shr](https://github.com/rust-lang/miri/pull/1923)
* [fix ICE due to out-of-bounds statement index when reporting borrowck error](https://github.com/rust-lang/rust/pull/91212)
* [fix ICE when lowering `trait A where for<'a> Self: 'a`](https://github.com/rust-lang/rust/pull/91308)
* [faster `Layout::array`](https://github.com/rust-lang/rust/pull/91246)
* [make `TypeFolder::fold_*` return `Result`](https://github.com/rust-lang/rust/pull/91230)
* [partially stabilize `duration_consts_2`](https://github.com/rust-lang/rust/pull/89542)
* [stabilize `nonzero_is_power_of_two`](https://github.com/rust-lang/rust/pull/91301)
* [stabilize some `MaybeUninit` behavior as const](https://github.com/rust-lang/rust/pull/90896)
* [add fast path to `is_descendant_of`](https://github.com/rust-lang/rust/pull/91043)
* [futures: remove dependency on proc-macro-hack](https://github.com/rust-lang/futures-rs/pull/2520)
* [cargo: improve unused patch message when source URLs mismatched](https://github.com/rust-lang/cargo/pull/10130)
* [clippy: add `needless_late_init` lint](https://github.com/rust-lang/rust-clippy/pull/7995)
* [clippy: add more descriptive help info for `needless_question_mark`](https://github.com/rust-lang/rust-clippy/pull/8028)
* [clippy: fix false positive on `if_then_some_else_none` with early return](https://github.com/rust-lang/rust-clippy/pull/7980)
* [clippy: improve `strlen_on_c_string`](https://github.com/rust-lang/rust-clippy/pull/8001)
* [clippy: extend `non_ascii_literal` to cover chars](https://github.com/rust-lang/rust-clippy/pull/8034)
* [clippy: apply iter_cloned_collect to collect() using copied()](https://github.com/rust-lang/rust-clippy/pull/8006)
* [rustdoc: avoid documenting top-level private imports](https://github.com/rust-lang/rust/pull/91094)
* [rustfmt: fix: do not wrap reference-style doc links](https://github.com/rust-lang/rustfmt/pull/5096)
* [rustfmt: maintain more AST info when formatting a RHS](https://github.com/rust-lang/rustfmt/pull/5113)

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

Rusty Events between 12/15/2021 - 1/15/2022 🦀

### Online
* [December 15, 2021 | Cardiff, UK | **Rust Book Study Session - Error Handling & Generic Types, Traits, and Lifetimes** | Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/282313169/)
* [December 16, 2021 | Linz, AT | **Rust Meetup Linz - 17th Edition** | Rust Linz](https://www.meetup.com/Rust-Linz/events/282559064/)
* [December 17, 2021 | Various cities, IR | **The Third Rust Iran online meetup** | Rust Iran Meetup](https://rust-meetup.ir)
* [December 18, 2021 | Vancouver, BC, CA | **Your Rust Web Development Toolset** | Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/nwcmpsyccqbtb/)
* [December 21, 2021 | Los Gatos, CA, US | **Book #24 - Rust for Rustaceans - Chapter 1 (session 3)** | Los Gatos Reading Group](https://www.meetup.com/Los-Gatos-Rust-Reading-Group/events/282687733/)
* [December 21, 2021 | Washington, DC, US | **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/vdhxgsyccqbcc/)
* [December 23, 2021 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/ttjjqsyccqbfc/)
* [December 28, 2021 | Dallas, TX, US | **Dallas Rust - Last Tuesday** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccqblc/)
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

**DeepSource**

* [Software Engineer, Static Analysis - Rust (Bangalore, India)](https://careers.deepsource.io/o/software-engineer-static-analysis-rust)

**Clear**

* [Full Stack Developer (Remote)](https://docs.google.com/document/d/1OuG5Ts_6s4eWO6CXGzcbklOweD7qGnOgADnSoPjEa10/edit)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The design of the safe/unsafe split means that there is an asymmetric trust relationship between Safe and Unsafe Rust. Safe Rust inherently has to trust that any Unsafe Rust it touches has been written correctly. On the other hand, Unsafe Rust cannot trust Safe Rust without care.
>
> As an example, Rust has the [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) and [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) traits to differentiate between types which can "just" be compared, and those that provide a "total" ordering (which basically means that comparison behaves reasonably).
>
> [`BTreeMap`](https://doc.rust-lang.org/nightly/std/collections/struct.BTreeMap.html) doesn't really make sense for partially-ordered types, and so it requires that its keys implement `Ord` . However, `BTreeMap` has Unsafe Rust code inside of its implementation. Because it would be unacceptable for a sloppy `Ord` implementation (which is Safe to write) to cause Undefined Behavior, the Unsafe code in BTreeMap must be written to be robust against `Ord` implementations which aren't actually total — even though that's the whole point of requiring `Ord` .

– [Gankra citing the Rustonomicon on github](https://github.com/rust-lang/rfcs/pull/3197#issuecomment-976032253)

Thanks to [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1144) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
