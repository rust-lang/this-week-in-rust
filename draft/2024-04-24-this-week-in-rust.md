Title: This Week in Rust 544
Number: 544
Date: 2024-04-24
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

### Foundation

### Newsletters

### Project/Tooling Updates
* [venndb 0.4.0 - custom validators](https://github.com/plabayo/venndb/releases/tag/0.4.0)
* [Tantivy 0.22: Performance improvements, new aggregations and stability fixes](https://quickwit.io/blog/tantivy-0.22)
* [Announcing r3bl_terminal_async - build async interactive CLIs in Rust](https://developerlife.com/2024/04/21/build-async-interactive-cli-apps-in-rust/)

* [video] [Demo + architecture overview of Ferrostar, a cross-platform turn-by-turn navigation SDK with a Rust core](https://www.youtube.com/watch?v=8PuWu_Pi2sk)

### Observations/Thoughts

### Rust Walkthroughs

* [video] [Build with Naz - Published crate r3bl_terminal_async for building async interactive CLIs in Rust](https://www.youtube.com/watch?v=X5wDVaZENOo)

### Research
* [Rust Digger: Does size matter? The biggest crate is 450MB; More than 100 crates are over 50MB](https://rust-digger.code-maven.com/news/biggest-crates-is-450-megabyte)

### Miscellaneous
- [Learn how to write TCP servers using Rust's std::net module](https://app.codecrafters.io/concepts/rust-tcp-server)

## Crate of the Week

This week's crate is [scandir](https://crates.io/crates/scandir), a high-performance file tree scanner.

Thanks to [Marty B.](https://users.rust-lang.org/t/crate-of-the-week/2704/1305) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No calls for testing were issued this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

432 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-04-16..2024-04-23

* [add simple async drop glue generation](https://github.com/rust-lang/rust/pull/121801)
* [llvm backend: Prevent creating the same `Instance::mono` multiple times](https://github.com/rust-lang/rust/pull/124178)
* [`static_mut_refs`: use raw pointers to remove the remaining FIXME](https://github.com/rust-lang/rust/pull/123967)
* [add a lower bound check to `unicode-table-generator` output](https://github.com/rust-lang/rust/pull/122013)
* [add an opt-in to store incoming edges in `VecGraph` + misc](https://github.com/rust-lang/rust/pull/123980)
* [add llvm-bitcode-linker to build manifest](https://github.com/rust-lang/rust/pull/124071)
* [allow workproducts without object files](https://github.com/rust-lang/rust/pull/124023)
* [at debuginfo=0, don't inline debuginfo when inlining](https://github.com/rust-lang/rust/pull/123949)
* [correctly change type when adding adjustments on top of `NeverToAny`](https://github.com/rust-lang/rust/pull/123571)
* [coverage: branch coverage tests for lazy boolean operators](https://github.com/rust-lang/rust/pull/124053)
* [coverage: prepare for improved branch coverage](https://github.com/rust-lang/rust/pull/124217)
* [delay interning errors to after validation](https://github.com/rust-lang/rust/pull/122684)
* [delay span bug when `Self` kw resolves to `DefKind::{Mod,Trait}`](https://github.com/rust-lang/rust/pull/123997)
* [don't ICE for kind mismatches during error rendering](https://github.com/rust-lang/rust/pull/123673)
* [don't ascend into parent bodies when collecting stmts for possible return suggestion](https://github.com/rust-lang/rust/pull/124037)
* [don't error on subtyping of equal types](https://github.com/rust-lang/rust/pull/124066)
* [don't inline integer literals when they overflow - new attempt](https://github.com/rust-lang/rust/pull/123935)
* [don't repeatedly duplicate TAIT lifetimes for each subsequently nested TAIT](https://github.com/rust-lang/rust/pull/124106)
* [fix ICE in `eval_body_using_ecx`](https://github.com/rust-lang/rust/pull/123491)
* [fix ICE when there is a non-Unicode entry in the incremental crate directory](https://github.com/rust-lang/rust/pull/124112)
* [fix capturing duplicated lifetimes via parent in `precise_captures` (`impl use<'...>`)](https://github.com/rust-lang/rust/pull/124104)
* [fix normalizing in different `ParamEnv`s with the same InferCtxt](https://github.com/rust-lang/rust/pull/124203)
* [fix trait solver overflow with `non_local_definitions` lint](https://github.com/rust-lang/rust/pull/123594)
* [flip spans for precise capturing syntax not capturing a ty/const param, and for implicit captures of lifetime params](https://github.com/rust-lang/rust/pull/124198)
* [give a name to each distinct manipulation of pretty-printer FixupContext](https://github.com/rust-lang/rust/pull/124191)
* [ignore `-C strip` on MSVC](https://github.com/rust-lang/rust/pull/115120)
* [implement Modified Condition/Decision Coverage](https://github.com/rust-lang/rust/pull/123409)
* [implement `PROBLEMATIC_CONSTS` generalization](https://github.com/rust-lang/rust/pull/115253)
* [implement syntax for `impl Trait` to specify its captures explicitly (`feature(precise_capturing)`)](https://github.com/rust-lang/rust/pull/123468)
* [improve ICE message for forbidden dep-graph reads](https://github.com/rust-lang/rust/pull/124252)
* [interpret: pass MemoryKind to `adjust_alloc_base_pointer`](https://github.com/rust-lang/rust/pull/124030)
* [interpret: pass MemoryKind to `before_memory_deallocation`](https://github.com/rust-lang/rust/pull/124018)
* [interpret: use ScalarInt for bin-ops; avoid PartialOrd for ScalarInt](https://github.com/rust-lang/rust/pull/124113)
* [introduce perma-unstable `wasm-c-abi` flag](https://github.com/rust-lang/rust/pull/117919)
* [let inherent associated types constrain opaque types during projection](https://github.com/rust-lang/rust/pull/124166)
* [make `suggest_deref_closure_return` more idiomatic/easier to understand](https://github.com/rust-lang/rust/pull/123990)
* [make `thir_tree` and `thir_flat` into hooks](https://github.com/rust-lang/rust/pull/123995)
* [make the comments for `ReturnDest` variants doc comments](https://github.com/rust-lang/rust/pull/124052)
* [match ergonomics 2024: Implement eat-one-layer](https://github.com/rust-lang/rust/pull/123512)
* [match ergonomics 2024: `mut` doesn't reset binding mode](https://github.com/rust-lang/rust/pull/123535)
* [match hyphen in multi-revision comment matchers](https://github.com/rust-lang/rust/pull/124137)
* [opaque types have no namespace](https://github.com/rust-lang/rust/pull/123998)
* [outline default query and hook provider function implementations](https://github.com/rust-lang/rust/pull/124016)
* [prefer identity equality over equating types during coercion](https://github.com/rust-lang/rust/pull/124027)
* [print note with closure signature on type mismatch](https://github.com/rust-lang/rust/pull/123379)
* [properly handle emojis as literal prefix in macros](https://github.com/rust-lang/rust/pull/123752)
* [remove `default_hidden_visibility: false` from wasm targets](https://github.com/rust-lang/rust/pull/124036)
* [remove uneeded clones now that TrustedStep implies Copy](https://github.com/rust-lang/rust/pull/123859)
* [silence some follow-up errors on trait impls in case the trait has conflicting or otherwise incoherent impls](https://github.com/rust-lang/rust/pull/123674)
* [simplify shallow resolver to just fold ty/consts](https://github.com/rust-lang/rust/pull/123537)
* [stop taking `ParamTy`/`ParamConst`/`EarlyParamRegion`/`AliasTy` by ref](https://github.com/rust-lang/rust/pull/124183)
* [subtype predicates only exist on inference types, so we can allow them to register opaque types within them](https://github.com/rust-lang/rust/pull/123979)
* [tell LLVM `Vec::len` is invariant across growth](https://github.com/rust-lang/rust/pull/123930)
* [use raw-dylib for Windows synchronization functions](https://github.com/rust-lang/rust/pull/124019)
* [refactor clippy in bootstrap](https://github.com/rust-lang/rust/pull/122883)
* [when suggesting `RUST_BACKTRACE=1,` add a special note for Miri's env var isolation](https://github.com/rust-lang/rust/pull/124116)
* [miri: `data_race`: make the release/acquire API more clear](https://github.com/rust-lang/miri/pull/3495)
* [miri: `no_std` works on Windows now](https://github.com/rust-lang/miri/pull/3477)
* [miri: add `localtime_r` shim](https://github.com/rust-lang/miri/pull/3461)
* [miri: address reuse improvements and fixes](https://github.com/rust-lang/miri/pull/3475)
* [miri: deadlock: show backtrace for all threads](https://github.com/rust-lang/miri/pull/3472)
* [miri: directly call `handle_alloc_error`](https://github.com/rust-lang/miri/pull/3480)
* [miri: implement support for `__rust_alloc_error_handler`](https://github.com/rust-lang/miri/pull/3478)
* [miri: make realloc with a size of zero fail](https://github.com/rust-lang/miri/pull/3484)
* [miri: move `read_byte_slice` to general helpers file, next to `read_c_str`](https://github.com/rust-lang/miri/pull/3485)
* [miri: threads: keep track of why we are blocked, and sanity-check that when waking up](https://github.com/rust-lang/miri/pull/3471)
* [`Box::into_raw`: make Miri understand that this is a box-to-raw cast](https://github.com/rust-lang/rust/pull/124013)
* [`PatRangeBoundary::compare_with`: also add a fast-path for signed integers](https://github.com/rust-lang/rust/pull/124190)
* [codegen ZSTs without an allocation](https://github.com/rust-lang/rust/pull/123936)
* [stabilize Wasm target features that are in phase 4 and 5](https://github.com/rust-lang/rust/pull/117457)
* [stabilize `const_io_structs`](https://github.com/rust-lang/rust/pull/124049)
* [stabilize checking of cfgs at compile-time: `--check-cfg` option](https://github.com/rust-lang/rust/pull/123501)
* [stabilize generic `NonZero`](https://github.com/rust-lang/rust/pull/124230)
* [make `checked` ops emit *unchecked* LLVM operations where feasible](https://github.com/rust-lang/rust/pull/124114)
* [improve `std::fs::Metadata` Debug representation](https://github.com/rust-lang/rust/pull/124103)
* [fix negating `f16` and `f128` constants](https://github.com/rust-lang/rust/pull/124110)
* [force exhaustion in `iter::ArrayChunks::into_remainder`](https://github.com/rust-lang/rust/pull/123406)
* [`checked_ilog`: improve performance](https://github.com/rust-lang/rust/pull/115913)
* [add an intrinsic for `ptr::from_raw_parts(_mut)`](https://github.com/rust-lang/rust/pull/123840)
* [fix: make `str::from_raw_parts_mut mut`](https://github.com/rust-lang/rust/pull/124100)
* [use queue-based `RwLock` on more platforms](https://github.com/rust-lang/rust/pull/123811)
* [add support for Arm64EC to the standard library](https://github.com/rust-lang/rust/pull/123144)
* [codegen\_gcc: fix `PassMode::Indirect` with params](https://github.com/rust-lang/rustc_codegen_gcc/pull/498)
* [codegen\_gcc: fix check for main function already declared](https://github.com/rust-lang/rustc_codegen_gcc/pull/497)
* [codegen\_gcc: fix panic when calling `get_fn` for a variable](https://github.com/rust-lang/rustc_codegen_gcc/pull/499)
* [codegen\_gcc: fix passing custom `CG_RUSTFLAGS` when building sysroot](https://github.com/rust-lang/rustc_codegen_gcc/pull/493)
* [codegen\_gcc: implement more type kinds](https://github.com/rust-lang/rustc_codegen_gcc/pull/500)
* [cargo install: including Locking message](https://github.com/rust-lang/cargo/pull/13764)
* [cargo resolver: add default Edition2024 to resolver v3](https://github.com/rust-lang/cargo/pull/13785)
* [cargo resolver: add v3 resolver for MSRV-aware resolving](https://github.com/rust-lang/cargo/pull/13776)
* [cargo credential: trim newlines in tokens from stdin](https://github.com/rust-lang/cargo/pull/13770)
* [cargo msrv: error, rather than panic, on rust-version 'x'](https://github.com/rust-lang/cargo/pull/13771)
* [cargo msrv: put MSRV-aware resolver behind a config](https://github.com/rust-lang/cargo/pull/13769)
* [cargo toml: don't crash on parse errors that point to multi-byte character](https://github.com/rust-lang/cargo/pull/13780)
* [cargo toml: disallow source-less dependencies](https://github.com/rust-lang/cargo/pull/13775)
* [cargo toml: error on `[project]` in Edition 2024](https://github.com/rust-lang/cargo/pull/13747)
* [cargo toml: report `_`fied variants (e.g. `dev_dependencies`) as deprecated](https://github.com/rust-lang/cargo/pull/13783)
* [cargo: fix 'cargo build' fails when `list_files()` with gix is triggered](https://github.com/rust-lang/cargo/pull/13777)
* [rustdoc: always display stability version even if it's the same as the containing item](https://github.com/rust-lang/rust/pull/118441)
* [rustdoc: fix copy path button](https://github.com/rust-lang/rust/pull/124041)
* [rustdoc: support type '/' to search](https://github.com/rust-lang/rust/pull/123355)
* [rustdoc-search: fix description on aliases in results](https://github.com/rust-lang/rust/pull/124149)
* [rustdoc-search: single result for items with multiple paths](https://github.com/rust-lang/rust/pull/119912)
* [clippy: `threadlocal_initializer_can_be_made_const` will not trigger for unreachable initializers](https://github.com/rust-lang/rust-clippy/pull/12685)
* [clippy: `arithmetic_side_effects` fix false negative on `+=`](https://github.com/rust-lang/rust-clippy/pull/12692)
* [clippy: `ptr_as_ptr`: fix duplicate diagnostics](https://github.com/rust-lang/rust-clippy/pull/12673)
* [clippy: emit the `needless_pass_by_ref_mut` lint on `self` arguments as well](https://github.com/rust-lang/rust-clippy/pull/12693)
* [clippy: fix `is_test_module_or_function`](https://github.com/rust-lang/rust-clippy/pull/12696)
* [clippy: reduce `single_char_pattern` to only lint on ascii chars](https://github.com/rust-lang/rust-clippy/pull/11852)
* [clippy: rework interior mutability detection](https://github.com/rust-lang/rust-clippy/pull/12691)
* [clippy: the `multiple_unsafe_ops_per_block` test needs `asm!`](https://github.com/rust-lang/rust-clippy/pull/12682)
* [rust-analyzer: cargo script mvp](https://github.com/rust-lang/rust-analyzer/pull/17110)
* [rust-analyzer: add convert From to TryFrom assist](https://github.com/rust-lang/rust-analyzer/pull/17094)
* [rust-analyzer: allow rust files to be used linkedProjects](https://github.com/rust-lang/rust-analyzer/pull/17118)
* [rust-analyzer: VFS should not walk circular symlinks](https://github.com/rust-lang/rust-analyzer/pull/17093)
* [rust-analyzer: handle escaped chars in doc comments](https://github.com/rust-lang/rust-analyzer/pull/17024)
* [rust-analyzer: replace Just the variable name in Unused Variable Diagnostic Fix](https://github.com/rust-lang/rust-analyzer/pull/17055)
* [rust-analyzer: implement `BeginPanic` handling in const eval](https://github.com/rust-lang/rust-analyzer/pull/16938)
* [rust-analyzer: make test harness arguments configurable and not `--nocapture`](https://github.com/rust-lang/rust-analyzer/pull/17105)
* [rust-analyzer: render matched macro arm on hover of macro calls](https://github.com/rust-lang/rust-analyzer/pull/16057)
* [rust-analyzer: try to generate more meaningful names in json converter](https://github.com/rust-lang/rust-analyzer/pull/17115)

### Rust Compiler Performance Triage

A week dominated by small mixed changes to perf with improvements slightly outweighing regressions. There were no pure regressions, and many of the mixed perf results were deemed worth it for their potential improvements to runtime performance through further optimization from LLVM.

Triage done by **@rylev**.
Revision range: [ccfcd950..a77f76e2](https://perf.rust-lang.org/?start=ccfcd950b333fed046275dd8d54fe736ca498aa7&end=a77f76e26302e9a084fb321817675b1dfc1dcd63&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.2%, 1.8%]   | 57    |
| Regressions ❌ <br /> (secondary)  | 0.4%  | [0.2%, 1.9%]   | 26    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-3.4%, -0.2%] | 50    |
| Improvements ✅ <br /> (secondary) | -0.6% | [-1.9%, -0.1%] | 32    |
| All ❌✅ (primary)                 | -0.2% | [-3.4%, 1.8%]  | 107   |


0 Regressions, 5 Improvements, 6 Mixed; 2 of them in rollups
62 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/e29814aa8d206406ae2711773bd882b39598a9d8/triage/2024-04-23.md)

## Call for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No calls for testing were issued this week.*

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

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
* [disposition: merge] [experimental project goal program for 2024 H2](https://github.com/rust-lang/rfcs/pull/3614)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for `std::path::absolute`](https://github.com/rust-lang/rust/issues/92750)
* [disposition: merge] [Tracking Issue for convenience methods on `NonNull`](https://github.com/rust-lang/rust/issues/117691)
* [disposition: merge] [Inline more svg images into CSS](https://github.com/rust-lang/rust/pull/123734)
* [disposition: merge] [Edition 2024: Make `!` fall back to `!`](https://github.com/rust-lang/rust/pull/123508)
* [disposition: merge] [static_mut_refs: Should the lint cover hidden references?](https://github.com/rust-lang/rust/issues/123060)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [fix(toml): Warn, rather than fail publish, if a target is excluded](https://github.com/rust-lang/cargo/pull/13713)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [experimental project goal program for 2024 H2](https://github.com/rust-lang/rfcs/pull/3614)

## Upcoming Events

Rusty Events between 2024-04-24 - 2024-05-22 🦀

### Virtual

* 2024-04-24 | Virtual + In Person (Prague, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**#2: Making Safe Rust Safer (Pavel Šimerda)**](https://www.meetup.com/rust-czech-republic/events/300388563)
* 2024-04-25 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477692/)
* 2024-04-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcgbnc/)
* 2024-05-01 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 5 - Project Structure**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325526/)
* 2024-05-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047895/)
* 2024-05-02 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368804/)
* 2024-05-07 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300100279/)
* 2024-05-09 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477697/)
* 2024-05-09 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Rust at Microsoft, Tel Aviv - Are we embedded yet?**](https://www.meetup.com/code-mavens/events/300144781/)
* 2024-05-09 | Virtual (Nuremberg/Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945257/)
* 2024-05-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341699/)
* 2024-05-14 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/300437775/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 2024-05-16 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298312423/)
* 2024-05-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—forensic parsing via Artemis**](https://www.meetup.com/rustdc/events/299346490/)

### Africa

* 2024-05-04 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)

### Asia

* 2024-05-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2024-rustacean-meetup/)

### Europe

* 2024-04-24 | Virtual + In Person (Prague, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**#2: Making Safe Rust Safer (Pavel Šimerda)**](https://www.meetup.com/rust-czech-republic/events/300388563)
* 2024-04-25 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at MFT Energy**](https://www.meetup.com/rust-aarhus/events/299564517/)
* 2024-04-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - TBD**](https://www.meetup.com/rust-berlin/events/299288960/)
* 2024-04-25 | København/Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust meetup #46 sponsored by Nine A/S**](https://www.meetup.com/copenhagen-rust-community/events/300458178/)
* 2024-04-25 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna x Python User Group - April**](https://www.meetup.com/rust-vienna/events/300389154/)
* 2024-04-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Fullstack Rust - Workshop #2 (Register by 23 April)**](https://www.meetup.com/rust-basel/events/299933581/)
* 2024-04-27 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #2**](https://www.meetup.com/stockholm-rust/events/300369409)
* 2024-04-30 | Budapest, HU | [Budapest Rust Meetup Group](https://www.meetup.com/budapest-rust-meetup-group/)
    * [**Rust Meetup Budapest 2**](https://www.meetup.com/budapest-rust-meetup-group/events/300269044/)
* 2024-04-30 | Salzburg, AT | Rust Salzburg
    * [**Rust Salzburg meetup**]: 6:30pm - CCC Salzburg, 1. OG, ArgeKultur, Ulrike-Gschwandtner-Straße 5, 5020 Salzburg
* 2024-05-01 | Köln/Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**This Month in Rust, May**](https://www.meetup.com/rustcologne/events/300610856/)
* 2024-05-01 | Utrecht, NL | [NL-RSE Community](https://nl-rse.org/events/2024-05-01-meetup)
    * [**NL-RSE RUST meetup**](https://www.eventbrite.nl/e/nl-rse-rust-meetup-tickets-871056271757)
* 2024-05-06 | Delft, NL | [GOSIM](https://www.gosim.org/)
    * [**GOSIM Europe 2024**](https://europe2024.gosim.org/)
* 2024-05-07 & 2024-05-08 | Delft, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2024**](https://2024.rustnl.org/)
* 2024-05-08 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/300573716/)
* 2024-05-09 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #2**](https://www.meetup.com/rust-gdansk/events/299766774/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-14 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (May 2024)**](https://www.meetup.com/rust-prague/events/300566374/)
* 2024-05-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/300307155/)
* 2024-05-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Save the date - Mai Meetup**](https://www.meetup.com/rust-zurich/events/300513957/)

### North America

* 2024-04-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299960315/)
* 2024-04-25 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers - Async Rust on Embedded**](https://www.meetup.com/music-city-rust-developers/events/299976876/)
* 2024-04-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Apr 26**](https://www.meetup.com/bostonrust/events/300116689/)
* 2024-05-04 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, May 4**](https://www.meetup.com/bostonrust/events/300116701/)
* 2024-05-09 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300020003/)
* 2024-05-12 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, May 12**](https://www.meetup.com/bostonrust/events/300116747/)
* 2024-05-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509369/)
* 2024-05-20 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, May 20**](https://www.meetup.com/bostonrust/events/300116765/)
* 2024-05-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186931/)
* 2024-05-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygchbdc/)


### Oceania

* 2024-04-30 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Why Rust? Convince Me!**](https://www.meetup.com/rust-akl/events/300304958/)
* 2024-04-30 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**CRUG April Meetup: Generics and Traits**](https://www.meetup.com/rust-canberra/events/300023000/)

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

> The learning curve for Rust is relatively steep compared to other languages, but once you climb it you'll never look down.

– [BD103 on Mastodon](https://hachyderm.io/@bd103/112318610927827520)

Thanks to [BD103](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1563) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
