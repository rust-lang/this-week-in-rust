Title: This Week in Rust 495
Number: 495
Date: 2023-05-17
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

### Foundation

### Newsletters
* [Shuttle Launchpad](https://www.shuttle.rs/launchpad)
* [This Month in Rust OSDev: April 2023](https://rust-osdev.com/this-month/2023-04/)

### Project/Tooling Updates
* [New in IntelliJ Rust for 2023.1 (Part 2)](https://blog.jetbrains.com/rust/2023/05/05/new-in-intellij-rust-for-2023-1-part-2/)
* [rust-analyzer changelog #181](https://rust-analyzer.github.io/thisweek/2023/05/15/changelog-181.html)
* [Trippy (Network diagnostic tool) - 0.8.0 release](https://github.com/fujiapple852/trippy/releases/tag/0.8.0)
* [Taking Rust to the Cloud: Blazingly Fast File Sharing](https://blog.orhun.dev/blazingly-fast-file-sharing/)
* [no more bit fiddling - introducing bilge](https://hecatia-elegua.github.io/blog/no-more-bit-fiddling/)

### Observations/Thoughts
* [You are holding it wrong](https://ochagavia.nl/blog/you-are-holding-it-wrong/)
* [Task scheduled time in tokio-console](https://hegdenu.net/posts/task-scheduled-time-in-console/)
* [Single Abstract Method Traits](https://mcyoung.xyz/2023/05/11/sam-closures/)
* [A locking war story](https://swatinem.de/blog/locking-war-story/)
* [Now is the time to bet big on Rust](https://tim.mcnamara.nz/post/717515899722137600/big-bet-on-rust)
* [Giving, lending, and async closures](https://smallcultfollowing.com/babysteps/blog/2023/05/09/giving-lending-and-async-closures/)
* [video] [RustNL 2023 Conference](https://www.youtube.com/watch?v=9Q4yNlbfiYk)
* [video] [Qdrant vector search in Rust | Arnaud Gourlay @ Rust Meetup Linz](https://www.youtube.com/watch?v=2cGM1fEbWJQ)
* [audio] [Glidesort with Orson Peters](https://rustacean-station.org/episode/orson-peters/)
* [audio] [smol with John Nunley](https://rustacean-station.org/episode/john-nunley/)

### Rust Walkthroughs
* [Build a simple grep CLI app in Rust](https://developerlife.com/2022/03/02/rust-grep-cli-app/)
* [video] [Topological Sort: The Hidden Gem of Graph Algorithms in Rust](https://www.youtube.com/watch?v=HS8-1Obn87M)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [Qdrant](https://github.com/qdrant/qdrant), an open source production ready vector database/similarity search engine written in Rust. There are APIs available for [Rust](https://github.com/qdrant/rust-client), Python, Javascript/Typescript and Go.

llogiq is overjoyed with his suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Add new CLI command to show available environment variables](https://github.com/build-trust/ockam/issues/4928)
* [Ockam - Update CLI documentation for `project` commands](https://github.com/build-trust/ockam/issues/4762)
* [Ockam - Allow binary messages in `ockam message send` CLI command](https://github.com/build-trust/ockam/issues/4918)
* [RustQuant - call for participants in project](https://github.com/avhz/RustQuant/issues)
* [Hyperswitch - Implement `CardsInfoInterface` for `MockDb`](https://github.com/juspay/hyperswitch/issues/1189)
* [Hyperswitch - Implement `DisputeInterface` for `MockDb`](https://github.com/juspay/hyperswitch/issues/1190)
* [Hyperswitch - Implement `EphemeralKeyInterface` for `MockDb`](https://github.com/juspay/hyperswitch/issues/1191)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

326 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-05-08..2023-05-15

* [add support for `cfg(overflow_checks)`](https://github.com/rust-lang/rust/pull/111096)
* [CFI: fix SIGILL reached via trait objects](https://github.com/rust-lang/rust/pull/111375)
* [add midpoint function for all integers and floating numbers](https://github.com/rust-lang/rust/pull/92048)
* [align unsized locals](https://github.com/rust-lang/rust/pull/111374)
* [allow mutating function args through `&raw const`](https://github.com/rust-lang/rust/pull/111517)
* [better diagnostics for `env!` where variable contains escape](https://github.com/rust-lang/rust/pull/111463)
* [better diagnostics for `impl<..> impl Trait for Type`](https://github.com/rust-lang/rust/pull/111477)
* [encode `VariantIdx` so we can decode ADT variants in the right order](https://github.com/rust-lang/rust/pull/111494)
* [encode types in SMIR](https://github.com/rust-lang/rust/pull/110747)
* [fix the `FileEncoder` buffer size](https://github.com/rust-lang/rust/pull/111088)
* [implement SSA-based reference propagation](https://github.com/rust-lang/rust/pull/106285)
* [implement builtin # syntax and use it for `offset_of!(...)`](https://github.com/rust-lang/rust/pull/110694)
* [improve suggestion for `self: Box<self>`](https://github.com/rust-lang/rust/pull/111460)
* [introduce `DynSend` and `DynSync` auto trait for parallel compiler](https://github.com/rust-lang/rust/pull/107586)
* [keep encoding attributes for closures](https://github.com/rust-lang/rust/pull/111381)
* [min specialization improvements](https://github.com/rust-lang/rust/pull/111252)
* [note user-facing types of coercion failure](https://github.com/rust-lang/rust/pull/111451)
* [only warn single-use lifetime when the binders match](https://github.com/rust-lang/rust/pull/111444)
* [require impl Trait in associated types to appear in method signatures](https://github.com/rust-lang/rust/pull/110454)
* [suggest let for possible binding with ty](https://github.com/rust-lang/rust/pull/111120)
* [support linking to rust dylib with --crate-type staticlib](https://github.com/rust-lang/rust/pull/106560)
* [switch to `EarlyBinder` for `thir_abstract_const` query](https://github.com/rust-lang/rust/pull/111410)
* [tweak "make mut" spans when assigning to locals](https://github.com/rust-lang/rust/pull/110583)
* [tweak borrow suggestion span](https://github.com/rust-lang/rust/pull/110504)
* [uplift `clippy::{drop,forget}_{ref,copy}` lints](https://github.com/rust-lang/rust/pull/109732)
* [use `visit_assign` to detect SSA locals](https://github.com/rust-lang/rust/pull/111432)
* [use by ref TokenTree iterator to avoid a few clones](https://github.com/rust-lang/rust/pull/111492)
* [use dynamic dispatch for queries](https://github.com/rust-lang/rust/pull/108638)
* [use implied bounds when checking opaque types](https://github.com/rust-lang/rust/pull/106038)
* [use proper impl self type for alias impl in rustdoc](https://github.com/rust-lang/rust/pull/111448)
* [allow MIR debuginfo to point to a variable's address](https://github.com/rust-lang/rust/pull/111440)
* [custom MIR: Support `Rvalue::CopyForDeref`](https://github.com/rust-lang/rust/pull/111587)
* [miri: add `./miri run-dep` for running a file with test dependencies available](https://github.com/rust-lang/miri/pull/2890)
* [miri: cargo-miri: fix forwarding arguments to cargo](https://github.com/rust-lang/miri/pull/2896)
* [miri: implement SIMD ptr casts](https://github.com/rust-lang/miri/pull/2894)
* [miri: support array return types in `simd_bitmask`](https://github.com/rust-lang/miri/pull/2895)
* [reduce BitSet size used in `Borrows` dataflow analysis](https://github.com/rust-lang/rust/pull/111396)
* [simplify `find_width_of_character_at_span`](https://github.com/rust-lang/rust/pull/111560)
* [simplify the implementation of iterators over slices of ZSTs](https://github.com/rust-lang/rust/pull/111395)
* [stabilize `const_ptr_read`](https://github.com/rust-lang/rust/pull/97320)
* [stabilize const `slice::split_at`](https://github.com/rust-lang/rust/pull/108273)
* [always const-evaluate the GCD in `slice::align_to_offsets`](https://github.com/rust-lang/rust/pull/111296)
* [limit lifetime of `format_args!()` with inlined args](https://github.com/rust-lang/rust/pull/110770)
* [implement `AsHandle`/`AsSocket` for `Arc`/`Rc`/`Box` on Windows](https://github.com/rust-lang/rust/pull/108196)
* [shorten lifetime of panic temporaries in `panic_fmt` case](https://github.com/rust-lang/rust/pull/104134)
* [start using `windows sys` for Windows FFI bindings in std](https://github.com/rust-lang/rust/pull/110152)
* [cargo: fix `check_for_file_and_add`'s check for conflict file](https://github.com/rust-lang/cargo/pull/12135)
* [cargo: fix dep/feat syntax with hidden implicit optional dependencies](https://github.com/rust-lang/cargo/pull/12130)
* [cargo: remove useless drop of copy type](https://github.com/rust-lang/cargo/pull/12136)
* [cargo: semver: note that it is not a breaking change to make an unsafe function safe](https://github.com/rust-lang/cargo/pull/12116)
* [clippy: `[arithmetic_side_effects]` Consider referenced allowed or hard-coded types](https://github.com/rust-lang/rust-clippy/pull/10768)
* [clippy: `needless_bool`: do not simplify code if it loses comments](https://github.com/rust-lang/rust-clippy/pull/10766)
* [clippy: add configuration options to `--explain`](https://github.com/rust-lang/rust-clippy/pull/10751)
* [clippy: add lint `manual_next_back`](https://github.com/rust-lang/rust-clippy/pull/10769)
* [clippy: don't emit `clippy::useless_conversion` on type aliases](https://github.com/rust-lang/rust-clippy/pull/10778)
* [clippy: extend `trait_duplication_in_bounds` to cover trait objects](https://github.com/rust-lang/rust-clippy/pull/10727)
* [clippy: warn on empty line outer `AttrKind::DocComment`](https://github.com/rust-lang/rust-clippy/pull/10691)
* [rust-analyzer: add basic support for `augmentsSyntaxTokens` and non-standard semantic token config](https://github.com/rust-lang/rust-analyzer/pull/14777)
* [rust-analyzer: add metrics for unevaluated constants, failed mir bodies, and failed data layouts](https://github.com/rust-lang/rust-analyzer/pull/14808)
* [rust-analyzer: expand more single ident macro calls upon item collection](https://github.com/rust-lang/rust-analyzer/pull/14800)
* [rust-analyzer: add `#[doc(alias(..))]-based` method completions](https://github.com/rust-lang/rust-analyzer/pull/14775)
* [rust-analyzer: add macro modifier for highlighting tokens in macro calls](https://github.com/rust-lang/rust-analyzer/pull/14795)
* [rust-analyzer: fix perf regression from symbol index refactor](https://github.com/rust-lang/rust-analyzer/pull/14797)
* [rust-analyzer: fix process-changes duplicating change events](https://github.com/rust-lang/rust-analyzer/pull/14801)
* [rust-analyzer: introduce macro sub-namespaces and `macro_use` prelude](https://github.com/rust-lang/rust-analyzer/pull/14781)
* [rust-analyzer: more APIs for `la_arena::IdxRange`](https://github.com/rust-lang/rust-analyzer/pull/14747)
* [rust-analyzer: remove root component from patched Windows UNC path prefix](https://github.com/rust-lang/rust-analyzer/pull/14799)
* [rust-analyzer: restructure InlayHint, no longer derive properties from its kind](https://github.com/rust-lang/rust-analyzer/pull/14794)
* [rust-analyzer: support `#[macro_use(name, ...)]`](https://github.com/rust-lang/rust-analyzer/pull/14809)

### Rust Compiler Performance Triage

The last two weeks mostly have small changes across a number of benchmarks, no
widespread large regressions or improvements.

Triage done by **@simulacrum**.
Revision range: [a368898d..3ea9ad532](https://perf.rust-lang.org/?start=a368898de758e1b8def6c9060044a5b40eb79e84&end=3ea9ad532474343426e564b997891e459cda89a6&absolute=false&stat=instructions%3Au)

6 Regressions, 3 Improvements, 4 Mixed; 2 of them in rollups
90 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-05-16.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [New rustc and Cargo options to allow path sanitisation by default](https://github.com/rust-lang/rfcs/pull/3127)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [The `#[diagnostic]` attribute namespace](https://github.com/rust-lang/rfcs/pull/3368)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Allow limited access to `OsStr` bytes](https://github.com/rust-lang/rust/pull/109698)
* [disposition: merge] [Tracking Issue for `#![feature(offset_of)]`](https://github.com/rust-lang/rust/issues/106655)
* [disposition: close] [Tracking Issue for the x86 `ud2` intrinsic](https://github.com/rust-lang/rust/issues/111193)
* [disposition: merge] [Document memory orderings of `thread::{park, unpark}`](https://github.com/rust-lang/rust/pull/99587)
* [disposition: merge] [Tracking Issue for `BuildHasher::hash_one`](https://github.com/rust-lang/rust/issues/86161)
* [disposition: merge] [Tracking Issue for #![feature(unix_chown)]](https://github.com/rust-lang/rust/issues/88989)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-05-17 - 2023-06-14 🦀

### Virtual

* 2023-05-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Atomics and Locks Book Club Chapter 2**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292847157/)
* 2023-05-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Spin and Kata Containers**](https://www.meetup.com/vancouver-rust/events/lqkkctyfchbwb/)
* 2023-05-18 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsyfchbxb/)
* 2023-05-20 | Virtual + In person (Singapore, SG) | [Web3Dev.Community](https://www.meetup.com/web3devc/)
    * [**[Hybrid] You'll Never Rust Alone - Rust Study Group**](https://www.meetup.com/web3devc/events/zcgndtyfchbbc/)
* 2023-05-23 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/293302808)
* 2023-05-25 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Practical Monads**](https://www.meetup.com/charlottesville-rust-meetup/events/293384348)
* 2023-05-25 | Virtual (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Proyecto "Taller de Rust"**](https://www.meetup.com/rust-mx/events/293332410)
* 2023-05-25 | Virtual (Karlsruhe, DE) | [The Karlsruhe Functional Programmers Meetup Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA) - various topics, from C++ to Rust**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/293349464)
* 2023-05-25 | Virtual (San Francisco, CA, US) | [Data + AI Online Meetup](https://www.meetup.com/data-ai-online/)
    * [**D3L2: Discussing Rust, Ballista, Ray SQL, DataFusion with Andy Grove**](https://www.meetup.com/data-ai-online/events/293432877)
* 2023-05-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/293014934)
* 2023-05-31 | Virtual (Chicago, IL, US) | [Chicago Healthcare Cloud Technology Community](https://www.meetup.com/chicago-healthcare-tech-and-ai/)
    * [**Rust for Mission-Critical AI: A Journey into Healthcare's Safest Language**](https://www.meetup.com/chicago-healthcare-tech-and-ai/events/293278396/?chapterContext=true&regToRsvp=true&isFromReg=true)
* 2023-06-06 | Virtual (Austin, TX, US) | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge/)
    * [**Monthly WasmEdge Community Meeting - Run Rust Warp in WasmEdge -- Alan, Poon Yong Quan**](https://www.meetup.com/webassembly-and-wasmedge/events/293014949)
* 2023-06-06 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/293485509)
* 2023-06-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/293296995)
* 2023-06-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309294)
* 2023-06-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732649)
* 2023-06-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/293014938)

### Asia

* 2023-05-18 | Seoul, KR | [Seoul Substrate Blockchain Meetup](https://www.meetup.com/seoul-substrate-blockchain-meetup/)
    * [**Seoul Substrate Meetup - 최신 cyprography - Rust**](https://www.meetup.com/seoul-substrate-blockchain-meetup/events/293016466)
* 2023-05-20 | Singapore, SG | [Web3Dev.Community](https://www.meetup.com/web3devc/)
    * [**[Hybrid] You'll Never Rust Alone - Rust Study Group**](https://www.meetup.com/web3devc/events/zcgndtyfchbbc/)
* 2023-05-25 | Amsterdam, NL | [Frontend Developer Meetup Amsterdam](https://www.meetup.com/frontend-developer-meetup-amsterdam/)
    * [**Svelte Frontend Meetup (signup required) - Building a Svelte-Rust app using Tauri**](https://www.meetup.com/frontend-developer-meetup-amsterdam/events/293272364)
* 2023-06-10 | Kuala Lumpur, MY | [GoLang Malaysia](https://t.me/golangmalaysia)
    * [**Rust Workshop/Hack and Learn Malaysia June 2023**](https://forms.gle/2fvbCG77HXCkWLfe6) | [Event updates Telegram](https://t.me/+dF46Fly4A_BjOTJl) | [Event group chat](https://t.me/golangmalaysia)

### Europe

* 2023-05-19 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfchbzb/)
* 2023-05-23 | Paris, FR | [Kaïbee](https://www.meetup.com/kaibee/)
    * [**Atelier Axum & Rust**](https://www.meetup.com/kaibee/events/293169086)
* 2023-05-24 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #4**](https://www.meetup.com/fr-FR/rust-lyon/events/293322211)
* 2023-05-25 | Barcelona, ES | [C++ Programmer Meetup.](https://www.meetup.com/c-programmer-meetup/)
    * [**Rust for C++ Developers.**](https://www.meetup.com/c-programmer-meetup/events/292816507)
* 2023-05-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #36 at Adapt Agency!**](https://www.meetup.com/copenhagen-rust-community/events/293293863)
* 2023-05-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #59**](https://www.meetup.com/rust-paris/events/293191172)
* 2023-05-30 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**10th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/293363107)
* 2023-06-08 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**Unsafe, Miri, SIMD - June Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/293322792/)

### North America

* 2023-05-17 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Too many unit tests: A tale of macros and BigDecimals**](https://www.meetup.com/rust-nyc/events/293316694)

### Oceania

* 2023-05-30 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**May Meetup**](https://www.meetup.com/rust-canberra/events/292717772/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs
<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> That's one of the great things about Rust: sometimes you can do something really dumb and get away with it.

– [Rik Arends at RustNL](https://www.youtube.com/live/9Q4yNlbfiYk?feature=share&t=1441)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1414) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
