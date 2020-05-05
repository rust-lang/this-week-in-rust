Title: This Week in Rust 337
Number: 337
Date: 2020-05-05
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

* [Async interviews: my take thus far](https://smallcultfollowing.com/babysteps/blog/2020/04/30/async-interviews-my-take-thus-far/)
* [Multi-threaded HTTP/WebSocket server in Rust](https://sergey-melnychuk.github.io/2020/04/27/multi-threaded-http-websocket-server-in-rust/).
* [Cheating Rank-n with Traits](https://leshow.github.io/post/cheat_rank_n/)
* [Understanding Memory and Thread Safety Practices
and Issues in Real-World Rust Programs](https://cseweb.ucsd.edu/~yiying/RustStudy-PLDI20.pdf)
* [A beginners guide to gRPC with Rust](https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o)
* [Setup Anaconda, Jupyter, and Rust](https://shahinrostami.com/posts/programming/rust-notebooks/setup-anaconda-jupyter-and-rust/)
* [Teleforking a process onto a different computer!](https://thume.ca/2020/04/18/telefork-forking-a-process-onto-a-different-computer/)
* [Stubbing out WASI manually in Rust](http://www.jakubkonka.com/2020/04/28/rust-wasi-from-scratch.html)
* [Rust Analyzer - Changelog #22](https://rust-analyzer.github.io/thisweek/2020/04/27/changelog-22.html)
* [IntelliJ Rust changelog #121](https://intellij-rust.github.io/2020/04/27/changelog-121.html)
* [Type-level programming in Rust](http://willcrichton.net/notes/type-level-programming/)
* [Rust at FullStory, Part 2: A Look Inside Our Mobile SDK](https://bionic.fullstory.com/rust-at-fullstory-part-2/)
* [Notes on Parsing in Rust](https://blog.wesleyac.com/posts/rust-parsing)
* [Create a blazingly fast REST API in Rust (Part 1/2)](https://docs.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/)
* [The Safety Boat: Kubernetes and Rust](https://msrc-blog.microsoft.com/2020/04/29/the-safety-boat-kubernetes-and-rust/)
* [From Pratt to Dijkstra](https://matklad.github.io//2020/04/15/from-pratt-to-dijkstra.html)
* [Learning embedded Rust by building RISC-V-powered robot - Part 4](https://matklad.github.io//2020/04/15/from-pratt-to-dijkstra.html)
* [How to write CRaP Rust Code](https://blog.logrocket.com/how-to-write-crap-rust-code/)
* [Rust concurrency: a streaming workflow, served with a side of back-pressure.](https://medium.com/@polyglot_factotum/rust-concurrency-a-streaming-workflow-served-with-a-side-of-back-pressure-955bdf0266b5)
* [video] [Crust of Rust: Declarative Macros](https://www.youtube.com/watch?v=q6paRBbLgNw)
* [video] [Rust Stream: String it All Together!](https://www.youtube.com/watch?v=7I11degAElQ)
* [video] [Educational Rust live coding - Building a git CLI](https://www.youtube.com/watch?v=YFzF1AHYjes)
* [video] [Maximum Sum of Subarrays (Leetcode) in Rust](https://www.youtube.com/watch?v=G1deF4Rehlw)
* [video] [WASM + Rust](https://www.youtube.com/watch?list=PLDWmoWFf46gj7htqRU1yNwM3SeaqfLKhH&v=gpaNGlka7FY&feature=emb_logo)
* [video] [Concurrency in Rust with Async/Await](https://www.youtube.com/watch?v=hrNoTZMG2MU)
* [video] [Rust and Tell Berlin Meetup - April 2020 [video]](https://www.youtube.com/watch?v=yGuxtodWYDs)
* [video] [Rust Zürisee, April: cargo crev and cargo audit](https://www.youtube.com/watch?v=_xS40wqO8GA)

# Crate of the Week

This week's crate is [coercible_errors](https://crates.io/crates/coercible_errors), a library that allows generic trait implementations to omit the size cost of `Result::Err` if errors never happen.

Thanks to [Zac Burns](https://users.rust-lang.org/t/crate-of-the-week/2704/763) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

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

* [RFC 2836: Introduce the ASM project group](https://github.com/rust-lang/rfcs/pull/2836).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Project Groups](https://github.com/rust-lang/rfcs/pull/2856).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize the `#[alloc_error_handler]` attribute (for no_std + liballoc)](https://github.com/rust-lang/rust/issues/66740).
* [disposition: merge] [Make `handle_alloc_error` default to panic (for no_std + liballoc)](https://github.com/rust-lang/rust/issues/66741).
* [disposition: merge] [Remove language-level UB for non-UTF-8 str](https://github.com/rust-lang/rust/issues/71033).
* [disposition: merge] [Define UB in float-to-int casts to saturate](https://github.com/rust-lang/rust/pull/71269).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online

* [May 20. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybchbbc/).

### North America

* [May  6. Portland, OR, US - PDXRust - NES Emulation in Rust](https://www.meetup.com/PDXRust/events/269165311/).
* [May  6. Indianapolis, IN, US - Indy.rs - Rust Meetup](https://www.meetup.com/indyrs/events/dtqwprybchbjb/).
* [May 15. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybchbsb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs
* [Backend Engineer, Data Processing - Kraken - Remote](https://jobs.lever.co/kraken/246f7fd2-000a-4f61-8f53-b1cc783d51cb)
* [Backend Engineer, Futures - Kraken - Remote](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Sr. Software Engineer - Distributed Systems - DFINITY - Palo Alto, CA or San Francisco, CA](https://boards.greenhouse.io/dfinity/jobs/4408999002)
* [Sr. Software Engineer - Distributed Systems - DFINITY - Zurich, CH](https://boards.greenhouse.io/dfinity/jobs/4409033002)
* [Sr. Software Engineer - Infrastructure and Tools - DFINITY - Palo Alto, CA or San Francisco, CA](https://boards.greenhouse.io/dfinity/jobs/4473085002)
* [Sr. Software Engineer - Networking - DFINITY - Palo Alto, CA or San Francisco, CA](https://boards.greenhouse.io/dfinity/jobs/4408980002)
* [Sr. Software Engineer - Systems - DFINITY - Palo Alto, CA or San Francisco, CA](https://boards.greenhouse.io/dfinity/jobs/4408974002)
* [Sr. Software Engineer - Systems - DFINITY - Zurich, CH](https://boards.greenhouse.io/dfinity/jobs/4408981002)
* [Software Engineer - SDK - DFINITY - Palo Alto, CA or San Francisco, CA](https://boards.greenhouse.io/dfinity/jobs/4286745002)
* [Software Architect - Applied Crytography - Ockam - Remote or San Francisco](https://www.ockam.io/team/Software-Architect-Applied-Cryptography-in-Rust/61e07e82-0589-51de-b250-42dbceb31c3c)
* [Senior Software Development Engineer - AWS Rust SDK - Amazon - Remote USA](https://www.amazon.jobs/en/jobs/1124901/senior-software-development-engineer-aws-rust-sdk)
* [Software Development Engineer - AWS Lambda - Amazon - Seattle](https://amazon.jobs/en/jobs/1104420/software-development-engineer-aws-lambda)
* [Sr. Software Engineer Rust/Go - Equilibrium](https://www.notion.so/Hiring-Senior-Software-Engineer-Rust-Go-e6c94ccc261f426c80a483c7fc642412)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Vecs in Rust in general, are crazy fast; faster than I can replicate in C. Amazing.

– [Jonathan Eisenzopf on rust-users](https://users.rust-lang.org/t/very-fast-initialization-of-a-vec-of-vecs/41301/17)

Thanks to [Louis Cloete](https://users.rust-lang.org/t/twir-quote-of-the-week/328/857) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [srikwit](https://github.com/srikwit), and [nasa42](https://github.com/nasa42).*

<small>[Discuss on r/rust]().</small>
