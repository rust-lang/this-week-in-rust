Title: This Week in Rust 377
Number: 377
Date: 2021-02-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Newsletters

### Project/Tooling Updates

* [Launching wasm.rs: a collection of crates, a community](https://yrashk.medium.com/launching-wasm-rs-a-collection-of-crates-a-community-4344d2ba75b3)

### Observations/Thoughts

### Rust Walkthroughs
* [Async Rust: Futures, Tasks, Wakers, Oh My!](https://msarmi9.github.io/posts/async-rust/)

### Miscellaneous
* [Introducing Artichoke Ruby: A Modular Ruby implementation written in Rust](https://github.com/artichoke/artichoke/blob/21045d8f1086c669dd428a3b5bdcc4a58e13acec/ARCHITECTURE.md)

# Crate of the Week

This week's crate is [threadIO](https://crates.io/crates/thread_io), a crate that makes disk IO in a background thread easy and elegant.

Thanks to [David Andersen](https://users.rust-lang.org/t/crate-of-the-week/2704/881) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

384 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-02-01..2021-02-08

* [add AArch64 big-endian and ILP32 targets](https://github.com/rust-lang/rust/pull/81455)
* [improve handling of spans around macro result parse errors](https://github.com/rust-lang/rust/pull/81608)
* [identify unreachable subpatterns more reliably](https://github.com/rust-lang/rust/pull/80632)
* [fix issues with move closures and mutability](https://github.com/rust-lang/rust/pull/80092)
* [const_evaluatable: consider sub-expressions to be evaluatable](https://github.com/rust-lang/rust/pull/81577)
* [introduce future-compatibility warning for forbidden lint groups](https://github.com/rust-lang/rust/pull/81556)
* [`Box` the biggest `ast::ItemKind` variants](https://github.com/rust-lang/rust/pull/81405)
* [improve error message for disallowed ptr-to-int casts in const eval](https://github.com/rust-lang/rust/pull/81779)
* [typeck: emit structured suggestions for tuple struct syntax](https://github.com/rust-lang/rust/pull/81737)
* [faster few span methods](https://github.com/rust-lang/rust/pull/81735)
* [fix bug with `assert!()` calling the wrong edition of `panic!()`](https://github.com/rust-lang/rust/pull/81647)
* [make `Allocator` object-safe](https://github.com/rust-lang/rust/pull/81730)
* [add Frames Iterator for Backtrace](https://github.com/rust-lang/rust/pull/81022)
* [add `Vec::extend_from_within` method under `vec_extend_from_within` feature gate](https://github.com/rust-lang/rust/pull/79015)
* [`BTreeMap`: make `Ord` bound explicit, compile-test its absence](https://github.com/rust-lang/rust/pull/81610)
* [implement `TrustedLen` for `Fuse<I: TrustedLen>`](https://github.com/rust-lang/rust/pull/81599)
* [rename `Iterator::fold_first` to `reduce` and stabilize it](https://github.com/rust-lang/rust/pull/79805)
* [stabilize the `Wake` trait](https://github.com/rust-lang/rust/pull/74304)
* [stabilize `peekable_next_if`](https://github.com/rust-lang/rust/pull/80011)
* [stabilize poison API of `Once`, rename `poisoned()`](https://github.com/rust-lang/rust/pull/81745)
* [stabilize remaining integer methods as `const fn`](https://github.com/rust-lang/rust/pull/80962)
* [futures-rs: avoid `once_cell` in static wakers](https://github.com/rust-lang/futures-rs/pull/2332)
* [hashbrown: implement `From<HashMap<T, ()>>` for `HashSet<T>`](https://github.com/rust-lang/hashbrown/pull/235)
* [cargo: fix panic with doc collision orphan](https://github.com/rust-lang/cargo/pull/9142)
* [cargo: fix env/cfg set for `cargo test` and `cargo run`](https://github.com/rust-lang/cargo/pull/9122)
* [make rustdoc respect `--error-format short` in doctests](https://github.com/rust-lang/rust/pull/81675)
* [clippy: fix `let_underscore_drop` false positive](https://github.com/rust-lang/rust-clippy/pull/6682)
* [clippy: fix `let_and_return` false positive](https://github.com/rust-lang/rust-clippy/pull/6659)
* [clippy: don't trigger `exhaustive_structs` for structs with private fields](https://github.com/rust-lang/rust-clippy/pull/6661)
* [clippy: add new lint `missing_panics_doc`](https://github.com/rust-lang/rust-clippy/pull/6523)
* [compiletest: Add option to emit compiler stderr per bitwidth](https://github.com/rust-lang/rust/pull/81817)

## Rust Compiler Performance Triage

Another week dominated by rollups, most of which had relatively small changes
with unclear causes embedded. Overall no major changes in performance this week.

Triage done by **@simulacrum**.
Revision range: [1483e67addd37d9bd20ba3b4613b678ee9ad4d68..f6cb45ad01a4518f615926f39801996622f46179](https://perf.rust-lang.org/?start=1483e67addd37d9bd20ba3b4613b678ee9ad4d68&end=f6cb45ad01a4518f615926f39801996622f46179&absolute=false&stat=instructions%3Au)

2 Regressions, 1 Improvements, 1 Mixed

3 of them in rollups

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-02-02.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Pointer metadata & VTable](https://github.com/rust-lang/rfcs/pull/2580)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Allow leading | anywhere we allow or-patterns](https://github.com/rust-lang/rust/issues/81415)
* [disposition: merge] [libtest: allow multiple filters](https://github.com/rust-lang/rust/pull/81356)
* [disposition: merge] [Stabilize remaining integer methods as `const fn`](https://github.com/rust-lang/rust/pull/80962)
* [disposition: merge] [Add an impl of Error on `Arc<impl Error>`.](https://github.com/rust-lang/rust/pull/80553)
* [disposition: merge] [expand/resolve: Turn `#[derive]` into a regular macro attribute](https://github.com/rust-lang/rust/pull/79078)
* [disposition: merge] [Tracking Issue for `partition_point`](https://github.com/rust-lang/rust/issues/73831)

## New RFCs

* [consolidated usage of feature-name header field](https://github.com/rust-lang/rfcs/pull/3071)
* [Use more common 'tests' module name over 'test' in examples](https://github.com/rust-lang/rfcs/pull/3070)

# Upcoming Events

### Online
* [February 4, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccdbgb/)
* [February 4, Budapest, HU - Rust meetup S03! - Rust Hungary Meetup](https://www.meetup.com/Rust-Hungary-Meetup/events/275579644/)
* [February 7, Indianapolis, IN, US - Monthly Meetup - Indy.rs](https://www.meetup.com/indyrs/events/246726699/)
* [February 9, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccdbmb/)
* [February 9, Saarbücken, Saarland, DE - Meetup: 8u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/275720207/)

### North America
* [February 10, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccdbnb/)
* [February 11, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccdbpb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Backend Engineer at TrueLayer (London, UK)](https://apply.workable.com/truelayer/j/D07759DAF6/)
* [Rust Backend Engineer at TrueLayer (Milan, Italy)](https://apply.workable.com/truelayer/j/F13E839E3B/)
* [Rust Technical Lead at TrueLayer (London, UK)](https://apply.workable.com/truelayer/j/3B78A6F6F4/)
* [Rust Technical Lead at TrueLayer (Milan, Italy)](https://apply.workable.com/truelayer/j/8D8D56C09E/)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The main theme of Rust *is not* systems programming, speed, or memory safety - it's moving runtime problems to compile time. Everything else is incidental. This is an invaluable quality of any language, and is something Rust greatly excels at.

– [/u/OS6aDohpegavod4 on /r/rust](https://www.reddit.com/r/rust/comments/leki5o/advantages_of_building_a_crud_web_application_in/gmfq2w9/)

Thanks to [Chris](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1001) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
