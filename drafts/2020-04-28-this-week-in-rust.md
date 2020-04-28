Title: This Week in Rust 336
Number: 336
Date: 2020-04-28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts
* 🎈🎉 [Announcing Rust 1.43.0](https://blog.rust-lang.org/2020/04/23/Rust-1.43.0.html). 🎉🎈
* [How to speed up the Rust compiler in 2020](https://blog.mozilla.org/nnethercote/2020/04/24/how-to-speed-up-the-rust-compiler-in-2020/)
* [A Taste of WebGPU in Firefox](https://hacks.mozilla.org/2020/04/experimental-webgpu-in-firefox/)
* [Data Ingestion with Rust and AWS Lambda](http://jamesmcm.github.io/blog/2020/04/19/data-engineering-with-rust-and-aws-lambda/#en)
* [Embedded Rust pattern - Zero Sized References](https://ferrous-systems.com/blog/zero-sized-references/)
* [Implementing a linear Quadtree in Rust](https://snorrwe.onrender.com/posts/morton-table/)
* [Memory efficient serialization of tagged union](https://robinmoussu.gitlab.io/blog/post/binary_serialisation_of_enum/)
* [Unpacking Serde](https://www.reddit.com/r/rust/comments/g6ubuv/unpacking_serde_a_series_of_presentations_i_made/)
* [Rust Stream: Ownership, Closures, and Threads - Oh My!](https://www.youtube.com/watch?v=2mwwYbBRJSo)
* [Crust of Rust: Lifetime Annotations](https://docs.google.com/spreadsheets/d/15pqsOlwc2eBXNRV0GJP7Taa3NnFi5kMFpmyVerONsf8/edit#gid=853276561)
* [First impressions on Rust and Webassembly](https://deedone.github.io/posts/rust-wasm/)
* [From Rust to WebAssembly: Building an Interactive Note-Taking WebApp with Actix & Yew](https://blog.mozilla.org/nnethercote/2020/04/15/better-stack-fixing-for-firefox/)
* [Rust’s Future: Internal Execution](https://blog.knoldus.com/rusts-future-internal-execution/)
* [Rust concurrency: five easy pieces.](https://medium.com/@polyglot_factotum/rust-concurrency-five-easy-pieces-871f1c62906a).
* [(Almost) Lockless Stream Buffering](https://mcfelix.me/blog/shared-buffers/)
* [Rust and Node.js: A match made in heaven](https://blog.logrocket.com/rust-and-node-js-a-match-made-in-heaven/)

# Crate of the Week

This week's crate is [coercible_errors](https://crates.io/crates/coercible_errors), a library that allows generic trait implementations to omit the size cost of `Result::Err` if errors never happen.

Thanks to [Zac Burns](https://users.rust-lang.org/t/crate-of-the-week/2704/763) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Compiler Explorer: Bytes support for Rust](https://github.com/mattgodbolt/compiler-explorer/issues/1925).
* [rlua is looking for maintainers](https://github.com/kyren/rlua/issues/172).
* [ALMA: Add flag to install to partition instead of formatting disk](https://github.com/r-darwish/alma/issues/46).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

367 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-04-20..2020-04-27

* [don't run various MIR optimizations at mir-opt-level=0](https://github.com/rust-lang/rust/pull/70073)
* [replace thread_local with generator resume arguments in `box_region`](https://github.com/rust-lang/rust/pull/71554)
* [fix `-Zast-json` to output correct JSON form](https://github.com/rust-lang/rust/pull/71284)
* [allow wasm32 compilation of `librustc_data_structures/profiling.rs`](https://github.com/rust-lang/rust/pull/71369)
* [`proc_macro::is_available()`](https://github.com/rust-lang/rust/pull/71400)
* [proc_macro: stabilize `Span::resolved_at` and `Span::located_at`](https://github.com/rust-lang/rust/pull/69041)
* [attempt to recover perf by removing `exports_all_green`](https://github.com/rust-lang/rust/pull/71267)
* [chalk: use `FxHashMap`/`FxHashSet` and add well-formed clause for tuples](https://github.com/rust-lang/chalk/pull/411)
* [ConstProp: use a `BitSet<Local>` instead of `IndexVec<Local, bool>`](https://github.com/rust-lang/rust/pull/71312)
* [fix span of `while` (`let`) expressions after lowering](https://github.com/rust-lang/rust/pull/71494)
* [Miri Frame: use `mir::Location` to represent position in function](https://github.com/rust-lang/rust/pull/71475)
* [add `BinaryHeap::retain`](https://github.com/rust-lang/rust/pull/71485)
* [add a function to turn `Box<T>` into `Box<[T]>`](https://github.com/rust-lang/rust/pull/71421)
* [add missing `Send` and `Sync` impls for linked list `Cursor` and `CursorMut`](https://github.com/rust-lang/rust/pull/71548)
* [implement `BitOr` and `BitOrAssign` for the `NonZero` integer types](https://github.com/rust-lang/rust/pull/69813)
* [stabilize most common subset of `alloc_layout_extras`](https://github.com/rust-lang/rust/pull/69362)
* [stabilize `Span::mixed_site`](https://github.com/rust-lang/rust/pull/68716)
* [stabilize `BTreeMap::remove_entry`](https://github.com/rust-lang/rust/pull/70712)
* [futures: introduce `ready_chunks` adaptor](https://github.com/rust-lang/futures-rs/pull/2123)
* [backport to 0.1: Avoid starvation from `FuturesUnordered::poll_next`](https://github.com/rust-lang/futures-rs/pull/2122)
* [futures: add `AsyncWriteExt::write_all_vectored` utility](https://github.com/rust-lang/futures-rs/pull/1741)
* [hashbrown: future-proof specialization code](https://github.com/rust-lang/hashbrown/pull/147)
* [hashbrown: remove unsound use of specialization](https://github.com/rust-lang/hashbrown/pull/154)
* [cargo: fix warning for `resolve` mismatch in workspace](https://github.com/rust-lang/cargo/pull/8169)
* [cargo: add `resolver` opt-in for new feature resolver](https://github.com/rust-lang/cargo/pull/8129)
* [rustdoc: replace big JS dict with JSON parsing](https://github.com/rust-lang/rust/pull/71250)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2834: Cargo report future-incompat](https://github.com/rust-lang/rfcs/pull/2834).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Major change proposal process for compiler team](https://github.com/rust-lang/rfcs/pull/2904).
* [disposition: merge] [Project Groups](https://github.com/rust-lang/rfcs/pull/2856).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: close] [Tracking issue for Vec::remove_item](https://github.com/rust-lang/rust/issues/40062).
* [disposition: close] [Tracking issue for Box::into_raw_non_null](https://github.com/rust-lang/rust/issues/47336).
* [disposition: merge] [Stabilize the `#[alloc_error_handler]` attribute (for no_std + liballoc)](https://github.com/rust-lang/rust/issues/66740).
* [disposition: merge] [Make `handle_alloc_error` default to panic (for no_std + liballoc)](https://github.com/rust-lang/rust/issues/66741).
* [disposition: merge] [proc_macro: Stabilize `Span::resolved_at` and `Span::located_at`](https://github.com/rust-lang/rust/pull/69041).
* [disposition: close] [Return error for current_exe on nonexistent file](https://github.com/rust-lang/rust/pull/69557).
* [disposition: merge] [Implement BitOr and BitOrAssign for the NonZero integer types](https://github.com/rust-lang/rust/pull/69813).
* [disposition: merge] [Should enum discriminants have generics in scope](https://github.com/rust-lang/rust/issues/70453).
* [disposition: merge] [stabilize BTreeMap::remove_entry](https://github.com/rust-lang/rust/pull/70712).
* [disposition: merge] [Remove language-level UB for non-UTF-8 str](https://github.com/rust-lang/rust/issues/71033).
* [disposition: merge] [Stabilize UNICODE_VERSION (feature unicode_version)](https://github.com/rust-lang/rust/pull/71068).
* [disposition: merge] [Define UB in float-to-int casts to saturate](https://github.com/rust-lang/rust/pull/71269).

## New RFCs

* [Transition to rust-analyzer as our official LSP (Language Server Protocol) implementation](https://github.com/rust-lang/rfcs/pull/2912).
* [Destructuring assignment](https://github.com/rust-lang/rfcs/pull/2909).

# Upcoming Events

### Online

* [Apr 24. Russia - Rust online meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/269992161/) ([translation](https://youtu.be/NCE4w42hb7o))
* [Apr 30. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gztznrybcgbnc/).
* [Apr 30. Zurich, CH - Rust Zurich - Security in Rust: cargo-crev and cargo-audit](https://www.meetup.com/Rust-Zurich/events/270169298/).

### Asia Pacific

* [May  4. Auckland, NZ - Rust AKL - Rust Meetup](https://www.meetup.com/rust-akl/events/266876545/).

### North America

* [Apr 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcgbkc/).
* [Apr 28. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybcgblc/).
* [May  6. Portland, OR, US - PDXRust - NES Emulation in Rust](https://www.meetup.com/PDXRust/events/269165311/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Systems Engineer - Findora](https://jobs.lever.co/findora/88501a0d-a86d-4cd2-b0b7-8625a107b02b)
* [Software Engineer, SDK - DFINITY](https://boards.greenhouse.io/dfinity/jobs/4286745002?gh_src=0f9148372us)
* [2D Graphics Programmer - DUNGEONFOG](https://www.dungeonfog.com/about/job-offers/)
* [Senior Backend Engineer - OneSignal](https://onesignal.com/careers/9a60a245-06d9-4e2a-82fb-da5e1e9d22d8)
* [Backend Software Engineer - OneSignal](https://onesignal.com/careers/c3760d91-4320-474f-bde5-676962ecf4e7)
* [Site Reliability Engineer - OneSignal](https://onesignal.com/careers/b070a1df-d888-4af6-b8aa-2d5a55d568a1)
* [Software Engineer in Test - OneSignal](https://onesignal.com/careers/9dba316a-407f-4fd3-a084-3f2c4d232a2b)
* [IT CYBER SECURITY ENGINEER - ALEPH ALPHA GMBH](https://aleph-alpha.de/itsec_engineer.html?language=de)
* [MACHINE LEARNING ENGINEER - ALEPH ALPHA GMBH](https://aleph-alpha.de/ml_engineer.html?language=de)
* [SOFTWARE & INFRASTRUCTURE ENGINEER - ALEPH ALPHA GMBH](https://aleph-alpha.de/sw_engineer.html?language=de)
* [Rust Developer at Fusion Engineering, Delft, NL](https://fusion.engineering/job-opening-sw.pdf).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Vecs in Rust in general, are crazy fast; faster than I can replicate in C. Amazing.

– [Jonathan Eisenzopf on rust-users](https://users.rust-lang.org/t/very-fast-initialization-of-a-vec-of-vecs/41301/17)

Thanks to [Louis Cloete](https://users.rust-lang.org/t/twir-quote-of-the-week/328/857) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [nasa42](https://github.com/nasa42).*

<small>[Discuss on r/rust]().</small>
