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

### Official
* [Inside] [1.47.0 pre-release testing](https://blog.rust-lang.org/inside-rust/2020/10/06/1.47.0-prerelease.html)
* [Inside] [1.47.0 second pre-release testing](https://blog.rust-lang.org/inside-rust/2020/10/06/1.47.0-prerelease-2.html)

### Newsletters
* [This Month in Rust OSDev (September 2020)](https://rust-osdev.com/this-month/2020-09/)
* [Rust in Blockchain Newsletter #16 - Secure Enclaves à la Crab](https://rustinblockchain.org/newsletters/2020-09-30-secure-enclaves-a-la-crab/)
* [This Month in Dimforge #1 (October 2020)](https://www.dimforge.com/blog/2020/10/01/this-month-in-dimforge/)

### Tooling
* [Rust Analyzer Changelog #45](https://rust-analyzer.github.io/thisweek/2020/10/05/changelog-45.html)
* [IntelliJ Rust Changelog #132](https://intellij-rust.github.io/2020/10/05/changelog-132.html)

### Observations/Thoughts
* [Why I scatter `use` statements throughout Rust code](https://tarquin-the-brave.github.io/blog/posts/rust_use_statements/)
* [Rust + Raspberry Pi Tide Clock](https://thefuntastic.com/blog/rust-tide-clock)
* [You're Allowed To Write Slow Rust Code](https://blog.jonstodle.com/youre-allowed-to-write-slow-rust-code/)
* [Rust meets the web - a clash of programming paradigms](https://www.jakobmeier.ch/blogging/Rust_on_the_Web.html)
* [Dynamic widget sketches](https://www.cmyr.net/blog/druid-dynamism.html)
* [Results of Authoring a JS Library with Rust and Wasm](https://nickb.dev/blog/results-of-authoring-a-js-library-with-rust-and-wasm)
* [Ringbahn III: A deeper dive into drivers](https://without.boats/blog/ringbahn-iii/)
* [Fast Thread Locals In Rust](https://matklad.github.io/2020/10/03/fast-thread-locals-in-rust.html)
* [Formicarium](https://gliderkite.github.io/posts/formicarium/)

### Learn Simple Rust
* [Dancing Links In Rust](https://ferrous-systems.com/blog/dlx-in-rust/)
* [Getting Started with the nRF52840 in Rust](https://nitschinger.at/Getting-Started-with-the-nRF52840-in-Rust/)
* [That's so Rust!: Smart pointers](https://dev.to/imaculate3/that-s-so-rusty-smart-pointers-245l)
* [video] [Test-Based and Graphical User Interfaces | Rust Project](https://youtu.be/dK9-oXptFcM)

### Learn More Rust
* [8 Solutions for Troubleshooting Your Rust Build Times](https://medium.com/@jondot/8-steps-for-troubleshooting-your-rust-build-times-2ffc965fd13e)
* [Benchmarking Apache Cassandra with Rust](https://pkolaczk.github.io/benchmarking-cassandra/)
* [Operating System development tutorials in Rust on the Raspberry Pi](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)
* [Learning Embedded Rust with Knurling-rs](https://ferrous-systems.com/blog/knurling-sessions-introduction/)
* [Building a Weater Station Bot](https://blog.kdubovikov.ml/articles/rust/building-a-weather-station-bot)
* [Next-Gen Rust Web Apps: Towards a Svelte Fulcro in Rust](https://blog.shortepic.com/blog/first/)
* [Next-Gen Rust Web Apps: TicTacToe in Valerie](https://blog.shortepic.com/blog/second/)
* [Next-Gen Rust Web Apps: Borrowing From Fulcro for your Rust WASM Spa](https://blog.shortepic.com/blog/third/)
* [Make A Language - Part Four: Backtracking](https://arzg.github.io/lang/4/)
* [Make A Language - Part Five: Binding Usages](https://arzg.github.io/lang/5/)
* [Make A Language - Part Six: Blocks](https://arzg.github.io/lang/6/)
* [Make A Language - Part Seven: A REPL](https://arzg.github.io/lang/7/)
* [audio] [What we've learned porting our apps to Windows - Astropad](https://astropad.com/what-weve-learned-porting-our-apps-to-windows/)
* [video] [Build a Discord Bot with Rust and Serenity](https://www.youtube.com/playlist?list=PLzIwronG0sE5lQCPFP69Ukgz4d9dngaSi)

### Project Updates
* [Headcrab: September 2020 progress report](https://headcrab.rs/2020/10/01/september-update.html)

### Miscellaneous
* [/r/rust is partnering with the Rust communities on Discord, Mozilla's Matrix, and Stack Overflow as venues for "IRC-like" real-time chat](https://www.reddit.com/r/rust/comments/j6b03y/rrust_is_partnering_with_the_rust_communities_on/)
* [[Mesa-dev] Rust drivers in Mesa](https://lists.freedesktop.org/archives/mesa-dev/2020-October/224639.html)
* [Designing a New Rust Class at Stanford: Safety in Systems Programming](https://reberhardt.com/blog/2020/10/05/designing-a-new-class-at-stanford-safety-in-systems-programming.html)
* [Memory Safety Alternative](https://github.com/finegeometer/articles/blob/master/memory-safety-alternative.md)

# Call for Blog Posts

The Rust Core Team wants input from the community!
If you haven't already, [read the official blog](https://blog.rust-lang.org/2020/09/03/Planning-2021-Roadmap.html) and submit a blog post - it will show up here!
Here are the wonderful submissions since the call for blog posts:

* [Rust 2021: APIs, ORMS and standards](https://medium.com/@v.mishra/rust-2021-apis-orms-and-standards-bfdf3ddc15a0)
* [Rust 2021 - Stability](https://jackh726.github.io/rust/2020/10/01/rust-2021.html)
* [The Rust Organization in 2021](https://tmandry.gitlab.io/blog/posts/rust-2021-organization/)
* [Rust 2021](https://nikhilism.com/post/2020/rust-2021/)
* [Rust 2021: lowering barriers](https://gist.github.com/samsieber/e976664ba333fd8a0b5f3b0ba167cb76#file-rust-2021-md)
* [Rust 2021](https://markentier.tech/posts/2020/10/rust-2021/)
* [The Rust 2021 Experience - Year of the Macro](https://casualhacks.net/blog/2020-10-05/the-rust-2021-experience-macros/)
* [The Rust Project in 2021](https://tmandry.gitlab.io/blog/posts/rust-2021-project/)

# Crate of the Week

This week's crate is [uniffi](https://github.com/mozilla/uniffi-rs), a unified ffi binding generator for Rust.

Thanks to [mark-i-m](https://users.rust-lang.org/t/crate-of-the-week/2704/823) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [dotenv-linter: Hacktoberfest Issues](https://github.com/dotenv-linter/dotenv-linter/issues?q=is%3Aopen+is%3Aissue+label%3Ahacktoberfest)
* [RustPython: [CFP] Implement the _sre module in Rust](https://github.com/RustPython/RustPython/issues/2258)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

427 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-09-28..2020-10-05

* [LLVM: handle rtcGPR64RegClassID in AArch64RegisterBankInfo::getRegBankFromRegClass()](https://github.com/rust-lang/llvm-project/pull/77)
* [fix AVR stack corruption bug](https://github.com/rust-lang/rust/pull/77441)
* [add `aarch64-unknown-linux-musl` support](https://github.com/rust-lang/rustup/pull/2493)
* [defer Apple SDKROOT detection to link time](https://github.com/rust-lang/rust/pull/77202)
* [force posix-style quoting on lld, independent of host platform](https://github.com/rust-lang/rust/pull/77543)
* [add option to pass a custom codegen backend from a driver](https://github.com/rust-lang/rust/pull/76474)
* [bypass const_item_mutation if const's type has Drop impl](https://github.com/rust-lang/rust/pull/77251)
* [clean up diagnostics for arithmetic operation errors](https://github.com/rust-lang/rust/pull/76754)
* [deduplicate and generalize some (de/)serializer impls](https://github.com/rust-lang/rust/pull/77261)
* [expand: stop normalizing `NtIdent`s before passing them to built-in macros](https://github.com/rust-lang/rust/pull/77275)
* [fix missing diagnostic span for `impl Trait` with const generics, and add various tests for `min_const_generics` and `const_generics`](https://github.com/rust-lang/rust/pull/77439)
* [improve rustdoc error for failed intra-doc link resolution](https://github.com/rust-lang/rust/pull/77469)
* [include scope id in SocketAddrV6::Display](https://github.com/rust-lang/rust/pull/77426)
* [library: forward compiler-builtins "mem" feature](https://github.com/rust-lang/rust/pull/77284)
* [liveness analysis for everybody](https://github.com/rust-lang/rust/pull/77281)
* [optimize `IntRange::from_pat`, then shrink `ParamEnv`](https://github.com/rust-lang/rust/pull/77257)
* [overhaul const-checking diagnostics](https://github.com/rust-lang/rust/pull/77354)
* [references to ZSTs may be at arbitrary aligned addresses](https://github.com/rust-lang/rust/pull/77360)
* [remove `#[rustc_allow_const_fn_ptr]` and add `#![feature(const_fn_fn_ptr_basics)]`](https://github.com/rust-lang/rust/pull/77170)
* [resolve: prohibit anon const non-static lifetimes](https://github.com/rust-lang/rust/pull/76739)
* [chalk: add static lifetime](https://github.com/rust-lang/chalk/pull/617)
* [chalk: implement generators](https://github.com/rust-lang/chalk/pull/593)
* [chalk: parse opaque types without bounds](https://github.com/rust-lang/chalk/pull/619)
* [chalk: fix assertion failure during recursive solving](https://github.com/rust-lang/chalk/pull/613)
* [chalk: support fundamental types with multiple type parameters](https://github.com/rust-lang/chalk/pull/616)
* [don't fire `const_item_mutation` lint on writes through a pointer](https://github.com/rust-lang/rust/pull/77324)
* [miri: check that all syscall arguments are scalars](https://github.com/rust-lang/miri/pull/1570)
* [add support for Miri backtraces](https://github.com/rust-lang/backtrace-rs/pull/372)
* [better error message for `async` blocks in a const-context](https://github.com/rust-lang/rust/pull/77415)
* [allow `Abort` terminators in all const-contexts](https://github.com/rust-lang/rust/pull/77512)
* [const evaluatable: improve `TooGeneric` handling](https://github.com/rust-lang/rust/pull/77303)
* [implement multiple return terminator optimization](https://github.com/rust-lang/rust/pull/74839)
* [disable the SimplifyArmIdentity mir-opt](https://github.com/rust-lang/rust/pull/77396)
* [implement Make `handle_alloc_error` default to panic (for no_std + liballoc)](https://github.com/rust-lang/rust/pull/76448)
* [change `AllocRef::by_ref` to take `&self` instead of `&mut self`](https://github.com/rust-lang/rust/pull/77289)
* [implement as_ne_bytes() for integers and floats](https://github.com/rust-lang/rust/pull/76610)
* [stabilize `slice_ptr_range`](https://github.com/rust-lang/rust/pull/77111)
* [add missing definitions required by the sparc-unknown-linux-gnu target](https://github.com/rust-lang/rust/pull/77282)
* [support vectors with fewer than 8 elements for `simd_select_bitmask`](https://github.com/rust-lang/rust/pull/77504)
* [unbox mutexes and condvars on some platforms](https://github.com/rust-lang/rust/pull/77380)
* [use futex-based `thread::park`/`unpark` on Linux](https://github.com/rust-lang/rust/pull/76919)
* [use less divisions in display u128/i128](https://github.com/rust-lang/rust/pull/76017)
* [fix `Debug` implementations of some of the `HashMap` and `BTreeMap` iterator types](https://github.com/rust-lang/rust/pull/75377)
* [add `Iterator::advancie_by` and `DoubleEndedIterator::advance_back_by`](https://github.com/rust-lang/rust/pull/76909)
* [backport LLVM apfloat commit to rustc_apfloat](https://github.com/rust-lang/rust/pull/77368)
* [cargo: fix dylib+rlib with LTO.](https://github.com/rust-lang/cargo/pull/8754)
* [uplift drop-bounds lint from clippy](https://github.com/rust-lang/rust/pull/75699)
* [clippy: add lint for inline assembly syntax style preference](https://github.com/rust-lang/rust-clippy/pull/6092)
* [clippy: lint for invisible Unicode characters other than ZWSP](https://github.com/rust-lang/rust-clippy/pull/6105)


## Rust Compiler Performance Triage

* [2020-10-05](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-05.md):
1 Regressions, 2 Improvements, 1 Mixed

A quiet week. One rather large regression on a synthetic benchmark and a few
small improvements.

[#77023](https://github.com/rust-lang/rust/issues/77023) is an interesting
case. It encoded an invariant about slice lengths as an `assume` intrinsic
inside `len` function. It seems to have caused a small compile-time slowdown,
but there was no improvement in `check` build performance (a proxy for generated
code quality). In fact, the LLVM documentation [specifically advises
against](https://llvm.org/docs/LangRef.html#llvm-assume-intrinsic) overuse of
the `assume` intrinsic in cases where the invariant is unlikely to be of much
help to the optimizer. That seems to be the case here.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-05.md) for more.

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
* [RFC: Permit _ in type aliases](https://github.com/rust-lang/rfcs/pull/2524)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize union with 'ManuallyDrop' fields and 'impl Drop for Union'](https://github.com/rust-lang/rust/pull/77547)
* [disposition: merge] [Set up CI for aarch64-apple-darwin](https://github.com/rust-lang/rust/pull/75991)
* [disposition: merge] [Promote aarch64-pc-windows-msvc to Tier 2 Development Platform](https://github.com/rust-lang/rust/pull/75914)
* [disposition: merge] [resolve: Do not put nonexistent crate `meta` into prelude](https://github.com/rust-lang/rust/pull/75802)
* [disposition: merge] [Tracking issue for slice_partition_at_index](https://github.com/rust-lang/rust/issues/55300)

## New RFCs

* [Add 0000-vecdeque-binary-search.md: Binary search fns for VecDeque](https://github.com/rust-lang/rfcs/pull/2997)

# Upcoming Events

### Online
* [October 7. Johannesburg, ZA - Johannesburg Rust Meetup - Monthly Joburg Rust Chat!](https://www.meetup.com/Johannesburg-Rust-Meetup/events/273455489/)
* [October 7. Dublin, IE - Rust Dublin - October Remote Meetup](https://www.meetup.com/Rust-Dublin/events/273014329/)
* [October 7. Indianapolis, IN, US - Indy.rs - Indy.rs - with Social Distancing](https://www.meetup.com/indyrs/events/jhfstrybcnbkb/)
* [October 8. Linz, AT - Rust Linz - Rust Meetup Linz](https://www.meetup.com/de-DE/Rust-Linz/events/271857253/)
* [October 8. San Diego, CA, US - San Diego Rust - San Diego Rust October 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/273486967/)
* [October 12 - 18. RustLab](https://www.rustlab.it/agenda)
* [October 13. Saabrücken, DE - Rust-Saar Meetup - `4u16`](https://www.meetup.com/Rust-Saar/events/273252813/)
* [Octover 15. Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrybcnbtb/)
* [October 20. Denver, CO, US - Rust Denver - Data Science with Rust](https://www.meetup.com/Rust-Boulder-Denver/events/272996842/)
* [October 21. Vancover, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/cxrtxrybcnbcc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Embedded Software Engineer](https://apply.workable.com/sense/j/ADDB5A7717/) at [Sense](https://sense.com) (Rust/C/Python, Remote or Cambridge, MA)
* [Software Engineer - Rust - IOHK (Remote)](https://iohk.io/en/careers/fk0qe6q/software-engineer-rust/)
* [Senior Software Engineer, Rust/Go & Linux - Red Canary (Remote)](https://jobs.lever.co/redcanary/51159c79-56ec-4335-8175-bf9cd72621a6)
* [Software Engineer at Let's Encrypt (Remote within US and Canada)](https://www.abetterinternet.org/careers/le-sre-sw2/)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> [...] clippy is for people who find a certain emptiness inside when they finally get code through the compiler.😉

- Unknown person answering the Rust survey

Thanks to [blonk](https://users.rust-lang.org/t/twir-quote-of-the-week/328/947) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/j71tq7/this_week_in_rust_359/)</small>
