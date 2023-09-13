Title: This Week in Rust 512
Number: 512
Date: 2023-09-13
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
* [Leadership change in the Rust Infrastructure Team](https://blog.rust-lang.org/inside-rust/2023/09/08/infra-team-leadership-change.html)

### Foundation

### Newsletters
* [This Month in Rust OSDev: August 2023](https://rust-osdev.com/this-month/2023-08/)

### Project/Tooling Updates
* [rust-analyzer changelog #198](https://rust-analyzer.github.io/thisweek/2023/09/11/changelog-198.html)
* [Meet Yazi: Blazing fast terminal file manager, written in Rust, based on async I/O](https://www.reddit.com/r/rust/comments/16fxr58/meet_yazi_blazing_fast_terminal_file_manager/)
* [Candle: A New Machine Learning Framework for Rust](https://thenewstack.io/candle-a-new-machine-learning-framework-for-rust/)
* [Wasmer 4.2 is Released: Upping the Ante with 50% Faster Module Load Times! 🚀](https://www.reddit.com/r/rust/comments/16cmgwg/wasmer_42_is_released_upping_the_ante_with_50/)
* [Roadmap to Tauri 2.0](https://beta.tauri.app/blog/roadmap-to-tauri-2-0/)
* [Introducing RustRover – A Standalone Rust IDE by JetBrains](https://blog.jetbrains.com/rust/2023/09/13/introducing-rustrover-a-standalone-rust-ide-by-jetbrains/)

### Observations/Thoughts
* [How Ferrocene improves Rust](https://ferrous-systems.com/blog/how-ferrocene-improves-rust/)
* [Owned values and Futures in Rust](https://www.snoyman.com/blog/owned-values-and-futures/)
* [Semver violations are common, better tooling is the answer](https://predr.ag/blog/semver-violations-are-common-better-tooling-is-the-answer/)
* [Rethinking Rust’s unsafe keyword](https://rainingcomputers.blog/dist/rethinking_rusts_unsafe_keyword.md)
* [Async Rust Is A Bad Language](https://bitbashing.io/async-rust.html)
* [Why you might actually want async in your project](https://notgull.net/why-you-want-async/)
* [I Wrote A String Type](https://mcyoung.xyz/2023/08/09/yarns/)
* [Writing a Web Scraper in Rust using Reqwest](https://www.shuttle.rs/blog/2023/09/13/web-scraping-rust-reqwest)

### Rust Walkthroughs

### Research

### Miscellaneous
* [What's The Time In Tokio?](https://work.yba.dev/what-s-the-time-in-tokio)
* [Teaching Rust in 5 days](https://mo8it.com/blog/teaching-rust/)
* [Semantic Search with Qdrant, OpenAI and Shuttle](https://www.shuttle.rs/blog/2023/09/08/building-semantic-search-in-rust)
* [ESP32 Standard Library Embedded Rust: GPIO Interrupts](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-gpio-interrupts)
* [video] [How to Do Embedded Development with Rust • Steve Klabnik • GOTO 2023](https://www.youtube.com/watch?v=7lHtXkYnip8)

## Crate of the Week

This week's crate is [irsenv](https://github.com/sysid/rs-env), a hierarchical environmant variable manager.

Thanks to [sysid](https://users.rust-lang.org/t/crate-of-the-week/2704/1237) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [zerocopy - Prevent panics statically](https://github.com/google/zerocopy/issues/202)
* [zerocopy - Implement traits for tuple types (and maybe other container types?)](https://github.com/google/zerocopy/issues/274)
* [zerocopy - [CI] Deduplicate `actions/cache` directives](https://github.com/google/zerocopy/issues/340)
* [zerocopy - Refactor version comparison logic in `check_versions` CI job](https://github.com/google/zerocopy/issues/307)
* [r3bl_rs_utils - Add styling support so that selected and unselected styles can be passed in](https://github.com/r3bl-org/r3bl_rs_utils/issues/116)
* [r3bl_rs_utils - Create a function that allows multiple selections from a list of strings](https://github.com/r3bl-org/r3bl_rs_utils/issues/123)
* [r3bl_rs_utils - Add support for syntect output](https://github.com/r3bl-org/r3bl_rs_utils/issues/125)
* [Ockam - Use the Terminal to print out RPC response instead of printlns - Issue #5904 - build-trust/ockam - GitHub](https://github.com/build-trust/ockam/issues/5904)
* [Ockam - Investigate and fix breaking changes in upgrading from nix crate 0.26.2 to 0.27.1](https://github.com/build-trust/ockam/issues/5936)
* [Ockam - Give user feedback on using Ockam Command CLI](https://github.com/build-trust/ockam/issues/5523)
* [RustQuant - Increase test coverage.](https://github.com/avhz/RustQuant/issues/57)
* [RustQuant - Create Python bindings.](https://github.com/avhz/RustQuant/issues/39)
* [RustQuant - Add/improve documentation.](https://github.com/avhz/RustQuant/issues/14)
* [RustQuant - Implement jump-diffusion simulator.](https://github.com/avhz/RustQuant/issues/89)
* [hyper - hyper 1.0 API docs polish - meta issue](https://github.com/hyperium/hyper/issues/3067)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

382 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-09-04..2023-09-11

* [move wasm32-wasi-preview1-threads target to Tier 2](https://github.com/rust-lang/rust/pull/115345)
* [`-Cllvm-args` usability improvement](https://github.com/rust-lang/rust/pull/115638)
* [debuginfo: add compiler option to allow compressed debuginfo sections](https://github.com/rust-lang/rust/pull/115358)
* [`rustc_layout, rustc_abi`: make sure the types are well-formed](https://github.com/rust-lang/rust/pull/115712)
* [abort if check nightly options failed on stable](https://github.com/rust-lang/rust/pull/115713)
* [add CL and CMD into to pdb debug info](https://github.com/rust-lang/rust/pull/113492)
* [add support to return value in StableMIR interface and not crash due to compilation error](https://github.com/rust-lang/rust/pull/115397)
* [allow `large_assignments` for Box/Arc/Rc initialization](https://github.com/rust-lang/rust/pull/115492)
* [allow redirecting subprocess stdout to our stderr etc. (redux)](https://github.com/rust-lang/rust/pull/114590)
* [avoid a `source_span` query when encoding Spans into query results](https://github.com/rust-lang/rust/pull/115657)
* [better Debug for `Ty` in smir](https://github.com/rust-lang/rust/pull/115605)
* [bubble up opaque `<eq>` opaque operations instead of picking an order](https://github.com/rust-lang/rust/pull/114586)
* [call `LateLintPass::check_attribute` from `with_lint_attrs`](https://github.com/rust-lang/rust/pull/115739)
* [do not require associated types with Self: Sized to uphold bounds when confirming object candidate](https://github.com/rust-lang/rust/pull/115467)
* [don't ICE on associated type projection without feature gate in new solver](https://github.com/rust-lang/rust/pull/115519)
* [don't ICE when computing ctype's `repr_nullable_ptr` for possibly-unsized ty](https://github.com/rust-lang/rust/pull/115631)
* [don't report any errors in `lower_intrinsics`](https://github.com/rust-lang/rust/pull/115602)
* [don't require `Drop` for `[PhantomData<T>; N]` where `N` and `T` are generic, if `T` requires `Drop`](https://github.com/rust-lang/rust/pull/115527)
* [don't suggest dereferencing to unsized type](https://github.com/rust-lang/rust/pull/115629)
* [dont suggest use between `use` and cfg attr](https://github.com/rust-lang/rust/pull/115630)
* [emit error instead of ICE when optimized MIR is missing](https://github.com/rust-lang/rust/pull/115353)
* [enable incremental-relative-spans by default](https://github.com/rust-lang/rust/pull/115656)
* [explain why we can mutate the FPU control word](https://github.com/rust-lang/rust/pull/114813)
* [expose more information with DefId in smir](https://github.com/rust-lang/rust/pull/115534)
* [extract parallel operations in `rustc_data_structures::sync` into a new `parallel` submodule](https://github.com/rust-lang/rust/pull/115548)
* [fix ICE in `improper_ctypes_definitions` lint](https://github.com/rust-lang/rust/pull/115698)
* [fix Step Skipping Caused by Using the `--exclude` Option](https://github.com/rust-lang/rust/pull/115088)
* [fix `homogeneous_aggregate` not ignoring some ZST](https://github.com/rust-lang/rust/pull/115708)
* [fix error report for size overflow from transmute](https://github.com/rust-lang/rust/pull/115529)
* [fix overflow in array length computation](https://github.com/rust-lang/rust/pull/115335)
* [fix: return early when has tainted in mir-lint](https://github.com/rust-lang/rust/pull/115643)
* [implement fallback for effect param](https://github.com/rust-lang/rust/pull/115727)
* [implement refinement lint for RPITIT](https://github.com/rust-lang/rust/pull/115582)
* [implied bounds: do not ICE on unconstrained region vars](https://github.com/rust-lang/rust/pull/115559)
* [improve "associated type not found" diagnostics](https://github.com/rust-lang/rust/pull/115662)
* [improve `AttrTokenStream`](https://github.com/rust-lang/rust/pull/115523)
* [improve diagnostic for generic params from outer items (E0401)](https://github.com/rust-lang/rust/pull/115744)
* [fix `unnecessary_unsafe` false positive](https://github.com/rust-lang/rust/pull/115587)
* [fix incorrect mutable suggestion information for binding in ref pattern like: `let &b = a;`](https://github.com/rust-lang/rust/pull/115595)
* [lint node for `private_bounds`/`private_interfaces` is the item which names the private type](https://github.com/rust-lang/rust/pull/115633)
* [lint on invalid usage of `UnsafeCell::raw_get` in reference casting](https://github.com/rust-lang/rust/pull/115166)
* [make if let guard parsing consistent with normal guards](https://github.com/rust-lang/rust/pull/115371)
* [make the deadlock panic clearly refer to a deadlock](https://github.com/rust-lang/rust/pull/115668)
* [make unknown/renamed/removed lints passed via command line respect lint levels](https://github.com/rust-lang/rust/pull/115387)
* [point out if a local trait has no implementations](https://github.com/rust-lang/rust/pull/115743)
* [preserve ASAN-related symbols during LTO](https://github.com/rust-lang/rust/pull/114946)
* [print the path of a return-position impl trait in trait when `return_type_notation` is enabled](https://github.com/rust-lang/rust/pull/115624)
* [remove the unhelpful let binding diag comes from `FormatArguments`](https://github.com/rust-lang/rust/pull/114511)
* [replace `rustc_data_structures` dependency with `rustc_index` in `rustc_parse_format`](https://github.com/rust-lang/rust/pull/115574)
* [suggest `iter_mut()` where trying to modify elements from `.iter()`](https://github.com/rust-lang/rust/pull/115308)
* [support debuginfo for custom MIR](https://github.com/rust-lang/rust/pull/115540)
* [take `&mut Results` in `ResultsVisitor`](https://github.com/rust-lang/rust/pull/115488)
* [use `newtype_index` for `IntVid` and `FloatVid`](https://github.com/rust-lang/rust/pull/115634)
* [use relative positions inside a SourceFile](https://github.com/rust-lang/rust/pull/115507)
* [use the same DISubprogram for each instance of the same inlined function within a caller](https://github.com/rust-lang/rust/pull/115417)
* [represent MIR composite debuginfo as projections instead of aggregates](https://github.com/rust-lang/rust/pull/115252)
* [encode only MIR reachable from other crates](https://github.com/rust-lang/rust/pull/115306)
* [allow loading the SMIR for constants and statics](https://github.com/rust-lang/rust/pull/115749)
* [implement SMIR generic parameter instantiation](https://github.com/rust-lang/rust/pull/115532)
* [miri: catch function calls where the argument is caller-invalid / the return value callee-invalid](https://github.com/rust-lang/rust/pull/115608)
* [miri: use `#!/usr/bin/env` shebang](https://github.com/rust-lang/miri/pull/3056)
* [add optimized lock methods for `Sharded` and refactor `Lock`](https://github.com/rust-lang/rust/pull/115388)
* [add `FreezeLock` type and use it to store `Definitions`](https://github.com/rust-lang/rust/pull/115401)
* [use `FreezeLock` for `CStore`](https://github.com/rust-lang/rust/pull/115711)
* [use `Freeze` for `SourceFile`](https://github.com/rust-lang/rust/pull/115418)
* [span tweaks](https://github.com/rust-lang/rust/pull/115594)
* [use a specialized varint + bitpacking scheme for DepGraph encoding](https://github.com/rust-lang/rust/pull/110050)
* [add `char::MIN`](https://github.com/rust-lang/rust/pull/114299)
* [stabilize `io_error_other` feature](https://github.com/rust-lang/rust/pull/115453)
* [hashbrown: make allocator not `Clone`](https://github.com/rust-lang/hashbrown/pull/468)
* [codegen\_gcc: fix const handling in ATT syntax](https://github.com/rust-lang/rustc_codegen_gcc/pull/330)
* [codegen\_gcc: set the correct gimple output format](https://github.com/rust-lang/rustc_codegen_gcc/pull/328)
* [cargo-credential: change serialization of cache expiration](https://github.com/rust-lang/cargo/pull/12622)
* [cargo: Add styling to help output](https://github.com/rust-lang/cargo/pull/12578)
* [cargo: Make resolver behavior independent of package order](https://github.com/rust-lang/cargo/pull/12602)
* [cargo: error out if `cargo clean --doc` is mixed with `-p`](https://github.com/rust-lang/cargo/pull/12637)
* [cargo: stabilize lints](https://github.com/rust-lang/cargo/pull/12648)
* [cargo: fix: don't print `_TOKEN` suggestion when not applicable](https://github.com/rust-lang/cargo/pull/12644)
* [cargo: fix: improve warning for both token & credential-provider](https://github.com/rust-lang/cargo/pull/12626)
* [rustdoc: add impl items from aliased type into sidebar](https://github.com/rust-lang/rust/pull/115682)
* [rustdoc: add missing "Aliased type" title in the sidebar](https://github.com/rust-lang/rust/pull/115752)
* [rustdoc: change syntax for anonymous functions set in JS](https://github.com/rust-lang/rust/pull/115669)
* [rustdoc: list matching impls on type aliases](https://github.com/rust-lang/rust/pull/115201)
* [rustdoc: render private fields in tuple `struct` as `/* private fields */`](https://github.com/rust-lang/rust/pull/115604)
* [rustdoc: show inner `enum` and `struct` in type definition for concrete type](https://github.com/rust-lang/rust/pull/114855)
* [rustfmt: Prefer `light_rewrite_comment` if it is not a doccomment](https://github.com/rust-lang/rustfmt/pull/5536)
* [rustfmt: fix checking if newline is needed before `else` in let-else statement](https://github.com/rust-lang/rustfmt/pull/5902)
* [clippy: `implied_bounds_in_impls`: include (previously omitted) associated types in suggestion](https://github.com/rust-lang/rust-clippy/pull/11459)
* [clippy: `slow_vector_initialization`: use the source span of `vec![]` macro and fix another FP](https://github.com/rust-lang/rust-clippy/pull/11451)
* [clippy: add suggestions for `std_instead_of_core`](https://github.com/rust-lang/rust-clippy/pull/11456)
* [clippy: auto deref does not apply on union field](https://github.com/rust-lang/rust-clippy/pull/11477)
* [clippy: check binary operators and attributes in `disallowed_macros`](https://github.com/rust-lang/rust-clippy/pull/11439)
* [clippy: ignore wildcards in function arguments and local bindings](https://github.com/rust-lang/rust-clippy/pull/11454)
* [clippy: preserve literals and range kinds in `manual_range_patterns`](https://github.com/rust-lang/rust-clippy/pull/11462)
* [clippy: rename `incorrect_impls` to `non_canonical_impls,` move them to warn by default](https://github.com/rust-lang/rust-clippy/pull/11358)
* [rust-analyzer: add "Bind unused parameter" assist](https://github.com/rust-lang/rust-analyzer/pull/15524)
* [rust-analyzer: add `into_to_qualified_from` assist](https://github.com/rust-lang/rust-analyzer/pull/15573)
* [rust-analyzer: diagnose mismatched arg count for tuple `struct` patterns](https://github.com/rust-lang/rust-analyzer/pull/15578)
* [rust-analyzer: diagnose private fields in record constructor](https://github.com/rust-lang/rust-analyzer/pull/15584)
* [rust-analyzer: enable `rust_analyzer` for cfgs when code is being analyzed by rust-analyzer](https://github.com/rust-lang/rust-analyzer/pull/15528)
* [rust-analyzer: implement `builtin#format_args,` using rustc's `format_args` parser](https://github.com/rust-lang/rust-analyzer/pull/15559)
* [rust-analyzer: on type format '(', by adding closing ')' automatically](https://github.com/rust-lang/rust-analyzer/pull/15532)
* [rust-analyzer: parse builtin# syntax and add typechecking for `builtin#offset_of` expression](https://github.com/rust-lang/rust-analyzer/pull/15557)
* [rust-analyzer: clear native diagnostics on file closing](https://github.com/rust-lang/rust-analyzer/pull/15577)
* [rust-analyzer: disallow renaming of non-local items](https://github.com/rust-lang/rust-analyzer/pull/15232)
* [rust-analyzer: use crate name for `CARGO_CRATE_NAME`](https://github.com/rust-lang/rust-analyzer/pull/15574)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Allow cfg-attributes in where clauses](https://github.com/rust-lang/rfcs/pull/3399)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Cargo Check T-lang Policy](https://github.com/rust-lang/rfcs/pull/3477)
* [disposition: merge] [[RFC2603] Extend `<const>` to include `str` and structural constants.](https://github.com/rust-lang/rfcs/pull/3161)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Raise minimum supported Apple OS versions](https://github.com/rust-lang/rust/pull/104385)
* [disposition: merge] [Stabilize const_transmute_copy](https://github.com/rust-lang/rust/pull/115520)
* [disposition: merge] [Don't resolve generic impls that may be shadowed by dyn built-in impls](https://github.com/rust-lang/rust/pull/114941)
* [disposition: merge] [closure field capturing: don't depend on alignment of packed fields](https://github.com/rust-lang/rust/pull/115315)
* [disposition: merge] [Accept additional user-defined syntax classes in fenced code blocks](https://github.com/rust-lang/rust/pull/110800)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Unify crate categories and keywords as tags](https://github.com/rust-lang/rfcs/pull/3488)
* [new] [RFC: Cargo feature visibility (private/public)](https://github.com/rust-lang/rfcs/pull/3487)
* [new] [RFC: Cargo feature deprecation](https://github.com/rust-lang/rfcs/pull/3486)
* [new] [RFC: Cargo feature descriptions](https://github.com/rust-lang/rfcs/pull/3485)
* [new] [Unsafe Extern Blocks](https://github.com/rust-lang/rfcs/pull/3484)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-09-13 - 2023-10-11 🦀

### Virtual

* 2023-09-12 - 2023-09-15 | Virtual (Albuquerque, NM, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-13 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/295011539)
* 2023-09-13 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**The unreasonable power of combinator APIs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294748626)
* 2023-09-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732655)
* 2023-09-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—NeuronBench by Greg Hale**](https://www.meetup.com/rustdc/events/295778065)
* 2023-09-20 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**SurrealDB for Rustaceans**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/295826608/)
* 2023-09-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/295057154/)
* 2023-09-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/295666673/)
* 2023-09-21 | Virtual (Cologne, DE) | [Cologne AWS User Group #AWSUGCGN](https://www.meetup.com/aws-cologne/)
    * [**AWS User Group Cologne - September Edition: Stefan Willenbrock: Developer Preview: Discovering Rust on AWS**](https://www.meetup.com/aws-cologne/events/294594401/)
* 2023-09-21 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 33rd Edition**](https://www.meetup.com/rust-linz/events/295363887/)
* 2023-09-21 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/295828383/)
* 2023-09-25 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**How we built the SurrealDB Python client in Rust.**](https://www.meetup.com/Rust-Dublin/events/294908596/)
* 2023-09-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/295942051/)
* 2023-10-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295919493/)
* 2023-10-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 2023-10-04 | Virtual (Various) | [Ferrous Systems](https://www.eventbrite.com/o/ferrous-systems-gmbh-68735392123)
    * [**A Decade of Rust with Ferrous Systems**](https://www.eventbrite.com/e/a-decade-of-rust-with-ferrous-systems-tickets-680492891557?aff=ebdssbdestsearch)
* 2023-10-07 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup: Mentorship (First Saturday)**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763617907?aff=erelpanelorg)
* 2023-10-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcnbnb/)
* 2023-10-11| Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**]https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcnbpb/)
 
### Asia

* 2023-10-03 | Taipei, TW | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/)
    * [**WebAssembly Meetup (Wasm Empowering AI) in Taipei**](https://www.meetup.com/wasm-rust-meetup/events/295672575/)

### Europe

* 2023-09-13 | Cologne, DE | [Rust User Group Cologne](https://rust.cologne/2023/09/13/rare-rust.html)
    * [**Rare Rust**](https://www.meetup.com/rustcologne/events/295869748/) | [**Group Detail Page**](https://rust.cologne/2023/09/13/rare-rust.html)
* 2023-09-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/295109905/)
* 2023-09-15 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Onsite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/295639296/)
* 2023-09-15 | Tiel, NL | [Rust, Getting Started](https://www.meetup.com/rust-getting-started/)
    * [**Rust Workshop - 2**](https://www.meetup.com/rust-getting-started/events/295880062/)
* 2023-09-16 | Brussels, BE | [HSBXL](https://hsbxl.be/events/software-freedom-day/2023-09-16/)
    * [**Software Freedom Day 2023**](https://www.meetup.com/brussels-hackerspace/events/295912633/)
* 2023-09-19 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Logging and tracing in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504245/)
* 2023-09-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus - Rust and Talk at Concordium**](https://www.meetup.com/rust-aarhus/events/294031975/)
* 2023-09-21 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Rust Bern Meetup #3 2023 🦀**](https://www.meetup.com/rust-bern/events/295503351/)
* 2023-09-26 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679767/)
* 2023-09-28 | Berlin, DE | [React Berlin](https://www.meetup.com/react-berlin-meetup/)
    * [**React Berlin September Meetup: Creating Videos with React & Remotion & More: Integrating Rust with React Native – Gheorghe Pinzaru**](https://www.meetup.com/react-berlin-meetup/events/295382108/)
* 2023-09-28 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/)
    * [**Primer evento Post COVID: ¡Cervezas MadRust!**](https://www.meetup.com/madrust/events/296063394/)
* 2023-09-30 | Saint Petersburg, RU | [Rust Saint Petersburg meetups](https://t.me/ruRust_spb)
    * [**Rust Community Meetup: A tale about how I tried to make my Blitz Basic - Vitaly; How to use nix to build projects on Rust – Danil; Getting to know tower middleware. General overview – Mikhail**](https://rurust-saint-petersburg-m.timepad.ru/event/2561864/) 
* 2023-10-10 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679773/)

### North America

* 2023-09-12 - 2023-09-15 | Albuquerque, NM, US  + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-14 | Seattle, WA, US | [Seattle Rust User Group Meetup](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - August Meetup**](https://www.meetup.com/seattle-rust-user-group/events/295484105)
* 2023-09-16 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/295579189/)
* 2023-09-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/295545278)
* 2023-09-21 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Real Time Multiplayer Game Server in Rust**](https://www.meetup.com/utah-rust/events/294972877/)
* 2023-09-21 | Mountain View, CA, US| [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/295747006/)
* 2023-09-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust on the web! Get started with Leptos**](https://www.meetup.com/music-city-rust-developers/events/295587220/)
* 2023-09-23 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/295869150/)
* 2023-09-26 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/295771515)
* 2023-09-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295466314)
* 2023-09-30 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/fktvgtyfcmbnc/)
* 2023-10-07 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/fktvgtyfcnbkb/)

### Oceania

* 2023-09-14 | Perth, WA, AU | [Rust Perth](https://www.linkedin.com/groups/7439562/)
    * [**Rust Meetup 2: Lunch & Learn**](https://www.linkedin.com/events/7097356771584880640/) | [**Ticket Link**](https://www.tickettailor.com/events/perthrustusergroup/984771)
* 2023-09-19 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/295602231/)
* 2023-09-26 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/295432237/)

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

> It's very much a positive feedback loop: good tooling makes good tooling easier to build, so more of it gets built and the cycle repeats.  
> `cargo-semver-checks` stands on the shoulders of giants like `rustc` and `rustdoc` and [Trustfall](https://github.com/obi1kenobi/trustfall). Remove any one of them (or even just `rustc`'s high-quality diagnostics!) and `cargo-semver-checks` wouldn't have been a viable project at all. 

– [Predrag Gruevski on /r/rust](https://www.reddit.com/r/rust/comments/16cj1mo/comment/jzjw4vk/?utm_source=share&utm_medium=web2x&context=3)

Thanks to [Vincent de Phily](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1466) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
