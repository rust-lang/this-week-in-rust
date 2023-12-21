Title: This Week in Rust 526
Number: 526
Date: 2023-12-20
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
* [Launching the 2023 State of Rust Survey](https://blog.rust-lang.org/2023/12/18/survey-launch.html)
* [A Call for Proposals for the Rust 2024 Edition](https://blog.rust-lang.org/2023/12/15/2024-Edition-CFP.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [ratatui: a Rust library for cooking up terminal user interfaces - v0.25.0](https://ratatui.rs/highlights/v025/)
* [Introducing Gooey: My take on a Rusty GUI framework](https://ecton.dev/introducing-gooey/)
* [Two New Open Source Rust Crates Create Easier Cedar Policy Management](https://aws.amazon.com/blogs/opensource/easier-cedar-policy-management/)
* [Introducing FireDBG - a Time Travel Visual Debugger for Rust](https://firedbg.sea-ql.org/blog/2023-12-12-introducing-firedbg/)
* [Fornjot 0.48.0 - open source b-rep CAD kernel written in Rust](https://www.fornjot.app/blog/release/0.48.0/)

### Observations/Thoughts
* [Rust is growing](https://flawless.dev/essays/rust-is-growing/)
* [A curiously recurring lifetime issue](https://blog.dureuill.net/articles/recurring-lifetime/)
* [The rabbit hole of unsafe Rust bugs](https://notgull.net/cautionary-unsafe-tale/)
* [Faster Rust Toolchains for Android](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html)
* [The Most Common Rust Compiler Errors as Encountered in RustRover: Part 1](https://blog.jetbrains.com/rust/2023/12/14/the-most-common-rust-compiler-errors-as-encountered-in-rustrover-part-1/)
* [Nine Rules for SIMD Acceleration of your Rust Code (Part 2): General Lessons from Boosting Data Ingestion in the range-set-blaze Crate by 7x](https://medium.com/towards-data-science/nine-rules-for-simd-acceleration-of-your-rust-code-part-2-6a104b3be6f3)
* [What I Learned Making an embedded-hal Driver in Rust (for the MAX6675 Thermocouple Digitizer)](https://barretts.club/posts/max6675-hal/)

### Rust Walkthroughs
* [Write a Toy VPN in Rust](https://write.yiransheng.com/vpn)
* [Getting Started with Actix Web in Rust](https://www.shuttle.rs/blog/2023/12/15/using-actix-rust)
* [Getting Started with Rocket in Rust](https://www.shuttle.rs/blog/2023/12/13/using-rocket-rust)
* [Generic types for function parameters in Rust 🦀](https://rust.code-maven.com/generic-types-for-simple-function)
* [Benchmarking Rust Compiler Settings with Criterion: Controlling Criterion with Scripts and Environment Variables](https://medium.com/towards-data-science/benchmarking-rust-compiler-settings-with-criterion-62db50cd62fb)
* [Rust: Traits](https://priver.dev/blog/rust/traits/)
* [Committing to Rust for kernel code](https://lwn.net/Articles/952029/)
* [A Rust implementation of Android's Binder](https://lwn.net/Articles/953116/)
* [Preventing atomic-context violations in Rust code with klint](https://lwn.net/Articles/951550/)
* [Rust for Linux — in space](https://lwn.net/Articles/954974/)
* [series] [Multithreading and Memory-Mapping: Refining ANN Performance with Arroy](https://blog.kerollmops.com/multithreading-and-memory-mapping-refining-ann-performance-with-arroy)

### Miscellaneous
* [Embedded Rust Education: 2023 Reflections & 2024 Visions](https://apollolabsblog.hashnode.dev/embedded-rust-education-2023-reflections-2024-visions)
* [Multithreading and Memory-Mapping: Refining ANN Performance with Arroy](https://blog.kerollmops.com/multithreading-and-memory-mapping-refining-ann-performance-with-arroy)
* [The Most Common Rust Compiler Errors as Encountered in RustRover: Part 1](https://blog.jetbrains.com/rust/2023/12/14/the-most-common-rust-compiler-errors-as-encountered-in-rustrover-part-1/)
* [Default arguments for functions in Rust using macros](https://rust.code-maven.com/default-arguments-for-functions)
* [Getting started with Tiny HTTP building a web application in Rust](https://rust.code-maven.com/tiny-http)
* [audio] [Rust in Production Ep 1 - InfluxData's Paul Dix](https://www.youtube.com/watch?v=DutVRGs9oZc)
* [audio] [Episode 160: Rust & Safety at Adobe with Sean Parent](https://adspthepodcast.com/2023/12/15/Episode-160.html)

## Crate of the Week

This week's crate is [constcat](https://crates.io/crates/constcat), a `std::concat!`-replacement with support for const variables and expressions.

Thanks to [Ross MacArthur](https://users.rust-lang.org/t/crate-of-the-week/2704/1272) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Fix documentation warnings](https://github.com/build-trust/ockam/issues/7145)
* [Ockam - Library - Validate CBOR structs according to the cddl schema for `nodes/models/secure_channel`](https://github.com/build-trust/ockam/issues/6692)
* [Ockam - Implement events in `SqlxDatabase`](https://github.com/build-trust/ockam/issues/7116)
* [Hyperswitch - [REFACTOR]: [Nuvei] MCA metadata validation](https://github.com/juspay/hyperswitch/issues/2910)
* [Hyperswitch - [FEATURE] : [Noon] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2904)
* [Hyperswitch - [FEATURE] : [Zen] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2908)
* [Hyperswitch - [REFACTOR] : [Authorizedotnet] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2909)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

386 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-12-13..2023-12-20

* [enable stack probes on aarch64 for LLVM 18](https://github.com/rust-lang/rust/pull/118491)
* [add new tier 3 aarch64-apple-watchos target](https://github.com/rust-lang/rust/pull/119074)
* [add hexagon support](https://github.com/rust-lang/compiler-builtins/pull/556)
* [add the function body span to StableMIR](https://github.com/rust-lang/rust/pull/119100)
* [allow `async_fn_in_trait` traits with Send variant](https://github.com/rust-lang/impl-trait-utils/pull/6)
* [cherry-pick "M68k: Fix ODR violation in GISel code (#72797)"](https://github.com/rust-lang/llvm-project/pull/159)
* [AIX: fix XCOFF metadata](https://github.com/rust-lang/rust/pull/118905)
* [`-Ztrait-solver=next` to `-Znext-solver`](https://github.com/rust-lang/rust/pull/118937)
* [actually parse async gen blocks correctly](https://github.com/rust-lang/rust/pull/118891)
* [add a method to StableMIR to check if a type is a CStr](https://github.com/rust-lang/rust/pull/119000)
* [add more suggestions to unexpected cfg names and values](https://github.com/rust-lang/rust/pull/118213)
* [add support for `--env` on `tracked_env::var`](https://github.com/rust-lang/rust/pull/118830)
* [add unstable `-Zdefault-hidden-visibility` cmdline flag for `rustc`](https://github.com/rust-lang/rust/pull/118417)
* [annotate panic reasons during enum layout](https://github.com/rust-lang/rust/pull/118974)
* [attempt to try to resolve blocking concerns](https://github.com/rust-lang/rust/pull/117050) (RFC [#3086](https://rust-lang.github.io/rfcs/3086-macro-metavar-expr.html))
* [avoid overflow in GVN constant indexing](https://github.com/rust-lang/rust/pull/119052)
* [cache param env canonicalization](https://github.com/rust-lang/rust/pull/117749)
* [check `FnPtr`/`FnDef` built-in fn traits correctly with effects](https://github.com/rust-lang/rust/pull/119023)
* [check generic params after sigature for main-fn-ty](https://github.com/rust-lang/rust/pull/119047)
* [collect lang items from AST, get rid of `GenericBound::LangItemTrait`](https://github.com/rust-lang/rust/pull/118396)
* [coroutine variant fields can be uninitialized](https://github.com/rust-lang/rust/pull/118871)
* [coverage: skip instrumenting a function if no spans were extracted from MIR](https://github.com/rust-lang/rust/pull/118852)
* [deny `~const` trait bounds in inherent impl headers](https://github.com/rust-lang/rust/pull/119059)
* [desugar `yield` in `async gen` correctly, ensure `gen` always returns unit](https://github.com/rust-lang/rust/pull/119061)
* [don't merge cfg and doc(cfg) attributes for re-exports](https://github.com/rust-lang/rust/pull/113091)
* [erase late bound regions from `Instance::fn_sig()` and add a few more details to StableMIR APIs](https://github.com/rust-lang/rust/pull/118927)
* [fix ICE `ProjectionKinds Deref and Field were mismatched`](https://github.com/rust-lang/rust/pull/118584)
* [fix LLD thread flags in bootstrap on Windows](https://github.com/rust-lang/rust/pull/118906)
* [fix `waker_getters` tracking issue number](https://github.com/rust-lang/rust/pull/118873)
* [fix alignment passed down to LLVM for `simd_masked_load`](https://github.com/rust-lang/rust/pull/118864)
* [fix dynamic size/align computation logic for packed types with dyn trait tail](https://github.com/rust-lang/rust/pull/118538)
* [fix overlapping spans in delimited meta-vars](https://github.com/rust-lang/rust/pull/118928)
* [ICE 110453: fixed with errors](https://github.com/rust-lang/glacier/pull/1702)
* [llvm-wrapper: adapt for LLVM API changes](https://github.com/rust-lang/rust/pull/118941)
* [make `IMPLIED_BOUNDS_ENTAILMENT` into a hard error from a lint](https://github.com/rust-lang/rust/pull/117984)
* [make exhaustiveness usable outside of rustc](https://github.com/rust-lang/rust/pull/118842)
* [match lowering: Remove the `make_target_blocks` hack](https://github.com/rust-lang/rust/pull/119112)
* [more expressions correctly are marked to end with curly braces](https://github.com/rust-lang/rust/pull/118880)
* [nudge the user to kill programs using excessive CPU](https://github.com/rust-lang/rust-playground/pull/1020)
* [opportunistically resolve region var in canonicalizer (instead of resolving root var)](https://github.com/rust-lang/rust/pull/118964)
* [properly reject `default` on free const items](https://github.com/rust-lang/rust/pull/117818)
* [remove unnecessary constness from ProjectionCandidate](https://github.com/rust-lang/rust/pull/119022)
* [replace some instances of `FxHashMap`/`FxHashSet` with stable alternatives (mostly in `rustc_hir` and `rustc_ast_lowering`)](https://github.com/rust-lang/rust/pull/119093)
* [resolve: replace visibility table in resolver outputs with query feeding](https://github.com/rust-lang/rust/pull/118657)
* [skip rpit constraint checker if borrowck return type error](https://github.com/rust-lang/rust/pull/117884)
* [some cleanup and improvement for invalid ref casting impl](https://github.com/rust-lang/rust/pull/118909)
* [tweak `short_ty_string` to reduce number of files](https://github.com/rust-lang/rust/pull/118389)
* [unconditionally register alias-relate in projection goal](https://github.com/rust-lang/rust/pull/118914)
* [update FreeBSD CI image](https://github.com/rust-lang/stdarch/pull/1507)
* [uplift `TypeAndMut` and `ClosureKind` to `rustc_type_ir`](https://github.com/rust-lang/rust/pull/118888)
* [use `if cfg!` instead of `#[cfg]`](https://github.com/rust-lang/rust/pull/118993)
* [use the LLVM option NoTrapAfterNoreturn](https://github.com/rust-lang/rust/pull/110494)
* [miri: visit the AllocIds and BorTags in borrow state FrameExtra](https://github.com/rust-lang/miri/pull/3229)
* [miri run: default to edition 2021](https://github.com/rust-lang/miri/pull/3221)
* [miri: make mmap not use expose semantics](https://github.com/rust-lang/miri/pull/3220)
* [fast path for `declared_generic_bounds_from_env`](https://github.com/rust-lang/rust/pull/119084)
* [stabilize `type_name_of_val`](https://github.com/rust-lang/rust/pull/118234)
* [stabilize `ptr::{from_ref, from_mut}`](https://github.com/rust-lang/rust/pull/117824)
* [add `core::intrinsics::simd`](https://github.com/rust-lang/rust/pull/118853)
* [add a column number to `dbg!()`](https://github.com/rust-lang/rust/pull/114962)
* [add more niches to `rawvec`](https://github.com/rust-lang/rust/pull/106790)
* [add ASCII whitespace trimming functions to `&str`](https://github.com/rust-lang/rust/pull/118523)
* [fix cases where std accidentally relied on inline(never)](https://github.com/rust-lang/rust/pull/118770)
* [Windows: allow `File::create` to work on hidden files](https://github.com/rust-lang/rust/pull/116438)
* [std: add xcoff in object's feature list](https://github.com/rust-lang/rust/pull/118851)
* [codegen: panic when trying to compute size/align of extern type](https://github.com/rust-lang/rust/pull/118534)
* [codegen\_gcc: simd: implement missing intrinsics from simd/generic-arithmetic-pass.rs](https://github.com/rust-lang/rustc_codegen_gcc/pull/382)
* [codegen\_llvm: set `DW_AT_accessibility`](https://github.com/rust-lang/rust/pull/115165)
* [cargo: clean up package metadata](https://github.com/rust-lang/cargo/pull/13184)
* [cargo: do not allow empty name in package ID spec](https://github.com/rust-lang/cargo/pull/13152)
* [cargo: fill in more empty name holes](https://github.com/rust-lang/cargo/pull/13164)
* [cargo: hold the mutate exclusive lock when vendoring](https://github.com/rust-lang/cargo/pull/12509)
* [rustdoc: use Map instead of Object for source files and search index](https://github.com/rust-lang/rust/pull/118910)
* [rustdoc: allow resizing the sidebar / hiding the top bar](https://github.com/rust-lang/rust/pull/115660)
* [rustdoc-search: fix a race condition in search index loading](https://github.com/rust-lang/rust/pull/118961)
* [rustdoc-search: use set ops for ranking and filtering](https://github.com/rust-lang/rust/pull/118402)
* [bindgen: use `\r\n` on windows](https://github.com/rust-lang/rust-bindgen/pull/2698)
* [bindgen: better working destructors on windows](https://github.com/rust-lang/rust-bindgen/pull/2663)
* [clippy: add new `unconditional_recursion` lint](https://github.com/rust-lang/rust-clippy/pull/11938)
* [clippy: new Lint: `result_filter_map` / Mirror of `option_filter_map`](https://github.com/rust-lang/rust-clippy/pull/11869)
* [clippy: don't visit nested bodies in `is_const_evaluatable`](https://github.com/rust-lang/rust-clippy/pull/11977)
* [clippy: `redundant_pattern_matching`: lint `if let true`, `while let true`, `matches!(.., true)`](https://github.com/rust-lang/rust-clippy/pull/11974)
* [clippy: do not lint `assertions_on_constants` for `const _: () = assert!(expr)`](https://github.com/rust-lang/rust-clippy/pull/11966)
* [clippy: `doc_markdown` Recognize words followed by empty parentheses `()` for quoting](https://github.com/rust-lang/rust-clippy/pull/11956)
* [clippy: fix binder handling in `unnecessary_to_owned`](https://github.com/rust-lang/rust-clippy/pull/11953)
* [rust-analyzer: deduplicate annotations](https://github.com/rust-lang/rust-analyzer/pull/16163)
* [rust-analyzer: optimizing Performance with `Promise.all` 🏎](https://github.com/rust-lang/rust-analyzer/pull/16162)
* [rust-analyzer: desugar doc correctly for mbe](https://github.com/rust-lang/rust-analyzer/pull/16158)
* [rust-analyzer: dont assume ascii in `remove_markdown`](https://github.com/rust-lang/rust-analyzer/pull/16155)
* [rust-analyzer: resolve alias before resolving enum variant](https://github.com/rust-lang/rust-analyzer/pull/16152)
* [rust-analyzer: add minimal support for the 2024 edition](https://github.com/rust-lang/rust-analyzer/pull/16151)
* [rust-analyzer: move out `WithFixture` into dev-dep only crate](https://github.com/rust-lang/rust-analyzer/pull/16150)
* [rust-analyzer: fix false positive type mismatch in const reference patterns](https://github.com/rust-lang/rust-analyzer/pull/16131)
* [rust-analyzer: syntax fixup now removes subtrees with fake spans](https://github.com/rust-lang/rust-analyzer/pull/16130)
* [rust-analyzer: update builtin attrs from rustc](https://github.com/rust-lang/rust-analyzer/pull/16115)
* [rust-analyzer: fix fragment parser replacing matches with dummies on incomplete parses](https://github.com/rust-lang/rust-analyzer/pull/16061)
* [rust-analyzer: fix incorrectly replacing references in macro invocation in "Convert to named struct" assist](https://github.com/rust-lang/rust-analyzer/pull/15887)

### Rust Compiler Performance Triage

A lot of noise in the results this week; there was an lull in the noise
recently, so our auto-inferred noise threshold went down, and thus five PR's
were artificially flagged this week (and three supposed improvements were just
reverting to the mean). Beyond that, we had three nice improvements: the first
to debug builds in #117962 (by ceasing emission of expensive+unused
`.debug_pubnames` and `.debug_pubtypes`), a second to diesel and serde in
#119048 (by avoiding some unnecessary work), and a third to several benchmarks
in #117749 (by adding some caching of an internal compiler structure).

Triage done by **@pnkfelix**.
Revision range: [57010939..bf9229a2](https://perf.rust-lang.org/?start=57010939ed1d00076b4af0ed06a81ec69ea5e4a8&end=bf9229a2e366b4c311f059014a4aa08af16de5d8&absolute=false&stat=instructions%3Au)

6 Regressions, 9 Improvements, 3 Mixed; 5 of them in rollups
67 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/b9ecf1aba002cd6b33d06f784e088839636d7e92/triage/2023-12-18.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: postpone] [RFC: Precise Pre-release Deps](https://github.com/rust-lang/rfcs/pull/3263)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Support async recursive calls (as long as they have indirection)](https://github.com/rust-lang/rust/pull/117703)
* [disposition: merge] [make soft_unstable show up in future breakage reports](https://github.com/rust-lang/rust/pull/116274)
* [disposition: merge] [Tracking Issue for ip_in_core](https://github.com/rust-lang/rust/issues/108443)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: patchable-function-entry](https://github.com/rust-lang/rfcs/pull/3543)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-12-20 - 2024-01-17 🦀

### Virtual

* 2023-12-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Adventures in egui app dev**](https://www.meetup.com/vancouver-rust/events/292763506/)
* 2023-12-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcqbjc/)
* 2023-12-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 2024-01-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)
* 2024-01-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtygccbmb/)
* 2024-01-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 2024-01-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128172/)

### Europe

* 2023-12-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust hacknight #1: CLIs, TUIs and plushies**](https://www.meetup.com/copenhagen-rust-community/events/297894275/)
* 2023-12-28 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Dojo 3: Holiday Edition**](https://www.meetup.com/rust-vienna/events/297826979/)
* 2024-01-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 2024-01-11 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 2024-01-13 | Helsinki, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**January Meetup**](https://www.meetup.com/finland-rust-meetup/events/297811750/)

### North America

* 2023-12-20 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297818036/)
* 2023-12-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcqbkc/)
* 2024-01-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Beacon Hill Rust Lunch**](https://www.meetup.com/bostonrust/events/297633937/)
* 2024-01-08 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/298003192/)
* 2024-01-09 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564978/)
* 2024-01-09 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760207/)
* 2024-01-14 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 2024-01-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 2024-01-17 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/298003233/)

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

> The Tianyi-33 satellite is a 50kg class space science experimental satellite equipped with an operating system independently developed by Beijing University of Posts and Telecommunications—the Rust-based dual-kernel real-time operating system **RROS**. RROS will carry out general tasks represented by tensorflow/k8s and real-time tasks represented by real-time file systems and real-time network transmission on the satellite. It will ensure the normal execution of upper-layer applications and scientific research tasks, such as time-delay measurement between satellite and ground, live video broadcasting, onboard web chat services, pseudo-SSH experiments, etc. This marks the world’s first official application of a Rust-written dual-kernel operating system in a satellite scenario.

– [Qichen on the RROS web page](https://bupt-os.github.io/website/news/2023_12_9/satellite_launch/)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1496) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
