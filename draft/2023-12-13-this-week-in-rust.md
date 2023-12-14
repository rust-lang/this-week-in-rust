Title: This Week in Rust 525
Number: 525
Date: 2023-12-13
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
* [Announcing Rust 1.74.1](https://blog.rust-lang.org/2023/12/07/Rust-1.74.1.html)
* [Cargo cache cleaning](https://blog.rust-lang.org/2023/12/11/cargo-cache-cleaning.html)

### Foundation

### Newsletters
* [This Month in Rust OSDev: November 2023](https://rust-osdev.com/this-month/2023-11/)

### Project/Tooling Updates
* [rust-analyzer changelog #211](https://rust-analyzer.github.io/thisweek/2023/12/11/changelog-211.html)
* [PC Music Generator](https://hjvt.dev/posts/5)

### Observations/Thoughts
* [Being Rusty: Discovering Rust's design axioms](https://smallcultfollowing.com/babysteps/blog/2023/12/07/rust-design-axioms/)
* [Non-Send Futures When?](https://matklad.github.io/2023/12/10/nsfw.html)
* [`for await` and the battle of buffered streams](https://tmandry.gitlab.io/blog/posts/for-await-buffered-streams/)
* [poll_progress](https://without.boats/blog/poll-progress/)
* [Rust Is Beyond Object-Oriented, Part 3: Inheritance](https://www.thecodedmessage.com/posts/oop-3-inheritance/)
* [Rust and ThreadX](https://ferrous-systems.com/blog/rust-and-threadx/)
* [audio] [Exploring Rust's impact on efficiency and cost-savings, with Stefan Baumgartner](https://rustacean-station.org/episode/stefan-baumgartner-rust-efficiency/)

* [Nine Rules for SIMD Acceleration of Your Rust Code (Part 1): General Lessons from Boosting Data Ingestion in the range-set-blaze Crate by 7x](https://towardsdatascience.com/nine-rules-for-simd-acceleration-of-your-rust-code-part-1-c16fe639ce21)

### Rust Walkthroughs
* [Common Mistakes with Rust Async](https://www.qovery.com/blog/common-mistakes-with-rust-async/)
* [Embassy on ESP: UART Transmitter](https://apollolabsblog.hashnode.dev/embassy-on-esp-uart-transmitter)
* [Writing a CLI Tool in Rust with Clap](https://www.shuttle.rs/blog/2023/12/08/clap-rust)
* [Memory and Iteration](https://yetanotherrustblog.net/memory-and-iteration/)
* [Getting Started with Axum - Rust's Most Popular Web Framework](https://www.shuttle.rs/blog/2023/12/06/using-axum-rust)
* [video] [Advent of Code 2023](https://www.youtube.com/playlist?list=PLWtPciJ1UMuD3_8Pb-EqrFhkYpastR2cn)

### Research

### Miscellaneous
* [Turbofish ::<>](https://rust.code-maven.com/turbofish)
* [Rust Meetup and user groups](https://rust.code-maven.com/meetups)
* [Adopting Rust: the missing playbook for managers and CTOs](https://mainmatter.com/blog/2023/12/13/rust-adoption-playbook-for-ctos-and-engineering-managers/)

## Crate of the Week

This week's crate is [io-adapters](https://github.com/SUPERCILEX/io-adapters), a crate tha lets you convert between different writeable APIs (`io` vs. `fmt`).

Thanks to [Alex Saveau](https://users.rust-lang.org/t/crate-of-the-week/2704/1271) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [greptimedb - API improvement for pretty print sql query result in http output 1](https://github.com/GreptimeTeam/greptimedb/issues/2877)
* [greptimedb - Unify builders' patterns](https://github.com/GreptimeTeam/greptimedb/issues/2853)
* [tokio - Run loom tests in oss-fuzz 4](https://github.com/tokio-rs/tokio/issues/6208)
* [Ockam - Library - Validate CBOR structs according to the cddl schema for `kafka/protocol_aware` and `nodes/services`](https://github.com/build-trust/ockam/issues/6689)
* [Ockam - Command - refactor to use typed interfaces to implement commands for `relays`](https://github.com/build-trust/ockam/issues/6704)
* [Ockam - Make install.sh not fail when the latest version is already installed](https://github.com/build-trust/ockam/issues/7118)
* [zerocopy - Use cargo-semver-checks to make sure `derive` feature doesn't change API surface](https://github.com/google/zerocopy/issues/422)
* [zerocopy - Verify that `all-jobs-succeeded` CI job depends on all other jobs](https://github.com/google/zerocopy/issues/444)
* [Hyperswitch - [REFACTOR]: [Nuvei] MCA metadata validation](https://github.com/juspay/hyperswitch/issues/2910)
* [Hyperswitch - [Feature] : [Noon] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2904)
* [Hyperswitch - [BUG]     : MCA metadata deserialization failures should be 4xx](https://github.com/juspay/hyperswitch/issues/2899)
* [Hyperswitch - [Feature] : [Zen] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2908)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

391 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-12-04..2023-12-11

* [introduce support for `async gen` blocks](https://github.com/rust-lang/rust/pull/118420)
* [implement 2024-edition lifetime capture rules RFC](https://github.com/rust-lang/rust/pull/116952)
* [`riscv32` platform support](https://github.com/rust-lang/rust/pull/117874)
* [add teeos std impl](https://github.com/rust-lang/rust/pull/116565)
* [`never_patterns`: Parse match arms with no body](https://github.com/rust-lang/rust/pull/118527)
* [`rustc_symbol_mangling,rustc_interface,rustc_driver_impl`: Enforce `rustc::potential_query_instability` lint](https://github.com/rust-lang/rust/pull/118637)
* [add ADT variant infomation to StableMIR and finish implementing `TyKind::internal()`](https://github.com/rust-lang/rust/pull/118516)
* [add `deeply_normalize_for_diagnostics`, use it in coherence](https://github.com/rust-lang/rust/pull/118346)
* [add comment about keeping flags in sync between bootstrap.py and bootstrap.rs](https://github.com/rust-lang/rust/pull/118650)
* [add emulated TLS support](https://github.com/rust-lang/rust/pull/117873)
* [add instance evaluation and methods to read an allocation in StableMIR](https://github.com/rust-lang/rust/pull/118694)
* [add lint against ambiguous wide pointer comparisons](https://github.com/rust-lang/rust/pull/117758)
* [add method to get type of an Rvalue in StableMIR](https://github.com/rust-lang/rust/pull/118688)
* [add more SIMD platform-intrinsics](https://github.com/rust-lang/rust/pull/117953)
* [add safe compilation options](https://github.com/rust-lang/rust/pull/117966)
* [add support for `gen fn`](https://github.com/rust-lang/rust/pull/118457)
* [add support for making lib features internal](https://github.com/rust-lang/rust/pull/118123)
* [added shadowed hint for overlapping associated types](https://github.com/rust-lang/rust/pull/117661)
* [avoid adding builtin functions to `symbols.o`](https://github.com/rust-lang/rust/pull/118568)
* [avoid instantiating infer vars with infer](https://github.com/rust-lang/rust/pull/118710)
* [change prefetch to avoid deadlock](https://github.com/rust-lang/rust/pull/118488)
* [compile-time evaluation: detect writes through immutable pointers](https://github.com/rust-lang/rust/pull/118324)
* [coverage: be more strict about what counts as a "visible macro"](https://github.com/rust-lang/rust/pull/118595)
* [coverage: merge refined spans in a separate final pass](https://github.com/rust-lang/rust/pull/118695)
* [coverage: simplify the heuristic for ignoring `async fn` return spans](https://github.com/rust-lang/rust/pull/118666)
* [coverage: use `SpanMarker` to improve coverage spans for `if !` expressions](https://github.com/rust-lang/rust/pull/118198)
* [dedup for duplicate suggestions](https://github.com/rust-lang/rust/pull/118057)
* [discard invalid spans in external blocks](https://github.com/rust-lang/rust/pull/116420)
* [do not parenthesize exterior `struct` lit inside match guards](https://github.com/rust-lang/rust/pull/118726)
* [don't include destruction scopes in THIR](https://github.com/rust-lang/rust/pull/116170)
* [don't print host effect param in pretty `path_generic_args`](https://github.com/rust-lang/rust/pull/118788)
* [don't warn an empty pattern unreachable if we're not sure the data is valid](https://github.com/rust-lang/rust/pull/118308)
* [enforce `must_use` on associated types and RPITITs that have a must-use trait in bounds](https://github.com/rust-lang/rust/pull/118504)
* [explicitly implement `DynSync` and `DynSend` for `TyCtxt`](https://github.com/rust-lang/rust/pull/117681)
* [fix `is_foreign_item` for StableMIR instance](https://github.com/rust-lang/rust/pull/118681)
* [fix const drop checking](https://github.com/rust-lang/rust/pull/118689)
* [fix in-place collect not reallocating when necessary](https://github.com/rust-lang/rust/pull/118460)
* [fix parser ICE when recovering `dyn`/`impl` after `for<...>`](https://github.com/rust-lang/rust/pull/118585)
* [fix: correct the arg for 'suggest to use associated function syntax' diagnostic](https://github.com/rust-lang/rust/pull/118502)
* [generalize LLD usage in bootstrap](https://github.com/rust-lang/rust/pull/116278)
* [generalize: handle occurs check failure in aliases](https://github.com/rust-lang/rust/pull/117088)
* [implement `--env` compiler flag (without `tracked_env` support)](https://github.com/rust-lang/rust/pull/118368)
* [implement `repr(packed)` for `repr(simd)`](https://github.com/rust-lang/rust/pull/117116)
* [improve `print_tts`](https://github.com/rust-lang/rust/pull/114571)
* [interpret: make `numeric_intrinsic` accessible from Miri](https://github.com/rust-lang/rust/pull/118565)
* [make async generators fused by default](https://github.com/rust-lang/rust/pull/118764)
* [make sure `panic_nounwind_fmt` can still be fully inlined (e.g. for `panic_immediate_abort)`](https://github.com/rust-lang/rust/pull/118362)
* [only check principal trait ref for object safety](https://github.com/rust-lang/rust/pull/118686)
* [pretty print `Fn<(..., ...)>` trait refs with parentheses (almost) always](https://github.com/rust-lang/rust/pull/118268)
* [privacy: visit trait def id of projections](https://github.com/rust-lang/rust/pull/118715)
* [provide context when `?` can't be called because of `Result<_, E>`](https://github.com/rust-lang/rust/pull/116496)
* [rearrange `default_configuration` and `CheckCfg::fill_well_known`](https://github.com/rust-lang/rust/pull/118494)
* [recurse into refs when comparing tys for diagnostics](https://github.com/rust-lang/rust/pull/118730)
* [remove `PolyGenSig` since it's always a dummy binder](https://github.com/rust-lang/rust/pull/118684)
* [remove the `precise_pointer_size_matching` feature gate](https://github.com/rust-lang/rust/pull/118598)
* [resolve associated item bindings by namespace](https://github.com/rust-lang/rust/pull/118668)
* [streamline MIR dataflow cursors](https://github.com/rust-lang/rust/pull/118230)
* [structured `use` suggestion on privacy error](https://github.com/rust-lang/rust/pull/118066)
* [tell MirUsedCollector that the pointer alignment checks calls its panic symbol](https://github.com/rust-lang/rust/pull/118693)
* [tip for define macro name after `macro_rules!`](https://github.com/rust-lang/rust/pull/118317)
* [tweak `.clone()` suggestion to work in more cases](https://github.com/rust-lang/rust/pull/118076)
* [tweak unclosed generics errors](https://github.com/rust-lang/rust/pull/117922)
* [unescaping cleanups](https://github.com/rust-lang/rust/pull/118734)
* [uplift the (new solver) canonicalizer into `rustc_next_trait_solver`](https://github.com/rust-lang/rust/pull/117586)
* [use `immediate_backend_type` when reading from a const alloc](https://github.com/rust-lang/rust/pull/118791)
* [use default params until effects in desugaring](https://github.com/rust-lang/rust/pull/118608)
* [miri: fix promising a very large alignment](https://github.com/rust-lang/miri/pull/3211)
* [miri: fix x86 SSE4.1 ptestnzc](https://github.com/rust-lang/miri/pull/3216)
* [miri: move some x86 intrinsics code to helper functions in `shims::x86`](https://github.com/rust-lang/miri/pull/3214)
* [miri: return `MAP_FAILED` when mmap fails](https://github.com/rust-lang/miri/pull/3219)
* [stablize `arc_unwrap_or_clone`](https://github.com/rust-lang/rust/pull/116949)
* [add `LinkedList::{retain,retain_mut}`](https://github.com/rust-lang/rust/pull/114136)
* [simplify Default for tuples](https://github.com/rust-lang/rust/pull/118350)
* [restore `const PartialEq`](https://github.com/rust-lang/rust/pull/118661)
* [split `Vec::dedup_by` into 2 cycles](https://github.com/rust-lang/rust/pull/118273)
* [futures: fillBuf: do not call `poll_fill_buf` twice](https://github.com/rust-lang/futures-rs/pull/2812)
* [futures: `FuturesOrdered`: use 64-bit index](https://github.com/rust-lang/futures-rs/pull/2810)
* [futures: `FuturesUnordered`: fix clear implementation](https://github.com/rust-lang/futures-rs/pull/2809)
* [futures: use `cfg(target_has_atomic)` on no-std targets](https://github.com/rust-lang/futures-rs/pull/2811)
* [cargo: spec: Extend PackageIdSpec with source kind + git ref for unambiguous specs](https://github.com/rust-lang/cargo/pull/12933)
* [cargo toml: disallow inheriting of dependency public status](https://github.com/rust-lang/cargo/pull/13125)
* [cargo toml: disallow `[lints]` in virtual workspaces](https://github.com/rust-lang/cargo/pull/13155)
* [cargo: schema: Remove reliance on cargo types](https://github.com/rust-lang/cargo/pull/13154)
* [cargo: schemas: Pull out mod for proposed schemas package](https://github.com/rust-lang/cargo/pull/13097)
* [cargo: trim-paths: assert `OSO` and `SO` cannot be trimmed](https://github.com/rust-lang/cargo/pull/13118)
* [cargo: avoid writing CACHEDIR.TAG if it already exists](https://github.com/rust-lang/cargo/pull/13132)
* [cargo: fix bash completion in directory with spaces](https://github.com/rust-lang/cargo/pull/13126)
* [cargo: explicitly remap current dir by using `.`](https://github.com/rust-lang/cargo/pull/13114)
* [cargo: print rustc messages colored on wincon](https://github.com/rust-lang/cargo/pull/13140)
* [cargo: limit exported-private-dependencies lints to libraries](https://github.com/rust-lang/cargo/pull/13135)
* [rustdoc-search: do not treat associated type names as types](https://github.com/rust-lang/rust/pull/118812)
* [rustdoc: Don't generate the "Fields" heading if there is no field displayed](https://github.com/rust-lang/rust/pull/118600)
* [rustdoc: Fix display of features](https://github.com/rust-lang/rust/pull/118677)
* [rustdoc: do not escape quotes in body text](https://github.com/rust-lang/rust/pull/118508)
* [rustdoc: remove unused parameter `reversed` from `onEach(Lazy)`](https://github.com/rust-lang/rust/pull/118722)
* [bindgen: support float16](https://github.com/rust-lang/rust-bindgen/pull/2667)
* [rustfmt: add `StyleEdition enum` and `StyleEditionDefault` trait](https://github.com/rust-lang/rustfmt/pull/5933)
* [clippy: `fix(ptr_as_ptr)`: handle `std::ptr::null{_mut}`](https://github.com/rust-lang/rust-clippy/pull/11913)
* [clippy: `needless_borrows_for_generic_args`: Handle when field operand impl Drop](https://github.com/rust-lang/rust-clippy/pull/11900)
* [clippy: `uninhabited_reference`: new lint](https://github.com/rust-lang/rust-clippy/pull/11878)
* [clippy: add a function to check whether binary oprands are nontrivial](https://github.com/rust-lang/rust-clippy/pull/11907)
* [clippy: fix `is_from_proc_macro` patterns](https://github.com/rust-lang/rust-clippy/pull/11538)
* [rust-analyzer: check if lhs is also a binexpr and use its rhs in flip binexpr assist](https://github.com/rust-lang/rust-analyzer/pull/15515)
* [rust-analyzer: fallback to method resolution on unresolved field access with matching method name](https://github.com/rust-lang/rust-analyzer/pull/16055)
* [rust-analyzer: add `trait_impl_reduntant_assoc_item` diagnostic](https://github.com/rust-lang/rust-analyzer/pull/15990)
* [rust-analyzer: allow navigation targets to be duplicated when the focus range lies in the macro definition site](https://github.com/rust-lang/rust-analyzer/pull/16034)
* [rust-analyzer: implicit format args support](https://github.com/rust-lang/rust-analyzer/pull/16027) (hooray!)
* [rust-analyzer: prioritize import suggestions based on the expected type](https://github.com/rust-lang/rust-analyzer/pull/15627)
* [rust-analyzer: fix WideChar offsets calculation in `line-index`](https://github.com/rust-lang/rust-analyzer/pull/16041)
* [rust-analyzer: fix panic with closure inside array len](https://github.com/rust-lang/rust-analyzer/pull/16045)
* [rust-analyzer: bug in `extract_function.rs`](https://github.com/rust-lang/rust-analyzer/pull/16009)
* [rust-analyzer: don't emit "missing items" diagnostic for negative impls](https://github.com/rust-lang/rust-analyzer/pull/16039)
* [rust-analyzer: don't print proc-macro panic backtraces in the logs](https://github.com/rust-lang/rust-analyzer/pull/16037)
* [rust-analyzer: fix `concat_bytes!` expansion emitting an identifier](https://github.com/rust-lang/rust-analyzer/pull/16048)
* [rust-analyzer: fix completion failing in `format_args!` with invalid template](https://github.com/rust-lang/rust-analyzer/pull/16060)
* [rust-analyzer: fix diagnostics panicking when resolving to different files due to macros](https://github.com/rust-lang/rust-analyzer/pull/16035)
* [rust-analyzer: fix item tree lowering `pub(self)` to `pub()`](https://github.com/rust-lang/rust-analyzer/pull/15486)
* [rust-analyzer: fix runnable cwd on Windows](https://github.com/rust-lang/rust-analyzer/pull/16024)
* [rust-analyzer: fix token downmapping being quadratic](https://github.com/rust-lang/rust-analyzer/pull/16054)
* [rust-analyzer: fix view mir, hir and eval function not working when cursor is inside macros](https://github.com/rust-lang/rust-analyzer/pull/16078)
* [rust-analyzer: insert fn call parens only if the parens inserted around field name](https://github.com/rust-lang/rust-analyzer/pull/16016)
* [rust-analyzer: make drop inlay hint more readable](https://github.com/rust-lang/rust-analyzer/pull/16028)
* [rust-analyzer: resolve Self type references in delegate method assist](https://github.com/rust-lang/rust-analyzer/pull/15705)
* [rust-analyzer: smaller spans for unresolved field and method diagnostics](https://github.com/rust-lang/rust-analyzer/pull/16058)
* [rust-analyzer: make ParamLoweringMode accessible](https://github.com/rust-lang/rust-analyzer/pull/16036)
* [rust-analyzer: query for nearest parent block around the hint to resolve](https://github.com/rust-lang/rust-analyzer/pull/16089)
* [rust-analyzer: replace `doc_comments_and_attrs` with `collect_attrs`](https://github.com/rust-lang/rust-analyzer/pull/16073)
* [rust-analyzer: show placeholder while run command gets runnables from server](https://github.com/rust-lang/rust-analyzer/pull/15896)
* [rustc-perf: add support for benchmarking Cranelift codegen backend](https://github.com/rust-lang/rustc-perf/pull/1762)

### Rust Compiler Performance Triage

A quiet week overall. A few smaller crate (e.g., helloworld) benchmarks saw
significant improvements in
[#118568](https://github.com/rust-lang/rust/pull/118568), but this merely
restores performance regressed earlier.

Triage done by **@simulacrum**.
Revision range: [9358642..5701093](https://perf.rust-lang.org/?start=9358642e3b8560eee89e6f40aa996c8394a3db31&end=57010939ed1d00076b4af0ed06a81ec69ea5e4a8&absolute=false&stat=instructions%3Au)

5 Regressions, 2 Improvements, 3 Mixed; 2 of them in rollups

69 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-12-11.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Merge RFC 3531: "Macro fragment specifiers edition policy"](https://github.com/rust-lang/rfcs/commit/dbdb738e80d49cb127907e5b40986dacb46ac4b6)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for `Bound::map`](https://github.com/rust-lang/rust/issues/86026)
* [disposition: merge] [Stabilize THIR unsafeck](https://github.com/rust-lang/rust/pull/117673)
* [disposition: merge] [Exhaustiveness: reveal opaque types properly](https://github.com/rust-lang/rust/pull/116821)
* [disposition: merge] [Properly reject `default` on free const items](https://github.com/rust-lang/rust/pull/117818)
* [disposition: merge] [Make inductive cycles in coherence ambiguous always](https://github.com/rust-lang/rust/pull/118649)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Async Drop](https://github.com/rust-lang/rfcs/pull/3541)
* [Closure PartialEq and Eq](https://github.com/rust-lang/rfcs/pull/3538)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [Cargo cache cleaning](https://blog.rust-lang.org/2023/12/11/cargo-cache-cleaning.html)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-12-13 - 2024-01-10 🦀

### Virtual

* 2023-12-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/297172140)
* 2023-12-10 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Finale**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583652/)
* 2023-12-12 | Virtual | [Mainmatter](https://mainmatter.com)
    * [**Workshop: Telemetry for Rust applications**](https://rust-telemetry-workshop.mainmatter.com)
* 2023-12-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/297532862/)
* 2023-12-14 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833674/)
* 2023-12-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679660/)
* 2023-12-17 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Don't panic! - Our journey to error handling in Rust**](https://www.meetup.com/code-mavens/events/297334993/)
* 2023-12-18 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-19 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679827/) | [**Mirror**](https://berline.rs/)
* 2023-12-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128156/)
* 2023-12-19 | Virtual (Linz, AT) [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Ivan Cernja (TBD) & "Creating Data Structures in Rust"**](https://www.meetup.com/rust-linz/events/297909995/)
* 2023-12-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Adventures in egui app dev**](https://www.meetup.com/vancouver-rust/events/292763506/)
* 2023-12-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcqbjc/)
* 2023-12-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 2024-01-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)

### Asia

* 2023-12-10 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Fun with Labels, Macros, and Unsafe Rust**](https://www.meetup.com/tokyo-rust-meetup/events/297674153)
* 2023-12-16 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Meetup #4**](https://www.meetup.com/rustdelhi/events/297652586/)

### Europe

* 2023-12-06 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Advent Of Rust**](https://www.meetup.com/rustcologne/events/297100007/)
* 2023-12-07 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 2023-12-07 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #5**](https://www.meetup.com/meetup-group-zgphbyet/events/297477578/)
* 2023-12-07 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust London Christmas Party with Realm (RSVP on Eventbrite)**](https://www.meetup.com/rust-london-user-group/events/297703135/)
* 2023-12-14 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #4**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297025700/)
* 2023-12-14 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Testing: Learn from the pros**](https://www.meetup.com/rust-basel/events/297599586/)
* 2023-12-18 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-19 | Heidelberg, DE | [Nix Your Bugs & Rust Your Engines](https://rheinneckar.events/@nix_rust)
    * [**Nix Your Bugs & Rust Your Engines #1**](https://rheinneckar.events/events/298e520c-89ca-4754-96f8-e252b96b7a46)
* 2023-12-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tauri, an Electron-alternative**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504276/)

### North America

* 2023-12-07 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/297533440/)
* 2023-12-11 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760172/)
* 2023-12-12 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)
* 2023-12-12 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)  
    * [**Rust NYC Monthly Mixer: Share, Show, & Tell! 🦀**](https://www.meetup.com/rust-nyc/events/297659937/)
* 2023-12-13 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/297671182/)
* 2023-12-14 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Sending Data with Channels w/Herbert Wolverson**](https://www.meetup.com/utah-rust/events/297720170/)
* 2023-12-14 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297628069/)
* 2023-12-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297633899/)
* 2023-12-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcqbzb/)
* 2023-12-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcqbkc/)

### Oceania

* 2023-12-11 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust End of Year Event**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

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

Sadly, the week went by without a nominated quote.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
