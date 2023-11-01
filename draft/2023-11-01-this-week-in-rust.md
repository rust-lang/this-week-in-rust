Title: This Week in Rust 519
Number: 519
Date: 2023-11-01
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
* [A tale of broken badges and 23,000 features](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html)
* [crates.io: Dropping support for non-canonical downloads](https://blog.rust-lang.org/2023/10/27/crates-io-non-canonical-downloads.html)
* [Generators are dead, long live coroutines, generators are back](https://blog.rust-lang.org/inside-rust/2023/10/23/coroutines.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [ratatui 0.24.0 is released! (Rust library that's all about cooking up terminal user interfaces)](https://ratatui.rs/highlights/v0.24.html)
* [git-cliff 1.4.0 is released! (highly customizable changelog generator)](https://git-cliff.org/blog/1.4.0/)
* [rust-analyzer changelog #205](https://rust-analyzer.github.io/thisweek/2023/10/30/changelog-205.html)
* [Progress report on rustc_codegen_cranelift (Oct 2023)](https://bjorn3.github.io/2023/10/31/progress-report-oct-2023.html)

### Observations/Thoughts
* [System dependencies are hard (so we made them easier)](https://blog.axo.dev/2023/10/dependencies)
* [Was Rust Worth It?](https://jsoverson.medium.com/was-rust-worth-it-f43d171fb1b3)
* [Can Rust prevent logic errors?](https://itsallaboutthebit.com/logic-errors-in-rust/)
* [Faster Rust Serialization](https://mo8it.com/blog/faster-rust-serialization/)
* [Dealing with Dependencies in Rust](https://tweedegolf.nl/en/blog/104/dealing-with-dependencies-in-rust)
* [How I learned to stop worrying and love byte ordering](https://udoprog.github.io/rust/2023-10-28/stop-worrying.html)
* [How To Move Fast With Rust](https://endler.dev/2023/move-fast-rust)
* [video] [Impl Trait aka Look ma’, no generics! by Jon Gjengset](https://www.youtube.com/watch?v=CWiz_RtA1Hw)

* [Starting a virtual Rust meet-up](https://hegdenu.net/posts/virtual-rust-meet-up/)

### Rust Walkthroughs
* [Fully Automated Releases for Rust Projects](https://blog.orhun.dev/automated-rust-releases/)
* [The beauty of a Rust message processor](https://worldwithouteng.com/articles/the-beauty-of-a-rust-message-processor/)
* [interrupts is threads](https://onevariable.com/blog/interrupts-is-threads/)
* [htmx, Rust & Shuttle: A New Rapid Prototyping Stack](https://www.shuttle.rs/blog/2023/10/25/htmx-with-rust)
* [video] [Async Not Required 🦀](https://www.youtube.com/watch?v=QXynWxALJmo)

### Research
* [Functional Ownership through Fractional Uniqueness](https://arxiv.org/abs/2310.18166)
* [Grading on a Curve: How Rust can Facilitate New Contributors while Decreasing Vulnerabilities](https://cypherpunks.ca/~iang/pubs/gradingcurve-secdev23.pdf)

### Miscellaneous
* [video] [5 Hours to 7.7 Seconds: How Database Tricks Sped up Rust Linting Over 2000x](https://www.youtube.com/watch?v=Fqo8r4bInsk)

## Crate of the Week

This week's crate is [silkenweb](https://crates.io/crates/silkenweb), a library for building web apps with fine-grained reactivity and a clean separation of logic and UI.

Thanks to [henrik](https://users.rust-lang.org/t/crate-of-the-week/2704/1255) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [pubgrub - Move to edition 2021](https://github.com/pubgrub-rs/pubgrub/issues/140)
* [pubgrub - Fix CI for conventional commits](https://github.com/pubgrub-rs/pubgrub/issues/139)
* [pubgrub - Rename v0.3 Range into BoundedRange](https://github.com/pubgrub-rs/pubgrub/issues/123)
* [pubgrub - `OfflineDependencyProvider` should get its own module](https://github.com/pubgrub-rs/pubgrub/issues/114)
* [Ockam - Library - Slim down the `NodeManagerWorker` for `node / tcp`](https://github.com/build-trust/ockam/issues/6708)
* [Ockam - Make `ockam vault delete` (no args) interactive by asking the user to choose from a list of vaults to delete (tuify)](https://github.com/build-trust/ockam/issues/6462)
* [Ockam - Command - refactor to use typed interfaces to implement commands for `services`](https://github.com/build-trust/ockam/issues/6700)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

408 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-10-23..2023-10-30

* [-Zunpretty help: add missing possible values](https://github.com/rust-lang/rust/pull/117311)
* [NVPTX: allow `PassMode::Direct` for ptx kernels for now](https://github.com/rust-lang/rust/pull/117247)
* [`OptWithInfcx` naming nits, trait bound simplifications](https://github.com/rust-lang/rust/pull/117091)
* [`deduce_param_attrs`: explain a read-only case](https://github.com/rust-lang/rust/pull/117385)
* [`rustc_interface` cleanups](https://github.com/rust-lang/rust/pull/117268)
* [`stack_overflow: get_stackp` using `MAP_STACK` flag on dragonflybsd too](https://github.com/rust-lang/rust/pull/117102)
* [add bootstrap flag `--skip-stage0-validation`](https://github.com/rust-lang/rust/pull/117043)
* [add method to convert internal to stable constructs](https://github.com/rust-lang/rust/pull/117010)
* [add support for i586-unknown-netbsd as target](https://github.com/rust-lang/rust/pull/117170)
* [add support for mipsel-unknown-netbsd, 32-bit LE mips](https://github.com/rust-lang/rust/pull/117356)
* [add target features for LoongArch](https://github.com/rust-lang/rust/pull/116943)
* [add way to differentiate argument locals from other locals in Stable MIR](https://github.com/rust-lang/rust/pull/117095)
* [allow partially moved values in match](https://github.com/rust-lang/rust/pull/103208)
* [allow target specs to use an LLD flavor, and self-contained linking components](https://github.com/rust-lang/rust/pull/116035)
* [allows `#[diagnostic::on_unimplemented]` attributes to have multiple](https://github.com/rust-lang/rust/pull/117205)
* [avoid repeated interning of `env!("CFG_RELEASE")`](https://github.com/rust-lang/rust/pull/117188)
* [avoid unbounded `O(n^2)` when parsing nested type args](https://github.com/rust-lang/rust/pull/117143)
* [avoid unnecessary builds/rebuilds of `rust-demangler`](https://github.com/rust-lang/rust/pull/117197)
* [avoid unnecessary renumbering during borrowck](https://github.com/rust-lang/rust/pull/116792)
* [c-variadic error improvements](https://github.com/rust-lang/rust/pull/117370)
* [consider alias bounds when computing liveness in NLL (but this time sound hopefully)](https://github.com/rust-lang/rust/pull/116733)
* [const stabilize `mem::discriminant`](https://github.com/rust-lang/rust/pull/116240)
* [correctly handle nested or-patterns in exhaustiveness](https://github.com/rust-lang/rust/pull/117398)
* [coverage: consistently remove unused counter IDs from expressions/mappings](https://github.com/rust-lang/rust/pull/117123)
* [coverage: replace manual debug indents with nested tracing spans in `counters`](https://github.com/rust-lang/rust/pull/117350)
* [create `windows/api.rs` for safer FFI](https://github.com/rust-lang/rust/pull/116816)
* [create a new ConstantKind variant (ZeroSized) for StableMIR](https://github.com/rust-lang/rust/pull/117262)
* [declare `rustc_target's` dependency on object/macho](https://github.com/rust-lang/rust/pull/117259)
* [deny providing explicit effect params](https://github.com/rust-lang/rust/pull/117171)
* [derive `Ord`, `PartialOrd` and `Hash` for `SocketAddr*`](https://github.com/rust-lang/rust/pull/116714)
* [detect when trait is implemented for type and suggest importing it](https://github.com/rust-lang/rust/pull/116862)
* [do not suggest `'Trait<Assoc=arg>'` when in trait impl](https://github.com/rust-lang/rust/pull/116553)
* [don't normalize to an un-revealed opaque when we hit the recursion limit](https://github.com/rust-lang/rust/pull/117414)
* [don't treat closures/coroutine types as part of the public API](https://github.com/rust-lang/rust/pull/117396)
* [don't use LFS64 symbols on musl](https://github.com/rust-lang/rust/pull/115968)
* [eat close paren if `capture_cfg` to avoid unbalanced parens](https://github.com/rust-lang/rust/pull/116889)
* [enable `cg_clif` tests for riscv64gc](https://github.com/rust-lang/rust/pull/117032)
* [fail typeck for illegal break-with-value](https://github.com/rust-lang/rust/pull/117382)
* [fix ICE: Restrict param constraint suggestion](https://github.com/rust-lang/rust/pull/117246)
* [fix failure to detect a too-big-type after adding padding](https://github.com/rust-lang/rust/pull/117277)
* [fix inline const pattern unsafety checking in THIR](https://github.com/rust-lang/rust/pull/116482)
* [fix missing leading space in suggestion](https://github.com/rust-lang/rust/pull/117395)
* [fix unused variables lint issue for args in macro](https://github.com/rust-lang/rust/pull/117390)
* [fix unwrap suggestion for async fn](https://github.com/rust-lang/rust/pull/117152)
* [generate aggregate constants in DataflowConstProp](https://github.com/rust-lang/rust/pull/115796)
* [handle `ReErased` in responses in new solver](https://github.com/rust-lang/rust/pull/116435)
* [ignore RPIT duplicated lifetimes in `opaque_types_defined_by`](https://github.com/rust-lang/rust/pull/117371)
* [implement C ABI lowering for CSKY](https://github.com/rust-lang/rust/pull/117154)
* [implement `gen` blocks in the 2024 edition](https://github.com/rust-lang/rust/pull/116447)
* [improve android-ndk property interface](https://github.com/rust-lang/rust/pull/116998)
* [improve some diagnostics around `?Trait` bounds](https://github.com/rust-lang/rust/pull/117411)
* [improve the warning messages for the `#[diagnostic::on_unimplemented]`](https://github.com/rust-lang/rust/pull/116931)
* [increase the reach of `panic_immediate_abort`](https://github.com/rust-lang/rust/pull/117332)
* [intern `LocalDefId` list from `opaque_types_defined_by` query](https://github.com/rust-lang/rust/pull/117136)
* [introduce `-C instrument-coverage=branch` to gate branch coverage](https://github.com/rust-lang/rust/pull/116094)
* [invalid `?` suggestion on mismatched `Ok(T)`](https://github.com/rust-lang/rust/pull/116968)
* [lint overlapping ranges as a separate pass](https://github.com/rust-lang/rust/pull/116751)
* [mark `.rmeta` files as `/SAFESEH` on x86 Windows](https://github.com/rust-lang/rust/pull/117115)
* [mark constructor of `BinaryHeap` as const fn](https://github.com/rust-lang/rust/pull/117316)
* [never consider raw pointer casts to be trival](https://github.com/rust-lang/rust/pull/113262)
* [on object safety error, mention new `enum` as alternative](https://github.com/rust-lang/rust/pull/117132)
* [on unresolved imports, suggest a disambiguated path if necessary to avoid collision with local items](https://github.com/rust-lang/rust/pull/117009)
* [only call `mir_const_qualif` if absolutely necessary](https://github.com/rust-lang/rust/pull/117166)
* [only emit one error per unsized binding, instead of one per usage](https://github.com/rust-lang/rust/pull/113183)
* [poison `check_well_formed` if method receivers are invalid to prevent typeck from running on it](https://github.com/rust-lang/rust/pull/117403)
* [print variadic argument pattern in HIR pretty printer](https://github.com/rust-lang/rust/pull/117147)
* [properly restore snapshot when failing to recover parsing ternary](https://github.com/rust-lang/rust/pull/117212)
* [quietly fail if an error has already occurred](https://github.com/rust-lang/rust/pull/117214)
* [rand use getrandom for freebsd (available since 12.x)](https://github.com/rust-lang/rust/pull/107159)
* [refactor type visitor walking](https://github.com/rust-lang/rust/pull/117076)
* [remap Cargo dependencies to /rust/deps](https://github.com/rust-lang/rust/pull/115872)
* [remove -Zdep-tasks](https://github.com/rust-lang/rust/pull/116534)
* [remove Apple RNG fallbacks and simplify implementation](https://github.com/rust-lang/rust/pull/116319)
* [remove fold code and add `Const::internal()` to StableMIR](https://github.com/rust-lang/rust/pull/117113)
* [remove support for alias `-Z instrument-coverage`](https://github.com/rust-lang/rust/pull/117111)
* [require target features to match exactly during inlining](https://github.com/rust-lang/rust/pull/117141)
* [return multiple object-safety violation errors and code improvements to the object-safety check](https://github.com/rust-lang/rust/pull/116401)
* [return unfixed len if pat has reported error](https://github.com/rust-lang/rust/pull/117046)
* [rework negative coherence to properly consider impls that only partly overlap](https://github.com/rust-lang/rust/pull/112875)
* [rustdoc: elide cross-crate default generic arguments](https://github.com/rust-lang/rust/pull/112463)
* [rustdoc: use `ThinVec` in `GenericParamDefKind`](https://github.com/rust-lang/rust/pull/117337)
* [see through aggregates in GVN](https://github.com/rust-lang/rust/pull/116270)
* [separate move path tracking between borrowck and drop elaboration](https://github.com/rust-lang/rust/pull/116300)
* [share some `track_caller` logic between interpret and codegen](https://github.com/rust-lang/rust/pull/117317)
* [small `ty::print` cleanups](https://github.com/rust-lang/rust/pull/117325)
* [some diagnostics improvements of `gen` blocks](https://github.com/rust-lang/rust/pull/117389)
* [stash and cancel cycle errors for auto trait leakage in opaques](https://github.com/rust-lang/rust/pull/117241)
* [stop telling people to submit bugs for internal feature ICEs](https://github.com/rust-lang/rust/pull/116818)
* [store `#[stable]` attribute's `since` value in structured form](https://github.com/rust-lang/rust/pull/117148)
* [suggest assoc fn `new` when trying to build tuple `struct` with private fields](https://github.com/rust-lang/rust/pull/116858)
* [suggest unwrap/expect for let binding type mismatch](https://github.com/rust-lang/rust/pull/116841)
* [tvOS simulator support on Apple Silicon for rustc](https://github.com/rust-lang/rust/pull/115773)
* [tweak suggestion span for outer attr and point at item following invalid inner attr](https://github.com/rust-lang/rust/pull/116868)
* [uplift `Canonical` to `rustc_type_ir`](https://github.com/rust-lang/rust/pull/117008)
* [uplift `ClauseKind` and `PredicateKind` into `rustc_type_ir`](https://github.com/rust-lang/rust/pull/116993)
* [use ImageDataType for allocation type](https://github.com/rust-lang/rust/pull/117177)
* [validate `feature` and `since` values inside `#[stable(…)]`](https://github.com/rust-lang/rust/pull/116773)
* [when encountering sealed traits, point types that implement it](https://github.com/rust-lang/rust/pull/116945)
* [when expecting closure argument but finding block provide suggestion](https://github.com/rust-lang/rust/pull/117106)
* [work around the fact that `check_mod_type_wf` may spuriously return `ErrorGuaranteed`](https://github.com/rust-lang/rust/pull/117159)
* [time: use `clock_gettime` on macos](https://github.com/rust-lang/rust/pull/116238)
* [windows: support sub-millisecond sleep](https://github.com/rust-lang/rust/pull/116461)
* [refactor some `char`, `u8` ASCII functions to be branchless](https://github.com/rust-lang/rust/pull/117260)
* [add `#[inline]` to some recalcitrant `ops::range` methods](https://github.com/rust-lang/rust/pull/117038)
* [stabilize ratified RISC-V Target Features](https://github.com/rust-lang/rust/pull/116485)
* [stabilize `[const_]pointer_byte_offsets`](https://github.com/rust-lang/rust/pull/116205)
* [stabilize inline asm usage with `rustc_codegen_cranelift`](https://github.com/rust-lang/rust/pull/117365)
* [futures: add `TryAll` and `TryAny` adapters](https://github.com/rust-lang/futures-rs/pull/2783)
* [futures: fix `Sync` impl of `FuturesUnordered`](https://github.com/rust-lang/futures-rs/pull/2788)
* [futures: provide AtomicWaker if portable-atomic feature is enabled, even if atomic CAS is not available](https://github.com/rust-lang/futures-rs/pull/2790)
* [codegen\_gcc: add basics for `test` command in build system](https://github.com/rust-lang/rustc_codegen_gcc/pull/363)
* [codegen\_gcc: fix `volatile_load`](https://github.com/rust-lang/rustc_codegen_gcc/pull/365)
* [cargo toml: Allow version-less manifests](https://github.com/rust-lang/cargo/pull/12786)
* [cargo toml: Decouple parsing from interning system](https://github.com/rust-lang/cargo/pull/12881)
* [cargo: shell: Write at once rather than in fragments](https://github.com/rust-lang/cargo/pull/12880)
* [cargo: add new packages to `[workspace.members]` automatically](https://github.com/rust-lang/cargo/pull/12779)
* [cargo: move up looking at index summary `enum`](https://github.com/rust-lang/cargo/pull/12749)
* [cargo: remove duplicate binaries during install](https://github.com/rust-lang/cargo/pull/12868)
* [cargo: remove outdated option to `-Zcheck-cfg` warnings](https://github.com/rust-lang/cargo/pull/12884)
* [rustfmt: fixed error caused by combination of `match_arm_blocks` and `control_brace_style`](https://github.com/rust-lang/rustfmt/pull/5923)
* [clippy: `ignored_unit_patterns`: check &(), &&(),](https://github.com/rust-lang/rust-clippy/pull/11670)
* [clippy: `iter_without_into_iter`: fix papercuts in suggestion and restrict linting to exported types](https://github.com/rust-lang/rust-clippy/pull/11696)
* [clippy: `let_and_return`: Wrap with parenthesis if necessary](https://github.com/rust-lang/rust-clippy/pull/11584)
* [clippy: add `waker_clone_and_wake` lint to check needless `Waker` clones](https://github.com/rust-lang/rust-clippy/pull/11698)
* [clippy: fix missing parenthesis in suboptimal floating point help](https://github.com/rust-lang/rust-clippy/pull/11724)
* [clippy: ignore lower-camel-case words in `doc_markdown`](https://github.com/rust-lang/rust-clippy/pull/11735)
* [clippy: move `read_zero_byte_vec` to nursery](https://github.com/rust-lang/rust-clippy/pull/11727)
* [clippy: remove internal feature from `clippy_utils`](https://github.com/rust-lang/rust-clippy/pull/11723)
* [clippy: remove the `internal_warn` lint category](https://github.com/rust-lang/rust-clippy/pull/11712)
* [rust-analyzer: make `extract_variable` assist in place](https://github.com/rust-lang/rust-analyzer/pull/15809)

### Rust Compiler Performance Triage

This week we have two sets of results as last week's arrived later than the publish date:

Triage done by **@rylev** and **@simulacrum**.

Revision range: [b9832e72..650991d](https://perf.rust-lang.org/?start=b9832e72c9223f4e96049aa5911effd258b92591&end=650991d62c3a2c80ba27009d06839adbb038bf5e&absolute=false&stat=instructions%3Au)

Across both reports:

9 Regressions, 7 Improvements, 5 Mixed
127 artifact comparisons made in total

* [Full report #1](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-10-26.md)
* [Full report #2](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-10-31.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [2024 Edition](https://github.com/rust-lang/rfcs/pull/3501)

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Lifetime Capture Rules 2024](https://github.com/rust-lang/rfcs/pull/3498)
* [disposition: merge] [Add "crates.io Policy Update" RFC](https://github.com/rust-lang/rfcs/pull/3463)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Override `Waker::clone`_from to avoid cloning Wakers unnecessarily](https://github.com/rust-lang/rust/pull/96979)
* [disposition: merge] [dropck_outlives check whether generator witness needs_drop](https://github.com/rust-lang/rust/pull/117134)
* [disposition: merge] [stricter hidden type wf-check](https://github.com/rust-lang/rust/pull/115008)
* [disposition: merge] [Tracking Issue for the GroupBy and GroupByMut iterators](https://github.com/rust-lang/rust/issues/80552)
* [disposition: merge] [Don't panic in `<BorrowedCursor as io::Write>::write`](https://github.com/rust-lang/rust/pull/115460)
* [disposition: merge] [Guarantee that `char` has the same size and alignment as `u32`](https://github.com/rust-lang/rust/pull/116894)
* [disposition: merge] [Stabilize `const_maybe_uninit_zeroed` and `const_mem_zeroed`](https://github.com/rust-lang/rust/pull/116218)
* [disposition: merge] [Clarify UB in `get_unchecked(_mut)`](https://github.com/rust-lang/rust/pull/117039)
* [disposition: merge] [document that the null pointer has the 0 address](https://github.com/rust-lang/rust/pull/116988)
* [disposition: close] [regression: parameter type may not live long enough](https://github.com/rust-lang/rust/issues/117055)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Guarantee that raw pointer conversions preserve slice element count](https://github.com/rust-lang/reference/pull/1417)

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* * No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

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

Rusty Events between 2023-11-01 - 2023-11-29 🦀

### Virtual

* 2023-10-30 | Virtual (Melbourne, VIC, AU) | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - online & in person) October 2023 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/296902361/)
* 2023-10-31 | Virtual (Europe / Africa) | [Rust for Lunch](https://lunch.rs/)
    * [**Rust Meet-up**](https://lunch.rs/meetups/2023-10-31/)
* 2023-11-01 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**ECS with Bevy Game Engine**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583207/)
* 2023-11-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyfcpbcb)
* 2023-11-02 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296661148/)
* 2023-11-07 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679790/) | [**Mirror**](https://berline.rs/)
* 2023-11-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/296827010/)
* 2023-11-09 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732666/)
* 2023-11-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcpbsb/)
* 2023-11-15 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Building Our Own Locks (Atomics & Locks Chapter 9)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296582223/)
* 2023-11-15 | Virtual (Richmond, VA, US) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)
* 2023-11-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/296600976/)
* 2023-11-16 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833657/)
* 2023-11-07 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679794/) | [**Mirror**](https://berline.rs/)
* 2023-11-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/296807537/)

### Europe

* 2023-10-25 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Biome, web development tooling with Rust**](https://www.meetup.com/rust-dublin/events/295179534/)
* 2023-10-25 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [Rust for the web - Paris meetup #61](https://mobilizon.fr/events/149c0367-66cb-42c6-aa0c-8495bf6d2a52)
* 2023-10-25 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup)
    * [Rust Meetup 2023/10: Lunatic](https://www.meetup.com/zagreb-rust-meetup/events/296765355/)
* 2023-10-26 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #3**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/296183126/)
* 2023-10-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #41 sponsored by Factbird**](https://www.meetup.com/copenhagen-rust-community/events/296819462/)
* 2023-10-26 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust at TU Delft**](https://www.meetup.com/rust-nederland/events/296488286/)
* 2023-10-26 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #4 at SFEIR**](https://www.meetup.com/meetup-group-zgphbyet/events/296766699/)
* 2022-10-30 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Aira + Netlight**](https://www.meetup.com/Stockholm-Rust/events/296578336/)
* 2023-11-01 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Web-applications with axum: Hello CRUD!**](https://www.meetup.com/rustcologne/events/296540949/)
* 2023-11-07 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/296809100/)
* 2023-11-07 | Brussels, BE | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus - Rust and Talk beginners edition**](https://www.meetup.com/rust-aarhus/events/296223647/)
* 2023-11-07 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #7**](https://www.meetup.com/rust-lyon/events/296853019/)
* 2023-11-09 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**11th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/296567395)
* 2023-11-09 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296083417/)
* 2023-11-21 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**GPU processing in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504264/)
* 2023-11-23 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)

### North America

* 2023-10-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/296495790)
* 2023-10-25 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/296657993/)
* 2023-11-01 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch**](https://www.meetup.com/bostonrust/events/296223910/)
* 2023-11-08 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Let's make a Discord bot!**](https://www.meetup.com/boulder-rust-meetup/events/296437292/)
* 2023-11-14 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer: Share, Show, & Tell! 🦀**](https://www.meetup.com/rust-nyc/events/296895126/)
* 2023-11-14 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/seattle-rust-user-group/events/296540653)
* 2023-11-15 | Richmond, VA, US + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)
* 2023-11-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Python loves Rust!**](https://www.meetup.com/music-city-rust-developers/events/296916567/)
* 2023-11-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/295483924)
* 2023-11-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/296917625/)
* 2023-11-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)

### Oceania

* 2023-10-26 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**October Meetup**](https://www.meetup.com/rust-brisbane/events/296628243/)
* 2023-10-30 | Melbourne, VIC, AU + Virtual | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) October 2023 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/296902362/)
* 2023-11-21 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296819540/)

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

> After doing a best fit, we found Rust projects were less likely to introduce vulnerabilities than their equivalent C++ projects at all relevant experience levels, but more importantly, we found the effect was most significant for first-time contributors, who were almost two orders of magnitude less likely to contribute vulnerabilities. That is, even though Rust may have a reputation as a harder language to learn, there is a very measurable effect that makes it better for newbies. Reviewers should not have to put as much effort into reviewing code to be confident that someone making their first foray into their project is accidentally adding a vulnerability.

– [Justin Tracey on crysp.org](https://ftp.crysp.org/@j3tracey/111315653313272566)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1473) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
