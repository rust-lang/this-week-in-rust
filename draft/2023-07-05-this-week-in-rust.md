Title: This Week in Rust 502
Number: 502
Date: 2023-07-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
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
* [Rustfmt support for let-else statements](https://blog.rust-lang.org/2023/07/01/rustfmt-supports-let-else-statements.html)

### Foundation

### Newsletters
* [This Month in Rust GameDev #46 - May 2023](https://gamedev.rs/news/046/)

### Project/Tooling Updates
* [rust-analyzer changelog #188](https://rust-analyzer.github.io/thisweek/2023/07/03/changelog-188.html)
* [Pavex DevLog #5: redesigning our runtime types](https://www.lpalmieri.com/posts/pavex-progress-report-05/)
* [Bevy XPBD: A physics engine for the Bevy game engine](https://joonaa.dev/blog/02/bevy-xpbd-0-1-0)

* [complgen: Generate {bash,fish,zsh} completions from a single EBNF-like grammar](https://github.com/adaszko/complgen)

### Observations/Thoughts
* [How To Wrap Your Errors With Enums When Using Error-Stack](https://betterprogramming.pub/how-to-wrap-your-errors-with-enums-when-using-error-stack-77b122016e6e)
* [A compressed indexable bitset](https://quickwit.io/blog/compressed-indexable-bitset)
* [Exploring Graphs in Rust. Yikes](https://www.confessionsofadataguy.com/exploring-graphs-in-rust-yikes/)
* [Writing a Linked List in Rust: A Walkthrough](https://cmooneycollett.github.io/2023/07/01/writing-a-linkedlist-in-rust)
* [Tree-Structured Concurrency](https://blog.yoshuawuyts.com/tree-structured-concurrency/)
* [Rust Notes on Temporary values (usage of Mutex) - 4](https://dev.to/nsengupta/rust-notes-on-temporary-values-usage-of-mutex-4-498c)
* [A persistent task queue in Rust](https://jmmv.dev/2023/06/iii-iv-task-queue.html)
* [Method Overloading (kinda), and Advanced Trait Usage](https://youtu.be/HA_e1c0HbgQ)
* [Unlocking Possibilities: 4 Reasons Why ESP32 and Rust Make a Winning Combination](https://apollolabsblog.hashnode.dev/unlocking-possibilities-4-reasons-why-esp32-and-rust-make-a-winning-combination)
* [The magic of dependency resolution](https://ochagavia.nl/blog/the-magic-of-dependency-resolution/)
* [Writing E2E Tests for Axum & GraphQL](https://estebanborai.com/en/notes/writing-e2e-tests-for-axum-and-graphql)
* [Detailed web-based 3D rendering of mining spatial data](https://www.kurtlawrence.info/blog/detailed-web-based-3d-rendering-of-mining-spatial-data)
* [video] [Choose the Right Option](https://www.youtube.com/watch?v=6c7pZYP_iIE)
* [video] [4 levels of Rust error handling](https://www.youtube.com/watch?v=kHxjiTv8r18)

- [Tree-Structured Concurrency](https://blog.yoshuawuyts.com/tree-structured-concurrency/)

### Rust Walkthroughs
* [Build a Ray Tracer, pt. 4 - The Next Dimension](https://www.superperfundo.dev/articles/ray-tracer-part4)

* [Nine Rules for Running Rust on the Web and on Embedded: Practical Lessons from Porting range-set-blaze to no_std and WASM](https://towardsdatascience.com/nine-rules-for-running-rust-on-the-web-and-on-embedded-94462ef249a2)

* [Full Stack Rust Workshop: Shuttle, Actix Web, SQLx & Diouxus](https://bcnrust.github.io/devbcn-workshop/)

- [A compressed indexable bitset](https://quickwit.io/blog/compressed-indexable-bitset)

* [Intercepting Allocations with the Global Allocator](https://bd103.github.io/blog/2023-06-27-global-allocators)

### Research

### Miscellaneous

* [Reduce memory footprint by about 600% for M.E.D. — Performance Matters](https://medium.com/gitconnected/reduce-memory-footprint-by-about-600-for-m-e-d-performance-matters-bec407833e7c)

## Crate of the Week

This week's crate is [rustypaste](https://github.com/orhun/rustypaste), a minimal file upload/pastebin service.

Thanks to [orhun](https://users.rust-lang.org/t/crate-of-the-week/2704/1209) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch - Implement Code coverage for local system using Makefile](https://github.com/juspay/hyperswitch/issues/1622)
* [Hyperswitch - Add scoped error enum for customer error](https://github.com/juspay/hyperswitch/issues/1580)
* [Hyperswitch - move redis key creation to a common module](https://github.com/juspay/hyperswitch/issues/917)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

410 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-06-26..2023-07-03

* [support embedding LLVM bitcode on AIX](https://github.com/rust-lang/rust/pull/109524)
* [support for native WASM exceptions](https://github.com/rust-lang/rust/pull/111322)
* [`fix(resolve)`: skip assertion judgment when NonModule is dummy](https://github.com/rust-lang/rust/pull/113168)
* [`thir`: Add `Become` expression kind](https://github.com/rust-lang/rust/pull/113093)
* [account for late-bound vars from parent arg-position impl trait](https://github.com/rust-lang/rust/pull/113071)
* [add `-Zremark-dir` unstable flag to write LLVM optimization remarks to YAML](https://github.com/rust-lang/rust/pull/113040)
* [add bidirectional where clauses on RPITIT synthesized GATs](https://github.com/rust-lang/rust/pull/112682)
* [add check for `ConstKind::Value(_)` to `in_operand()`](https://github.com/rust-lang/rust/pull/113107)
* [avoid calling queries during query stack printing](https://github.com/rust-lang/rust/pull/112708)
* [better messages for next on a iterator inside for loops](https://github.com/rust-lang/rust/pull/113174)
* [detect actual span for getting unexpected token from parsing macros](https://github.com/rust-lang/rust/pull/112518)
* [don't perform selection if inherent associated types are not enabled](https://github.com/rust-lang/rust/pull/113286)
* [don't suggest `move` for borrows that aren't closures](https://github.com/rust-lang/rust/pull/113137)
* [encode item bounds for `DefKind::ImplTraitPlaceholder`](https://github.com/rust-lang/rust/pull/113165)
* [error when RPITITs' hidden types capture more lifetimes than their trait definitions](https://github.com/rust-lang/rust/pull/113182)
* [export AnalysisResults trait in `rustc_mir_dataflow`](https://github.com/rust-lang/rust/pull/113089)
* [fix `dropping_copy_types` lint from linting in match-arm with side-effects](https://github.com/rust-lang/rust/pull/113231)
* [fix associated items effective visibility calculation for type privacy lints](https://github.com/rust-lang/rust/pull/113196)
* [fix type privacy lints error message](https://github.com/rust-lang/rust/pull/113161)
* [fix unset `e_flags` in ELF files generated for AVR targets](https://github.com/rust-lang/rust/pull/106619)
* [implement deep normalization via the new solver](https://github.com/rust-lang/rust/pull/113086)
* [implement most of MCP510](https://github.com/rust-lang/rust/pull/112910)
* [implement proposed API for `proc_macro_span`](https://github.com/rust-lang/rust/pull/111571)
* [implement selection via new trait solver](https://github.com/rust-lang/rust/pull/112869)
* [lint/ctypes: ext. abi fn-ptr in internal abi fn](https://github.com/rust-lang/rust/pull/108611)
* [make associated type bounds in supertrait position implied](https://github.com/rust-lang/rust/pull/112629)
* [make compiletest aware of targets without dynamic linking](https://github.com/rust-lang/rust/pull/112454)
* [make the `Elaboratable` trait take clauses](https://github.com/rust-lang/rust/pull/113144)
* [normalize opaques with late-bound vars again](https://github.com/rust-lang/rust/pull/113108)
* [normalize types when applying uninhabited predicate](https://github.com/rust-lang/rust/pull/113103)
* [privacy: type privacy lints fixes and cleanups](https://github.com/rust-lang/rust/pull/112670)
* [properly implement `variances_of` for RPITIT GAT](https://github.com/rust-lang/rust/pull/113171)
* [refactor metadata emission to avoid visiting HIR](https://github.com/rust-lang/rust/pull/98867)
* [resolve: remove artificial import ambiguity errors](https://github.com/rust-lang/rust/pull/112086)
* [simplify computation of killed borrows](https://github.com/rust-lang/rust/pull/112236)
* [suggest `slice::swap` for `mem::swap(&mut x[0], &mut x[1])` borrowck error](https://github.com/rust-lang/rust/pull/111403)
* [add suggestion for bad block fragment error](https://github.com/rust-lang/rust/pull/112978)
* [use structured suggestion when telling user about `for<'a>`](https://github.com/rust-lang/rust/pull/113177)
* [mark wrapped intrinsics as `inline(always)`](https://github.com/rust-lang/rust/pull/113194)
* [make `simd_shuffle_indices` use valtrees](https://github.com/rust-lang/rust/pull/112718)
* [make `UnwindAction::Continue` explicit in MIR dump](https://github.com/rust-lang/rust/pull/112972)
* [mir opt + codegen: handle subtyping](https://github.com/rust-lang/rust/pull/112307)
* [miri: cargo-miri: better error message when RUSTC is not set](https://github.com/rust-lang/miri/pull/2950)
* [miri: make `--quiet` actually do something](https://github.com/rust-lang/miri/pull/2942)
* [miri: optional semantics for `Unique`](https://github.com/rust-lang/miri/pull/2936)
* [shrink error variants for layout and `fn_abi`](https://github.com/rust-lang/rust/pull/111035)
* [a mish-mash of micro-optimizations](https://github.com/rust-lang/rust/pull/113116)
* [codegen\_gcc: add support for `#[cold]` attribute](https://github.com/rust-lang/rustc_codegen_gcc/pull/286)
* [allow comparing `Box`es with different allocators](https://github.com/rust-lang/rust/pull/112628)
* [make `rustc_on_unimplemented` std-agnostic](https://github.com/rust-lang/rust/pull/113054)
* [stabilize `const_cstr_methods`](https://github.com/rust-lang/rust/pull/107624)
* [cargo: add READMEs for the credential helpers](https://github.com/rust-lang/cargo/pull/12322)
* [cargo: don't try to compile cargo-credential-gnome-secret on non-Linux platforms](https://github.com/rust-lang/cargo/pull/12321)
* [rustdoc: fix display of long items in search results](https://github.com/rust-lang/rust/pull/113100)
* [rustdoc: fix display of long inline cfg labels](https://github.com/rust-lang/rust/pull/113285)
* [rustdoc: allow whitespace as path separator like double colon](https://github.com/rust-lang/rust/pull/108537)
* [rustdoc: render generic params & where-clauses of cross-crate assoc tys in impls](https://github.com/rust-lang/rust/pull/112920)
* [rustfmt: don't skip semicolon if expressions follow](https://github.com/rust-lang/rustfmt/pull/5798)
* [rustfmt: implement `single_line_let_else_max_width`](https://github.com/rust-lang/rustfmt/pull/5790)
* [rustfmt: rewrite float literals ending in dots with parens in method calls](https://github.com/rust-lang/rustfmt/pull/5794)
* [rustfmt: switch to tracing for logging](https://github.com/rust-lang/rustfmt/pull/5800)
* clippy: new lints: [`manual_try_fold`](https://github.com/rust-lang/rust-clippy/pull/11012),
  [`needless_raw_string_hashes`](https://github.com/rust-lang/rust-clippy/pull/10884),
  [`redundant_at_rest_pattern`](https://github.com/rust-lang/rust-clippy/pull/11013),
  [`tuple_array_conversions`](https://github.com/rust-lang/rust-clippy/pull/11020),
  [`manual_range_patterns`](https://github.com/rust-lang/rust-clippy/pull/10968),
  [`type_id_on_box`](https://github.com/rust-lang/rust-clippy/pull/10987),
  [`needless_pub_self`, `pub_with_shorthand` and `pub_without_shorthand`](https://github.com/rust-lang/rust-clippy/pull/10967)
* [clippy: `significant_drop_tightening`: fix incorrect suggestion](https://github.com/rust-lang/rust-clippy/pull/10774)
* [clippy: `arc_with_non_send_sync`: don't lint if type has nested type parameters](https://github.com/rust-lang/rust-clippy/pull/11077)
* [clippy: `let_and_return`: lint `'static` lifetimes, don't lint borrows in closures](https://github.com/rust-lang/rust-clippy/pull/11061)
* [clippy: `missing_fields_in_debug`: make sure self type is an adt](https://github.com/rust-lang/rust-clippy/pull/11069)
* [clippy: `needless_raw_string_hashes`: only reset hashes needed if not following quote](https://github.com/rust-lang/rust-clippy/pull/11078)
* [clippy: `option_if_let_else`: suggest `.as_ref()` if scrutinee is of type `&Option<_>`](https://github.com/rust-lang/rust-clippy/pull/11035)
* [clippy: `question_mark`: don't lint inside of `try` block](https://github.com/rust-lang/rust-clippy/pull/11001)
* [clippy: `unused_async`: don't lint if function is part of a trait](https://github.com/rust-lang/rust-clippy/pull/11042)
* [clippy: `useless_vec`: add more tests and don't lint inside of macros](https://github.com/rust-lang/rust-clippy/pull/11094)
* [clippy: `useless_vec`: use the source span for initializer](https://github.com/rust-lang/rust-clippy/pull/11081)
* [clippy: don't lint `manual_let_else` in cases where `?` would work](https://github.com/rust-lang/rust-clippy/pull/10924)
* [clippy: don't lint code from external macros for 8 lints](https://github.com/rust-lang/rust-clippy/pull/11009)
* [clippy: make `eq_op` suggest `.is_nan()`](https://github.com/rust-lang/rust-clippy/pull/11051)
* [clippy: suggest `is_some_and` over `map().unwrap`](https://github.com/rust-lang/rust-clippy/pull/11030)
* [rust-analyzer: check Workspace Edit ResourceOps](https://github.com/rust-lang/rust-analyzer/pull/15101)
* [rust-analyzer: disable mir interpreter for targets with different pointer size from host](https://github.com/rust-lang/rust-analyzer/pull/15184)
* [rust-analyzer: editor/code: enable `noImplicitOverride` ts option](https://github.com/rust-lang/rust-analyzer/pull/15159)
* [rust-analyzer: editor/code: use `@tsconfig/strictest` to define type checking rules](https://github.com/rust-lang/rust-analyzer/pull/15154)
* [rust-analyzer: don't add panics to error jump list by default](https://github.com/rust-lang/rust-analyzer/pull/15186)
* [rust-analyzer: fix `self` and `super` path resolution in block modules](https://github.com/rust-lang/rust-analyzer/pull/15148)
* [rust-analyzer: fix data layout of reference to nested unsized structs](https://github.com/rust-lang/rust-analyzer/pull/15173)
* [rust-analyzer: fix layout of simd types and respect align in mir interpreter](https://github.com/rust-lang/rust-analyzer/pull/15194)
* [rust-analyzer: fix overflow checking in shift operator](https://github.com/rust-lang/rust-analyzer/pull/15189)
* [rust-analyzer: fix panic in `handle_code_action`](https://github.com/rust-lang/rust-analyzer/pull/15177)
* [rust-analyzer: fix realloc problem in allocating smaller amounts](https://github.com/rust-lang/rust-analyzer/pull/15168)
* [rust-analyzer: fix runnable detection for `#[tokio::test]`](https://github.com/rust-lang/rust-analyzer/pull/15157)
* [rust-analyzer: follow raw pointers in autoderef chain when resolving methods with custom receiver](https://github.com/rust-lang/rust-analyzer/pull/15118)
* [rust-analyzer: map our diagnostics to rustc and clippy's ones](https://github.com/rust-lang/rust-analyzer/pull/14990)
* [rust-analyzer: support `#[derive_const(Trait)]`](https://github.com/rust-lang/rust-analyzer/pull/15172)

### Rust Compiler Performance Triage

A quiet week, with a mixed set of improvements and regressions. Overall
slightly more improvements than regressions.

Triage done by **@simulacrum**.
Revision range: [b5e51db16..52d8c490](https://perf.rust-lang.org/?start=b5e51db16dfbf5685e32dfe2d9a835a5c695afe4&end=52d8c490a3aabe65cdd9f2d3aed95034dd5dbad7&absolute=false&stat=instructions%3Au)

4 Regressions, 4 Improvements, 2 Mixed; 0 of them in rollups

51 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-07-04.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Start working on a Rust specification](https://github.com/rust-lang/rfcs/pull/3355)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Enable coinduction support for Safe Transmute](https://github.com/rust-lang/rust/pull/113175)
* [disposition: close] [feat: split `unsafe_code` lint into lint group](https://github.com/rust-lang/rust/pull/108975)
* [disposition: merge] [Correct the Android stat struct definitions](https://github.com/rust-lang/rust/pull/113130)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Create a Testing sub-team](https://github.com/rust-lang/rfcs/pull/3455)
* [new] [Add `f16` and `f128` float types](https://github.com/rust-lang/rfcs/pull/3453)
* [new] [RFC: Nested Cargo packages](https://github.com/rust-lang/rfcs/pull/3452)
* [new] [Additional float types](https://github.com/rust-lang/rfcs/pull/3451)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-07-05 - 2023-08-02 🦀

### Virtual

* 2023-06-28 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Building Our Own 'Arc' in Rust (Atomics & Locks Chapter 6)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294184120/)
* 2023-06-28 | Virtual (Chicago, IL, US) | [Chicago Healthcare Cloud Technology Community](https://www.meetup.com/chicago-healthcare-tech-and-ai/)
    * [**Rust for Mission-Critical AI: A Journey into Healthcare's Safest Language**](https://www.meetup.com/chicago-healthcare-tech-and-ai/events/293278396)
* 2023-06-29 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/294294905)
* 2023-07-01 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 4: Protohackers Exercises Mob Coding (Problem II onwards)**](https://www.meetup.com/rust-noris/events/293800373)
* 2023-07-04 | Virtual (Berlin, DE) | [Berline.rs / OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfckbgb/)
* 2023-07-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfckbgb/)
* 2023-07-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309295)
* 2023-07-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfckbhb)
* 2023-07-06 | Virtual (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Rust y Haskell**](https://www.meetup.com/rust-mx/events/294152158)
* 2023-07-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfckbpb)
* 2023-07-13 - 2023-07-14 | Virtual | [Scientific Computing in Rust](https://scientificcomputing.rs/)
    * [**Scientific Computing in Rust workshop**](https://scientificcomputing.rs/)
* 2023-07-13 | Virtual (Edinburgh, UK) | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**Reasoning about Rust: an introduction to Rustdoc’s JSON format**](https://www.meetup.com/rust-edi/events/293820336/)
* 2023-07-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763486)
* 2023-07-20 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Iran Rust Meetup #12 - Ownership and Memory management**](https://rust-meetup.ir/2023/07/20/12th-meetup.html)

### Asia

* 2023-06-29 | Seoul, KR | [T-RUST meetup](https://www.meetup.com/t-rust-meetup/)
    * [**🦀 T-RUST Meetup 🦀**](https://www.meetup.com/t-rust-meetup/events/294280140/)

### Europe

* 2023-06-28 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/293732916)
* 2023-06-29 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup**](https://www.meetup.com/rust-meetup-augsburg/events/293566071/)
* 2023-06-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #37 at Samsung!**](https://www.meetup.com/copenhagen-rust-community/events/294024476)
* 2023-06-29 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna Meetup - June - final meetup before a summer break**](https://www.meetup.com/rust-vienna/events/294225540/)
* 2023-07-01 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**(Beginner) Rust Workshop**](https://www.meetup.com/rust-basel/events/293906330/)
* 2023-07-03 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Rust in the Linux Kernel - July Meetup**](https://www.meetup.com/rust-zurich/events/293322905)
* 2023-07-05 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #5**](https://www.meetup.com/fr-FR/rust-lyon/events/294325808)
* 2023-07-11 | Breda, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust: Advanced Graphics and User Interfaces**](https://www.meetup.com/rust-nederland/events/294199533/)
* 2023-07-11 | Virtual | [Mainmatter](https://mainmatter.com/)
    * [Web-based Services in Rust, 3-day Workshop with Stefan Baumgartner](https://rust-web-services-workshop.mainmatter.com/)
* 2023-07-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns***](https://www.meetup.com/reading-rust-workshop/events/mstlftyfckbrb/)

### North America

* 2023-06-21 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/BostonRust/)
    * [**Ball Square Rust Lunch, June 21**](https://www.meetup.com/BostonRust/events/293725119)
* 2023-06-22 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Learn How to Use cargo-semver-checks and Closure Traits to Write Better Code**](https://www.meetup.com/rust-nyc/events/294123104)
* 2023-06-24 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfcjbgc/)
* 2023-06-28 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/BostonRust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/BostonRust/events/293725559)
* 2023-06-29 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294223607)
* 2023-07-01 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfckbcb/)
* 2023-07-07 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Lunch**](https://www.meetup.com/deep-dish-rust/events/293794930/)
* 2023-07-12 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/294373345)
* 2023-07-12 | Waterloo, ON, CA | [Rust KW](https://www.meetup.com/rust-kw/)
    * [**Overengineering FizzBuzz**](https://www.meetup.com/rust-kw/events/294355516/)
* 2023-07-13 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**July Meetup**](https://www.meetup.com/seattle-rust-user-group/events/294191599/)
* 2023-07-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfckbxb)

### Oceania

* 2023-07-11 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/294447461/)
* 2023-07-11 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) July 2023 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/294274774/)

### South America

* 2023-07-04 | Medellín, CO | [Rust Medellín](https://www.meetup.com/rust-medellin/)
    * [**Introduccion a rust, ownership and safety code**](https://www.meetup.com/rust-medellin/events/294438119/)

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

> I'm not here to tell you that Rust is the best language....... you should have figured that out by now. 

– [Jester Hartman on youtube](https://www.youtube.com/watch?v=TGfQu0bQTKc)

Thanks to [newpavlov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1443) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
