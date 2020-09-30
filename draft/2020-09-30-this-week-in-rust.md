Title: This Week in Rust 358
Number: 358
Date: 2020-09-30
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official

### Tooling

### Observations/Thoughts
* [Benchmarking vol. 2: Pitting Actix against Rocket v0.4 and v0.5-dev](https://matej.laitl.cz/bench-actix-rocket/)

### Learn Standard Rust

### Learn More Rust
* [Are we observable yet? An introduction to Rust telemetry - Zero To Production #4](https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/)
* [A Fistful of States: More State Machine Patterns in Rust](https://deislabs.io/posts/a-fistful-of-states/)
* [OS in Rust: An executable that runs on bare metal: Part-1](https://blog.knoldus.com/os-in-rust-an-executable-that-runs-on-bare-metal-part-1/)
* [OS in Rust: An executable that runs on bare metal: Part-2](https://blog.knoldus.com/os-in-rust-an-executable-that-runs-on-bare-metal-part-2/)
* [Build a Discord Bot with Rust and Serenity](https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/)
* [Porting EBU R128 audio loudness analysis from C to Rust – Porting Details](https://coaxion.net/blog/2020/09/porting-ebu-r128-audio-loudness-analysis-from-c-to-rust-porting-details/)
* [Building a runtime reflection system for Rust 🦀️ (Part 1)](https://www.osohq.com/post/rust-reflection-pt-1)

### Project Updates
* [Krustlet v0.5.0 Release](https://github.com/deislabs/krustlet/releases/tag/v0.5.0)
* [oso, an open-source policy engine for authorization written in Rust](https://github.com/osohq/oso), added [a Rust library for implementing authorization for Rust projects!](https://docs.rs/oso/0.6.0-alpha/oso/)

### Miscellaneous

# Call for Blog Posts

The Rust Core Team wants input from the community!
If you haven't already, [read the official blog](https://blog.rust-lang.org/2020/09/03/Planning-2021-Roadmap.html) and submit a blog post - it will show up here!
Here are the wonderful submissions since the call for blog posts:

# Crate of the Week

This week's crate is [fs-err](https://crates.io/crates/fs-err), a library to make filesystem errors usable.

Thanks to [Emerentius](https://users.rust-lang.org/t/crate-of-the-week/2704/821) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

370 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-09-21..2020-09-28

* [return values up to 128 bits in registers](https://github.com/rust-lang/rust/pull/76986)
* [add `asm!` support for MIPS](https://github.com/rust-lang/rust/pull/76839)
* [diagnostics: improve closure/generic parameter mismatch](https://github.com/rust-lang/rust/pull/76711)
* [avoiding unnecesary allocations at `rustc_errors`](https://github.com/rust-lang/rust/pull/76846)
* [add fast path for match checking](https://github.com/rust-lang/rust/pull/76918)
* [cache types during normalization](https://github.com/rust-lang/rust/pull/76928)
* [fix the performance regression of #76244](https://github.com/rust-lang/rust/pull/76913)
* [encode less metadata for proc-macro crates](https://github.com/rust-lang/rust/pull/76897)
* [invalidate local LLVM cache less often](https://github.com/rust-lang/rust/pull/77126)
* [introduce a new flag to enable experimental/unsound mir opts](https://github.com/rust-lang/rust/pull/76899)
* [MIR pass to remove unneeded drops on types not needing drop](https://github.com/rust-lang/rust/pull/76673)
* [add optimization to avoid load of address](https://github.com/rust-lang/rust/pull/76683)
* [miri: more informative deallocation error messages](https://github.com/rust-lang/rust/pull/77047)
* [miri: add API for capturing backtrace](https://github.com/rust-lang/miri/pull/1559)
* [`DroplessArena`: allocate objects from the end of memory chunk](https://github.com/rust-lang/rust/pull/77014)
* [unstably allow `assume` intrinsic in const contexts](https://github.com/rust-lang/rust/pull/76973)
* [add `array::from_ref`](https://github.com/rust-lang/rust/pull/77074)
* [add `#[track_caller]` to more panicking `Cell` functions](https://github.com/rust-lang/rust/pull/77055)
* [make some methods of `Pin` unstably const](https://github.com/rust-lang/rust/pull/76655)
* [revert `const_type_id` stabilization](https://github.com/rust-lang/rust/pull/77083)
* [revert adding `Atomic::from_mut`](https://github.com/rust-lang/rust/pull/76967)
* [add `cfg(target_has_atomic_equal_alignment)` and use it for `Atomic::from_mut`](https://github.com/rust-lang/rust/pull/76965)
* [make `[].as_`[`mut_`]`ptr_range()` (unstably) const](https://github.com/rust-lang/rust/pull/77097)
* [log: implement `Log` for `Box<Log>`](https://github.com/rust-lang/log/pull/414)

## Rust Compiler Performance Triage

* [2020-09-28](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-09-28.md):
0 Regressions, 1 Improvements, 3 Mixed


Most significant changes this week came in response to regressions discussed in
last week's triage report. Curious readers may be interested in
[#77058](https://github.com/rust-lang/rust/issues/77058), in which the removal
of a single field from a struct caused a 25% decrease in wall-times for one
seemingly unrelated benchmark, or
[#76986](https://github.com/rust-lang/rust/issues/76986), an ABI change that
should be a pretty clear win but seems to have mixed results.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-09-28.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [[RFC]: Portable SIMD Libs Project Group](https://github.com/rust-lang/rfcs/pull/2977)
* [Establish a new error handling project group](https://github.com/rust-lang/rfcs/pull/2965)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Get type of an arbitrary expression](https://github.com/rust-lang/rfcs/pull/2706)
* [Add generalized arity tuples](https://github.com/rust-lang/rfcs/pull/2702)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge][Make RawFd implement the RawFd traits](https://github.com/rust-lang/rust/pull/76969)
* [disposition: merge][Permit uninhabited enums to cast into ints](https://github.com/rust-lang/rust/pull/76199)
* [disposition: merge][Stabilize move_ref_pattern](https://github.com/rust-lang/rust/pull/76119)
* [disposition: merge][Write manifest for MAJOR.MINOR channel to enable rustup convenience](https://github.com/rust-lang/rust/pull/76107)
* [disposition: merge][Explicitly document the size guarantees that Option makes.](https://github.com/rust-lang/rust/pull/75454)
* [disposition: merge][Stabilize intra-doc links](https://github.com/rust-lang/rust/pull/74430)
* [disposition: merge][Add PartialEq impls for Vec <-> slice](https://github.com/rust-lang/rust/pull/74194)
* [disposition: merge][target-feature 1.1: should closures inherit target-feature annotations?](https://github.com/rust-lang/rust/issues/73631)
* [disposition: merge][might_permit_raw_init: also check aggregate fields](https://github.com/rust-lang/rust/pull/71274)

## New RFCs

* [RFC 2582: fix implicit auto-deref of raw pointers](https://github.com/rust-lang/rfcs/pull/2987)
* [Stable Rustdoc URLs](https://github.com/rust-lang/rfcs/pull/2988)

# Upcoming Events

### Online
* [September 29. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/jqxqwrybcmbmc/)
* [October 1. Berlin, DE - Berline.rs - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcnbcb/)
* [October 13. Saarbrücken, DE - Rust-Saar Meetup - `4u16`](https://www.meetup.com/Rust-Saar/events/273252813/)

### Asia Pacific
* [October 5. Auckland, NZ - Rust AKL - Rust meetup](https://www.meetup.com/rust-akl/events/266876708/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

[3D Driver Development Engineer - Rust tooling for GPUs at AMD (Boxborough, MA, USA)](https://jobs.amd.com/job/Boxborough-3D-Driver-Development-Engineer-80489-Mass/677678000/)

# Quote of the Week

> Rust has a curse (it has many, but this one is critical): inefficient code is generally visible. Experienced developers hate to notice that their code is inefficient. They will recoil at seeing `Arc<RefCell<T>>` , but won't bat an eye at using Python.

- [Esteban Kuber on rust-users](https://users.rust-lang.org/t/failed-to-contribute-due-to-difficulty-in-understanding-rust/49148/6)

Thanks to [Jon G Stødle](https://users.rust-lang.org/t/twir-quote-of-the-week/328/945) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/iu3ge0/this_week_in_rust_356/)</small>
