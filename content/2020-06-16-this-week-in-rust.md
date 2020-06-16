Title: This Week in Rust 343
Number: 343
Date: 2020-06-16
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://audio.rustacean-station.org/file/rustacean-station/twir-2020-06-16.mp3)

# Updates from Rust Community

## News & Blog Posts

* [2020 Event Lineup - Update](https://blog.rust-lang.org/2020/06/10/event-lineup-update.html)
* [New inline assembly syntax available in nightly](https://blog.rust-lang.org/inside-rust/2020/06/08/new-inline-asm.html)
* [Announcing RustFest Global 2020 🎉](https://blog.rustfest.eu/announcing-rustfest-2020)
* [RustConf 2020 Registration is Open](https://rustconf.com/)
* [Understanding the Rust Ecosystem](https://joeprevite.com/rust-lang-ecosystem)
* [You Want to Learn Rust but You Don’t Know Where to Start](https://towardsdatascience.com/you-want-to-learn-rust-but-you-dont-know-where-to-start-fc826402d5ba)
* [7 Awesome Rust-powered Command-line Utilities](https://towardsdatascience.com/awesome-rust-powered-command-line-utilities-b5359c38692)
* [Rust-Powered Command-Line Utilities to Increase Your Productivity](https://towardsdatascience.com/rust-powered-command-line-utilities-to-increase-your-productivity-eea03a4cf83a)
* [Errors in Rust: A Deep Dive](https://www.halcyon.hr/posts/error-handling-in-rust/)
* [Hyper Traps](https://vorner.github.io/2020/04/13/hyper-traps.html)
* [Getting Started With The STM32 Nucleo-F302R8 and Rust](https://blue42.net/code/rust/examples/embedded/nucleo-f30248/getting-started/post/)
* [Microsoft: Rust is the Industry's 'Best Chance' at Safe Systems Programming](https://thenewstack.io/microsoft-rust-is-the-industrys-best-chance-at-safe-systems-programming/)
* [Tour of Rust: Chapter 7 - Object Oriented Programming](https://tourofrust.com/chapter_7_en.html)
* [Rust Analyzer Changelog #29](https://rust-analyzer.github.io/thisweek/2020/06/15/changelog-29.html)
* [Rustls Security Review & Audit Report](https://github.com/ctz/rustls/blob/master/audit/TLS-01-report.pdf)
* [NDArray Index Arrays and Mask Index Arrays](https://shahinrostami.com/posts/programming/rust-notebooks/ndarray-index-arrays-and-mask-index-arrays/)
* [Two Memory Bugs From Ringbahn](https://without.boats/blog/two-memory-bugs-from-ringbahn/)
* [Dart Meets Rust: a match made in heaven](https://dev.to/sunshine-chain/dart-meets-rust-a-match-made-in-heaven-9f5)
* [C++ Developer Learning Rust!](https://dev.to/rhymu8354/c-developer-learning-rust-2oal)
* [Container with Most Water](https://dev.to/steadbytes/container-with-most-water-3ige)
* [Generics and Compile-Time in Rust](https://pingcap.com/blog/generics-and-compile-time-in-rust/)
* [Improve the performances of your Phoenix app with Rust: in both back and front](https://dev.to/scorsi/improve-the-performances-of-our-phoenix-app-with-rust-4d7a)
* [Improving the IDE for Rust/WinRT](https://kennykerr.ca/2020/06/09/improving-the-ide-for-rust-winrt/)
* [My Experiences with Rust Open Source Projects](https://javednissar.ca/getting-involved-in-rust-open-source/)
* [Playing Codenames with Rust](https://rolisz.ro/2020/06/10/playing-codenames-with-rust/)
* [Spend your novelty budget on Rust](https://tim.mcnamara.nz/post/621040767010504704/spend-your-novelty-budget-on-rust)
* [The Rust compiler isn't slow; we are.](https://blog.kodewerx.org/2020/06/the-rust-compiler-isnt-slow-we-are.html)
* [The Story of Tail Call Optimizations in Rust](https://dev.to/seanchen1991/the-story-of-tail-call-optimizations-in-rust-35hf)
* [audio] [AreWePodcastYet - Interview with Tim McNamara, author of Rust in Action](https://soundcloud.com/arewepodcastyet/awpy-05-tim-mcnamara-timclicks)
* [video] [Rust Notebooks (Jupyter and Evcxr) - Getting Started](https://www.youtube.com/watch?v=SZKEzNL9als)

# Crate of the Week

This week's crate is [safer_ffi](https://github.com/getditto/safer_ffi), a library to help write safe FFI code.

Thanks to [Vlad Frolov](https://users.rust-lang.org/t/crate-of-the-week/2704/780) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [gooseberry: Add related tags to each page](https://github.com/out-of-cheese-error/gooseberry/issues/3)
* [gooseberry: General contributing](https://github.com/out-of-cheese-error/gooseberry/blob/master/CONTRIBUTING.md)
* [rust: `fs::remove_dir_all` rarely succeeds for large directories on windows](https://github.com/rust-lang/rust/issues/29497#issuecomment-573353391)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

354 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-06-08..2020-06-15

* [enable AVR as a Tier 3 target upstream](https://github.com/rust-lang/rust/pull/69478)
* [enable LVI hardening for x86_64-fortanix-unknown-sgx](https://github.com/rust-lang/rust/pull/72655)
* [add `-Z span-debug` to allow for easier debugging of proc macros](https://github.com/rust-lang/rust/pull/72799)
* [add methods to go from a null-terminated `Vec<u8>` to a `CString`](https://github.com/rust-lang/rust/pull/73139)
* [check for live drops in constants after drop elaboration](https://github.com/rust-lang/rust/pull/71824)
* [display information about captured variable in `FnMut` error](https://github.com/rust-lang/rust/pull/72598)
* [don't create impl candidates when obligation contains errors](https://github.com/rust-lang/rust/pull/73005)
* [enforce unwind invariants](https://github.com/rust-lang/rust/pull/73133)
* [explain move errors that occur due to method calls involving `self`](https://github.com/rust-lang/rust/pull/72389)
* [fix `#[thread_local]` statics as `asm!` sym operands](https://github.com/rust-lang/rust/pull/73033)
* [fix trait alias inherent impl resolution](https://github.com/rust-lang/rust/pull/72556)
* [free `default()` forwarding to `Default::default()`](https://github.com/rust-lang/rust/pull/73001)
* [handle assembler warnings properly](https://github.com/rust-lang/rust/pull/73169)
* [on recursive ADT, provide indirection structured suggestion](https://github.com/rust-lang/rust/pull/72740)
* [provide suggestion to convert numeric op LHS rather than unwrapping RHS](https://github.com/rust-lang/rust/pull/73195)
* [querify whether a type has structural equality](https://github.com/rust-lang/rust/pull/73066)
* [relate existential associated types with variance Invariant](https://github.com/rust-lang/rust/pull/71896)
* [suggest including unused asm arguments in a comment to avoid error](https://github.com/rust-lang/rust/pull/73230)
* [support proc macros in intra doc link resolution](https://github.com/rust-lang/rust/pull/73183)
* [track span of function in method calls, and use this in `#[track_caller]`](https://github.com/rust-lang/rust/pull/73182)
* [use `min_specialization` in the remaining rustc crates](https://github.com/rust-lang/rust/pull/72707)
* [use shorthand linker strip arguments in order to support MacOS](https://github.com/rust-lang/rust/pull/73138)
* [expand: more precise locations for expansion-time lints](https://github.com/rust-lang/rust/pull/73178)
* [extend network support for HermitCore](https://github.com/rust-lang/rust/pull/73331)
* [fix caller_location intrinsic for Miri](https://github.com/rust-lang/rust/pull/73277)
* [improper ctypes: normalize return types and transparent structs](https://github.com/rust-lang/rust/pull/72890)
* [normalize adt fields during structural match checking](https://github.com/rust-lang/rust/pull/72897)
* [resolve: do not suggest imports from the same module in which we are resolving](https://github.com/rust-lang/rust/pull/72789)
* [structural_match: non-structural-match ty closures](https://github.com/rust-lang/rust/pull/73353)
* [chalk: add FnOnce trait, and provide impl for Function type](https://github.com/rust-lang/chalk/pull/494)
* [chalk: model function ABI in the Rust IR](https://github.com/rust-lang/chalk/pull/481)
* [chalk: recursive solver factoring and privacy](https://github.com/rust-lang/chalk/pull/513)
* [chalk: refactor ProgramClauseData to remove Implies variant](https://github.com/rust-lang/chalk/pull/514)
* [chalk: add `Unsize` trait implementation](https://github.com/rust-lang/chalk/pull/427)
* [miri: avoid tracking current location three times](https://github.com/rust-lang/rust/pull/72879)
* [remove `RawVec::reserve_in_place`](https://github.com/rust-lang/rust/pull/72417)
* [stabilize `Option::zip`](https://github.com/rust-lang/rust/pull/72938)
* [stabilize `vec::Drain::as_slice`](https://github.com/rust-lang/rust/pull/72584)
* [impl `AsRef<[T]>` for `vec::IntoIter<T>`](https://github.com/rust-lang/rust/pull/72583)
* [std: enable atomic.fence emission on wasm32](https://github.com/rust-lang/rust/pull/73036)
* [stdarch: fix x86 extract_epi{8,16} functions](https://github.com/rust-lang/stdarch/pull/868)
* [implement new gdb/lldb pretty-printers](https://github.com/rust-lang/rust/pull/72357)
* [cargo: add environment variables to identify the binary and crate name](https://github.com/rust-lang/cargo/pull/8270)
* [cargo: allow passing a registry index url directly to `cargo install`](https://github.com/rust-lang/cargo/pull/8344)
* [cargo: fix doctests not running with `--target=HOST`](https://github.com/rust-lang/cargo/pull/8358)
* [cargo: support `{prefix}` and `{lowerprefix}` markers in `config.json` `dl` key](https://github.com/rust-lang/cargo/pull/8267)
* [crates.io: allow configuring the application's domain name](https://github.com/rust-lang/crates.io/pull/2543)
* [crates.io: modifiers/highlight-syntax: Disable aggressive whitespace stripping](https://github.com/rust-lang/crates.io/pull/2564)
* [doc: make impl block collapsible if it has an associated constant](https://github.com/rust-lang/rust/pull/71842)
* [docs.rs: add compression for uploaded documentation](https://github.com/rust-lang/docs.rs/pull/780)
* [docs.rs: limit the size of served files](https://github.com/rust-lang/docs.rs/pull/834)
* [clippy: macro use suggestion](https://github.com/rust-lang/rust-clippy/pull/5279)
* [clippy: let_and_return: avoid "does not live long enough" errors](https://github.com/rust-lang/rust-clippy/pull/5680)
* [rustfmt: pick up comments between visibility modifier and item name](https://github.com/rust-lang/rustfmt/pull/4239)

## Rust Compiler Performance Triage

* [2020-06-16](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-06-16)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [add Windows system error codes that should map to `io::ErrorKind::TimedOut`](https://github.com/rust-lang/rust/pull/71756)
* [disposition: merge] [impl `PartialEq<Vec<B>> for &[A], &mut [A]`](https://github.com/rust-lang/rust/pull/71660)

## New RFCs

[RFC: add the Freeze trait to libcore/libstd](https://github.com/rust-lang/rfcs/pull/2944)

# Upcoming Events

### Online
* [June 18. Zurich, CH - Remote - Embedded Rust Update: probe.rs](https://www.meetup.com/Rust-Zurich/events/271020472/)
* [June 18. Turin, IT - Remote - Rust Turin Study Group](https://community.mozilla.org/events/gruppo-di-studio-di-rust-2/)
* [June 25. Edinburgh, UK - Remote - Pirrigator - Growing Tomatoes Free From Memory Errors and Race Conditions](https://www.meetup.com/rust-edi/events/271129693/)
* [June 25. Berlin, DE - Remote - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcjbhc/)

### North America
* [June 17. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcjbwb/)
* [June 18. Durham, NC - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcjbdc/)
* [June 30. Dallas, TX - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybcjbnc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

* [Software Engineer at Cloudflare, London, UK or Lisbon, PT](https://boards.greenhouse.io/cloudflare/jobs/2209495?gh_jid=2209495)

# Quote of the Week

> It feels like being part of a village that learns to love the dragon it battles.

– [turbinerneiter on Hacker News](https://news.ycombinator.com/item?id=23437950)

Thanks to [blonk](https://users.rust-lang.org/t/twir-quote-of-the-week/328/892) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/gvwvep/this_week_in_rust_341/)</small>
