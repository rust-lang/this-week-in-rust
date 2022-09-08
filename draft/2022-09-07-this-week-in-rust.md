Title: This Week in Rust 459
Number: 459
Date: 2022-09-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Foundation
* [Hello JFrog!](https://foundation.rust-lang.org/news/2022-09-06-hello-jfrog/)

### Newsletters
* [This Month in Rust OSDev: August 2022](https://rust-osdev.com/this-month/2022-08/)

### Project/Tooling Updates
* [Announcing Unreal Rust](https://maikklein.github.io/unreal-rust-1/)
* [Introducing `cxx-async`](https://pcwalton.github.io/_posts/2022-08-19-introducing-cxx-async.html)
* [Arti 1.0.0 is released: Our Rust Tor implementation is ready for production use](https://blog.torproject.org/arti_100_released/)
* [rust-analyzer changelog #145](https://rust-analyzer.github.io/thisweek/2022/09/05/changelog-145.html)
* [rustc_codegen_gcc: Progress Report #15](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-15)
* [Lapce editor 0.2.0 released](https://github.com/lapce/lapce/releases/tag/v0.2.0)
* [This week in Databend #58: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-09-07-databend-weekly/)
* [HexoSynth 2022 - Devlog #11: VST3/CLAP Plugin Integration](https://m8geil.de/posts/hexosynth-11/)
* [Fornjot (code-first CAD in Rust) - Weekly Release - 2022-W36](https://www.fornjot.app/blog/weekly-release/2022-w36/)
* [One year of Relm4](https://relm4.org/blog/posts/one_year_of_relm4/)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-09-05.html)

### Observations/Thoughts
* [How to Use Rust Traits, Generics and Bounds](https://rsdlt.github.io/posts/rust-trait-super-generic-polymorphism-subtrait-supertrait-bounds/)
* [Wasmtime 1.0: A Look at Performance](https://bytecodealliance.org/articles/wasmtime-10-performance)
* [Understanding primitive data types in Rust](https://blog.logrocket.com/understanding-primitive-data-types-rust/)
* [video] [RustConf 2022 Talks](https://www.youtube.com/playlist?list=PL85XCvVPmGQhXeH3QiYct6eMLH1un1dcu)
* [video] [Rust Education Workshop 2022](https://www.youtube.com/playlist?list=PL85XCvVPmGQhVCcPkmgbMUfJv4iGVixj-)
* [audio] [Fyrox with Dmitry Stepanov](https://rustacean-station.org/episode/dmitry-stepanov/)

### Rust Walkthroughs
* [Filtering a Vector with SIMD Instructions (AVX-2 and AVX-512)](https://quickwit.io/blog/filtering%20a%20vector%20with%20simd%20instructions%20avx-2%20and%20avx-512/)
* [Learning Rust: Async Combinators](https://kerkour.com/rust-async-combinators)
* [Solving The Witness with Z3 (Part 2)](https://www.techofnote.com/witness-part-2)
* [How to build a (simple) blog using Rust](https://mortenvistisen.com/posts/how-to-build-a-simple-blog-using-rust)
* [STM32F4 Embedded Rust at the HAL: The RTIC Framework](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-hal-the-rtic-framework)
* [video] [Nine Rules for Elegant Rust Library APIs](https://www.youtube.com/watch?v=6-8-9ZV-2WQ)

### Miscellaneous
* [DE] [Security: FreeBSD-Kernel bekommt experimentelle Rust-Patches](https://www.golem.de/news/security-freebsd-kernel-bekommt-experimentelle-rust-patches-2209-168027.html)

* [Random File Lines and Iterator Items: Understanding Reservoir Sampling with Rust](https://towardsdatascience.com/interview-question-select-a-random-line-from-a-file-in-rust-c0a8cddcddfb)

## Crate of the Week

This week's crate is [sql-query-builder](https://crates.io/crates/sql_query_builder), a library to write SQL queries in a simple and composable way.

Thanks to [Belchior Oliveira](https://users.rust-lang.org/t/crate-of-the-week/2704/1102) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Show ockam command help in $PAGER or less (clap based)](https://github.com/build-trust/ockam/issues/3434)
* [Ockam - Detect new release is available and display upgrade instructions in ockam command](https://github.com/build-trust/ockam/issues/3379)
* [Ockam - Remove the --node argument from ockam enroll clap command](https://github.com/build-trust/ockam/issues/3410)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

417 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-08-29..2022-09-05

* [add tier-3 support for powerpc64 and riscv64 openbsd](https://github.com/rust-lang/rust/pull/101025)
* [support `#[unix_sigpipe = "inherit|sig_dfl"]` on `fn main()` to prevent ignoring `SIGPIPE`](https://github.com/rust-lang/rust/pull/97802)
* [proc\_macro/bridge: send diagnostics over the bridge as a struct](https://github.com/rust-lang/rust/pull/100210)
* [proc\_macro/bridge: use the cross-thread executor for nested proc-macros](https://github.com/rust-lang/rust/pull/101414)
* [do not leak type variables from opaque type relation](https://github.com/rust-lang/rust/pull/99928)
* [attempt to normalize `FnDef` signature in `InferCtxt::cmp`](https://github.com/rust-lang/rust/pull/100473)
* [do not report too many expr field candidates](https://github.com/rust-lang/rust/pull/100898)
* [do not suggest adding `move` to closure when `move` is already used](https://github.com/rust-lang/rust/pull/101285)
* [don't suggest reborrow if usage is inside a closure](https://github.com/rust-lang/rust/pull/101429)
* [suggest `{Option,Result}::{copied,clone}()` to satisfy type mismatch](https://github.com/rust-lang/rust/pull/101367)
* [suggest associated method on deref types when path syntax method fails](https://github.com/rust-lang/rust/pull/100302)
* [suggest moving redundant generic args of an assoc fn to its trait](https://github.com/rust-lang/rust/pull/100838)
* [suggest returning closure as `impl Fn`](https://github.com/rust-lang/rust/pull/101019)
* [add `special_module_name` lint](https://github.com/rust-lang/rust/pull/94467)
* [uplift the `let_underscore` lints from clippy into rustc](https://github.com/rust-lang/rust/pull/97739)
* [strengthen `invalid_value` lint to forbid uninit primitives, adjust docs to say that's UB](https://github.com/rust-lang/rust/pull/98919)
* [forbid mixing `System` with direct sytem allocator calls](https://github.com/rust-lang/rust/pull/101394)
* [use head span for `rustc_on_unimplemented`'s `enclosing_scope` attr](https://github.com/rust-lang/rust/pull/101296)
* [make call suggestions more general and more accurate](https://github.com/rust-lang/rust/pull/101100)
* [make trait bound not satisfied specify kind](https://github.com/rust-lang/rust/pull/100647)
* [miri: adjust for supporting more implicit ptr-to-int transmutation](https://github.com/rust-lang/miri/pull/2516)
* [miri: re-enable FFI support](https://github.com/rust-lang/miri/pull/2529)
* [allow deriving multipart suggestions](https://github.com/rust-lang/rust/pull/100970)
* [replace `rustc_data_structures::thin_vec::ThinVec` with `thin_vec::ThinVec`](https://github.com/rust-lang/rust/pull/100869)
* [separate the receiver from arguments in HIR](https://github.com/rust-lang/rust/pull/101261)
* [shrink `thir::Pat`](https://github.com/rust-lang/rust/pull/101139)
* [shrink suggestion span of argument mismatch error](https://github.com/rust-lang/rust/pull/101364)
* [simplify `hir::PathSegment`](https://github.com/rust-lang/rust/pull/101228)
* [interpret: fix unnecessary allocation in validation visitor](https://github.com/rust-lang/rust/pull/101154)
* [more `clippy::perf` fixes](https://github.com/rust-lang/rust/pull/101391)
* [optimization of access level table construction](https://github.com/rust-lang/rust/pull/100147)
* [migrate `rustc_session` to use `SessionDiagnostic` - Pt. 1](https://github.com/rust-lang/rust/pull/100753)
* [migrate `rustc_metadata` to `SessionDiagnostics`](https://github.com/rust-lang/rust/pull/100928)
* [migrate `rustc_monomorphize` to use `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100730)
* [porting 'compiler/rustc_trait_selection' to translatable diagnostics - Part 1](https://github.com/rust-lang/rust/pull/100814)
* [fix UB from misalignment and provenance widening in `std::sys::windows`](https://github.com/rust-lang/rust/pull/101171)
* [avoid needless buffer zeroing in `std::sys::windows::fs`](https://github.com/rust-lang/rust/pull/101236)
* [avoid zeroing large stack buffers in stdio on Windows](https://github.com/rust-lang/rust/pull/101193)
* [add `AsFd` implementations for stdio types on WASI](https://github.com/rust-lang/rust/pull/100892)
* [implement internal `IsZero` for Wrapping and Saturating for `Vec` optimizations](https://github.com/rust-lang/rust/pull/93455)
* [add `vec::Drain{,Filter}::keep_rest`](https://github.com/rust-lang/rust/pull/95376)
* [fix `into_iter` on ZST](https://github.com/rust-lang/rust/pull/101237)
* [provider API: add additional methods to the `Demand` type](https://github.com/rust-lang/rust/pull/99583)
* [make `ReentrantMutex` movable and `const`](https://github.com/rust-lang/rust/pull/100576)
* [make `char::is_lowercase` and `char::is_uppercase` const](https://github.com/rust-lang/rust/pull/101401)
* [make `const_eval_select` a real intrinsic](https://github.com/rust-lang/rust/pull/100759)
* [hashbrown: `Equivalent` trait](https://github.com/rust-lang/hashbrown/pull/350)
* [cargo: rework test error handling](https://github.com/rust-lang/cargo/pull/11028)
* [clippy: add `--explain` subcommand](https://github.com/rust-lang/rust-clippy/pull/8952)
* [clippy: new lint `bool_to_int_with_if`](https://github.com/rust-lang/rust-clippy/pull/9412)
* [clippy: initial implementation `result_large_err`](https://github.com/rust-lang/rust-clippy/pull/9373)
* [clippy: don't use `hir_ty_to_ty` in `result_large_err`](https://github.com/rust-lang/rust-clippy/pull/9417)
* [clippy: use `approx_ty_size` for `large_enum_variant`](https://github.com/rust-lang/rust-clippy/pull/9400)
* [clippy: fix `mut_mutex_lock` when mutex is behind immutable deref](https://github.com/rust-lang/rust-clippy/pull/9418)
* [clippy: fix `suboptimal_float` not linting on `{const}.powf({const})`](https://github.com/rust-lang/rust-clippy/pull/9404)
* [clippy: fix `unnecessary_to_owned` false positive](https://github.com/rust-lang/rust-clippy/pull/9424)
* [clippy: fix missing parens in `suboptimal_flops` suggestion](https://github.com/rust-lang/rust-clippy/pull/9394)
* [clippy: fix the emission order of `trait_duplication_in_bounds`](https://github.com/rust-lang/rust-clippy/pull/9397)
* [clippy: suggest `Entry::or_default` for `Entry::or_insert(Default::default())`](https://github.com/rust-lang/rust-clippy/pull/9342)
* [rust-analyzer: clarify the state of (extern) preludes for block def maps](https://github.com/rust-lang/rust-analyzer/pull/13175)
* [rust-analyzer: don't store `SyntheticSyntax` in the reverse maps in `BodySourceMap`](https://github.com/rust-lang/rust-analyzer/pull/13173)
* [rust-analyzer: drop the expander borrow in all control flow paths](https://github.com/rust-lang/rust-analyzer/pull/13154)
* [rust-analyzer: fix nested break expressions, expecting unknown types](https://github.com/rust-lang/rust-analyzer/pull/13183)
* [rust-analyzer: highlight namerefs by syntax until proc-macros have been loaded](https://github.com/rust-lang/rust-analyzer/pull/13134)
* [rust-analyzer: lift out the module scope into a field in the resolver](https://github.com/rust-lang/rust-analyzer/pull/13174)
* [rust-analyzer: prefer the type of expression in "Replace turbofish with type"](https://github.com/rust-lang/rust-analyzer/pull/13151)
* [rust-analyzer: properly handle break resolution inside non-breakable expressions](https://github.com/rust-lang/rust-analyzer/pull/13165)
* [rust-analyzer: remove `hir::Expr::MacroStmts`](https://github.com/rust-lang/rust-analyzer/pull/13156)
* [rust-analyzer: remove type alias definition on inline](https://github.com/rust-lang/rust-analyzer/pull/13091)
* [rust-analyzer: suggest struct when completing enum](https://github.com/rust-lang/rust-analyzer/pull/13139)
* [rust-analyzer: add a "Unmerge match arm" assist to split or-patterns inside match expressions](https://github.com/rust-lang/rust-analyzer/pull/13145)
* [rust-analyzer: implement `feature(exhaustive_patterns)` from unstable Rust](https://github.com/rust-lang/rust-analyzer/pull/13167)
* [rust-analyzer: assist to turn `match` into `matches!` invocation](https://github.com/rust-lang/rust-analyzer/pull/13005)
* [rust-analyzer: insert whitespaces into `static` & `const` bodies if they are expanded from macro on hover](https://github.com/rust-lang/rust-analyzer/pull/13185)
* [rust-analyzer: lower float literals with underscores](https://github.com/rust-lang/rust-analyzer/pull/13161)
* [rust-analyzer: only move comments when extracting a struct from an enum variant](https://github.com/rust-lang/rust-analyzer/pull/13051)
* [rust-analyzer: parse `TypePathFn` with preceding `::`](https://github.com/rust-lang/rust-analyzer/pull/13160)
* [rust-analyzer: correct broken logic for return completion](https://github.com/rust-lang/rust-analyzer/pull/13187)
* [rust-analyzer: unescape all occurrences of module name in module resolution](https://github.com/rust-lang/rust-analyzer/pull/13149)

### Rust Compiler Performance Triage

A relatively quiet week where regressions unfortunately outweighed improvements. What's more, many of the regressions that were found seemed somewhat mysterious requiring some deeper investigations.

Triage done by **@rylev**.
Revision range: [0631ea5d73..09fb0bc6e](https://perf.rust-lang.org/?start=0631ea5d73f4a3199c776687b12c20c50a91f0d2&end=09fb0bc6ecef62201d7c662db24b984d03245ac6&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u) | mean | range | count |
|:----------------:|:----:|:-----:|:-----:|
| Regressions ❌ <br /> (primary) | 0.7% | [0.2%, 4.5%] | 85    |
| Regressions ❌ <br /> (secondary) | 1.0% | [0.3%, 5.4%] | 87    |
| Improvements ✅ <br /> (primary) | -0.7% | [-1.0%, -0.5%] | 9     |
| Improvements ✅ <br /> (secondary) | -1.4% | [-2.7%, -0.5%] | 22    |
| All ❌✅ (primary) | 0.5% | [-1.0%, 4.5%] | 94    |


2 Regressions, 3 Improvements, 2 Mixed; 3 of them in rollups
40 artifact comparisons made in total

[Full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-09-06.md)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [De-RFC: Remove type ascription](https://github.com/rust-lang/rfcs/pull/3307)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Stabilize generic associated types](https://github.com/rust-lang/rust/pull/96709)
* [disposition: merge] [Fix handling of trailing bare CR in str::lines](https://github.com/rust-lang/rust/pull/100311)
* [disposition: merge] [Implement Default for some alloc/core iterators](https://github.com/rust-lang/rust/pull/99929)
* [disposition: merge] [Tracking Issue for `#[instruction_set]` attribute (RFC 2867)](https://github.com/rust-lang/rust/issues/74727)
* [disposition: merge] [Consider `#[must_use]` annotation on `async fn` as also affecting the `Future::Output`](https://github.com/rust-lang/rust/pull/100633)
* [disposition: merge] [Tracking issue for `..X`, and `..=X` (`#![feature(half_open_range_patterns)]`)](https://github.com/rust-lang/rust/issues/67264)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Add descriptive names to doctest](https://github.com/rust-lang/rfcs/pull/3311)
* [new] [Add a general mechanism of setting RUSTFLAGS in Cargo for the root crate only](https://github.com/rust-lang/rfcs/pull/3310)
* [new] [Rust Style Team](https://github.com/rust-lang/rfcs/pull/3309)
* [new] [Add `core::mem::offset_of!` RFC](https://github.com/rust-lang/rfcs/pull/3308)

## Upcoming Events

Rusty Events between 2022-09-07 - 2022-10-05 🦀

### Virtual

* 2022-09-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/285121715/)
* 2022-09-08 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Workshop: Introduction To Rust For Artists - by Lisa Passing**](https://www.meetup.com/rust-linz/events/288027737/)
* 2022-09-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #17**](https://www.meetup.com/rust-noris/events/hlvbvsydcmblb/)
* 2022-09-09 | Virtual + Linz, AT | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 25th Edition**](https://www.meetup.com/rust-linz/events/288048260/)
* 2022-09-10 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-09-10 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059979/)
* 2022-09-12 | Virtual + Dublin, IE | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022 (Sep 12-14)**](https://lpc.events/event/16/sessions/150/)
* 2022-09-13 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/288032274/)
* 2022-09-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcmbrb/)
* 2022-09-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/286485815/)
* 2022-09-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcmbsb/)
* 2022-09-14 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Introduction to Async in Rust**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/288154493/)
* 2022-09-14 | Virtual (Malaysia)| [Golang Malaysia](https://docs.google.com/forms/d/e/1FAIpQLScKGolWclIOR1OBCzTOitVU5Am5lSYxSlVhK71DGsc-fa-Yhg/viewform)
    * [**Rust Meetup September 2022**](https://discord.gg/9Xj8H2EXTD)
* 2022-09-15 | Virtual (Columbus, OH, US) | [GDG Columbus](https://www.meetup.com/gdg-columbus/)
    * [**Past, Present, and Future of Internet Money! (Custom tokenomics, RUST and CosmWASM library...)**](https://www.meetup.com/gdg-columbus/events/287972746/)
* 2022-09-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcmbtb/)
* 2022-09-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful: Bencher—Catch Performance Regressions in CI—Everett Pompeii**](https://www.meetup.com/rustdc/events/287004599/)
* 2022-09-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out (Call for Participation)**](https://www.meetup.com/vancouver-rust/events/285933975/)
* 2022-09-22 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Rust based Bluetooth tools (BlueR) you can use today**](https://www.meetup.com/charlottesville-rust-meetup/events/288123436/)
* 2022-09-23 | Virtual (Tokyo, JP) | [Rust Tokyo](https://rust.tokyo)
    * [**Rust Tokyo 2022**](https://rust.tokyo/2022)
* 2022-09-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcmbkc/)
* 2022-10-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcnbgb/)
* 2022-10-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcnbhb/)
* 2022-10-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcnbhb/)

### Europe

* 2022-09-09 | Linz, AT + Virtual | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 25th Edition**](https://www.meetup.com/rust-linz/events/288048260/)
* 2022-09-12 | Dublin, IE + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022 (Sep 12-14)**](https://lpc.events/event/16/sessions/150/)
* 2022-09-13 | Rome, IT | [Rust Roma](https://www.meetup.com/Rust-Roma/)
    * [**Rust learning and hacking evening #Aperitech**](https://www.meetup.com/Rust-Roma/events/288323150/)
* 2022-09-15 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #52**](https://www.meetup.com/rust-paris/events/288136736/)
* 2022-09-29 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/288266277/)
* 2022-09-29 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #29**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179100/)
* 2022-09-29 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Going full stack on Rust**](https://www.meetup.com/dutch-rust-meetup/events/286727064/)
* 2022-10-02 | Florence, IT + Virtual | [RustLab](https://rustlab.it/)
    * [**RustLab Conference 2022 (Oct 2-4)**](https://rustlab.it/schedule/)

### North America

* 2022-09-08 | Columbus, OH, US| [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcmblb/)
* 2022-09-14 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/288287206/)
* 2022-09-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcmbbc/)
* 2022-09-22 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Game Prototyping with Rusty Engine with Nathan Stocks and Food!**](https://www.meetup.com/utah-rust/events/rvpgxsydcmbmc/)

### Oceania

* 2022-09-14 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust-Sydney Lightning Talks**](https://www.meetup.com/rust-sydney/events/287979855/)

### South America

* 2022-09-10 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/)
    * [**iFood Tech Day: Rust**](https://www.meetup.com/rust-sao-paulo-meetup/events/288147015/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/wm0k6q/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> So long, and thanks for all the turbofish.

– [moltonel on r/rust](https://www.reddit.com/r/rust/comments/wzuoqz/comment/im4pek6)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1286) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/x8pm33/this_week_in_rust_459/)</small>
