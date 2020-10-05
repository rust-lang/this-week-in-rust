Title: This Week in Rust 359
Number: 359
Date: 2020-10-07
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

### Learn Simple Rust

### Learn More Rust

### Project Updates

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

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Destructuring assignment](https://github.com/rust-lang/rfcs/pull/2909)
* [RFC: impl-only glob imports](https://github.com/rust-lang/rfcs/pull/2782)
* [Fetching cargo registry tokens from external processes](https://github.com/rust-lang/rfcs/pull/2730)
* [RFC: Permit _ in type aliases](https://github.com/rust-lang/rfcs/pull/2524)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize slice_ptr_range.](https://github.com/rust-lang/rust/pull/77111)
* [disposition: merge] [Make RawFd implement the RawFd traits](https://github.com/rust-lang/rust/pull/76969)
* [disposition: merge] [Fix Debug implementations of some of the HashMap and BTreeMap iterator types](https://github.com/rust-lang/rust/pull/75377)
* [disposition: merge] [Tracking issue for slice_partition_at_index](https://github.com/rust-lang/rust/issues/55300)

## New RFCs

* [[RFC] A new stack-based vector](https://github.com/rust-lang/rfcs/pull/2990)
* [RFC: Add `target` configuration](https://github.com/rust-lang/rfcs/pull/2991)
* [RFC: Add `target_abi` configuration](https://github.com/rust-lang/rfcs/pull/2992)
* [adds async stream rfc](https://github.com/rust-lang/rfcs/pull/2996)

# Upcoming Events

### Online
* [October 1. Berlin, DE - Berline.rs - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcnbcb/)
* [October 7. Johannesburg, ZA - Johannesburg Rust Meetup - Monthly Joburg Rust Chat!](https://www.meetup.com/Johannesburg-Rust-Meetup/events/273455489/)
* [October 7. Dublin, IE - Rust Dublin - October Remote Meetup](https://www.meetup.com/Rust-Dublin/events/273014329/)
* [October 7. Indianapolis, IN, US - Indy.rs - Indy.rs - with Social Distancing](https://www.meetup.com/indyrs/events/jhfstrybcnbkb/)
* [October 8. Linz, AT - Rust Linz - Rust Meetup Linz](https://www.meetup.com/de-DE/Rust-Linz/events/271857253/)
* [October 8. San Diego, CA, US - San Diego Rust - San Diego Rust October 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/273486967/)
* [October 13. Saabrücken, DE - Rust-Saar Meetup - `4u16`](https://www.meetup.com/Rust-Saar/events/273252813/)
* [October 12 - 18. RustLab](https://www.rustlab.it/agenda)

### Asia Pacific
* [October 4. Auckland, NZ - Rust AKL - Rust meetup](https://www.meetup.com/rust-akl/events/266876708/)

### North America
* [October 8. Lehi, UT - Utah Rust - The Blue Pill: Rust on Microcontrollers](https://www.meetup.com/utah-rust/events/268567961/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust has a curse (it has many, but this one is critical): inefficient code is generally visible. Experienced developers hate to notice that their code is inefficient. They will recoil at seeing `Arc<RefCell<T>>` , but won't bat an eye at using Python.

- [Esteban Kuber on rust-users](https://users.rust-lang.org/t/failed-to-contribute-due-to-difficulty-in-understanding-rust/49148/6)

Thanks to [Jon G Stødle](https://users.rust-lang.org/t/twir-quote-of-the-week/328/945) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/iu3ge0/this_week_in_rust_356/)</small>
