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

* [Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/)
* [video] [Graphs in Rust: Let's Build a Maze!](https://youtu.be/UEAg4qCALb8)

### Miscellaneous

* [Cleora - an ultra fast graph embedding tool written in Rust](https://github.com/Synerise/cleora)
* [Cost-based query optimizations in multithreaded environments](https://vertexclique.com/cost-based-query-optimizations/)
* [Writing our own Cheat Engine: Exact Value scanning](https://lonami.dev/blog/woce-2/)

# Crate of the Week

Despite having no nominations, this week's crate is [firestorm](https://crates.io/crates/firestorm), a fast intrusive flamegraph profiling library.

llogiq is pretty pleased anyway with the suggestion.

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

340 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-02-01..2021-02-08

* [parser: fix panic in 'const impl' recovery](https://github.com/rust-lang/rust/pull/81876)
* [fix derived `PartialOrd` operators](https://github.com/rust-lang/rust/pull/81384)
* [borrowck: refactor visited map to a bitset](https://github.com/rust-lang/rust/pull/81132)
* [add suggest `mut` method for loop](https://github.com/rust-lang/rust/pull/81466)
* [miri: Remove non-power-of-two SIMD vectors](https://github.com/rust-lang/miri/pull/1703)
* [chalk: add Movability to Generator](https://github.com/rust-lang/chalk/pull/685)
* [try `fast_reject::simplify_type` in coherence before doing full check](https://github.com/rust-lang/rust/pull/81744)
* [fix suggestion to introduce explicit lifetime](https://github.com/rust-lang/rust/pull/81995)
* [make suggestion of changing mutability of arguments broader](https://github.com/rust-lang/rust/pull/81990)
* [optimize `Vec::retain`](https://github.com/rust-lang/rust/pull/81126)
* [make `Vec::split_at_spare_mut` public](https://github.com/rust-lang/rust/pull/81687)
* [`BTreeMap`: disentangle `Drop` implementation from `IntoIter`](https://github.com/rust-lang/rust/pull/81486)
* [initialize `BTree` nodes directly in the heap](https://github.com/rust-lang/rust/pull/81494)
* [stabilize the `partition_point` feature](https://github.com/rust-lang/rust/pull/81012)
* [add `Box::into_inner`](https://github.com/rust-lang/rust/pull/80438)
* [stdsimd: add SIMD shuffles for `SimdType`{`2`, `4`, `8`, `16`, `32`, `64`}](https://github.com/rust-lang/stdsimd/pull/62)
* [stdsimd: add bitmasks and simplify mask API](https://github.com/rust-lang/stdsimd/pull/61)
* [libtest: allow multiple filters](https://github.com/rust-lang/rust/pull/81356)
* [cargo: change git dependencies to use `HEAD` by default ](https://github.com/rust-lang/cargo/pull/9133)
* [cargo: emit warning on env variable case mismatch](https://github.com/rust-lang/cargo/pull/9169)
* [crates.io: add `COM0` and `LPT0` to the list of reserved crate names](https://github.com/rust-lang/crates.io/pull/3271) (Windows users rejoice)
* [clippy: fix suggestions that need parens in `from_iter_instead_of_collect` lint](https://github.com/rust-lang/rust-clippy/pull/6657)
* [clippy: fix `missing_panics_doc` warning on `unreachable!`](https://github.com/rust-lang/rust-clippy/pull/6700)
* [clippy: fix `vec_init_then_push` false positives](https://github.com/rust-lang/rust-clippy/pull/6697)
* [clippy: downgrade `trivial_regex` to nursery](https://github.com/rust-lang/rust-clippy/pull/6696)
* [clippy: new lint: `bytes_nth`](https://github.com/rust-lang/rust-clippy/pull/6695)

## Rust Compiler Performance Triage

A mostly quiet week, though with an excellent improvement in bootstrap times, shaving off a couple percent off the total and 10% off of rustc_middle due to changes in the code being compiled.

Triage done by @simulacrum. Revision range: [ea09825..f1c47c7](https://perf.rust-lang.org/?start=ea098255f74923d69ea234ee526df6b9cecc3b9b&end=f1c47c79fe8438ed241630f885797eebef3a6cab&absolute=false&stat=instructions%3Au)

1 Regressions, 2 Improvements, 1 Mixed

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

* [Rust Core Engineer at The Zcash Foundation (Remote)](https://www.zfnd.org/blog/opening-core-engineer/)
* [Rust Cryptography Engineer at The Zcash Foundation (Remote)](https://www.zfnd.org/blog/open-position-cryptography-engineer/)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Have you seen someone juggle several items with one hand? That's the point of async. Blocking (non-async) it like writing - it requires constant work from each hand. If you want to write twice as fast you'll need two hands and write with both at the same time. That's multithreading. If you juggle, the moment the item leaves your hand and is in the air, you have it left with nothing to do. That's similar to network IO - you make a request and are just waiting for the server to respond. You could be doing something in the meantime, like catching another item and throwing it back up again. That's what "await" does - it says I threw and item into the air, so I want my current thread / hand to switch over to catch something else now.

– [/u/OS6aDohpegavod4 on /r/rust](https://www.reddit.com/r/rust/comments/lia5fu/why_async_rust/gn2q25e/)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1002) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
