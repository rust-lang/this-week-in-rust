Title: This Week in Rust 389
Number: 389
Date: 2021-05-05
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

### Observations/Thoughts

### Rust Walkthroughs

### Papers/Research Projects

### Miscellaneous

# Crate of the Week

This week's crate is [cargo-rr](https://github.com/danielzfranklin/cargo-rr), a cargo subcommand to use the time-traveling rr debugger on our code.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/905) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

350 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-04-19..2021-04-26

* [use LLVM's new saturating float-to-int intrinsics](https://github.com/rust-lang/rust/pull/84339)
* [enable sanitizers for `x86_64-unknown-linux-musl`](https://github.com/rust-lang/rust/pull/84126)
* [add coverage to `continue` statements](https://github.com/rust-lang/rust/pull/84295)
* [further split up `const_fn` feature flag](https://github.com/rust-lang/rust/pull/84310)
* [various const parameter defaults improvements](https://github.com/rust-lang/rust/pull/84299)
* [tweak trait not `use d suggestion](https://github.com/rust-lang/rust/pull/84499)
* [on stable, suggest removing `#![feature]` for features that have been stabilized](https://github.com/rust-lang/rust/pull/83722)
* [improve diagnostics for function passed when a type was expected](https://github.com/rust-lang/rust/pull/84520)
* [add suggestion to "use break" when attempting to implicit-break a loop](https://github.com/rust-lang/rust/pull/84516)
* [suggest `.as_ref()` on borrow error involving `Option`/`Result`](https://github.com/rust-lang/rust/pull/84353)
* [implement a lint that highlights all moves larger than a configured limit](https://github.com/rust-lang/rust/pull/83519)
* [introduce `CompileMonoItem` `DepNode`](https://github.com/rust-lang/rust/pull/84123)
* [cautiously add `IntoIterator` for arrays by value](https://github.com/rust-lang/rust/pull/84147)
* [stabilize `Duration::MAX`](https://github.com/rust-lang/rust/pull/84120)
* [stabilize `core::array::`{`from_ref`, `from_mut`} in 1.53.0](https://github.com/rust-lang/rust/pull/84105)
* [implement `TrustedRandomAccess` for `Take` iterator adapter](https://github.com/rust-lang/rust/pull/83990)
* [format `Struct { .. }` on one line even with `{:#?}`](https://github.com/rust-lang/rust/pull/84390)
* [added `CharIndices::offset` function](https://github.com/rust-lang/rust/pull/82585)
* [improve rebuilding behaviour of `BinaryHeap::retain`](https://github.com/rust-lang/rust/pull/78681)
* [hashbrown: add an `allocator()` getter to `HashMap` and `HashSet`](https://github.com/rust-lang/hashbrown/pull/257)
* [libz: disable forced zlib vendoring on musl](https://github.com/rust-lang/libz-sys/pull/78)
* [cargo: some changes to rustdoc fingerprint checking](https://github.com/rust-lang/cargo/pull/9404)
* [rustdoc: remove most fields from `ExternalCrate`](https://github.com/rust-lang/rust/pull/84457)
* [clippy: refactor MSRV aliases](https://github.com/rust-lang/rust-clippy/pull/7137)
* [clippy: finish MSRV for `cloned_instead_of_copied`](https://github.com/rust-lang/rust-clippy/pull/7134)
* [clippy: `manual_unwrap_or`: fix invalid code suggestion due to macro expansion](https://github.com/rust-lang/rust-clippy/pull/7136)
* [clippy: `cloned_instead_of_copied` MSRV](https://github.com/rust-lang/rust-clippy/pull/7129)
* [clippy: add `flat_map_option` lint](https://github.com/rust-lang/rust-clippy/pull/7101)
* [clippy: `unused_io_amount` detects `.read().ok()?`](https://github.com/rust-lang/rust-clippy/pull/7100)
* [clippy: add lint to check for boolean comparison in assert macro calls](https://github.com/rust-lang/rust-clippy/pull/7083)

## Rust Compiler Performance Triage

It's always nice to have a week without any regressions and 2 small improvements 🎉🎉.

Triage done by **@rylev**.
Revision range: [6df26f8..537544](https://perf.rust-lang.org/?start=6df26f897cffb2d86880544bb451c6b5f8509b2d&end=537544b1061467ee4b74ef7f552fab3d513e5caf&absolute=false&stat=instructions%3Au)

0 Regressions, 2 Improvements, 0 Mixed

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Reserved prefixes in the 2021 edition](https://github.com/rust-lang/rfcs/pull/3101)
* [disposition: postpone] [Enum variant types](https://github.com/rust-lang/rfcs/pull/2593)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add default search path to `Target::search()`](https://github.com/rust-lang/rust/pull/83800)
* [disposition: merge] [parser: Remove support for inner attributes on non-block expressions](https://github.com/rust-lang/rust/pull/83312)
* [disposition: merge] [Tracking Issue for {HashMap,BTreeMap}::into_{keys,values}](https://github.com/rust-lang/rust/issues/75294)

## New RFCs

* [Add bitfields support](https://github.com/rust-lang/rfcs/pull/3113)
* [A new prelude for the 2021 edition (trait-only edition)](https://github.com/rust-lang/rfcs/pull/3114)

# Upcoming Events

### Online
* [April 28, Online - Ockam Open Source Community Call - Live coding walkthrough of building end-to-end encrypted communication in Rust](https://github.com/ockam-network/ockam/discussions/1303)
* [May 3, 2021, Online - Cloud Native Rust Day](https://events.linuxfoundation.org/cloud-native-rust-day/)
* [May 4, 2021, Online - Cloud Native WASM Day](https://events.linuxfoundation.org/cloud-native-wasm-day/)
* [May 4, 2021, Dublin, IE - Rust Dublin May Remote Meetup - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/277860218/)
* [May 4, Buffalo, NY, US - Buffalo Rust User Group, Tues May 4th - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/277402612/)
* [May 11, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrycchbpb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> this error message is UNREAL

– [Ash 2X3 on Twitter](https://twitter.com/ash2x3/status/1384986537167892483)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1046) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
