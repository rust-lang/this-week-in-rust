Title: This Week in Rust 419
Number: 419
Date: 2021-12-01
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

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

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

This week, there were a number of cases where the `incr-unchanged` variants of `inflate` went up or down by 5% to 6%; we believe these are instances of increased noise in benchmarks documented on [rustc-perf#1105](https://github.com/rust-lang/rustc-perf/issues/1105). I was tempted to remove these from the report, but its non-trivial to re-construct the report "as if" some benchmark were omitted.

Otherwise, there were some nice wins for performance. For example, PR [#90996](https://github.com/rust-lang/rust/issues/90996) more than halved the time to document builds of `diesel` by revising how we hash `ObligationCauseData`. If anyone is interested, it might be good to follow-up on the effects of PR [#90352](https://github.com/rust-lang/rust/issues/90352), "Simplify `for` loop desugar", where we have hypothesized that the increased compilation time is due to more LLVM optimizations being applied.

Triage done by **@pnkfelix**.
Revision range: [934624fe..22c2d9dd](https://perf.rust-lang.org/?start=934624fe5f66ce3fb8abf0597a6deb079783335f&end=22c2d9ddbf356bcdb718e88ca6ee3665e1e42690&absolute=false&stat=instructions%3Au)

1 Regressions, 3 Improvements, 8 Mixed; 3 of them in rollups
34 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-11-23.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

[Constrained Naked Functions](https://github.com/rust-lang/rfcs/pull/2972)
[Cargo --crate-type CLI Argument](https://github.com/rust-lang/rfcs/pull/3180)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered final comment period this week.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize some MaybeUninit behavior as const](https://github.com/rust-lang/rust/pull/90896)
* [disposition: merge] [Document setgroups call caused by std::os::unix::process::CommandExt.uid](https://github.com/rust-lang/rust/pull/90292)
* [disposition: merge] [Tracking Issue for const-initialized thread locals](https://github.com/rust-lang/rust/issues/84223)
* [disposition: merge] [Tracking Issue for NonZeroUn::is_power_of_two (feature nonzero_is_power_of_two)](https://github.com/rust-lang/rust/issues/81106)
* [disposition: merge] [Tracking Issue for #![feature(available_parallelism)]](https://github.com/rust-lang/rust/issues/74479)
* [disposition: merge] [Tracking Issue for inline assembly (asm!)](https://github.com/rust-lang/rust/issues/72016)
* [disposition: merge] [Tracking Issue for cargo report future-incompat](https://github.com/rust-lang/rust/issues/71249)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* RFC Update: [Clarify that RFC1520 does not permit the compiler to replace calls to Clone::clone with a memcpy](https://github.com/rust-lang/rfcs/pull/3197)

## Upcoming Events

Rusty Events between 11/24-12/08 🦀

### Online

* [November 25, 2021 | Cardiff, WLS | **Rust Book Study Session - Packages, Crates and Modules & Common Collections** | Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/282025037)
* [November 25, 2021 | Nuremberg, DE | **Rust Nürnberg online #7** | Rust Nuremberg](https://www.meetup.com/rust-noris/events/281829098)
* [November 25, 2021 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282000759)
* [November 25, 2021 | Linz, AT | **Rust Meetup Linz - 16th Edition** | Rust Linz](https://www.meetup.com/Rust-Linz/events/282093961)
* [November 30, 2021 | Dallas, TX, US | **Last Tuesday** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccpbnc/)
* [November 30, 2021 | Graz, AT | **Rust and memory safety** | Rust Graz Meetup](https://www.meetup.com/Graz-Rust-Meetup/events/281955585)
* [December 7, 2021 | Buffalo, NY, US | **First Tuesdays** | Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/281833990/)
* [December 8, 2021 | Los Gatos, CA, US | **Book #24 - Rust for Rustaceans - Chapter 1** | Los Gatos Reading Group](https://www.meetup.com/Los-Gatos-Rust-Reading-Group/events/281966245)
* [December 8, 2021 | Los Angeles, CA, US | **Rust Computer Vision Project with Geordon Worley** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/281944671/)
* [December 8, 2021 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282009864)

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

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [marriannegoldin](https://github.com/marriannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
