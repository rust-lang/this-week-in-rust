Title: This Week in Rust 491
Number: 491
Date: 2023-04-19
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

### Foundation
* [Rust Trademark Policy Draft Revision – Next Steps](https://foundation.rust-lang.org/news/rust-trademark-policy-draft-revision-next-steps/)

### Project/Tooling Updates
* [Nutype v0.2](https://github.com/greyblake/nutype/releases/tag/v0.2.0)
* [Announcing tuxedo-rs](https://aaronerhardt.github.io/blog/posts/announcing_tuxedo_rs/)
* [rust-analyzer changelog #177](https://rust-analyzer.github.io/thisweek/2023/04/17/changelog-177.html)

### Observations/Thoughts
* [Improving build times for derive macros by 3x or more](https://blog.kodewerx.org/2023/04/improving-build-times-for-derive-macros.html)
* [Oxidizing bmap-tools: rewriting a Python project in Rust](https://www.collabora.com/news-and-blog/blog/2023/03/03/oxidizing-bmap-tools-rewriting-a-python-project-in-rust/)
* [Rust: A New Attempt at C++'s Main Goal](https://www.thecodedmessage.com/posts/rust-new-cpp/)
* [Traits are more than interfaces](https://felix-knorr.net/posts/2023-04-17-traits.html)
* [Optional If Expressions](https://animaomnium.github.io/optional-if-expressions/)
* [Building a GStreamer plugin in Rust with meson instead of cargo](https://coaxion.net/blog/2023/04/building-a-gstreamer-plugin-in-rust-with-meson-instead-of-cargo/)
* [Rutie and Magnus, Two Good Ways to Build Ruby Extensions in Rust](https://www.hezmatt.org/~mpalmer/blog/2023/04/18/rutie-magnus-rust-extensions-for-ruby.html)
* [Two Rust features that I miss in Other languages](https://dev.to/yjdoc2/two-rust-features-that-i-miss-in-other-languages-7ec)
* [audio] [Rust Analyzer with Lukas Wirth](https://rustacean-station.org/episode/lukas-wirth/)
* [audio] [Wasmer with Syrus Akbary](https://rustacean-station.org/episode/syrus-akbary/)

### Rust Walkthroughs
* [Understanding Rust Thread Safety](https://onesignal.com/blog/thread-safety-rust/)
* [how to run rust code on a circuit playground classic / atmega32u4](https://zoe.kittycat.homes/log/rust_atmega32u4_tutorial)
* [Hello World in Rust for m68k with #[no_core] and compiler patches](https://iandouglasscott.com/2023/04/12/hello-world-in-rust-for-m68k-with-no-core-and-compiler-patches/)
* [A syntax-level async join macro supporting branching control flow and synchronous shared mutable borrowing](https://wishawa.github.io/posts/enjoin/)
* [Build a Lua Interpreter in Rust (English version)](https://wubingzheng.github.io/build-lua-in-rust/en/)
* [CN] [Zino's implementation of an error type with tracing functionalities using 100 lines of code](https://zhuanlan.zhihu.com/p/621839043)
* [A blog article and project demonstrating GitHub actions in Rust](https://blog.ngerakines.me/posts/github-actions-in-rust/)

### Miscellaneous
* [Introducing Shuttle Batch 2.0](https://www.shuttle.rs/blog/2023/04/14/Shuttle-Batch-2)
* [The Rust Foundation's draft trademark policy is far too restrictive](https://diziet.dreamwidth.org/14929.html)
* [video] [Rust Trademark: Argle-bargle or Foofaraw?](https://www.youtube.com/watch?v=N-ADQ5n7HoY)

## Crate of the Week

This week's crate is [onlyerror](https://crates.io/crates/onlyerror), a `#[derive(Error)]` macro with support for `no_std` on nightly compilers.

Thanks to [Jay Oster](https://users.rust-lang.org/t/crate-of-the-week/2704/1186) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.
* [Quickwit - Upgrade clap from 3.1 to 4.0](https://github.com/quickwit-oss/quickwit/issues/2236)
* [Quickwit - Implement quickwit dataset CLI command](https://github.com/quickwit-oss/quickwit/issues/2681)
* [Hyperswitch - Migrate to `enum_dispatch` to reduce runtime overhead](https://github.com/juspay/hyperswitch/issues/921)
* [Hyperswitch - move redis key creation to a common module](https://github.com/juspay/hyperswitch/issues/917)
* [Hyperswitch - add `connector_label` field in error type](https://github.com/juspay/hyperswitch/issues/899)
* [Ockam - Update `ockam status --all` to list more of the available resources](https://github.com/build-trust/ockam/issues/4719)
* [Ockam - Remove `rustcrypto` feature from `ockam_vault`](https://github.com/build-trust/ockam/issues/4704)
* [Ockam - Create Github Action to update Ockam Command Manual](https://github.com/build-trust/ockam/issues/4650)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

450 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-04-10..2023-04-17

* [initial support for loongarch64-unknown-linux-gnu](https://github.com/rust-lang/rust/pull/96971)
* [add inline assembly support for m68k](https://github.com/rust-lang/rust/pull/109989)
* [make rust-intrinsic ABI unwindable](https://github.com/rust-lang/rust/pull/110233)
* [allow `repr(align = x)` on inherent methods](https://github.com/rust-lang/rust/pull/110313)
* [add a backtrace to Allocation, display it in leak reports](https://github.com/rust-lang/rust/pull/109061)
* [add a message for if an overflow occurs in `core::intrinsics::is_nonoverlapping`](https://github.com/rust-lang/rust/pull/110388)
* [add suggestion to remove `derive()` if invoked macro is non-derive](https://github.com/rust-lang/rust/pull/109638)
* [added diagnostic for `pin!` macro in addition to `Box::pin` if Unpin isn't implemented](https://github.com/rust-lang/rust/pull/110259)
* [assemble `Unpin` candidates specially for generators in new solver](https://github.com/rust-lang/rust/pull/110207)
* [check for body owner fallibly in error reporting](https://github.com/rust-lang/rust/pull/110193)
* [correct default value for default-linker-libraries](https://github.com/rust-lang/rust/pull/110337)
* [emits non-overlapping suggestions for arguments with wrong types](https://github.com/rust-lang/rust/pull/109850)
* [encode def span for `ConstParam`](https://github.com/rust-lang/rust/pull/110425)
* [erase lifetimes above `ty::INNERMOST` when probing ambiguous types](https://github.com/rust-lang/rust/pull/110195)
* [erase regions when confirming transmutability candidate](https://github.com/rust-lang/rust/pull/110038)
* [fix false positives for `unused_parens` around unary and binary operations](https://github.com/rust-lang/rust/pull/110257)
* [fix transmute intrinsic mir validation ICE](https://github.com/rust-lang/rust/pull/109959)
* [fix: ensure bad `#[test]` invocs retain correct AST](https://github.com/rust-lang/rust/pull/110035)
* [fix: skip implied bounds if unconstrained lifetime exists](https://github.com/rust-lang/rust/pull/110272)
* [improve safe transmute error reporting](https://github.com/rust-lang/rust/pull/109800)
* [improve the error message when forwarding a matched fragment to another macro](https://github.com/rust-lang/rust/pull/110222)
* [incr.comp.: make sure dependencies are recorded when feeding queries during eval-always queries](https://github.com/rust-lang/rust/pull/109935)
* [preserve argument indexes when inlining MIR](https://github.com/rust-lang/rust/pull/109466)
* [reformulate `point_at_expr_source_of_inferred_type` to be more accurate](https://github.com/rust-lang/rust/pull/108687)
* [report overflows gracefully with new solver](https://github.com/rust-lang/rust/pull/110103)
* [resolve: pre-compute non-reexport module children](https://github.com/rust-lang/rust/pull/110160)
* [tweak output for 'add line' suggestion](https://github.com/rust-lang/rust/pull/109786)
* [suggest lifetime for closure parameter type when mismatch](https://github.com/rust-lang/rust/pull/105888)
* [support safe transmute in new solver](https://github.com/rust-lang/rust/pull/110126)
* [add a stable MIR way to get the main function](https://github.com/rust-lang/rust/pull/110315)
* [custom MIR: Support `BinOp::Offset`](https://github.com/rust-lang/rust/pull/110190)
* [switch to `EarlyBinder` for `impl_subject` query](https://github.com/rust-lang/rust/pull/110299)
* [tagged pointers, now with strict provenance!](https://github.com/rust-lang/rust/pull/110243)
* [alloc `hir::Lit` in an arena to remove the destructor from `Expr`](https://github.com/rust-lang/rust/pull/109588)
* [only emit alignment checks if we have a `panic_impl`](https://github.com/rust-lang/rust/pull/110283)
* [only enable ConstProp at `mir-opt-level >= 2`](https://github.com/rust-lang/rust/pull/109900)
* [permit MIR inlining without `#[inline]`](https://github.com/rust-lang/rust/pull/109247)
* [`rustc_metadata`: Filter encoded data more aggressively using `DefKind`](https://github.com/rust-lang/rust/pull/109765)
* [stabilize IsTerminal](https://github.com/rust-lang/rust/pull/110072)
* [don't splice from files into pipes in `io::copy`](https://github.com/rust-lang/rust/pull/108283)
* [`sync::mpsc`: synchronize receiver disconnect with initialization](https://github.com/rust-lang/rust/pull/110089)
* [windows: map a few more error codes to ErrorKind](https://github.com/rust-lang/rust/pull/110433)
* [hashbrown: remove drain-on-drop behavior from DrainFilter](https://github.com/rust-lang/hashbrown/pull/374)
* [regex: first phase of migrating to regex-automata](https://github.com/rust-lang/regex/pull/977)
* [cargo: change -C to be unstable](https://github.com/rust-lang/cargo/pull/11960)
* [cargo: stabilize `cargo logout`](https://github.com/rust-lang/cargo/pull/11950)
* [cargo: use registry.default for login/logout](https://github.com/rust-lang/cargo/pull/11949)
* [cargo: use restricted Damerau-Levenshtein algorithm](https://github.com/rust-lang/cargo/pull/11963)
* [rustdoc-search: add support for nested generics](https://github.com/rust-lang/rust/pull/109802)
* [rustdoc: correctly handle built-in compiler proc-macros as proc-macro and not macro](https://github.com/rust-lang/rust/pull/110279)
* [stabilize rustdoc `--test-run-directory`](https://github.com/rust-lang/rust/pull/103682)
* [clippy: `collection_is_never_read`: Handle unit type](https://github.com/rust-lang/rust-clippy/pull/10492)
* [clippy: add `manual_slice_size_calculation` applicable suggestion](https://github.com/rust-lang/rust-clippy/pull/10661)
* [clippy: clear with drain](https://github.com/rust-lang/rust-clippy/pull/10614)
* [clippy: fix false positives and false negatives in `octal_escapes`](https://github.com/rust-lang/rust-clippy/pull/10603)
* [clippy: suggest `std::mem::size_of_val` instead of `std::mem::size_of_value`](https://github.com/rust-lang/rust-clippy/pull/10659)
* [rust-analyzer: don't suggest unstable items on stable toolchain](https://github.com/rust-lang/rust-analyzer/pull/14549)
* [rust-analyzer: make inlay hints insertable](https://github.com/rust-lang/rust-analyzer/pull/14533)
* [rust-analyzer: map tokens from `include!` expansion to the included file](https://github.com/rust-lang/rust-analyzer/pull/14561)
* [rust-analyzer: fix allow extracting function from single brace of block expression](https://github.com/rust-lang/rust-analyzer/pull/14540)
* [rust-analyzer: fix explicit deref problems in closure capture](https://github.com/rust-lang/rust-analyzer/pull/14576)
* [rust-analyzer: bring back LRU limit for `macro_expand` query](https://github.com/rust-lang/rust-analyzer/pull/14588)
* [rust-analyzer: fix inference in nested closures](https://github.com/rust-lang/rust-analyzer/pull/14550)
* [rust-analyzer: fix inverted code lens resolve file version check](https://github.com/rust-lang/rust-analyzer/pull/14570)
* [rust-analyzer: fix receiver adjustments for `extract_variable` assist](https://github.com/rust-lang/rust-analyzer/pull/14547)
* [rust-analyzer: infer types of nested RPITs](https://github.com/rust-lang/rust-analyzer/pull/14544)
* [rust-analyzer: when running the "discoverProjectCommand", use the Rust file's parent directory instead of the workspace folder](https://github.com/rust-lang/rust-analyzer/pull/14535)
* [rust-analyzer: parse more exclusive range patterns and inline const patterns](https://github.com/rust-lang/rust-analyzer/pull/14580)

### Rust Compiler Performance Triage

A busy two weeks (as last week perf triage was not done). Overall improvements outweigh regressions with an average improvement of -2.6% across a large swath of the test cases. Of particular note was the move to use SipHash-1-3 instead of SipHash-2-4 for StableHasher which improved 184 benchmark tests by an average of 2.3%! 

Triage done by **@rylev**.
Revision range: [7c96e40..74864f](https://perf.rust-lang.org/?start=7c96e40da81165beef4f273f44e96eeef5a1bd30&end=74864fa496997a6498e623f0d2019ccb7eb6dad0&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 3.1%  | [0.2%, 24.4%]   | 11    |
| Regressions ❌ <br /> (secondary)  | 4.9%  | [0.4%, 37.4%]   | 32    |
| Improvements ✅ <br /> (primary)   | -2.9% | [-20.4%, -0.3%] | 205   |
| Improvements ✅ <br /> (secondary) | -4.0% | [-43.5%, -0.3%] | 160   |
| All ❌✅ (primary)                 | -2.6% | [-20.4%, 24.4%] | 216   |


6 Regressions, 8 Improvements, 11 Mixed; 6 of them in rollups
119 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-04-18.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Update the version of musl used on `*-linux-musl` targets to 1.2.3](https://github.com/rust-lang/rust/pull/107129)
* [disposition: merge] [Tracking Issue for `debugger_visualizer`](https://github.com/rust-lang/rust/issues/95939)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Use `actions/deploy-pages` to deploy `mdbook` output](https://github.com/rust-lang/rfcs/pull/3419)
* [new] [RFC for associated mathematical constants](https://github.com/rust-lang/rfcs/pull/3418)
* [new] [improve `#[may_dangle]` for type parameters](https://github.com/rust-lang/rfcs/pull/3417)
* [new] [RFC: Cargo feature descriptions & `rustdoc` configuration](https://github.com/rust-lang/rfcs/pull/3416)
* [new] [Traits for lossy conversions](https://github.com/rust-lang/rfcs/pull/3415)
* [new] [Split may_dangle and make PhantomData less weird](https://github.com/rust-lang/rfcs/pull/3414)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [1.69.0 pre-release testing](https://blog.rust-lang.org/inside-rust/2023/04/17/1.69.0-prerelease.html)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-04-19 - 2023-05-17 🦀

### Virtual

* 2023-04-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Kaskada, Rust and Apache Arrow**](https://www.meetup.com/vancouver-rust/events/lqkkctyfcgbzb/)
* 2023-04-20 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/291965920/)
* 2023-04-20 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsyfcgbbc/)
* 2023-04-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcgbhc/)
* 2023-04-26 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust-friendly websites and web apps**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292559177/)
* 2023-04-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Testing Tock, how unit tests in Rust improve and teach**](https://www.meetup.com/charlottesville-rust-meetup/events/292193436/)
* 2023-04-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #35 at Google Cloud**](https://www.meetup.com/copenhagen-rust-community/events/292424926/)
* 2023-04-29 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 3: Protohackers Exercises Mob Coding (as far as we get)**](https://www.meetup.com/rust-noris/events/292149688/)
* 2023-05-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfchbdb/)
* 2023-05-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfchbfb/)
* 2023-05-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfchbmb/)
* 2023-05-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/gmkpctyfchbpb/)
* 2023-05-13 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-05-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—Introducing duplicate! and the peculiarities of proc macros**](https://www.meetup.com/rustdc/events/jkxsctyfchbvb/)
* 2023-05-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Atomics and Locks Book Club Chapter 2**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292847157/)
* 2023-05-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/lqkkctyfchbwb/)

### Europe

* 2023-04-19 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #58**](https://www.meetup.com/rust-paris/events/292575461/)
* 2023-04-19 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Rust Embedded with MicroBit:V2**](https://www.meetup.com/rust-trondheim/events/292680021/)
* 2023-04-19 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**sett: data encryption and transfer made easy(ier)**](https://www.meetup.com/de-DE/rust-zurich/events/292151879/)
* 2023-04-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus meetup #1 at Geanix**](https://www.meetup.com/rust-aarhus/events/292185072/)
* 2023-04-20 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/291965920/)
* 2023-04-20 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**First Rust Bern Meetup!**](https://www.meetup.com/de-DE/rust-bern/events/292206056/)
* 2023-04-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfcgbcc/)
* 2023-04-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Hack & Learn April 2023**](https://www.meetup.com/rust-london-user-group/events/292729308/)
* 2023-04-27 | Bordeaux, FR | [DedoTalk](https://www.meetup.com/dedotalk/)
    * [**#2 DedoTalk 🎙️ : Comment tester son code Rust?**](https://www.meetup.com/dedotalk/events/292842782/)
* 2023-04-27 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna - April - Hosted by Sentry**](https://www.meetup.com/rust-vienna/events/292751465/)
* 2023-05-02 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/292297784/)
* 2023-05-10 | Amsterdam, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2023**](https://2023.rustnl.org/)

### North America

* 2023-04-19 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/292749528/)
* 2023-04-19 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Happy Hour and Beginner WASM Rust Hacking Session (#2!)**](https://www.meetup.com/minneapolis-rust-meetup/events/292814034/)
* 2023-04-20 | Mountain View, CA, US | [Mountain View Rust Study Group](https://www.meetup.com/rust-study-group/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/rust-study-group/events/292694348/)
* 2023-04-29 | Durham, NC, US | [Triangle Rust](https://www.meetup.com/triangle-rust/)
    * [**Rust Social / Coffee Chat at Boxyard RTP**](https://www.meetup.com/triangle-rust/events/292833711/)
* 2023-05-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Upcoming Event**](https://www.meetup.com/utah-rust/events/rrwbctyfchbpb/)
* 2023-05-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfchbvb/)

### Oceania

* 2023-04-27 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**April Meetup**](https://www.meetup.com/rust-brisbane/events/292965270/)
* 2023-05-03 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/292993051/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/11naac9/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Error types should be located near to their unit of fallibility.

– [Sabrina Jewson on her blog](https://sabrinajewson.org/blog/errors)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1394) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/12sgmyq/this_week_in_rust_491/)</small>
