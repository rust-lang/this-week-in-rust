Title: This Week in Rust 534
Number: 534
Date: 2024-02-14
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

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [microflow](https://github.com/matteocarnelos/microflow-rs), a robust and efficient TinyML inference engine for embedded systems.

Thanks to [matteocarnelos](https://users.rust-lang.org/t/crate-of-the-week/2704/1289) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

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

* [RustFest Zürich 2024](https://rustfest.ch/cfp/) CFP closes 2024-03-31 | Zürich, Switzerland | Event date: 2024-06-19 - 2024-06-24

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

466 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-02-06..2024-02-13

* [add armv8r-none-eabihf target for the Cortex-R52](https://github.com/rust-lang/rust/pull/110482)
* [add lahfsahf and prfchw target feature](https://github.com/rust-lang/rust/pull/120965)
* [`check_consts`: fix duplicate errors, make importance consistent](https://github.com/rust-lang/rust/pull/120933)
* [`interpret/write_discriminant`: when encoding niched variant, ensure the stored value matches](https://github.com/rust-lang/rust/pull/120882)
* [`large_assignments`: Allow moves into functions](https://github.com/rust-lang/rust/pull/120773)
* [`pattern_analysis`: gather up place-relevant info](https://github.com/rust-lang/rust/pull/120633)
* [`pattern_analysis`: track usefulness without interior mutability](https://github.com/rust-lang/rust/pull/120324)
* [account for non-overlapping unmet trait bounds in suggestion](https://github.com/rust-lang/rust/pull/120507)
* [account for unbounded type param receiver in suggestions](https://github.com/rust-lang/rust/pull/120396)
* [add support for custom JSON targets when using build-std](https://github.com/rust-lang/rust/pull/120232)
* [add unstable `-Z direct-access-external-data` cmdline flag for `rustc`](https://github.com/rust-lang/rust/pull/119162)
* [allow restricted trait impls under `#[allow_internal_unstable(min_specialization)]`](https://github.com/rust-lang/rust/pull/120870)
* [always check the result of `pthread_mutex_lock`](https://github.com/rust-lang/rust/pull/120238)
* [avoid ICE in drop recursion check in case of invalid drop impls](https://github.com/rust-lang/rust/pull/120801)
* [avoid a collection and iteration on empty passes](https://github.com/rust-lang/rust/pull/120853)
* [avoid accessing the HIR in the happy path of `coherent_trait`](https://github.com/rust-lang/rust/pull/120835)
* [bail out of drop elaboration when encountering error types](https://github.com/rust-lang/rust/pull/120802)
* [build DebugInfo for async closures](https://github.com/rust-lang/rust/pull/120844)
* [check that the ABI of the instance we are inlining is correct](https://github.com/rust-lang/rust/pull/120944)
* [clean inlined type alias with correct param-env](https://github.com/rust-lang/rust/pull/120956)
* [continue to borrowck even if there were previous errors](https://github.com/rust-lang/rust/pull/120550)
* [coverage: split out counter increment sites from BCB node/edge counters](https://github.com/rust-lang/rust/pull/120564)
* [create `try_new` function for ThinBox](https://github.com/rust-lang/rust/pull/110483)
* [deduplicate `tcx.instance_mir(instance)` calls in `try_instance_mir`](https://github.com/rust-lang/rust/pull/120778)
* [don't expect early-bound region to be local when reporting errors in RPITIT well-formedness](https://github.com/rust-lang/rust/pull/120707)
* [don't skip coercions for types with errors](https://github.com/rust-lang/rust/pull/120895)
* [emit a diagnostic for invalid target options](https://github.com/rust-lang/rust/pull/120631)
* [emit more specific diagnostics when enums fail to cast with `as`](https://github.com/rust-lang/rust/pull/120822)
* [encode `coroutine_for_closure` for foreign crates](https://github.com/rust-lang/rust/pull/120897)
* [exhaustiveness: prefer "`0..MAX` not covered" to "`_` not covered"](https://github.com/rust-lang/rust/pull/120727)
* [fix ICE for deref coercions with type errors](https://github.com/rust-lang/rust/pull/120972)
* [fix `ErrorGuaranteed` unsoundness with stash/steal](https://github.com/rust-lang/rust/pull/120828)
* [fix cycle error when a static and a promoted are mutually recursive](https://github.com/rust-lang/rust/pull/120960)
* [fix more `ty::Error` ICEs in MIR passes](https://github.com/rust-lang/rust/pull/120817)
* [for E0223, suggest associated functions that are similar to the path](https://github.com/rust-lang/rust/pull/120632)
* [for a rigid projection, recursively look at the self type's item bounds to fix the `associated_type_bounds` feature](https://github.com/rust-lang/rust/pull/120584)
* [gracefully handle non-WF alias in `assemble_alias_bound_candidates_recur`](https://github.com/rust-lang/rust/pull/120899)
* [harmonize `AsyncFn` implementations, make async closures conditionally impl `Fn*` traits](https://github.com/rust-lang/rust/pull/120712)
* [hide impls if trait bound is proven from env](https://github.com/rust-lang/rust/pull/120836)
* [hir: make sure all `HirId`s have corresponding HIR Node`s`](https://github.com/rust-lang/rust/pull/120206)
* [improve 'generic param from outer item' error for `Self` and inside `static`/`const` items](https://github.com/rust-lang/rust/pull/119939)
* [improve normalization of `Pointee::Metadata`](https://github.com/rust-lang/rust/pull/120354)
* [improve pretty printing for associated items in trait objects](https://github.com/rust-lang/rust/pull/120739)
* [introduce `enter_forall` to supercede `instantiate_binder_with_placeholders`](https://github.com/rust-lang/rust/pull/120544)
* [lowering unnamed fields and anonymous adt](https://github.com/rust-lang/rust/pull/115367)
* [make `min_exhaustive_patterns` match `exhaustive_patterns` better](https://github.com/rust-lang/rust/pull/120775)
* [make it so that async-fn-in-trait is compatible with a concrete future in implementation](https://github.com/rust-lang/rust/pull/120103)
* [make privacy visitor use types more (instead of HIR)](https://github.com/rust-lang/rust/pull/113671)
* [make traits / trait methods detected by the dead code lint](https://github.com/rust-lang/rust/pull/118257)
* [mark "unused binding" suggestion as maybe incorrect](https://github.com/rust-lang/rust/pull/120470)
* [match lowering: consistently lower bindings deepest-first](https://github.com/rust-lang/rust/pull/120214)
* [merge `impl_polarity` and `impl_trait_ref` queries](https://github.com/rust-lang/rust/pull/120919)
* [more internal emit diagnostics cleanups](https://github.com/rust-lang/rust/pull/120833)
* [move path implementations into `sys`](https://github.com/rust-lang/rust/pull/120776)
* [normalize type outlives obligations in NLL for new solver](https://github.com/rust-lang/rust/pull/120513)
* [print image input file and checksum in CI only](https://github.com/rust-lang/rust/pull/120827)
* [print kind of coroutine closure](https://github.com/rust-lang/rust/pull/120896)
* [properly handle `async` block and `async fn` in `if` exprs without `else`](https://github.com/rust-lang/rust/pull/120696)
* [provide more suggestions on invalid equality where bounds](https://github.com/rust-lang/rust/pull/120751)
* [record coroutine kind in coroutine generics](https://github.com/rust-lang/rust/pull/120746)
* [remove some `unchecked_claim_error_was_emitted` calls](https://github.com/rust-lang/rust/pull/120735)
* [resolve: unload speculatively resolved crates before freezing cstore](https://github.com/rust-lang/rust/pull/119592)
* [rework support for async closures; allow them to return futures that borrow from the closure's captures](https://github.com/rust-lang/rust/pull/120361)
* [static mut: allow mutable reference to arbitrary types, not just slices and arrays](https://github.com/rust-lang/rust/pull/117614)
* [stop bailing out from compilation just because there were incoherent traits](https://github.com/rust-lang/rust/pull/120558)
* [suggest `[tail @ ..]` on `[..tail]` and `[...tail]` where `tail` is unresolved](https://github.com/rust-lang/rust/pull/120597)
* [suggest less bug-prone construction of Duration in docs](https://github.com/rust-lang/rust/pull/119242)
* [suggest name value cfg when only value is used for check-cfg](https://github.com/rust-lang/rust/pull/120435)
* [suggest pattern tests when modifying exhaustiveness](https://github.com/rust-lang/rust/pull/120763)
* [suggest turning `if let` into irrefutable `let` if appropriate](https://github.com/rust-lang/rust/pull/120479)
* [suppress suggestions in derive macro](https://github.com/rust-lang/rust/pull/120272)
* [take empty `where` bounds into account when suggesting predicates](https://github.com/rust-lang/rust/pull/120874)
* [toggle `assert_unsafe_precondition` in codegen instead of expansion](https://github.com/rust-lang/rust/pull/120594)
* [turn the "no saved object file in work product" ICE into a translatable fatal error](https://github.com/rust-lang/rust/pull/120865)
* [warn on references casting to bigger memory layout](https://github.com/rust-lang/rust/pull/118983)
* [unstably allow constants to refer to statics and read from immutable statics](https://github.com/rust-lang/rust/pull/119614)
* [use the same mir-opt bless targets on all platforms](https://github.com/rust-lang/rust/pull/120060)
* [enable MIR JumpThreading by default](https://github.com/rust-lang/rust/pull/117206)
* [fix mir pass ICE in the presence of other errors](https://github.com/rust-lang/rust/pull/120782)
* [miri: fix ICE with symbolic alignment check on extern static](https://github.com/rust-lang/rust/pull/120683)
* [miri: implement the `mmap64` foreign item](https://github.com/rust-lang/miri/pull/3285)
* [prevent running some code if it is already in the map](https://github.com/rust-lang/rust/pull/120579)
* [A trait's local impls are trivially coherent if there are no impls](https://github.com/rust-lang/rust/pull/120834)
* [use `ensure` when the result of the query is not needed beyond its `Result`ness](https://github.com/rust-lang/rust/pull/120771)
* [implement SystemTime for UEFI](https://github.com/rust-lang/rust/pull/120351)
* [implement sys/thread for UEFI](https://github.com/rust-lang/rust/pull/120938)
* [core/time: avoid divisions in `Duration::new`](https://github.com/rust-lang/rust/pull/120308)
* [core: add Duration constructors](https://github.com/rust-lang/rust/pull/120307)
* [make `NonZero` constructors generic](https://github.com/rust-lang/rust/pull/120521)
* [reconstify `Add`](https://github.com/rust-lang/rust/pull/120381)
* [replace pthread `RwLock` with custom implementation](https://github.com/rust-lang/rust/pull/110211)
* [simd intrinsics: add `simd_shuffle_generic` and other missing intrinsics](https://github.com/rust-lang/rust/pull/119213)
* [cargo: test-support: remove special case for `$message_type`](https://github.com/rust-lang/cargo/pull/13424)
* [cargo: don't add the new package to workspace.members if there is no existing workspace in Cargo.toml](https://github.com/rust-lang/cargo/pull/13391)
* [cargo: enable edition migration for 2024](https://github.com/rust-lang/cargo/pull/13429)
* [cargo: feat: add hint for adding members to workspace](https://github.com/rust-lang/cargo/pull/13411)
* [cargo: fix confusing error messages for sparse index replaced source](https://github.com/rust-lang/cargo/pull/13433)
* [cargo: fix: don't duplicate comments when editing TOML](https://github.com/rust-lang/cargo/pull/13402)
* [cargo: relax a test to permit warnings to be emitted, too](https://github.com/rust-lang/cargo/pull/13415)
* [rustdoc: Correctly generate path for non-local items in source code pages](https://github.com/rust-lang/rust/pull/120596)
* [bindgen: add target mappings for `riscv64imac` and `riscv32imafc`](https://github.com/rust-lang/rust-bindgen/pull/2751)
* [bindgen: feat: add `headers` option](https://github.com/rust-lang/rust-bindgen/pull/2743)
* [clippy: `mem_replace_with_default` No longer triggers on unused expression](https://github.com/rust-lang/rust-clippy/pull/12278)
* [clippy: `similar_names`: don't raise if the first character is different](https://github.com/rust-lang/rust-clippy/pull/12258)
* [clippy: `to_string_trait_impl`: avoid linting if the impl is a specialization](https://github.com/rust-lang/rust-clippy/pull/12264)
* [clippy: `unconditional_recursion`: compare by `Ty`s instead of DefId`s`](https://github.com/rust-lang/rust-clippy/pull/12177)
* [clippy: don't allow derive macros to silence `disallowed_macros`](https://github.com/rust-lang/rust-clippy/pull/12267)
* [clippy: don't lint `incompatible_msrv` in test code](https://github.com/rust-lang/rust-clippy/pull/12261)
* [clippy: extend `NONMINIMAL_BOOL` lint](https://github.com/rust-lang/rust-clippy/pull/12248)
* [clippy: fix broken URL in `Lint Configuration`](https://github.com/rust-lang/rust-clippy/pull/12272)
* [clippy: fix false positive in `redundant_type_annotations` lint](https://github.com/rust-lang/rust-clippy/pull/12216)
* [clippy: add autofixes for `unnecessary_fallible_conversions`](https://github.com/rust-lang/rust-clippy/pull/12070)
* [clippy: fix: ICE when array index exceeds usize](https://github.com/rust-lang/rust-clippy/pull/12266)
* [clippy: refactor `implied_bounds_in_impls` lint](https://github.com/rust-lang/rust-clippy/pull/12269)
* [clippy: return `Some` from `walk_to_expr_usage` more](https://github.com/rust-lang/rust-clippy/pull/11812)
* [clippy: stop linting `blocks_in_conditions` on `match` with weird attr macro case](https://github.com/rust-lang/rust-clippy/pull/12040)
* [rust-analyzer: abstract more over ItemTreeLoc-like structs](https://github.com/rust-lang/rust-analyzer/pull/16525)
* [rust-analyzer: better error message for when proc-macros have not yet been built](https://github.com/rust-lang/rust-analyzer/pull/16462)
* [rust-analyzer: add "unnecessary else" diagnostic and fix](https://github.com/rust-lang/rust-analyzer/pull/16502)
* [rust-analyzer: add break and return postfix keyword completions](https://github.com/rust-lang/rust-analyzer/pull/16454)
* [rust-analyzer: add diagnostic with fix to replace trailing `return <val>;` with `<val>`](https://github.com/rust-lang/rust-analyzer/pull/16460)
* [rust-analyzer: add incorrect case diagnostics for traits and their associated items](https://github.com/rust-lang/rust-analyzer/pull/16477)
* [rust-analyzer: allow cargo check to run on only the current package](https://github.com/rust-lang/rust-analyzer/pull/16510)
* [rust-analyzer: completion list suggests constructor like & builder methods first](https://github.com/rust-lang/rust-analyzer/pull/16117)
* [rust-analyzer: improve support for ignored proc macros](https://github.com/rust-lang/rust-analyzer/pull/15923)
* [rust-analyzer: introduce term search to rust-analyzer](https://github.com/rust-lang/rust-analyzer/pull/16092)
* [rust-analyzer: create `UnindexedProject` notification to be sent to the client](https://github.com/rust-lang/rust-analyzer/pull/15863)
* [rust-analyzer: substitute `$saved_file` in custom check commands](https://github.com/rust-lang/rust-analyzer/pull/15476)
* [rust-analyzer: fix incorrect inlining of functions that come from MBE macros](https://github.com/rust-lang/rust-analyzer/pull/16497)
* [rust-analyzer: `waker_getters` tracking issue from 87021 for 96992](https://github.com/rust-lang/rust-analyzer/pull/16134)
* [rust-analyzer: fix macro transcriber emitting incorrect lifetime tokens](https://github.com/rust-lang/rust-analyzer/pull/16530)
* [rust-analyzer: fix target layout fetching](https://github.com/rust-lang/rust-analyzer/pull/16545)
* [rust-analyzer: fix tuple structs not rendering visibility in their fields](https://github.com/rust-lang/rust-analyzer/pull/16509)
* [rust-analyzer: highlight rustdoc](https://github.com/rust-lang/rust-analyzer/pull/16541)
* [rust-analyzer: preserve where clause when builtin derive](https://github.com/rust-lang/rust-analyzer/pull/16484)
* [rust-analyzer: recover from missing argument in call expressions](https://github.com/rust-lang/rust-analyzer/pull/16124)
* [rust-analyzer: remove unnecessary `.as_ref()` in generate getter assist](https://github.com/rust-lang/rust-analyzer/pull/16487)
* [rust-analyzer: validate literals in proc-macro-srv `FreeFunctions::literal_from_str`](https://github.com/rust-lang/rust-analyzer/pull/16547)
* [rust-analyzer: implement `literal_from_str` for proc macro server](https://github.com/rust-lang/rust-analyzer/pull/16446)
* [rust-analyzer: implement convert to guarded return assist for `let` statement with type that implements `std::ops::Try`](https://github.com/rust-lang/rust-analyzer/pull/16424)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [eRFC: Iterate on and stabilize libtest's programmatic output](https://github.com/rust-lang/rfcs/pull/3558)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Rust Has Provenance](https://github.com/rust-lang/rfcs/pull/3559)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: close] [Implement `Future` for `Option<F>`](https://github.com/rust-lang/rust/pull/109691)
* [disposition: merge] [Tracking Issue for min_exhaustive_patterns](https://github.com/rust-lang/rust/issues/119612)
* [disposition: merge] [Make unsafe_op_in_unsafe_fn warn-by-default starting in 2024 edition](https://github.com/rust-lang/rust/issues/120535)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [feat: respect rust-version when generating lockfile](https://github.com/rust-lang/cargo/pull/12861)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

#### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [RFC: Checking conditional compilation at compile time](https://github.com/rust-lang/rfcs/pull/3013)
  * [Testing steps](https://github.com/rust-lang/rfcs/pull/3013#issuecomment-1936648479)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### Rusty Events between 2024-02-14 - 2024-03-13 💕 🦀 💕

### Virtual

* 2024-02-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457899/)
* 2024-02-15 | Virtual + In person (Praha, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-19 | Virtual (Melbourne, VIC, AU)| [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) February 2024 Rust Melbourne Meetup - Day 1**](https://www.meetup.com/rust-melbourne/events/298877455/)
* 2024-02-20 | Virtual (Melbourne, VIC, AU) | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) February 2024 Rust Melbourne Meetup - Day 2**](https://www.meetup.com/rust-melbourne/events/298877469/)
* 2024-02-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128189/)
* 2024-02-20 | Virtual | [Rust for Lunch](https://lunch.rs/about/)
    * [**Lunch**](https://lunch.rs/meetups/2024-02-20/)
* 2024-02-21 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 2 - Types**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298991687/)
* 2024-02-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 2024-02-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)
* 2024-02-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/299068302/)
* 2024-02-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457901/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/02/29/rust-hack-and-learn.html)
* 2024-02-29 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Surfing the Rusty Wireless Waves with the ESP32-C3 Board**](https://www.meetup.com/charlottesville-rust-meetup/events/298372724/)
* 2024-03-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047891/)
* 2024-03-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368787/)
* 2024-03-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341582/)
* 2024-03-12 | Hybrid (Virtual + In-person) Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)

### Asia

* 2024-02-17 | New Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Meetup #5**](https://www.meetup.com/rustdelhi/events/298864671/)

### Europe

* 2024-02-15 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust Hacknight #2: Compilers**](https://www.meetup.com/copenhagen-rust-community/events/298999792/)
* 2024-02-15 | Praha, CZ - Virtual + In-person | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 2024-02-22 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)
* 2024-02-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Season start 2024**](https://www.meetup.com/rust-berlin/events/299190389/)
* 2024-03-12 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)

### North America

* 2024-02-15 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Back Bay Rust Lunch, Feb 15**](https://www.meetup.com/bostonrust/events/297635043/)
* 2024-02-15 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631774/)
* 2024-02-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer (Moved to Feb 20th)**](https://www.meetup.com/rust-nyc/events/298593474/)
* 2024-02-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/298603354/)
* 2024-02-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Evening Boston Rust Meetup at Microsoft, February 21**](https://www.meetup.com/bostonrust/events/299054786/)
* 2024-02-22 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043763/)
* 2024-02-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)
* 2024-03-07 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043793/)

### Oceania

* 2024-02-19 | Melbourne, VIC, AU + Virtual | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) February 2024 Rust Melbourne Meetup - Day 1**](https://www.meetup.com/rust-melbourne/events/298877455/)
* 2024-02-20 | Melbourne, VIC, AU + Virtual | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) February 2024 Rust Melbourne Meetup - Day 2**](https://www.meetup.com/rust-melbourne/events/298877469/)
* 2024-02-27 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/297650401/)
* 2024-02-27 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)
* 2024-03-05 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Introduction to Embedded Rust + The State of Rust UI**](https://www.meetup.com/rust-akl/events/299158887/)

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

> For some weird reason the Elixir Discord community has a distinct lack of programmer-socks-wearing queer furries, at least compared to Rust, or even most other tech-y Discord servers I’ve seen. It caused some weird cognitive dissonance. Why do I feel vaguely strange hanging out online with all these kind, knowledgeable, friendly and compassionate techbro’s? Then I see a name I recognized from elsewhere and my hindbrain goes “oh thank gods, I know for a fact she’s actually a snow leopard in her free time”. Okay, this nitpick is firmly tongue-in-cheek, but the Rust user-base continues to be a fascinating case study in how many weirdos you can get together in one place when you very explicitly say it’s ok to be a weirdo.

– [SimonHeath on the alopex Wiki's ElixirNitpicks page](https://wiki.alopex.li/ElixirNitpicks#Misc)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1522) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
