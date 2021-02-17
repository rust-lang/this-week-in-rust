Title: This Week in Rust 378
Number: 378
Date: 2021-02-17
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

 - [Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/)

### Miscellaneous

- [Cleora - an ultra fast graph embedding tool written in Rust](https://github.com/Synerise/cleora)
- [Cost-based query optimizations in multithreaded environments](https://vertexclique.com/cost-based-query-optimizations/)

# Crate of the Week

This week's crate is [threadIO](https://crates.io/crates/thread_io), a crate that makes disk IO in a background thread easy and elegant.

Thanks to [David Andersen](https://users.rust-lang.org/t/crate-of-the-week/2704/881) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

Fuchsia has several issues available:

* [[netstack3] ARP: Add tests for ARP on a broadcast medium](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=34979)
* [[netstack3] Migrate all transport protocols to trait associated types](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=48364)
* [[netstack3] Split IpProto into Ipv4Proto and Ipv6NextHeader](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=47454)
* [[net-types] Add common prefix length calculation for IP addresses](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=47008)
* [[netstack3] Make sure ICMP messages are not sent in response to non-initial fragment packets](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=21432)
* [[internet-checksum] Clarify documentation around odd byte lengths](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=69355)
* [[netstack3] IP fragment reassembly vulnerable to FragmentSmack](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=50830)

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

*No triage report this week*

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Rust 2021 Roadmap](https://github.com/rust-lang/rfcs/pull/3037)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [adds async stream rfc](https://github.com/rust-lang/rfcs/pull/2996)
* [RFC: add the Freeze trait to libcore/libstd](https://github.com/rust-lang/rfcs/pull/2944)
* [Generic Pointer to Field](https://github.com/rust-lang/rfcs/pull/2708)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Allow leading | anywhere we allow or-patterns](https://github.com/rust-lang/rust/issues/81415)
* [disposition: merge] [`impl PartialEq<Punct> for char`; symmetry for #78636](https://github.com/rust-lang/rust/pull/80595)
* [disposition: merge] [Add an impl of Error on `Arc<impl Error>`.](https://github.com/rust-lang/rust/pull/80553)
* [disposition: merge] [Add `NotSupported` to `std::io::ErrorKind`](https://github.com/rust-lang/rust/pull/78880)
* [disposition: close] [Tracking issue for `Option::expect_none(msg)` and `unwrap_none()`](https://github.com/rust-lang/rust/issues/62633)

## New RFCs

* [Add named path bases to cargo](https://github.com/rust-lang/rfcs/pull/3074)

# Upcoming Events

### Online
* [February 11, Washington, DC, US - Let's learn to Rust nice with others - Rust DC](https://www.meetup.com/RustDC/events/275569653)
* [February 11, San Diego, CA, US - February 2021 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/276272745/)
* [February 17, Vancouver, BC, CA - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsyccdbwb/)
* [February 18, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccdbxb/)
* [February 23, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccdbfc/)


### North America
* [February 11, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccdbpb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The main theme of Rust *is not* systems programming, speed, or memory safety - it's moving runtime problems to compile time. Everything else is incidental. This is an invaluable quality of any language, and is something Rust greatly excels at.

– [/u/OS6aDohpegavod4 on /r/rust](https://www.reddit.com/r/rust/comments/leki5o/advantages_of_building_a_crud_web_application_in/gmfq2w9/)

Thanks to [Chris](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1001) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
