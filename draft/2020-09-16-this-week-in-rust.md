Title: This Week in Rust 356
Number: 356
Date: 2020-09-16
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
* [Launching the 2020 State of Rust Survey](https://blog.rust-lang.org/2020/09/10/survey-launch.html)
* [A call for contributors from the WG-prioritization team](https://blog.rust-lang.org/2020/09/14/wg-prio-call-for-contributors.html)

### Tooling
* [Rust Analyzer Changelog #42](https://rust-analyzer.github.io/thisweek/2020/09/14/changelog-42.html)

### Observations/Thoughts
* [Which Parsing Approach?](https://tratt.net/laurie/blog/entries/which_parsing_approach.html)
* [Challenging LR Parsing](https://rust-analyzer.github.io//blog/2020/09/16/challeging-LR-parsing.html)
* [What I Learnt from Benchmarking Http4k, Ktor (Kotlin) and Actix v2, v3 (Rust) Microservices](https://matej.laitl.cz/bench-rust-kotlin-microservices/)
* [No Namespaces in Rust is a Feature](https://samsieber.tech/posts/2020/09/registry-structure-influence/)
* [Building devserver: An Ultra-Tiny Rust Server](https://ianjk.com/devserver/)
* [Taming nalgebra's Rustdoc](https://jack.wrenn.fyi/blog/rustdocing-nalgebra/)

### Learn Standard Rust
* [Collecting Data from an API](https://davidmaceachern.com/posts/collecting-data-from-an-api)
* [SQLite File Parser Pt. 1: The Header](https://wiredforge.com/blog/sqlite-parser-pt-1/index.html)
* [An introduction to Data Oriented Design with Rust](https://jamesmcm.github.io/blog/2020/07/25/intro-dod/)
* [Rust for Java developers](https://blog.codecentric.de/en/2020/09/rust-for-java-developers/)
* [Oxidizing portals with zbus](https://belmoussaoui.com/article/13-oxidizing-portals)

### Learn More Rust
* [Implementing Records in x7](https://dpbriggs.ca/blog/Implementing-Method-Calls-In-x7)
* [Let's build a single binary gRPC server-client with Rust in 2020 - Part 4](https://dev.to/tjtelan/let-s-build-a-single-binary-grpc-server-client-with-rust-in-2020-part-4-3k9f)
* [Make A Language - Part Three: Defining variables](https://arzg.github.io/lang/3/)
* [Rust HTTP Testing with httpmock](https://alexliesenfeld.com/posts/2020/rust-http-testing-with-httpmock/)

### Project Updates
* [Announcing the CCS811 indoor air quality sensor driver](https://blog.eldruin.com/ccs811-indoor-air-quality-sensor-driver-in-rust/).
* [OpenPGP in Rust: the Sequoia project](https://lwn.net/SubscriberLink/830902/b751810a99460a39/)
* [AssemblyLift v0.2 preview: RPC-based IO modules](https://dev.to/dotxlem/assemblylift-v0-2-preview-rpc-based-io-modules-2d38)
* [Announcing Actix Web v3.0](https://paper.dropbox.com/published/Announcing-Actix-Web-v3.0-QOXXb1lXgTubzXHzUq9ONY5)

### Miscellaneous
* [Your Language Sucks, It Doesn't Matter](https://matklad.github.io//2020/09/13/your-language-sucks.html)
* [Rust is #18 in the TIOBE Index for September 2020](https://www.tiobe.com/tiobe-index/)
* [A Few Github Action "Recipes" for Rust](https://shift.click/blog/github-actions-rust/)
* [Writing an x86 bootloader in Rust that can launch vmlinux](https://vmm.dev/en/rust/krabs.md)
* [video] [GOTO 2020 - Next-Generation Programming: Rust & Elm - Richard Feldman](https://youtu.be/ukVqQGbxM9A)

# Call for Blog Posts

The Rust Core Team wants input from the community!
If you haven't already, [read the official blog](https://blog.rust-lang.org/2020/09/03/Planning-2021-Roadmap.html) and submit a blog post - it will show up here!
Here are the wonderful submissions since the call for blog posts:

* [Rust in 2021](https://matklad.github.io/2020/09/12/rust-in-2021.html)
* [Rust 2021](https://hsivonen.fi/rust2021/)

# Crate of the Week

This week's crate is [gitoxide](https://github.com/Byron/gitoxide), an idiomatic, modern, lean, fast, safe & pure Rust implementation of git.

Thanks again to [Vlad Frolov](https://users.rust-lang.org/t/crate-of-the-week/2704/812) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

336 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-09-07..2020-09-14

* [add rust-dev component to support rustc development](https://github.com/rust-lang/rust/pull/76332)
* [properly encode spans with a dummy location and non-root `SyntaxContext`](https://github.com/rust-lang/rust/pull/76658)
* [add `const_item_mutation` lint](https://github.com/rust-lang/rust/pull/75573)
* [more structured suggestions for boxed trait objects instead of impl Trait on non-coerceable tail expressions](https://github.com/rust-lang/rust/pull/75608)
* [add help note when using type in place of const](https://github.com/rust-lang/rust/pull/75611)
* [do not promote `&mut` of a non-ZST ever](https://github.com/rust-lang/rust/pull/75585)
* [chalk: simplify lowering](https://github.com/rust-lang/chalk/pull/602)
* [inliner: emit storage markers for introduced arg temporaries](https://github.com/rust-lang/rust/pull/76123)
* [enable the `SimplifyArmIdentity` MIR optimization at `mir-opt-level=1`](https://github.com/rust-lang/rust/pull/76308)
* [stabilize `doc_alias`](https://github.com/rust-lang/rust/pull/75740)
* [stabilize `core::future::`{`pending`,`ready`}](https://github.com/rust-lang/rust/pull/74328)
* [add saturating methods for `Duration`](https://github.com/rust-lang/rust/pull/76114)
* [add `slice::array_chunks_mut`](https://github.com/rust-lang/rust/pull/75021)
* [eliminate mut reference UB in `Drop` impl for `Rc<T>`](https://github.com/rust-lang/rust/pull/76530)
* [`BTreeMap` mutable iterators should not take any reference to visited nodes during iteration](https://github.com/rust-lang/rust/pull/73971)
* [`BTreeMap`: move up reference to map's root from `NodeRef`](https://github.com/rust-lang/rust/pull/74437)
* [add `drain_filter` method to `HashMap` and `HashSet`](https://github.com/rust-lang/rust/pull/76458)
* [arch: AVX512F](https://github.com/rust-lang/stdarch/pull/896)
* [add `MaybeUninit::assume_init_drop`](https://github.com/rust-lang/rust/pull/76484)
* [remove internal and unstable `MaybeUninit::UNINIT`](https://github.com/rust-lang/rust/pull/76527)
* [cargo: fix non-determinism with new feature resolver](https://github.com/rust-lang/cargo/pull/8701)

## Rust Compiler Performance Triage

* [2020-09-15](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-09-15.md):
  0 regressions, 2 improvements.

A few small compile-time regressions this week. The first was
[#70793](https://github.com/rust-lang/rust/pull/70793), which added some
specializations to the standard library in order to increase runtime
performance. The second was
[#73996](https://github.com/rust-lang/rust/pull/73996), which adds an option to
the diagnostics code to print only the names of types and traits when they are
unique instead of the whole path. The third was
[#75200](https://github.com/rust-lang/rust/pull/75200), which refactored part
of `BTreeMap` to avoid aliasing mutable references.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [[RFC]: Portable SIMD Libs Project Group](https://github.com/rust-lang/rfcs/pull/2977)
* [Establish a new error handling project group](https://github.com/rust-lang/rfcs/pull/2965)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)
* [disposition: merge][Add `[T; N]: TryFrom<Vec<T>>` (insta-stable)](https://github.com/rust-lang/rust/pull/76310)
* [disposition: merge][Make some Ordering methods const](https://github.com/rust-lang/rust/pull/76198)
* [disposition: merge][Stabilize some Result methods as const](https://github.com/rust-lang/rust/pull/76136)
* [disposition: merge][Stabilize some Option methods as const](https://github.com/rust-lang/rust/pull/76135)
* [disposition: merge][Use implicit (not explicit) rules for promotability by default in `const fn`](https://github.com/rust-lang/rust/pull/75502)
* [disposition: merge][Implement `Index` and `IndexMut` for arrays](https://github.com/rust-lang/rust/pull/74989)
* [disposition: merge][Tracking issue for `#[doc(alias = "...")]`](https://github.com/rust-lang/rust/issues/50146)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [September 9. East Coast, US - Rust East Coast Virtual Talks + Q&A](https://www.meetup.com/Rust-NYC/events/272982073/)
* [September 11. Russia - Russian Rust Online Meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/272798484/)
* [September 16. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/gbzjxrybcmbvb/)
* [September 17. Linz, AT - Rust Linz - Rust Meetup Linz - Meaghan Lewis on Rust, Embedded Rust with Roland Ruckerbauer](https://www.meetup.com/de-DE/Rust-Linz/events/271857244/)
* [September 17. Berlin, DE - Berline.rs - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcmbwb/)

### North America
* [September 9. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybcmbmb/)
* [September 10. Lehi, UT, US - Utah Rust - The Blue Pill: Rust on Microcontrollers (Sept / Second Round)](https://www.meetup.com/utah-rust/events/268567961/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> When you have a lifetime `<'a>` on a struct, that lifetime denotes references to values stored *outside* of the struct. If you try to store a reference that points inside the struct rather than outside, you will run into a compiler error when the compiler notices you **lied** to it.

- [Alice Ryhl on rust-users](https://users.rust-lang.org/t/how-to-resolve-error-e0499-cannot-borrow-as-mutable-more-than-once-at-a-time-in-this-case/48815/3)

Thanks to [Tom Phinney](https://users.rust-lang.org/t/twir-quote-of-the-week/328/939) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/imoogj/this_week_in_rust_354/)</small>
