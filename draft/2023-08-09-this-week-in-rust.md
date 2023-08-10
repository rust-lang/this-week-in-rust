Title: This Week in Rust 507
Number: 507
Date: 2023-08-09
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
* [2022 Annual Rust Survey Results](https://blog.rust-lang.org/2023/08/07/Rust-Survey-2023-Results.html)
* [Security advisory for Cargo (CVE-2023-38497)](https://blog.rust-lang.org/2023/08/03/cve-2023-38497.html)
* [Announcing Rust 1.71.1](https://blog.rust-lang.org/2023/08/03/Rust-1.71.1.html)
* [Rotating Rust compiler team leadership](https://blog.rust-lang.org/inside-rust/2023/08/02/rotating-compiler-leads.html)

### Foundation
* [Announcing Speakers & Schedule for Inaugural Rust Global Event](https://foundation.rust-lang.org/news/announcing-speakers-schedule-for-inaugural-rust-global-event/)

### Newsletters
* [This Month in Rust OSDev: July 2023](https://rust-osdev.com/this-month/2023-07/)
* [Rust Nigeria Issue 20](https://rustnigeria.curated.co/issues/20)

### Project/Tooling Updates
* [Turbocharging Rust Code Verification](https://model-checking.github.io/kani-verifier-blog/2023/08/03/turbocharging-rust-code-verification.html)
* [Changelog #193](https://rust-analyzer.github.io/thisweek/2023/08/07/changelog-193.html)

### Observations/Thoughts
* [No telemetry in the Rust compiler: metrics without betraying user privacy](https://estebank.github.io/rustc-metrics.html)
* [A failed experiment with Rust static dispatch](https://jmmv.dev/2023/08/rust-static-dispatch-failed-experiment.html)
* [nesting allocators](https://blog.yoshuawuyts.com/nesting-allocators/)
* [Allocator trait 1: Let’s talk about the Allocator trait](https://shift.click/blog/allocator-trait-talk/)
* [How to improve Rust compiler’s CI in 2023](https://kobzol.github.io/rust/rustc/2023/07/30/optimizing-rust-ci-2023.html)
* [video] [Andreas Monitzer  - Bevy-ECS explained - Rust Vienna June 2023](https://www.youtube.com/watch?v=TBjmRmjr4JA)

* [Optimizing Rust Enum `Debug`-ing with Perfect Hashing](https://swatinem.de/blog/optimizing-enums/)

### Rust Walkthroughs
* [Let's Build a Cargo Compatible Build Tool - Part 1](https://blog.mgattozzi.dev/freight-part-1/)
* [Instrumenting Axum projects](https://determinate.systems/posts/instrumenting-axum)
* [Rust Server Components](https://anto.pt/articles/rust-server-components)
* [Optimizing Rust Enum `Debug`-ing with Perfect Hashing](https://swatinem.de/blog/optimizing-enums/)
* [Running a Bevy game in SvelteKit](https://sneakycrow.dev/blog/2023-07-30-bevy-game-in-svelte-kit)
* [ESP32 Standard Library Embedded Rust: Timers](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-timers)


### Research

### Miscellaneous
* [Shuttle Launchpad #5: Our first foray into traits!](https://www.shuttle.rs/launchpad/issues/2023-28-07-issue-05-Traits-Image-Processing)
* [video] [Rust API design: the curious case of Result](https://www.youtube.com/watch?v=lYjMp9obcZ0)
* [video] [A Tour of Iced 0.10](https://www.youtube.com/watch?v=XrR4VCKB0cQ)
* [video] [5 programs you can't compile with Rust](https://www.youtube.com/watch?v=HbXrStY68_0)
* [video] [Rich Terminal Interfaces with Ratatui](https://www.youtube.com/watch?v=pgFCjtwPBYI)

## Crate of the Week

This week's crate is [deep_causality](https://github.com/deepcausality-rs/deep_causality), a hyper-geometric computational causality library.

Thanks to [Marvin Hansen](https://users.rust-lang.org/t/crate-of-the-week/2704/1221) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [zerocopy - Install OpenSSF Scorecard and consider adopting its recommendations](https://github.com/google/zerocopy/issues/230)
* [Ockam - Add icons to the menu items in Tauri system tray app 1](https://github.com/build-trust/ockam/issues/5491)
* [Ockam - Improve docs of `ockam completion` clap command to specify how to use it](https://github.com/build-trust/ockam/issues/5565)
* [Ockam - Remove unused `Error` enum members and avoid appearing of such members in the future 1](https://github.com/build-trust/ockam/issues/5564)
* [Hyperswitch - Add Create Merchant and Create Merchant Key Store in a DB transaction](https://github.com/juspay/hyperswitch/issues/1793)
* [Hyperswitch - Use proxy exclusion instead of a separate proxied client](https://github.com/juspay/hyperswitch/issues/1039)
* [Hyperswitch - Schedule webhook for retry](https://github.com/juspay/hyperswitch/issues/217)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

417 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-07-31..2023-08-07

* [WASI threads, implementation of wasm32-wasi-preview1-threads target](https://github.com/rust-lang/rust/pull/112922)
* [set `max_atomic_width` for AVR to 16](https://github.com/rust-lang/rust/pull/114495)
* [set `max_atomic_width` for sparc-unknown-linux-gnu to 32](https://github.com/rust-lang/rust/pull/114496)
* [re-enable atomic loads and stores for all RISC-V targets](https://github.com/rust-lang/rust/pull/98333)
* [account for `Rc` and `Arc` when suggesting to clone](https://github.com/rust-lang/rust/pull/114477)
* [account for macros when suggesting a new `let` binding](https://github.com/rust-lang/rust/pull/114178)
* [avoid invalid NaN lint machine-applicable suggestion in const context](https://github.com/rust-lang/rust/pull/114486)
* [avoid wrong code suggesting for attribute macro](https://github.com/rust-lang/rust/pull/107254)
* [change default panic handler message format](https://github.com/rust-lang/rust/pull/112849)
* [parser: more friendly hints for handling `async move` in the 2015 edition](https://github.com/rust-lang/rust/pull/114237)
* [coverage: consolidate FFI types into one module](https://github.com/rust-lang/rust/pull/114360)
* [coverage: replace `ExpressionOperandId` with `enum Operand`](https://github.com/rust-lang/rust/pull/113428)
* [detect trait upcasting through `struct` tail unsizing in new solver select](https://github.com/rust-lang/rust/pull/114200)
* [don't ICE on higher ranked hidden types](https://github.com/rust-lang/rust/pull/113575)
* [fix ICE failed to get layout for ReferencesError](https://github.com/rust-lang/rust/pull/114450)
* [fix invalid slice coercion suggestion reported in turbofish](https://github.com/rust-lang/rust/pull/114322)
* [fix suggestion spans for expr from macro expansions](https://github.com/rust-lang/rust/pull/112043)
* [fix the span in the suggestion of remove question mark](https://github.com/rust-lang/rust/pull/114403)
* [fix wrong span for trait selection failure error reporting](https://github.com/rust-lang/rust/pull/113945)
* [expand, rename and improve `incorrect_fn_null_checks` lint](https://github.com/rust-lang/rust/pull/113657)
* [improve `invalid_reference_casting` lint](https://github.com/rust-lang/rust/pull/112431)
* [improve diagnostic for wrong borrow on binary operations](https://github.com/rust-lang/rust/pull/114288)
* [improve spans for indexing expressions](https://github.com/rust-lang/rust/pull/114434)
* [infer type in irrefutable slice patterns with fixed length as array](https://github.com/rust-lang/rust/pull/113199)
* [interpret: fix alignment handling for Repeat expressions](https://github.com/rust-lang/rust/pull/114296)
* [make `Debug` representations of `[Lazy, Once]*[Cell, Lock]` consistent with `Mutex` and `RwLock`](https://github.com/rust-lang/rust/pull/109318)
* [make `unconditional_recursion` warning detect recursive drops](https://github.com/rust-lang/rust/pull/113902)
* [make lint missing-copy-implementations honor negative `Copy` impls](https://github.com/rust-lang/rust/pull/114248)
* [make test harness lint about unnnameable tests](https://github.com/rust-lang/rust/pull/114414)
* [only consider places with the same local in `each_borrow_involving_path`](https://github.com/rust-lang/rust/pull/111753)
* [only unpack tupled args in inliner if we expect args to be unpacked](https://github.com/rust-lang/rust/pull/110833)
* [const validation: point at where we found a pointer but expected an integer](https://github.com/rust-lang/rust/pull/114372)
* [optimize `Iterator` implementation for `&mut impl Iterator + Sized`](https://github.com/rust-lang/rust/pull/111200)
* [perform OpaqueCast field projection on HIR, too](https://github.com/rust-lang/rust/pull/114022)
* [remove `constness` from `TraitPredicate`](https://github.com/rust-lang/rust/pull/114202)
* [resolve before canonicalization in new solver, ICE if unresolved](https://github.com/rust-lang/rust/pull/114355)
* [resolve visibility paths as modules not as types](https://github.com/rust-lang/rust/pull/109348)
* [reword `confusable_idents` lint](https://github.com/rust-lang/rust/pull/114472)
* [rework upcasting confirmation to support upcasting to fewer projections in target bounds](https://github.com/rust-lang/rust/pull/114036)
* [specify macro is invalid in certain contexts](https://github.com/rust-lang/rust/pull/113999)
* [steal MIR for CTFE when possible](https://github.com/rust-lang/rust/pull/114502)
* [strip unexpected debuginfo from `libLLVM.so` and `librustc_driver.so` when not requesting any debuginfo](https://github.com/rust-lang/rust/pull/114305)
* [suggests turbofish in patterns](https://github.com/rust-lang/rust/pull/114300)
* [add allocation to SMIR](https://github.com/rust-lang/rust/pull/114466)
* [add missing rvalues to SMIR](https://github.com/rust-lang/rust/pull/114165)
* [add trait decls to SMIR](https://github.com/rust-lang/rust/pull/114485)
* [miri-script and cargo-miri cleanups](https://github.com/rust-lang/miri/pull/3006)
* [miri-script: simplify flag computation a bit](https://github.com/rust-lang/miri/pull/3005)
* [miri: fix error on dangling pointer inbounds offset](https://github.com/rust-lang/rust/pull/114333)
* [miri: add some SB and TB tests](https://github.com/rust-lang/miri/pull/3004)
* [miri: avoid infinite recursion for auto-fmt and auto-clippy](https://github.com/rust-lang/miri/pull/3009)
* [miri: tree borrows: consider some retags as writes for the purpose of data races](https://github.com/rust-lang/miri/pull/3013)
* [do not run ConstProp on `mir_for_ctfe`](https://github.com/rust-lang/rust/pull/114459)
* [add a new `compare_bytes` intrinsic instead of calling `memcmp` directly](https://github.com/rust-lang/rust/pull/114382)
* [some parser and AST cleanups](https://github.com/rust-lang/rust/pull/114353)
* [convert builtin "global" late lints to run per module](https://github.com/rust-lang/rust/pull/113734)
* [use parking lot's rwlock even without parallel-rustc](https://github.com/rust-lang/rust/pull/114283)
* [`parent_module_from_def_id` does not need to be a query](https://github.com/rust-lang/rust/pull/114516)
* [`rustc_data_structures`: Simplify `base_n::push_str`](https://github.com/rust-lang/rust/pull/114306)
* [`rustc_span`: Hoist lookup sorted by words out of the loop](https://github.com/rust-lang/rust/pull/114395)
* [`cg_llvm`: stop identifying ADTs in LLVM IR](https://github.com/rust-lang/rust/pull/114350)
* [filter out short-lived LLVM diagnostics before they reach the rustc handler](https://github.com/rust-lang/rust/pull/113339)
* [stabilize `abi_thiscall`](https://github.com/rust-lang/rust/pull/114562)
* [impl `SliceIndex<str>` for `(Bound<usize>, Bound<usize>)`](https://github.com/rust-lang/rust/pull/111081)
* [implement RefUnwindSafe for Backtrace](https://github.com/rust-lang/rust/pull/100455)
* [implement `Option::take_if`](https://github.com/rust-lang/rust/pull/98935)
* [`unix/kernel_copy.rs: copy_file_range_candidate` allows empty output files](https://github.com/rust-lang/rust/pull/114373)
* [regex-automata: fix incorrect offsets reported by reverse inner optimization](https://github.com/rust-lang/regex/pull/1063)
* [regex: fix memory usage regression for RegexSet with capture groups](https://github.com/rust-lang/regex/pull/1062)
* [cargo: bail out an error when using cargo: in custom build script](https://github.com/rust-lang/cargo/pull/12332)
* [cargo: display crate version on timings graph](https://github.com/rust-lang/cargo/pull/12420)
* [cargo: don't attempt to read a token from stdin if a cmdline token is provided](https://github.com/rust-lang/cargo/pull/12440)
* [cargo: fix CVE-2023-38497 for master](https://github.com/rust-lang/cargo/pull/12443)
* [cargo: fix printing multiple warning messages for unused fields in registries table](https://github.com/rust-lang/cargo/pull/12439)
* [cargo: refactor: migrate to `tracing`](https://github.com/rust-lang/cargo/pull/12458)
* [rustfmt: fix: add parenthesis around `..` closure if it's a method call receiver](https://github.com/rust-lang/rustfmt/pull/5842)
* [clippy: `ptr_as_ptr`: Take snippet instead of pretty printing type](https://github.com/rust-lang/rust-clippy/pull/11288)
* [clippy: `redundant_type_annotations`: only pass certain def kinds to `type_of`](https://github.com/rust-lang/rust-clippy/pull/11191)
* [clippy: `unnecessary_mut_passed`: don't lint in macro expansions](https://github.com/rust-lang/rust-clippy/pull/11269)
* [clippy: `unwrap_used`: Do not lint unwrapping on `!` or never-like enums](https://github.com/rust-lang/rust-clippy/pull/11252)
* [clippy: alphabetically order arms in `methods/mod.rs` match](https://github.com/rust-lang/rust-clippy/pull/11284)
* [clippy: fix `suspicious_xor_used_as_pow.rs` performance](https://github.com/rust-lang/rust-clippy/pull/11255)
* [clippy: new lint `ignored_unit_patterns`](https://github.com/rust-lang/rust-clippy/pull/11242)
* [clippy: new lints: `impossible_comparisons` and `redundant_comparisons`](https://github.com/rust-lang/rust-clippy/pull/10843)
* [clippy: suppress `question_mark` warning if `question_mark_used` is not allowed](https://github.com/rust-lang/rust-clippy/pull/11286)
* [rust-analyzer: allow match to matches assist to trigger on non-literal bool arms](https://github.com/rust-lang/rust-analyzer/pull/15376)
* [rust-analyzer: skip `doc(hidden)` default members](https://github.com/rust-lang/rust-analyzer/pull/15050)
* [rust-analyzer: don't provide `generate_default_from_new` when impl self ty is missing](https://github.com/rust-lang/rust-analyzer/pull/15406)
* [rust-analyzer: exclude non-identifier aliases from completion filtering text](https://github.com/rust-lang/rust-analyzer/pull/15348)
* [rust-analyzer: added remove unused imports assist](https://github.com/rust-lang/rust-analyzer/pull/14723)
* [rust-analyzer: fix unsized `struct` problems in mir eval](https://github.com/rust-lang/rust-analyzer/pull/15380)
* [rust-analyzer: don't provide `add_missing_match_arms` assist when upmapping match arm list failed](https://github.com/rust-lang/rust-analyzer/pull/15345)
* [rust-analyzer: remove unwraps from "Generate delegate trait"](https://github.com/rust-lang/rust-analyzer/pull/15397)
* [rust-analyzer: strip unused token ids from eager macro input token maps](https://github.com/rust-lang/rust-analyzer/pull/15367)
* [rust-analyzer: name change Import to Use in hir-def, add unused placeholder variants for UseId](https://github.com/rust-lang/rust-analyzer/pull/15378)
* [rust-analyzer: set the default status bar action to openLogs](https://github.com/rust-lang/rust-analyzer/pull/15391)
* [rust-analyzer: use the warning color when rust-analyzer is stopped](https://github.com/rust-lang/rust-analyzer/pull/15392)

### Rust Compiler Performance Triage

Overall a very positive last week, primarily due to an [upgrade to LLVM
17](https://github.com/rust-lang/rust/pull/114048) and some changes to [lint
execution](https://github.com/rust-lang/rust/pull/113734). Memory usage is down
[4-7%](https://perf.rust-lang.org/?start=828bdc2c26f5c95773c4ecf72870919f16417b66&end=443c3161dd04f4c1b656a626f9079921bee9c326&absolute=false&stat=max-rss&kind=percentfromfirst)
over the last week and wall times are down
[3-5%](https://perf.rust-lang.org/?start=828bdc2c26f5c95773c4ecf72870919f16417b66&end=443c3161dd04f4c1b656a626f9079921bee9c326&absolute=false&stat=wall-time&kind=percentfromfirst).

Triage done by **@simulacrum**.
Revision range: [828bdc2c..443c3161](https://perf.rust-lang.org/?start=828bdc2c26f5c95773c4ecf72870919f16417b66&end=443c3161dd04f4c1b656a626f9079921bee9c326&absolute=false&stat=instructions%3Au)

2 Regressions, 7 Improvements, 2 Mixed; 2 of them in rollups
64 artifact comparisons made in total

[Full report 7/22-8/1](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-08-01.md),
[Full report 8/1-8/8](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-08-08.md).

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
* [disposition: merge] [Specialize count for range iterators](https://github.com/rust-lang/rust/pull/112229)
* [disposition: merge] [Accept additional user-defined classes in fenced code blocks](https://github.com/rust-lang/rust/pull/110800)
* [disposition: merge] [Warn on inductive cycle in coherence leading to impls being considered not overlapping](https://github.com/rust-lang/rust/pull/114023)
* [disposition: close] [Named format arguments can be used as positional](https://github.com/rust-lang/rust/issues/93415)
* [disposition: merge] [Tracking Issue for `const_collections_with_hasher`](https://github.com/rust-lang/rust/issues/102575)
* [disposition: merge] [Document soundness of Integer -> Pointer -> Integer conversions in `const` contexts.](https://github.com/rust-lang/rust/pull/113510)
* [disposition: merge] [Allow explicit `#[repr(Rust)]`](https://github.com/rust-lang/rust/pull/114201)
* [disposition: merge] [Tracking issue for thread local Cell methods](https://github.com/rust-lang/rust/issues/92122)
* [disposition: merge] [Implement From\<OwnedFd/Handle\> for ChildStdin/out/err object](https://github.com/rust-lang/rust/pull/98704)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [CPU feature detection in core](https://github.com/rust-lang/rfcs/pull/3469)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-08-09 - 2023-09-06 🦀

### Virtual

* 2023-08-09 | Virtual (New York, NY, US) | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Helping Rust Developers See Data Dependencies in the IDE**](https://www.meetup.com/rust-nyc/events/295078036)
* 2023-08-10 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust and Tell - August Edition**](https://berline.rs/2023/08/10/rust-and-tell-august-edition.html)
* 2023-08-10 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfclbnb/)
* 2023-08-10 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732653)
* 2023-08-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfclbtb/)
* 2023-08-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/293553331/)
* 2023-08-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/kmhpftyfclbvb/)
* 2023-08-17 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 32nd Edition**](https://www.meetup.com/rust-linz/events/294718621/)
* 2023-08-17 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295051663/)
* 2023-08-22 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**Rust, Serverless and AWS**](https://www.meetup.com/Rust-Dublin/events/294587280/)
* 2023-09-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295207389/)
* 2023-09-05 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 4 - hybrid**](https://www.meetup.com/rust-munich/events/294186101/)
* 2023-09-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/294049877)

### Asia

* 2023-08-09 | Kuala Lumpur, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup August 2023**](https://forms.gle/tL68U1PZF5bAV1LY7)
* 2023-08-10 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Bring Your Laptop: The Great Oxidation Event**](https://www.meetup.com/tokyo-rust-meetup/events/295275684)

### Europe

* 2023-08-17 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/294201562/)
* 2023-08-19 | Augsburg, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/)
    * [**Rust Frontend Workshop (Yew + WebAssembly + Axum)**](https://www.meetup.com/rust-rhein-main/events/295298582/)
* 2023-08-22 | Helsinki, FI | [Finland Rust Meetup](https://www.meetup.com/helsinki-rust-meetup-group)
    * [**Helsink Rustaceans First Gathering**](https://www.meetup.com/helsinki-rust-meetup-group/events/294616573/)
* 2023-08-23 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**LDN Talks Aug 2023: Rust London x RNL (The next Frontier in App Development)**](https://www.meetup.com/rust-london-user-group/events/295338396/)
* 2023-08-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus Hack and Learn at Trifork**](https://www.meetup.com/rust-aarhus/events/293950871/)
* 2023-08-31 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #2**](https://www.meetup.com/rust-meetup-augsburg/events/294538503/)
* 2023-09-05 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 4 - hybrid**](https://www.meetup.com/rust-munich/events/294186101/)

### North America

* 2023-08-10 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294911475/)
* 2023-08-10 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Building a simplified JVM in Rust**](https://www.meetup.com/utah-rust/events/294972766/)
* 2023-08-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfclbtb)
* 2023-08-15 | Seattle, WA, US | [Seattle Rust User Group Meetup](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - August Meetup**](https://www.meetup.com/seattle-rust-user-group/events/294804636/)
* 2023-08-16 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/294910746/)
* 2023-08-16 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #39 sponsored by Fermyon**](https://www.meetup.com/copenhagen-rust-community/events/294806394)
* 2023-08-17 | Nashville, TN, US | [Seattle Rust User Group Meetup](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust goes where it pleases. Rust on the web and embedded**](https://www.meetup.com/music-city-rust-developers/events/294805470/)
* 2023-08-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295008514)
* 2023-08-24 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/295107743/)
* 2023-09-06 | Bellevue, WA, US | [The Linux Foundation](https://www.linuxfoundation.org/)
    * [**Rust Global**](https://events.linuxfoundation.org/rust-global/)

### Oceania

* 2023-08-09 | Perth, WA, AU | [Rust Perth](https://www.linkedin.com/groups/7439562/)
    * [**August Meetup**](https://www.tickettailor.com/events/perthrustusergroup/970279)
* 2023-08-15 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) August 2023 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/295143203/)

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

> Claiming Rust won't help you because you're doing so many unsafe things is like claiming protective gear won't help you because you're handling so many dangerous substances.

– [llogiq on twitter](https://twitter.com/llogiq/status/1686730795564535809)

[llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1455) feels very smug about his self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
