Title: This Week in Rust 515
Number: 515
Date: 2023-10-04
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

### Foundation
* [Rust Foundation Project Director Election Update](https://foundation.rust-lang.org/news/rust-foundation-project-director-election-update/)

### Project/Tooling Updates
* [Progress Report #26: rustc_codegen_gcc can now compile Rust for Linux!](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-26)
* [Leptos Release v0.5.0](https://github.com/leptos-rs/leptos/releases/tag/v0.5.0)
* [Announcing Yew 0.21](https://yew.rs/blog/2023/09/23/release-0-21)
* [Ambient Platform](https://ambient.run/blog/platform)
* [Open Sourcing Ferrocene](https://ferrous-systems.com/blog/ferrocene-open-source/)
* [rust-analyzer changelog #201](https://rust-analyzer.github.io/thisweek/2023/10/02/changelog-201.html)
* [Zenoh 0.10.0, a pure Rust Pub/Sub/Query protocol for cloud-to-thing continuum, was released and it is packed with new features.](https://zenoh.io/blog/2023-10-03-zenoh-dragonite/)
* [Announcing Divan: Fast and Simple Benchmarking for Rust](https://nikolaivazquez.com/blog/divan/)

### Observations/Thoughts
* [Polonius revisited, part 2](https://smallcultfollowing.com/babysteps/blog/2023/09/29/polonius-part-2/)
* [Easing tradeoffs with profiles](https://smallcultfollowing.com/babysteps/blog/2023/09/30/profiles/)
* [Async Iteration III: The Async Iterator Trait](https://blog.yoshuawuyts.com/async-iterator-trait/)
* [Rustconf 2023 recap](https://blog.adamchalmers.com/rustconf-2023-recap/)
* [Was async fn a mistake?](https://seanmonstar.com/post/66832922686/was-async-fn-a-mistake)
* [Rust Vs Go: A Hands-On Comparison](https://www.shuttle.rs/blog/2023/09/27/rust-vs-go-comparison)
* [video] [The ultimate Rust IDE is here](https://www.youtube.com/watch?v=sZaAP4AS0hc)
* [audio] [rb-sys with Ian Ker-Seymer](https://rustacean-station.org/episode/ian-ker-sey/)
* [audio] [What's New in Rust 1.70 and 1.71](https://rustacean-station.org/episode/rust-1.70-1.71/)

### Rust Walkthroughs
* [Create an RSS Feed in Axum or Leptos](https://benw.is/posts/rss-in-axum)
* [Rust Dockerfile Boilerplate](https://peterprototypes.com/blog/rust-dockerfile-boilerplate/)
* [Nine Rules to Formally Validate Rust Algorithms with Dafny (Part 1): Lessons from Verifying the range-set-blaze Crate](https://towardsdatascience.com/nine-rules-to-formally-validate-rust-algorithms-with-dafny-part-1-5cb8c8a0bb92)
* [Send the logs of your Shuttle-powered backend to Datadog](https://robertohuertas.com/2023/09/30/shuttle-datadog-logs/)
* [Tracing Tokio Tasks](https://hegdenu.net/posts/tracing-tokio-tasks/)
* [series] [Distributed Tracing in Rust, Episode 4: correlating logs and traces](https://heikoseeberger.de/2023-09-30-dist-tracing-4/)
* [video] [Two Ways To Do Dynamic Dispatch](https://www.youtube.com/watch?v=wU8hQvU8aKM)

### Miscellaneous
* [Writing the Matrix Bridge in Rust using matrix-rust-sdk](https://harshil-jani.github.io/GSoC-Book-2.0/)
* [Ferrocene Open Source documentation](https://public-docs.ferrocene.dev/main/)

## Crate of the Week

This week's crate is [loole](https://github.com/mahdi-shojaee/loole), another multiple-producer multiple-consumer channel.

Thanks to [Mahdi Shojaee](https://users.rust-lang.org/t/crate-of-the-week/2704/1243) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

* [Hyperswitch (Hacktoberfest)- Bitpay: Use connector_response_reference_id as reference to merchant](https://github.com/juspay/hyperswitch/issues/2325)
* [Hyperswitch (Hacktoberfest)- Trustpay: Remove Default Case Handling](https://github.com/juspay/hyperswitch/issues/2287)
* [Hyperswitch (Hacktoberfest)- Worldline: Currency Unit Conversion](https://github.com/juspay/hyperswitch/issues/2249)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

358 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-09-25..2023-10-02

* [suggestion: insert projection to associated types](https://github.com/rust-lang/rust/pull/112123)
* [`skip_binder` to `instantiate_identity`](https://github.com/rust-lang/rust/pull/116340)
* [add "integer square root" method to integer primitive types](https://github.com/rust-lang/rust/pull/116176)
* [add Exclusive forwarding impls (FnOnce, FnMut, Generator)](https://github.com/rust-lang/rust/pull/116241)
* [add context to `let: Ty = loop { break };`](https://github.com/rust-lang/rust/pull/116187)
* [allow LTO on `proc-macro` crates with `-Zdylib-lto`](https://github.com/rust-lang/rust/pull/115986)
* [anonymize binders for `refining_impl_trait` check](https://github.com/rust-lang/rust/pull/116149)
* [assorted improvements for `rustc_middle::mir::traversal`](https://github.com/rust-lang/rust/pull/116254)
* [build `rustc` with a single CGU on x64 Linux](https://github.com/rust-lang/rust/pull/115554)
* [cleanup number handling in match exhaustiveness](https://github.com/rust-lang/rust/pull/116281)
* [constParamTy: require Eq as supertrait](https://github.com/rust-lang/rust/pull/116125)
* [correct codegen of `ConstValue::Indirect` scalar and scalar pair](https://github.com/rust-lang/rust/pull/116102)
* [don't store lazyness in `DefKind::TyAlias`](https://github.com/rust-lang/rust/pull/116163)
* [don't use a thread to load the dep graph](https://github.com/rust-lang/rust/pull/116109)
* [expose `try_destructure_mir_constant_for_diagnostics` directly to clippy](https://github.com/rust-lang/rust/pull/115819)
* [factor out duplicated `entry_point_type` functions](https://github.com/rust-lang/rust/pull/116259)
* [fix `noop_method_call` detection](https://github.com/rust-lang/rust/pull/116201)
* [gate and validate `#[rustc_safe_intrinsic]`](https://github.com/rust-lang/rust/pull/116162)
* [implement Region for smir](https://github.com/rust-lang/rust/pull/116024)
* [implement a global value numbering MIR optimization](https://github.com/rust-lang/rust/pull/109597)
* [lint towards rejecting consts in patterns that do not implement PartialEq](https://github.com/rust-lang/rust/pull/115893)
* [make `adt_const_params` feature suggestion consistent with other features and improve when it is emitted](https://github.com/rust-lang/rust/pull/116253)
* [make `link_llvm_intrinsics` and `platform_intrinsics` features internal](https://github.com/rust-lang/rust/pull/116093)
* [mark drop calls in landing pads cold instead of noinline](https://github.com/rust-lang/rust/pull/102099)
* [migrate `rustc_hir_analysis` to session diagnostic (Part 5)](https://github.com/rust-lang/rust/pull/115821)
* [more fixes for running the test suite on a bare metal target](https://github.com/rust-lang/rust/pull/116263)
* [new solver: remove provisional cache](https://github.com/rust-lang/rust/pull/115843)
* [only prevent field projections into opaque types, not types containing opaque types](https://github.com/rust-lang/rust/pull/116156)
* [only visit reachable nodes in SsaLocals](https://github.com/rust-lang/rust/pull/116239)
* [point at more causes of expectation of break value when possible](https://github.com/rust-lang/rust/pull/116080)
* [print GHA log groups to stdout instead of stderr](https://github.com/rust-lang/rust/pull/114453)
* [promote loongarch64-unknown-none* to Tier 2](https://github.com/rust-lang/rust/pull/115368)
* [properly print `cstr` literals in `proc_macro::Literal::to_string`](https://github.com/rust-lang/rust/pull/116124)
* [prototype using const generic for `simd_shuffle` IDX array](https://github.com/rust-lang/rust/pull/115933)
* [remove `rustc_lint_defs::lint_array`](https://github.com/rust-lang/rust/pull/116231)
* [replace `HashMap` with `IndexMap` in pattern binding resolve](https://github.com/rust-lang/rust/pull/114454)
* [resolve: skip underscore character during candidate lookup](https://github.com/rust-lang/rust/pull/116228)
* [reveal opaque types before drop elaboration](https://github.com/rust-lang/rust/pull/115759)
* [run abi/compatibility test against a whole bunch of targets](https://github.com/rust-lang/rust/pull/116030)
* [simplify some of the logic in the `invalid_reference_casting` lint](https://github.com/rust-lang/rust/pull/116199)
* [skip MIR pass `UnreachablePropagation` when coverage is enabled](https://github.com/rust-lang/rust/pull/116166)
* [split out the stable part of smir into its own crate to prevent accidental usage of forever unstable things](https://github.com/rust-lang/rust/pull/115934)
* [stabilize `impl_trait_projections`](https://github.com/rust-lang/rust/pull/115659)
* [stabilize combining +bundle and +whole-archive link modifiers](https://github.com/rust-lang/rust/pull/113301)
* [use `Vec::retain` in `remove_dead_blocks`](https://github.com/rust-lang/rust/pull/116154)
* [use absolute paths in `rustc_lint::passes` macros](https://github.com/rust-lang/rust/pull/116204)
* [use placeholders to prevent using inferred RPITIT types to imply their own well-formedness](https://github.com/rust-lang/rust/pull/116072)
* [when suggesting `self.x` for `S { x }`, use `S { x: self.x }`](https://github.com/rust-lang/rust/pull/116089)
* [miri: implement SSE3 and SSSE3 intrinsics](https://github.com/rust-lang/miri/pull/3086)
* [miri: implement the `llvm.x86.sse2.pmadd.wd` intrinsic](https://github.com/rust-lang/miri/pull/3093)
* [stdio support for UEFI](https://github.com/rust-lang/rust/pull/116207)
* [Build `rustc` with a single CGU on x64 Linux](https://github.com/rust-lang/rust/pull/107651)
* [`rustc_arena` overhaul](https://github.com/rust-lang/rust/pull/116224)
* [add `ptr::addr_eq`](https://github.com/rust-lang/rust/pull/116325)
* [add `track_caller` attribute to `Result::unwrap_or_else`](https://github.com/rust-lang/rust/pull/116317)
* [implement `From<OwnedFd/Handle>` for `ChildStdin`/`out`/`err` object](https://github.com/rust-lang/rust/pull/98704)
* [implement `From<[T; N]>` for `Rc<[T]>` and `Arc<[T]>`](https://github.com/rust-lang/rust/pull/114041)
* [implement `From<{&,&mut} [T; N]>` for `Vec<T>` where `T: Clone`](https://github.com/rust-lang/rust/pull/111278)
* [weaken needlessly restrictive orderings on `Arc::*_count`](https://github.com/rust-lang/rust/pull/115546)
* [cargo: add better suggestion for the unsupported silent flag](https://github.com/rust-lang/cargo/pull/12723)
* [cargo: add missing `strip` entries in `dev` and `release` profiles](https://github.com/rust-lang/cargo/pull/12748)
* [cargo: fix corruption when cargo killed while writing](https://github.com/rust-lang/cargo/pull/12744)
* [cargo: use full target spec for `cargo rustc --print --target`](https://github.com/rust-lang/cargo/pull/12743)
* [rustdoc: speed up processing of cross-crate fns to fix a perf regression](https://github.com/rust-lang/rust/pull/116195)
* [clippy: `manual_let_else`: only omit block if span is from same ctxt](https://github.com/rust-lang/rust-clippy/pull/11580)
* [clippy: `mir_to_const` improvements](https://github.com/rust-lang/rust-clippy/pull/11565)
* [clippy: `write_literal`: Fix index of the remaining positional arguments](https://github.com/rust-lang/rust-clippy/pull/11576)
* [clippy: add `manual_hash_one` lint](https://github.com/rust-lang/rust-clippy/pull/11556)
* [clippy: add lint: `into_iter_without_iter`](https://github.com/rust-lang/rust-clippy/pull/11587)
* [clippy: add lint: `iter_without_into_iter`](https://github.com/rust-lang/rust-clippy/pull/11527)
* [clippy: describe the type of string in `raw_strings` lints](https://github.com/rust-lang/rust-clippy/pull/11569)
* [clippy: do not lint `wildcard_imports` when imported item contains underscore](https://github.com/rust-lang/rust-clippy/pull/10300)
* [clippy: don't escape unicode escape braces in `print_literal`](https://github.com/rust-lang/rust-clippy/pull/11265)
* [clippy: don't lint `manual_non_exhaustive` when `enum` is `#[non_exhaustive]`](https://github.com/rust-lang/rust-clippy/pull/11590)
* [clippy: mention that `missing_assert_message` lint ignores test functions](https://github.com/rust-lang/rust-clippy/pull/11574)
* [clippy: move `needless_pass_by_ref_mut: suspicious` → `nursery`](https://github.com/rust-lang/rust-clippy/pull/11596)
* [clippy: move `needless_raw_string_hashes` to `pedantic`](https://github.com/rust-lang/rust-clippy/pull/11415)
* [rust-analyzer: allow more kinds of if let patterns in guarded return assist](https://github.com/rust-lang/rust-analyzer/pull/15698)
* [rust-analyzer: downgrade `unused_variables` to experimental](https://github.com/rust-lang/rust-analyzer/pull/15693)
* [rust-analyzer: make `bool_to_enum` assist create `enum` at top-level](https://github.com/rust-lang/rust-analyzer/pull/15667)
* [rust-analyzer: make `rustc_layout_scalar_valid_range` attributes work for non-decimal literals](https://github.com/rust-lang/rust-analyzer/pull/15688)
* [rust-analyzer: panic with wrapping/unwrapping result return type assists](https://github.com/rust-lang/rust-analyzer/pull/15662)
* [rust-analyzer: recover better on missing parameter in param list](https://github.com/rust-lang/rust-analyzer/pull/15682)
* [rust-analyzer: strip base prefix in `layout_scalar_valid_range`](https://github.com/rust-lang/rust-analyzer/pull/15701)
* [rust-analyzer: typing underscore should not trigger completions in types or patterns](https://github.com/rust-lang/rust-analyzer/pull/15692)
* [rust-analyzer: scip: allow customizing cargo config](https://github.com/rust-lang/rust-analyzer/pull/15633)

### Rust Compiler Performance Triage

A week completely free of pure regressions! The compiler has definitely come out of this week a decent amount faster and less memory hungry than before with the large gain coming from building the compiler with a single CGU on x64 Linux. This not only allows LLVM to do more optimizations across the entire compiler, but should hopefully also result in less non-deterministic performance regressions in the future. This improvement largely comes only at the expense of a few more minutes spent when bootstrapping the compiler.

Triage done by **@rylev**.
Revision range: [27b4eb..9998f4](https://perf.rust-lang.org/?start=27b4eb96d13106332d511be2ea6d0c008a57aa6e&end=9998f4add08c3d09c82e00975cf3a293b30160ec&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.0%  | [0.3%, 6.1%]   | 41    |
| Regressions ❌ <br /> (secondary)  | 2.0%  | [0.9%, 7.8%]   | 21    |
| Improvements ✅ <br /> (primary)   | -1.3% | [-5.1%, -0.2%] | 134   |
| Improvements ✅ <br /> (secondary) | -1.8% | [-6.9%, -0.2%] | 175   |
| All ❌✅ (primary)                 | -0.8% | [-5.1%, 6.1%]  | 175   |


0 Regressions, 2 Improvements, 4 Mixed; 0 of them in rollups
74 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-10-03.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Remove implicit features in a new edition](https://github.com/rust-lang/rfcs/pull/3491)
* [Add `f16` and `f128` float types](https://github.com/rust-lang/rfcs/pull/3453)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for pointer_bytes_offsets](https://github.com/rust-lang/rust/issues/96283)
* [disposition: merge] [Stabilize `atomic_from_ptr`](https://github.com/rust-lang/rust/pull/115719)
* [disposition: merge] [document when atomic loads are guaranteed read-only](https://github.com/rust-lang/rust/pull/115577)
* [disposition: merge] [const_eval: allow function pointer signatures containing &mut T in const contexts](https://github.com/rust-lang/rust/pull/116015)
* [disposition: merge] [const-eval: make misalignment a hard error](https://github.com/rust-lang/rust/pull/115524)
* [disposition: merge] [Stabilize `async fn` and `return-position impl` Trait in trait](https://github.com/rust-lang/rust/pull/115822)
* [disposition: merge] [rustdoc: hide `#[repr(transparent)]` if it isn't part of the public ABI](https://github.com/rust-lang/rust/pull/115439)
* [disposition: merge] [Stabilize `const_maybe_uninit_assume_init_read`](https://github.com/rust-lang/rust/pull/116233)
* [disposition: merge] [Fix exit status / wait status on non-Unix cfg(unix) platforms](https://github.com/rust-lang/rust/pull/115108)
* [disposition: merge] [impl Default for ExitCode](https://github.com/rust-lang/rust/pull/114589)
* [disposition: merge] [rustdoc-search: add impl disambiguator to duplicate assoc items](https://github.com/rust-lang/rust/pull/109422)
* [disposition: merge] [rustdoc: show crate name beside smaller logo](https://github.com/rust-lang/rust/pull/115948)
* [disposition: merge] [Implement iterator specialization traits on more adapters](https://github.com/rust-lang/rust/pull/85528)
* [disposition: merge] [Fix generic bound of `str::SplitInclusive`'s `DoubleEndedIterator` impl](https://github.com/rust-lang/rust/pull/100806)
* [disposition: merge] [fix detecting references to packed unsized fields](https://github.com/rust-lang/rust/pull/115583)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Move rustdoc-types to T-Rustdoc ownership.](https://github.com/rust-lang/rfcs/pull/3505)
* [new] [RFC: Syntax for embedding cargo-script manifests](https://github.com/rust-lang/rfcs/pull/3503)
* [new] [RFC: cargo-script](https://github.com/rust-lang/rfcs/pull/3502)
* [new] [2024 Edition](https://github.com/rust-lang/rfcs/pull/3501)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-10-04 - 2023-11-01 🦀

### Virtual

* 2023-10-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 2023-10-04 | Virtual (Various) | [Ferrous Systems](https://www.eventbrite.com/o/ferrous-systems-gmbh-68735392123)
    * [**A Decade of Rust with Ferrous Systems**](https://www.eventbrite.com/e/a-decade-of-rust-with-ferrous-systems-tickets-680492891557?aff=ebdssbdestsearch)
* 2023-10-04 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Understanding the Processor (Atomics & Locks Chapter 7)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296278202/)
* 2023-10-05 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296135640/)
* 2023-10-07 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup: Mentorship (First Saturday)**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763617907?aff=erelpanelorg)
* 2023-10-10 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679773/) | [**Mirror**](https://berline.rs/)
* 2023-10-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcnbnb/)
* 2023-10-11| Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcnbpb/)
* 2023-10-12 - 2023-10-13 | Virtual (Brussels, BE) | [EuroRust](https://eurorust.eu)
    * [**EuroRust 2023**](https://eurorust.eu)
* 2023-10-12 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732662/)
* 2023-10-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/295057159/)
* 2023-10-19 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfcnbzb/)
* 2023-10-19 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 2023-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679778/) | [**Mirror**](https://berline.rs/)
* 2023-10-31 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcnbpc/)

### Asia

* 2023-10-11 | Kuala Lumpur, MY | [GoLang Malaysia](https://t.me/golangmalaysia)
    * [**Rust Meetup Malaysia October 2023**](https://forms.gle/wwJAEipFgwQtEfJB9) | [Event updates Telegram](https://t.me/+dF46Fly4A_BjOTJl) | [Event group chat](https://t.me/golangmalaysia)

### Europe

* 2023-10-04 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #6**](https://www.meetup.com/fr-FR/rust-lyon/events/296186641/)
* 2023-10-10 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679773/)
* 2023-10-11 | Brussels, BE | [BeCode Brussels Meetup](https://www.eventbrite.be/e/becode-brussels-meetup-rust-on-web-tickets-728375238947)
    * [**Rust on Web - EuroRust Conference**](https://rust-on-web.glitch.me/)
* 2023-10-12 - 2023-10-13 | Brussels, BE | [EuroRust](https://eurorust.eu)
    * [**EuroRust 2023**](https://eurorust.eu)
* 2023-10-12 | Brussels, BE | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus - EuroRust Conference**](https://www.meetup.com/rust-aarhus/events/295673220/)
* 2023-10-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/295955356/)
* 2023-10-17 | Helsinki, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**Helsinki Rustaceans Meetup**](https://www.meetup.com/finland-rust-meetup/events/295680333/)
* 2023-10-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**SIMD in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504251/)
* 2023-10-19 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Amsterdam Meetup @ Terraform**](https://www.meetup.com/rust-amsterdam-group/events/296495570/)
* 2023-10-19 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #35**](https://www.meetup.com/rust-wroclaw/events/296507983/)
* 2023-10-25 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Biome, web development tooling with Rust**](https://www.meetup.com/rust-dublin/events/295179534/)
* 2023-10-26 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #3**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/296183126/)

### North America

* 2023-10-05 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/296369949/)
* 2023-10-09 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/296346749/)
* 2023-10-09 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/296497475/)
* 2023-10-11 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**First Meetup - Demo Day and Office Hours**](https://www.meetup.com/boulder-rust-meetup/events/296193722/)
* 2023-10-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**The Actor Model: Fearless Concurrency, Made Easy w/Chris Mena**](https://www.meetup.com/utah-rust/events/295771376/)
* 2023-10-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcnbwb/)
* 2023-10-18 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston University Rust Lunch**](https://www.meetup.com/bostonrust/events/296223807/)
* 2023-10-19 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/296369976/)
* 2023-10-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust Goes Where It Pleases Pt2 - Rust on the front end!**](https://www.meetup.com/music-city-rust-developers/events/296254420/)
* 2023-10-19 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - October Meetup**](https://www.meetup.com/seattle-rust-user-group/events/296110729)
* 2023-10-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/296495790)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/163w6fl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I've been writing Rust code everyday for years, and I used to say Rust wasn't great for writing prototypes because if forced you to ask yourself many questions that you may want to avoid at that time.
>
> I recently realized this is all wrong: you can write Rust pretty much as fast as you can write code in any other language, with a meaningful difference: with a little discipline it's easy to make the rough edges obvious so you can sort them out later. 

– [/u/moiaussi4213 on /r/rust](https://www.reddit.com/r/rust/comments/16yljmi/realization_rust_lets_you_comfortably_leave/)

There was no suggestion this week, but llogiq is pleased with his choice nonetheless!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1708new/this_week_in_rust_515/)</small>
