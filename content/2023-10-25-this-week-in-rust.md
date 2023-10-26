Title: This Week in Rust 518
Number: 518
Date: 2023-10-25
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

## Project/Tooling Updates

* [Strobe Crate](https://jlogan03.github.io/jekyll/update/2023/10/21/strobe.html)
* [System dependencies are hard (so we made them easier)](https://blog.axo.dev/2023/10/dependencies)

### Observations/Thoughts

* [Trying to invent a better substring search algorithm](https://marcos.unsafe.rs/trying-to-invent-a-better-substring-search-algorithm/)
* [Improving Node.js with Rust-Wasm Library](https://elvisbrevi.hashnode.dev/improving-nodejs-with-rust-wasm-library)
* [Mixing C# and Rust - Interop](https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_0_3_2.html)
* [A fresh look on incremental zero copy serialization](https://udoprog.github.io/rust/2023-10-19/musli-zerocopy.html)
* [Make the Rust compiler 5% faster with this one weird trick](https://kobzol.github.io/rust/rustc/2023/10/21/make-rust-compiler-5percent-faster.html)
* [Part 3: Rowing Afloat Datatype Boats](https://thunderseethe.dev/posts/row-types/)
* [Recreating concurrent futures combinators in smol](https://notgull.net/futures-concurrency-in-smol/)
* [Unpacking some Rust ergonomics: getting a single Result from an iterator of them](https://ntietz.com/blog/rust-vec-of-result/)
* [Idea: "Using Rust", a living document](https://smallcultfollowing.com/babysteps/blog/2023/10/20/using-rust/)
* [Object Soup is Made of Indexes](https://jacko.io/object_soup.html)
* [Analyzing Data 180,000x Faster with Rust](https://willcrichton.net/notes/k-corrset/)
* [Issue #10: Serving HTML](https://www.shuttle.rs/launchpad/issues/2023-10-17-issue-10-Serving-HTML)
* [Rust vs C on an ATTiny85; an embedded war story](https://diziet.dreamwidth.org/16771.html)
* [Make the Rust compiler 5% faster with this one weird trick](https://kobzol.github.io/rust/rustc/2023/10/21/make-rust-compiler-5percent-faster.html)

### Rust Walkthroughs

* [Analyzing Data /,000x Faster with Rust](https://willcrichton.net/notes/k-corrset/)
* [Fully Automated Releases for Rust Projects](https://blog.orhun.dev/automated-rust-releases/)
* [Make your Rust code unit testable with dependency inversion](https://worldwithouteng.com/articles/make-your-rust-code-unit-testable-with-dependency-inversion/)
* [Nine Rules to Formally Validate Rust Algorithms with Dafny (Part 2): Lessons from Verifying the range-set-blaze Crate](https://medium.com/towards-data-science/nine-rules-to-formally-validate-rust-algorithms-with-dafny-part-2-f2a279686700)
* [video] [Let's write a message broker using QUIC - Broke But Quick Episode 1](https://www.youtube.com/watch?v=lpsduJy2EIM)
* [video] [Publishing Messages over QUIC Streams!! - Broke But Quick episode 2](https://www.youtube.com/watch?v=auXpVgUMZjU)

### Miscellaneous

* [video] [Associated types in Iterator bounds](https://youtu.be/Sa5bYF5a-Ng)
* [video] [Rust and the Age of High-Integrity Languages](https://www.youtube.com/watch?v=pJoATjuc50w)
* [video] [Implementing (part of) a BitTorrent client in Rust](https://www.youtube.com/watch?v=jf_ddGnum_4)

## Crate of the Week

This week's crate is [cargo-show-asm](https://lib.rs/crates/cargo-show-asm), a cargo subcommand to show the optimized assembly of any function.

Thanks to [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1250) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch (Hacktoberfest)- [FEATURE] separate payments_session from payments core](https://github.com/juspay/hyperswitch/issues/888)
* [Hyperswitch (Hacktoberfest)- [NMI] Use connector_response_reference_id as reference to merchant](https://github.com/juspay/hyperswitch/issues/2338)
* [Hyperswitch (Hacktoberfest)- [Airwallex] Use connector_response_reference_id as reference to merchant](https://github.com/juspay/hyperswitch/issues/2322)
* [Hyperswitch (Hacktoberfest)- [Worldline] Use connector_response_reference_id as reference to merchant](https://github.com/juspay/hyperswitch/issues/2351)
* [Ockam - Make `ockam project delete` (no args) interactive by asking the user to choose from a list of space and project names to delete (tuify)](https://github.com/build-trust/ockam/issues/6461)
* [Ockam - Validate CBOR structs according to the cddl schema for `authenticator/direct/types`](https://github.com/build-trust/ockam/issues/6682)
* [Ockam - Slim down the `NodeManagerWorker` for `node / node status`](https://github.com/build-trust/ockam/issues/6707)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

397 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-10-16..2023-10-23

* [rewrite gdb pretty-printer registration](https://github.com/rust-lang/rust/pull/116978)
* [add FileCheck annotations to mir-opt tests](https://github.com/rust-lang/rust/pull/116810)
* [add MonoItems and Instance to `stable_mir`](https://github.com/rust-lang/rust/pull/116719)
* [add a `csky-unknown-linux-gnuabiv2hf` target](https://github.com/rust-lang/rust/pull/117049)
* [add a test showing failing closure signature inference in new solver](https://github.com/rust-lang/rust/pull/116899)
* [add new simpler and more explicit syntax for check-cfg](https://github.com/rust-lang/rust/pull/111072)
* [add stable `Instance::body()` and RustcInternal trait](https://github.com/rust-lang/rust/pull/116964)
* [automatically enable cross-crate inlining for small functions](https://github.com/rust-lang/rust/pull/116505)
* [avoid a `track_errors` by bubbling up most errors from `check_well_formed`](https://github.com/rust-lang/rust/pull/116849)
* [avoid having `rustc_smir` depend on `rustc_interface` or `rustc_driver`](https://github.com/rust-lang/rust/pull/116837)
* [coverage: emit mappings for unused functions without generating stubs](https://github.com/rust-lang/rust/pull/116922)
* [coverage: emit the filenames section before encoding per-function mappings](https://github.com/rust-lang/rust/pull/117042)
* [coverage: fix inconsistent handling of function signature spans](https://github.com/rust-lang/rust/pull/116974)
* [coverage: move most per-function coverage info into `mir::Body`](https://github.com/rust-lang/rust/pull/116046)
* [coverage: simplify the injection of coverage statements](https://github.com/rust-lang/rust/pull/116917)
* [disable `missing_copy_implementations` lint on `non_exhaustive` types](https://github.com/rust-lang/rust/pull/116812)
* [do not bold main message in `--error-format=short`](https://github.com/rust-lang/rust/pull/116962)
* [don't ICE when encountering unresolved regions in `fully_resolve`](https://github.com/rust-lang/rust/pull/116663)
* [don't compare host param by name](https://github.com/rust-lang/rust/pull/116870)
* [don't crash on empty match in the `nonexhaustive_omitted_patterns` lint](https://github.com/rust-lang/rust/pull/117034)
* [duplicate `~const` bounds with a non-const one in effects desugaring](https://github.com/rust-lang/rust/pull/116756)
* [eliminate `rustc_attrs::builtin::handle_errors` in favor of emitting errors directly](https://github.com/rust-lang/rust/pull/117064)
* [fix a performance regression in obligation deduplication](https://github.com/rust-lang/rust/pull/116826)
* [fix implied outlives check for GAT in RPITIT](https://github.com/rust-lang/rust/pull/116800)
* [fix spans for removing `.await` on `for` expressions](https://github.com/rust-lang/rust/pull/117019)
* [fix suggestion for renamed coroutines feature](https://github.com/rust-lang/rust/pull/117073)
* [implement an internal lint encouraging use of `Span::eq_ctxt`](https://github.com/rust-lang/rust/pull/116787)
* [implement jump threading MIR opt](https://github.com/rust-lang/rust/pull/107009)
* [implement rustc part of RFC 3127 trim-paths](https://github.com/rust-lang/rust/pull/115214)
* [improve display of parallel jobs in rustdoc-gui tester script](https://github.com/rust-lang/rust/pull/116798)
* [initiate the inner usage of `cfg_match` (Compiler)](https://github.com/rust-lang/rust/pull/116312)
* [lint `non_exhaustive_omitted_patterns` by columns](https://github.com/rust-lang/rust/pull/116734)
* [location-insensitive polonius: consider a loan escaping if an SCC has member constraints applied only](https://github.com/rust-lang/rust/pull/116960)
* [make `#[repr(Rust)]` incompatible with other (non-modifier) representation hints like `C` and `simd`](https://github.com/rust-lang/rust/pull/116829)
* [make `rustc_onunimplemented` export path agnostic](https://github.com/rust-lang/rust/pull/116805)
* [mention `into_iter` on borrow errors suggestions when appropriate](https://github.com/rust-lang/rust/pull/116990)
* [mention the syntax for `use` on `mod foo;` if `foo` doesn't exist](https://github.com/rust-lang/rust/pull/116992)
* [panic when the global allocator tries to register a TLS destructor](https://github.com/rust-lang/rust/pull/116402)
* [point at assoc fn definition on type param divergence](https://github.com/rust-lang/rust/pull/116995)
* [preserve unicode escapes in format string literals when pretty-printing AST](https://github.com/rust-lang/rust/pull/116811)
* [properly account for self ty in method disambiguation suggestion](https://github.com/rust-lang/rust/pull/116713)
* [report `unused_import` for empty reexports even it is pub](https://github.com/rust-lang/rust/pull/116033)
* [special case iterator chain checks for suggestion](https://github.com/rust-lang/rust/pull/116717)
* [strict provenance unwind](https://github.com/rust-lang/rust/pull/114534)
* [suggest `;` after bare `match` expression E0308](https://github.com/rust-lang/rust/pull/106601)
* [suggest constraining assoc types in more cases](https://github.com/rust-lang/rust/pull/116865)
* [suggest relaxing implicit `type Assoc: Sized;` bound](https://github.com/rust-lang/rust/pull/116911)
* [suggest removing redundant arguments in `format!()`](https://github.com/rust-lang/rust/pull/115324)
* [uplift movability and mutability, the simple way](https://github.com/rust-lang/rust/pull/116946)
* [miri: avoid a linear scan over the entire `int_to_ptr_map` on each deallocation](https://github.com/rust-lang/miri/pull/3134)
* [miri: fix rounding mode check in SSE4.1 round functions](https://github.com/rust-lang/miri/pull/3124)
* [miri: intptrcast: remove information about dead allocations](https://github.com/rust-lang/miri/pull/3122)
* [disable effects in libcore again](https://github.com/rust-lang/rust/pull/116856)
* [add `#[track_caller]` to `Option::unwrap_or_else`](https://github.com/rust-lang/rust/pull/116795)
* [specialize `Bytes<R>::next` when `R` is a `BufReader`](https://github.com/rust-lang/rust/pull/116785)
* [make TCP connect handle EINTR correctly](https://github.com/rust-lang/rust/pull/116132)
* [on Windows make `read_dir` error on the empty path](https://github.com/rust-lang/rust/pull/116606)
* [hashbrown: add low-level `HashTable` API](https://github.com/rust-lang/hashbrown/pull/466)
* [codegen\_gcc: add support for NonNull function attribute](https://github.com/rust-lang/rustc_codegen_gcc/pull/326)
* [codegen\_gcc: fix `#[inline(always)]` attribute and support unsigned comparison for signed integers](https://github.com/rust-lang/rustc_codegen_gcc/pull/352)
* [codegen\_gcc: fix endianness](https://github.com/rust-lang/rustc_codegen_gcc/pull/346)
* [codegen\_gcc: fix int types alignment](https://github.com/rust-lang/rustc_codegen_gcc/pull/353)
* [codegen\_gcc: optimize popcount implementation](https://github.com/rust-lang/rustc_codegen_gcc/pull/348)
* [codegen\_gcc: optimize u128/i128 popcounts further](https://github.com/rust-lang/rustc_codegen_gcc/pull/354)
* [cargo add: Preserve more comments](https://github.com/rust-lang/cargo/pull/12838)
* [cargo remove: Preserve feature comments](https://github.com/rust-lang/cargo/pull/12837)
* [cargo replace: Partial-version spec support](https://github.com/rust-lang/cargo/pull/12806)
* [cargo: Provide next steps for bad -Z flag](https://github.com/rust-lang/cargo/pull/12857)
* [cargo: Suggest cargo-search on bad commands](https://github.com/rust-lang/cargo/pull/12840)
* [cargo: adjust `-Zcheck-cfg` for new rustc syntax and behavior](https://github.com/rust-lang/cargo/pull/12845)
* [cargo: if there's a version in the lock file only use that exact version](https://github.com/rust-lang/cargo/pull/12772)
* [cargo: make the precise field of a source an Enum](https://github.com/rust-lang/cargo/pull/12849)
* [cargo: print environment variables for build script executions with `-vv`](https://github.com/rust-lang/cargo/pull/12829)
* [cargo: warn about crate name's format when creating new crate](https://github.com/rust-lang/cargo/pull/12766)
* [rustdoc: align stability badge to baseline instead of bottom](https://github.com/rust-lang/rust/pull/105666)
* [rustdoc: avoid allocating strings primitive link printing](https://github.com/rust-lang/rust/pull/117007)
* [clippy: `map_identity`: allow closure with type annotations](https://github.com/rust-lang/rust-clippy/pull/11521)
* [clippy: `map_identity`: recognize tuple identity function](https://github.com/rust-lang/rust-clippy/pull/10943)
* [clippy: add lint for `struct` field names](https://github.com/rust-lang/rust-clippy/pull/11496)
* [clippy: don't emit `needless_pass_by_ref_mut` if the variable is used in an unsafe block or function](https://github.com/rust-lang/rust-clippy/pull/11624)
* [clippy: make `multiple_unsafe_ops_per_block` ignore await desugaring](https://github.com/rust-lang/rust-clippy/pull/11646)
* [clippy: needless pass by ref mut closure non async fn](https://github.com/rust-lang/rust-clippy/pull/11621)
* [clippy: now `declare_interior_mutable_const` and `borrow_interior_mutable_const` respect the `ignore-interior-mutability` configuration entry](https://github.com/rust-lang/rust-clippy/pull/11678)
* [clippy: skip `if_not_else` lint for '!= 0'-style checks](https://github.com/rust-lang/rust-clippy/pull/11028)
* [clippy: suggest passing function instead of calling it in closure for `option_if_let_else`](https://github.com/rust-lang/rust-clippy/pull/11460)
* [clippy: warn `missing_enforced_import_renames` by default](https://github.com/rust-lang/rust-clippy/pull/11539)
* [rust-analyzer: generate descriptors for all unstable features](https://github.com/rust-lang/rust-analyzer/pull/15727)
* [rust-analyzer: add command for only opening external docs and attempt to fix vscode-remote issue](https://github.com/rust-lang/rust-analyzer/pull/15779)
* [rust-analyzer: add incorrect case diagnostics for module names](https://github.com/rust-lang/rust-analyzer/pull/15736)
* [rust-analyzer: fix VS Code detection for Insiders version](https://github.com/rust-lang/rust-analyzer/pull/15786)
* [rust-analyzer: import trait if needed for `unqualify_method_call` assist](https://github.com/rust-lang/rust-analyzer/pull/15780)
* [rust-analyzer: pick a better name for variables introduced by `replace_is_some_with_if_let_some`](https://github.com/rust-lang/rust-analyzer/pull/15775)
* [rust-analyzer: store binding mode for each instance of a binding independently](https://github.com/rust-lang/rust-analyzer/pull/15789)
* [perf: add NES emulation runtime benchmark](https://github.com/rust-lang/rustc-perf/pull/1730)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Add f16 and f128 float types](https://github.com/rust-lang/rfcs/pull/3453)
* [Unicode and escape codes in literals](https://github.com/rust-lang/rfcs/pull/3349)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Consider alias bounds when computing liveness in NLL (but this time sound hopefully)](https://github.com/rust-lang/rust/pull/116733)
* [disposition: close] [regression: parameter type may not live long enough](https://github.com/rust-lang/rust/issues/117055)
* [disposition: merge] [Remove support for compiler plugins.](https://github.com/rust-lang/rust/pull/116412)
* [disposition: merge] [rustdoc: Document lack of object safety on affected traits](https://github.com/rust-lang/rust/pull/113241)
* [disposition: merge] [Stabilize Ratified RISC-V Target Features](https://github.com/rust-lang/rust/pull/116485)
* [disposition: merge] [Tracking Issue for const mem::discriminant](https://github.com/rust-lang/rust/issues/69821)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [eRFC: #[should_move] attribute for per-function opting out of Copy semantics](https://github.com/rust-lang/rfcs/pull/3518)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-10-25 - 2023-11-22 🦀

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/163w6fl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> When your Rust build times get slower after adding some procedural macros:
>
> We call that the syn tax :ferris:

– [Janet on Fosstodon](https://fosstodon.org/@janet/111223564960983226)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1472) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/17gndm2/this_week_in_rust_518/)</small>
