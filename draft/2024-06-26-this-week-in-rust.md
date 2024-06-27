Title: This Week in Rust 553
Number: 553
Date: 2024-06-26
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

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

### Project/Tooling Updates
* [ratatui - v0.27.0](https://ratatui.rs/highlights/v027/)
* [Introduction - ChoRus](https://lsd-ucsc.github.io/ChoRus/)
* [uuid now properly supports version 7 counters](https://kodraus.github.io/rust/2024/06/24/uuid-v7-counters.html)
* [Godot-Rust - June 2024 update](https://godot-rust.github.io/dev/june-2024-update/)
* [piggui v0.2.0](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.2.0) 

- [git-cliff 2.4.0 is released! (highly customizable changelog generator)](https://git-cliff.org/blog/2.4.0)

### Observations/Thoughts
* [Claiming, auto and otherwise](https://smallcultfollowing.com/babysteps/blog/2024/06/21/claim-auto-and-otherwise/)
* [Ownership](https://without.boats/blog/ownership/)
* [Puzzle: Sharing declarative args between top level and subcommand using Clap](https://gribnau.dev/posts/puzzle-sharing-declarative-args-between-top-level-and-subcommand/)
* [Will Rust be alive in 10 years?](https://tweedegolf.nl/en/blog/128/rust-in-ten-years)
* [Why WebAssembly came to the Backend (Wasm in the wild part 3)](https://www.jakobmeier.ch/wasm-road-2)
* [in-place construction seems surprisingly simple?](https://blog.yoshuawuyts.com/in-place-construction-seems-surprisingly-simple/)
* [Igneous Linearizer](https://domain-j.com/Igneous-Linearizer/uuid/9e30337c-b890-4fd9-a0bd-51a7aa6e65b0)
* [Life in the FastLanes](https://blog.spiraldb.com/life-in-the-fastlanes/)

* [Rust's concurrency model vs Go's concurrency model: stackless vs stackfull coroutines](https://kerkour.com/rust-vs-go-concurrency-models-stackfull-vs-stackless-coroutines)

### Rust Walkthroughs
* [Master Rust by Playing Video Games!](https://jonte-osterberg.medium.com/master-rust-by-playing-video-games-cf5f7d8b1e28)
* [Tokio Waker Instrumentation](https://hegdenu.net/posts/tokio-waker-instrumentation/)
* [Build with Naz : Comprehensive guide to nom parsing](https://developerlife.com/2023/02/20/guide-to-nom-parsing/)
* [Running a TLC5940 with an ESP32 using the RMT peripheral](https://wapl.es/esp32-tlc5940-rmt/)
* [Rust Data-Structures: What is a CIDR trie and how can it help you?](https://d34dl0ck.me/rust-bites-cidr-trie/index.html)

* [Rust patterns: Micro SDKs](https://kerkour.com/rust-patterns-micro-sdks)

### Research
* [When Is Parallelism Fearless and Zero-Cost with Rust?](https://dl.acm.org/doi/10.1145/3626183.3659966)

### Miscellaneous
* [An Interview with Luca Palmieri of Mainmatter](https://filtra.io/rust-mainmatter-jun-24)

## Crate of the Week

This week's crate is [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), a cargo subcommand to install crates from binaries out of their github releases.

Thanks to [Jiahao XU](https://users.rust-lang.org/t/crate-of-the-week/2704/1317) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No calls for testing were issued this week.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No calls for testing were issued this week.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No calls for testing were issued this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [cargo-generate - RFC on reading toml values into placeholders](https://github.com/cargo-generate/cargo-generate/issues/770)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

428 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-06-18..2024-06-25

* [`hir_typeck`: be more conservative in making "note caller chooses ty param" note](https://github.com/rust-lang/rust/pull/126558)
* [`rustc_type_ir`: Omit some `struct` fields from Debug output](https://github.com/rust-lang/rust/pull/126656)
* [account for things that optimize out in inlining costs](https://github.com/rust-lang/rust/pull/126578)
* [actually taint `InferCtxt` when a fulfillment error is emitted](https://github.com/rust-lang/rust/pull/126620)
* [add `#[rustc_dump_{predicates,item_bounds}]`](https://github.com/rust-lang/rust/pull/126686)
* [add hard error and migration lint for unsafe attrs](https://github.com/rust-lang/rust/pull/126177)
* [allow "C-unwind" fn to have C variadics](https://github.com/rust-lang/rust/pull/126843)
* [allow constraining opaque types during subtyping in the trait system](https://github.com/rust-lang/rust/pull/125447)
* [allow constraining opaque types during various unsizing casts](https://github.com/rust-lang/rust/pull/125610)
* [allow tracing through `item_bounds` query invocations on opaques](https://github.com/rust-lang/rust/pull/126674)
* [ban `ArrayToPointer` and `MutToConstPointer` from runtime MIR](https://github.com/rust-lang/rust/pull/126308)
* [change a `DefineOpaqueTypes::No` to `Yes` in diagnostics code](https://github.com/rust-lang/rust/pull/126675)
* [collect attrs in const block expr](https://github.com/rust-lang/rust/pull/126735)
* [coverage: add debugging flag `-Zcoverage-options=no-mir-spans`](https://github.com/rust-lang/rust/pull/126587)
* [coverage: overhaul validation of the `#[coverage(..)]` attribute](https://github.com/rust-lang/rust/pull/126682)
* [do not allow safe/unsafe on static and fn items](https://github.com/rust-lang/rust/pull/126758)
* [don't ICE when encountering an extern type field during validation](https://github.com/rust-lang/rust/pull/126833)
* [fix: break inside async closure has incorrect span for enclosing closure](https://github.com/rust-lang/rust/pull/125078)
* [E0308: mismatched types, when expr is in an arm's body, don't add semicolon ';' at the end of it](https://github.com/rust-lang/rust/pull/126455)
* [improve conflict marker recovery](https://github.com/rust-lang/rust/pull/126125)
* [improve tip for inaccessible traits](https://github.com/rust-lang/rust/pull/125852)
* [interpret: better error when we ran out of memory](https://github.com/rust-lang/rust/pull/126583)
* [make async drop code more consistent with regular drop code](https://github.com/rust-lang/rust/pull/126594)
* [make edition dependent `:expr` macro fragment act like the edition-dependent `:pat` fragment does](https://github.com/rust-lang/rust/pull/126700)
* [make pretty printing for `f16` and `f128` consistent](https://github.com/rust-lang/rust/pull/126654)
* [match lowering: expand or-candidates mixed with candidates above](https://github.com/rust-lang/rust/pull/126553)
* [show notice about "never used" of Debug for `enum`](https://github.com/rust-lang/rust/pull/124460)
* [stop sorting `Span`s' `SyntaxContext`, as that is incompatible with incremental](https://github.com/rust-lang/rust/pull/123165)
* [suggest inline const blocks for array initialization](https://github.com/rust-lang/rust/pull/126899)
* [suggest removing unused tuple fields if they are the last fields](https://github.com/rust-lang/rust/pull/124580)
* [uplift next trait solver to `rustc_next_trait_solver`](https://github.com/rust-lang/rust/pull/126614)
* [add `f16` and `f128`](https://github.com/rust-lang/chalk/pull/811)
* [miri: /miri: nicer error when building miri-script fails](https://github.com/rust-lang/miri/pull/3700)
* [miri: `unix/foreign_items`: move getpid to the right part of the file](https://github.com/rust-lang/miri/pull/3705)
* [miri: don't rely on libc existing on Windows](https://github.com/rust-lang/miri/pull/3695)
* [miri: fix ICE caused by seeking past `i64::MAX`](https://github.com/rust-lang/miri/pull/3689)
* [miri: implement LLVM x86 adx intrinsics](https://github.com/rust-lang/miri/pull/3690)
* [miri: implement LLVM x86 bmi intrinsics](https://github.com/rust-lang/miri/pull/3674)
* [miri: nicer batch file error when building miri-script fails](https://github.com/rust-lang/miri/pull/3703)
* [miri: use strict ops instead of checked ops](https://github.com/rust-lang/miri/pull/3694)
* [save 2 pointers in `TerminatorKind` (96 → 80 bytes)](https://github.com/rust-lang/rust/pull/126784)
* [add `SliceLike` to `rustc_type_ir`, use it in the generic solver code (+ some other changes)](https://github.com/rust-lang/rust/pull/126813)
* [`std::unix::fs`: copy simplification for apple](https://github.com/rust-lang/rust/pull/126807)
* [`std::unix::os::home_dir`: fallback's optimisation](https://github.com/rust-lang/rust/pull/126854)
* [replace `f16` and `f128` pattern matching stubs with real implementations](https://github.com/rust-lang/rust/pull/123088)
* [add `PidFd::`{`kill`, `wait`, `try_wait`}](https://github.com/rust-lang/rust/pull/124101)
* [also get `add nuw` from `uN::checked_add`](https://github.com/rust-lang/rust/pull/126852)
* [generalize {`Rc`, `Arc`}`::make_mut()` to unsized types](https://github.com/rust-lang/rust/pull/116113)
* [implement `array::repeat`](https://github.com/rust-lang/rust/pull/119127)
* [make `Option::as_[mut_]slice` `const`](https://github.com/rust-lang/rust/pull/126711)
* [rename `std::fs::try_exists` to `std::fs::exists` and stabilize `fs_try_exists`](https://github.com/rust-lang/rust/pull/126140)
* [replace sort implementations](https://github.com/rust-lang/rust/pull/124032)
* [return opaque type from `PanicInfo::message()`](https://github.com/rust-lang/rust/pull/126330)
* [stabilise `c_unwind`](https://github.com/rust-lang/rust/pull/116088)
* [std: refactor the thread-local storage implementation](https://github.com/rust-lang/rust/pull/126523)
* [hashbrown: implement XxxAssign operations on HashSets](https://github.com/rust-lang/hashbrown/pull/529)
* [hashbrown: replace "ahash" with "default-hasher" in Cargo features](https://github.com/rust-lang/hashbrown/pull/533)
* [cargo toml: warn when edition is unset, even when MSRV is unset](https://github.com/rust-lang/cargo/pull/14110)
* [cargo: add `CodeFix::apply_solution` and impl `Clone`](https://github.com/rust-lang/cargo/pull/14092)
* [cargo: make `-Cmetadata` consistent across platforms](https://github.com/rust-lang/cargo/pull/14107)
* [cargo: simplify checking feature syntax](https://github.com/rust-lang/cargo/pull/14106)
* [cargo: simplify checking for dependency cycles](https://github.com/rust-lang/cargo/pull/14089)
* [cargo test: add auto-redaction for not found error](https://github.com/rust-lang/cargo/pull/14124)
* [cargo test: auto-redact file number](https://github.com/rust-lang/cargo/pull/14121)
* [rustdoc: add support for `missing_unsafe_on_extern` feature](https://github.com/rust-lang/rust/pull/126761)
* [implement `use<>` formatting in rustfmt](https://github.com/rust-lang/rust/pull/126754)
* [rustfmt: format safety keywords on static items](https://github.com/rust-lang/rustfmt/pull/6204)
* [remove stray println from rustfmt's `rewrite_static`](https://github.com/rust-lang/rust/pull/126888)
* [resolve clippy `f16` and `f128 unimplemented!`/`FIXME`s](https://github.com/rust-lang/rust/pull/126636)
* [clippy: `missing_const_for_fn`: add machine-applicable suggestion](https://github.com/rust-lang/rust-clippy/pull/12930)
* [clippy: add applicability filter to lint list page](https://github.com/rust-lang/rust-clippy/pull/12655)
* [clippy: add more types to `is_from_proc_macro`](https://github.com/rust-lang/rust-clippy/pull/12942)
* [clippy: don't lint `implicit_return` on proc macros](https://github.com/rust-lang/rust-clippy/pull/12963)
* [clippy: fix incorrect suggestion for `manual_unwrap_or_default`](https://github.com/rust-lang/rust-clippy/pull/12961)
* [clippy: resolve `clippy::invalid_paths` on `bool::then`](https://github.com/rust-lang/rust-clippy/pull/12965)
* [clippy: unnecessary call to min/max method](https://github.com/rust-lang/rust-clippy/pull/12368)
* [rust-analyzer: complete async keyword](https://github.com/rust-lang/rust-analyzer/pull/17459)
* [rust-analyzer: check that Expr is none before adding fixup](https://github.com/rust-lang/rust-analyzer/pull/17464)
* [rust-analyzer: add `toggleLSPLogs` command](https://github.com/rust-lang/rust-analyzer/pull/17438)
* [rust-analyzer: add space after specific keywords in completion](https://github.com/rust-lang/rust-analyzer/pull/17431)
* [rust-analyzer: filter builtin macro expansion](https://github.com/rust-lang/rust-analyzer/pull/17419)
* [rust-analyzer: don't remove parentheses for calls of function-like pointers that are members of a `struct` or union](https://github.com/rust-lang/rust-analyzer/pull/17471)
* [rust-analyzer: ensure there are no cycles in the `source_root_parent_map`](https://github.com/rust-lang/rust-analyzer/pull/17457)
* [rust-analyzer: fix IDE features breaking in some attr macros](https://github.com/rust-lang/rust-analyzer/pull/17462)
* [rust-analyzer: fix flycheck panicking when cancelled](https://github.com/rust-lang/rust-analyzer/pull/17461)
* [rust-analyzer: handle character boundaries for wide chars in `extend_selection`](https://github.com/rust-lang/rust-analyzer/pull/17426)
* [rust-analyzer: improve hover text in unlinked file diagnostics](https://github.com/rust-lang/rust-analyzer/pull/17411)
* [rust-analyzer: only show unlinked-file diagnostic on first line during startup](https://github.com/rust-lang/rust-analyzer/pull/17415)
* [rust-analyzer: pattern completions in let-stmt](https://github.com/rust-lang/rust-analyzer/pull/17481)
* [rust-analyzer: use `ItemInNs::Macros` to convert ModuleItem to ItemInNs](https://github.com/rust-lang/rust-analyzer/pull/17469)
* [rust-analyzer: remove panicbit.cargo extension warning](https://github.com/rust-lang/rust-analyzer/pull/17456)
* [rust-analyzer: simplify some term search tactics](https://github.com/rust-lang/rust-analyzer/pull/17478)
* [rust-analyzer: term search: new tactic for associated item constants](https://github.com/rust-lang/rust-analyzer/pull/17449)

### Rust Compiler Performance Triage

Mostly a number of improvements driven by [MIR inliner improvements], with a small number
benchmarks having a significant regression due to improvements in
[sort algorithms], which are runtime improvements at the cost of usually slight or
neutral compile time regressions, with outliers in a few cases.

[MIR inliner improvements]: https://github.com/rust-lang/rust/pull/126578
[sort algorithms]: https://github.com/rust-lang/rust/pull/124032

Triage done by **@simulacrum**.
Revision range: [c2932aaf..c3d7fb39](https://perf.rust-lang.org/?start=c2932aaf9d20acbc9259c762f1a06f8767c6f13f&end=c3d7fb398569407350abe044e786bc7890c90397&absolute=false&stat=instructions%3Au)

[See full report for details](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-06-23.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:
* [Change crates.io policy to not offer crate transfer mediation](https://github.com/rust-lang/rfcs/pull/3646)
* [UnsafePinned: allow aliasing of pinned mutable references](https://github.com/rust-lang/rfcs/pull/3467)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [RFC: Return Type Notation](https://github.com/rust-lang/rfcs/pull/3654)
* [disposition: merge] [Add a general mechanism of setting RUSTFLAGS in Cargo for the root crate only](https://github.com/rust-lang/rfcs/pull/3310)
* [disposition: close] [Allow specifying dependencies for individual artifacts](https://github.com/rust-lang/rfcs/pull/2887)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [`#![crate_name = EXPR]` semantically allows `EXPR` to be a macro call but otherwise mostly ignores it](https://github.com/rust-lang/rust/issues/122001)
* [disposition: merge] [Add nightly style guide section for `precise_capturing` `use<>` syntax](https://github.com/rust-lang/rust/pull/126753)
* [disposition: merge] [Tracking issue for PanicInfo::message](https://github.com/rust-lang/rust/issues/66745)
* [disposition: merge] [Tracking issue for Cell::update](https://github.com/rust-lang/rust/issues/50186)
* [disposition: \<unspecified\>] [Tracking issue for core::arch::{x86, x86_64}::has_cpuid](https://github.com/rust-lang/rust/issues/60123)
* [disposition: merge] [Syntax for precise capturing: `impl Trait + use<..>`](https://github.com/rust-lang/rust/issues/125836)
* [disposition: merge] [Remove the `box_pointers` lint.](https://github.com/rust-lang/rust/pull/126018)
* [disposition: merge] [Re-implement a type-size based limit](https://github.com/rust-lang/rust/pull/125507)
* [disposition: merge] [Tracking Issue for `duration_abs_diff`](https://github.com/rust-lang/rust/issues/117618)
* [disposition: merge] [Check alias args for WF even if they have escaping bound vars](https://github.com/rust-lang/rust/pull/123737)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference Tracking Issues or PRs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Cargo structured syntax for feature dependencies on crates](https://github.com/rust-lang/rfcs/pull/3663)
* [new] [Mergeable rustdoc cross-crate info](https://github.com/rust-lang/rfcs/pull/3662)
* [new] [Add "crates.io: Crate Deletions" RFC](https://github.com/rust-lang/rfcs/pull/3660)

## Upcoming Events

Rusty Events between 2024-06-26 - 2024-07-24 🦀

### Virtual
* 2024-06-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897826/)
* 2024-07-02 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191673/)
* 2024-07-02 | Hybrid - Virtual and In-person (Los Angeles, CA, US) | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/)
    * [**Rust LA Reboot**](https://www.meetup.com/rust-los-angeles/events/301645611/)
* 2024-07-03 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Build Web Apps with Rust and Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 2024-07-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328025/)
* 2024-07-04 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298488820/)
* 2024-07-06 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-07-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346976/)
* 2024-07-10 | Virtual | [Centre for eResearch](https://www.eventbrite.co.nz/o/centre-for-eresearch-75893560993)
    * [**Research Computing With The Rust Programming Language**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-908002037537?aff=ebdssbdestsearch&keep_tld=1)
* 2024-07-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897842/)
* 2024-07-11 | Hybrid - Virtual and In-person (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 2024-07-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/298076822/)
* 2024-07-11 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Reading JSON files in Rust (English)**](https://www.meetup.com/code-mavens/events/301636580/)
* 2024-07-16 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Web development in Rust using Rocket - part 2 (English)**](https://www.meetup.com/code-mavens/events/301736709/)
* 2024-07-17 | Hybrid - Virtual and In-person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)

### Asia
* 2024-06-30 | Kyoto, JP | [Kyoto Rust](https://www.meetup.com/kyoto-rust/)
    * [**Rust Talk: Cross Platform Apps**](https://www.meetup.com/kyoto-rust/events/301499550/)

### Europe
* 2024-06-27 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288965/)
* 2024-06-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #48 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/300458252/)
* 2024-07-10 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup - July**](https://www.meetup.com/reading-rust-workshop/events/301359031/)
* 2024-07-11 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (July 2024)**](https://www.meetup.com/rust-prague/events/301227195)
* 2024-07-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Building a REST API in Rust using Axum, SQLx and SQLite**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/301716171/)
* 2024-07-16 | Mannheim, DE | [Hackschool - Rhein-Neckar](https://www.meetup.com/hackschool-rhein-neckar)
    * [**Nix Your Bugs & Rust Your Engines #4**](https://www.meetup.com/hackschool-rhein-neckar/events/301504325/)

### North America
* 2024-06-26 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/301066942/)
* 2024-06-27 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613483/)
* 2024-06-27 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Holding Pattern**](https://www.meetup.com/music-city-rust-developers/events/301411746/)
* 2024-06-27 | St. Louis, MO, US | [STl Rust](https://www.meetup.com/stl-rust/)
    * [**Meet and Greet Hacker session**](https://www.meetup.com/stl-rust/events/301321974/)
* 2024-07-02 | Hybrid - Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/)
    * [**Rust LA Reboot**](https://www.meetup.com/rust-los-angeles/events/301645611/)
* 2024-07-05 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston University Rust Lunch, July 5**](https://www.meetup.com/bostonrust/events/301549737/)
* 2024-07-11 | Hybrid - Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 2024-07-11 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613495/)
* 2024-07-17 | Hybrid - Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)

### Oceania


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

> Rust has no theoretical inconsistencies... a remarkable achievement...

– [Simon Peyton-Jones on YouTube](https://youtu.be/UBgam9XUHs0?t=2756)

Thanks to [ZiCog](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1579) for the suggestion and [Simon Farnsworth](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1580) for the improved link!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
