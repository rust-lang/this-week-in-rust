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
* [Writing a Python module in Rust using PyO3](https://kushaldas.in/posts/writing-python-module-in-rust-using-pyo3.html)
* [Creating a Robust, Reusable Link-Checker](http://adventures.michaelfbryan.com/posts/linkchecker/)
* [video] [Crust of Rust: Declarative Macros](https://www.youtube.com/watch?v=q6paRBbLgNw)
* [video] [Rust Stream: String it All Together!](https://www.youtube.com/watch?v=7I11degAElQ)
* [video] [Educational Rust live coding - Building a git CLI](https://www.youtube.com/watch?v=YFzF1AHYjes)
* [video] [Maximum Sum of Subarrays (Leetcode) in Rust](https://www.youtube.com/watch?v=G1deF4Rehlw)
* [video] [WASM + Rust](https://www.youtube.com/watch?list=PLDWmoWFf46gj7htqRU1yNwM3SeaqfLKhH&v=gpaNGlka7FY&feature=emb_logo)
* [video] [Concurrency in Rust with Async/Await](https://www.youtube.com/watch?v=hrNoTZMG2MU)
* [video] [Rust and Tell Berlin Meetup - April 2020 [video]](https://www.youtube.com/watch?v=yGuxtodWYDs)
* [video] [Rust Zürisee, April: cargo crev and cargo audit](https://www.youtube.com/watch?v=_xS40wqO8GA)

# Crate of the Week

This week's crate is [WinRT-rs](https://github.com/microsoft/winrt-rs), Microsoft™'s official WinRT API for Rust.

Thanks to [JLalu](https://users.rust-lang.org/t/crate-of-the-week/2704/767) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

* [clap-rs: Implement and Derive common traits](https://github.com/clap-rs/clap/issues/952)
* [Boa has several issues marked "easy", "good first issues", and "help wanted"](https://github.com/jasonwilliams/boa/issues)
* [Tokio: Non-consume method to wait for new messages on watch channel](https://github.com/tokio-rs/tokio/issues/2404)
* [Tokio: Add is_closed to mpsc channels](https://github.com/tokio-rs/tokio/issues/2469)
* [Tokio: Unicode characters are split when writing to windows terminal](https://github.com/tokio-rs/tokio/issues/2380)
* [Tokio: select! with one branch causes a clippy error](https://github.com/tokio-rs/tokio/issues/2251)
* [Stick: Call for Participation - Test More Joysticks On Linux](https://github.com/libcala/stick/issues/5)

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

372 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-04-27..2020-05-04

* implement RFC [#2523](https://rust-lang.github.io/rfcs/2523-cfg-path-version.html), [`#[cfg(version(..))]`](https://github.com/rust-lang/rust/pull/71314)
* [have the per-query caches store the results on arenas](https://github.com/rust-lang/rust/pull/70674)
* [avoid duplicating code for each query](https://github.com/rust-lang/rust/pull/69808)
* [forbid `dyn Trait` in patterns](https://github.com/rust-lang/rust/pull/71038)
* [fix wrong argument in autoderef process](https://github.com/rust-lang/rust/pull/71627)
* [suggest `into` instead of `try_into` if possible with int types](https://github.com/rust-lang/rust/pull/71617)
* [tweak some suggestions in `rustc_resolve`](https://github.com/rust-lang/rust/pull/71438)
* [add message for resolution failure because wrong namespace](https://github.com/rust-lang/rust/pull/71419)
* [point at the return type on `.into()` failure caused by `?`](https://github.com/rust-lang/rust/pull/71409)
* [suggest `;` or assignment to drop borrows in tail exprs](https://github.com/rust-lang/rust/pull/71217)
* [on type mismatch involving associated type, suggest constraint](https://github.com/rust-lang/rust/pull/71108)
* [minimize parameter of `coerce_borrowed_pointer`](https://github.com/rust-lang/rust/pull/71524)
* [remove some `Vec` allocations to improve performance](https://github.com/rust-lang/rust/pull/71268)
* [allow `Unreachable` terminators unconditionally in const-checking](https://github.com/rust-lang/rust/pull/71691)
* [allow `Downcast` projections unconditionally in const-checking](https://github.com/rust-lang/rust/pull/71688)
* [added MIR constant propagation of Scalars into function call arguments](https://github.com/rust-lang/rust/pull/71697)
* [Miri: unleash all feature gates](https://github.com/rust-lang/rust/pull/71631)
* [use existing framework for backward dataflow analyses](https://github.com/rust-lang/rust/pull/71006)
* [add Read/Write::can_read/write_vectored](https://github.com/rust-lang/rust/pull/67841)
* [add `RefCell::take`](https://github.com/rust-lang/rust/pull/71398)
* [`slice::fill`: use `T` instead of generic arg](https://github.com/rust-lang/rust/pull/71165)
* [`Vec` `drop` and `truncate`: drop using raw slice `*mut [T]`](https://github.com/rust-lang/rust/pull/71148)
* [hashbrown: mark `RawTable::par_iter` `unsafe`](https://github.com/rust-lang/hashbrown/pull/157)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2585: FC for unsafe blocks in unsafe fn](https://github.com/rust-lang/rfcs/pull/2585)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.


### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are in final comment period this week*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for RFC 2432, "Allow `if` and `match` in constants"](https://github.com/rust-lang/rust/issues/49146)
* [disposition: merge] [Tracking issue for std::sync::Once poisoning](https://github.com/rust-lang/rust/issues/33577)

## New RFCs

* [Inline `const` expressions and patterns](https://github.com/rust-lang/rfcs/pull/2920)
* [Add the `experimental_keywords` ability](https://github.com/rust-lang/rfcs/pull/2919)
* [sigil-option-notation](https://github.com/rust-lang/rfcs/pull/2918)

# Upcoming Events

### Online

* [May 20. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybchbbc/).

### North America

* [May 6. Johannesburg, ZA - Johannesburg meetup](https://www.meetup.com/Johannesburg-Rust-Meetup)
* [May  6. Portland, OR, US - PDXRust - NES Emulation in Rust](https://www.meetup.com/PDXRust/events/269165311/).
* [May  6. Indianapolis, IN, US - Indy.rs - Rust Meetup](https://www.meetup.com/indyrs/events/dtqwprybchbjb/).
* [May 6. Atlanta, GA, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/)
* [May 11. Seattle, WA, US - Seattle Rust Meetup](http://www.meetup.com/Seattle-Rust-Meetup/)
* [May 13. Denver, CO, US Rust Boulder/Denver Monthly Meeting](https://www.meetup.com/Rust-Boulder-Denver/)
* [May 13. Vancouver, BC, CA - Vancouver Rust Meetup](https://www.meetup.com/Vancouver-Rust/events/)
* [May 14. Berlin, DE - Berlin Rust Hack and Learn](https://berline.rs/)
* [May 14. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybchbsb/).
* [May 14. Lehi, UT, US - Utah Rust Monthly Meetup](https://www.meetup.com/utahrust)
* [May 14. San Diego, CA, US - San Diego Rust](http://meetu.ps/c/2vF0G/4DXV4/a)
* [May 19. Paris, FR - Rust Paris](https://www.meetup.com/Rust-Paris)

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

> I love Rust like I love Dark Souls.  
> It's difficult, but fair. I can not praise enough the software developers that realize proper errors are vastly superior to extensive docs.

– [seph-reed on Hacker News](https://news.ycombinator.com/item?id=23032636)

Thanks to [Armando Pérez Marqués](https://users.rust-lang.org/t/twir-quote-of-the-week/328/864) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [srikwit](https://github.com/srikwit), and [nasa42](https://github.com/nasa42).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/geagy0/this_week_in_rust_337/).</small>
