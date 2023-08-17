Title: This Week in Rust 508
Number: 508
Date: 2023-08-16
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
* [Announcing the Rust Foundation’s 2023 Fellows](https://foundation.rust-lang.org/news/announcing-the-rust-foundation-s-2023-fellows/)

### Newsletters
* [This Week in Ars Militaris VI](https://arsmilitaris.com/#this-week-in-ars-militaris-vi)

### Project/Tooling Updates
* [rust-analyzer changelog #194](https://rust-analyzer.github.io/thisweek/2023/08/14/changelog-194.html)
* [cargo-binstall v1.2.0](https://github.com/cargo-bins/cargo-binstall/releases/tag/v1.2.0)
* [Announcing SeaORM 0.12](https://www.sea-ql.org/blog/2023-08-12-announcing-seaorm-0.12/)

### Observations/Thoughts
* [I built a garbage collector for a language that doesn’t need one](https://claytonwramsey.github.io/2023/08/14/dumpster.html)
* [The Case for Rust on the Web](https://mainmatter.com/blog/2023/08/14/the-case-for-rust-on-the-web/)
* [Learning Async Rust With Entirely Too Many Web Servers](https://ibraheem.ca/posts/too-many-web-servers/)
* [fn main( ) - Rust Type System: P2](https://sanjuvi.github.io/Blog/posts/Rust-Type-System-Part-2/)
* [Our latest adventures with bindgen](https://ferrous-systems.com/blog/bindgen/)
* [Autogenerating Rust-JS bindings with UniFFI](https://hacks.mozilla.org/2023/08/autogenerating-rust-js-bindings-with-uniffi/)
* [Corrode Rust Consulting](https://corrode.dev/blog/illegal-state/)
* [Why you should consider Rust for your Lambdas](https://loige.co/why-you-should-consider-rust-for-your-lambdas)
* [Explaining Rust’s Deref trait](https://timclicks.dev/explaining-rusts-deref-trait/)

### Miscellaneous
* [DE] [Programmiersprache Rust gewinnt im Arbeitsumfeld an Bedeutung](https://www.heise.de/news/Programmiersprache-Rust-gewinnt-im-Arbeitsumfeld-an-Bedeutung-9239296.html)
* [audio] [Adopting Rust: present and future of the Rust web ecosystem, with Luca Palmieri](https://rustacean-station.org/episode/luca-palmieri-web-ecosystem/)
* [video] [Physics in Bevy: How to get Rapier in your games](https://www.youtube.com/watch?v=hrsoyBqE--A)
* [video] [Open the Rust compiler's puzzle book - weird-exprs.rs](https://www.youtube.com/watch?v=RydzFK11-bE)
* [video] [Causal inference in Rust - deep_causality | Crate of the Week 507](https://www.youtube.com/watch?v=vNz-iKRhneA)
* [video] [Dioxus 0.4: Server Functions](https://www.youtube.com/watch?v=BbQzRdxekao)
* [video] [history.txt vs sqlite with Atuin](https://www.youtube.com/watch?v=WB7qojkkVVU)
* [video] [The Database of Four Dimensional Reality - SpacetimeDB](https://www.youtube.com/watch?v=v4EF3d2_QTg)
* [video] [noisy material shaders in bevy](https://www.youtube.com/watch?v=OoluUyC3KEM)
* [video] [I spent six months rewriting everything in Rust](https://www.youtube.com/watch?v=vL2nB1VwX1M)
* [video] [Game Dev Log 4 - Schedules](https://www.youtube.com/watch?v=iyzW8w-2s00) ([Entire Series](https://youtube.com/playlist?list=PL4iN-WluNkURv_EbC3b1vFM3hH-pGEc2U))
* [audio] [Episode 092 - Moving to Rust in the Age of AI with Noah Gift](https://podcasts.google.com/feed/aHR0cHM6Ly9mZWVkcy5zb3VuZGNsb3VkLmNvbS91c2Vycy9zb3VuZGNsb3VkOnVzZXJzOjk5NDM2MzU0OS9zb3VuZHMucnNz/episode/dGFnOnNvdW5kY2xvdWQsMjAxMDp0cmFja3MvMTU5MDkyMDQ4Mw)

## Crate of the Week

This week's crate is [agree](https://crates.io/crates/agree), a command-line tool implementing Shamir's Secret Sharing.

Thanks to [Alexander Weber](https://users.rust-lang.org/t/crate-of-the-week/2704/1225) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch - add domain type for client secret](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - deserialization error exposes sensitive values in the logs](https://github.com/juspay/hyperswitch/issues/1943)
* [Hyperswitch - move redis key creation to a common module](https://github.com/juspay/hyperswitch/issues/917)
* [Ockam - Check key/secret size before casting as a fixed-length array](https://github.com/build-trust/ockam/issues/5631)
* [Ockam - Ockam CLI should gracefully handle invalid state when initializing](https://github.com/build-trust/ockam/issues/5633)
* [Ockam - Use TCP Outlets' "worker address" as identifiers in the Ockam App's tray menu items](https://github.com/build-trust/ockam/issues/5600)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

344 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-08-07..2023-08-14

* [add csky-unknown-linux-gnuabiv2 target](https://github.com/rust-lang/rust/pull/113658)
* [add aarch64-unknown-teeos target](https://github.com/rust-lang/rust/pull/113480)
* [`riscv-interrupt-{m,s}` calling conventions](https://github.com/rust-lang/rust/pull/111891)
* [set `max_atomic_width` for AVR to 16](https://github.com/rust-lang/rust/pull/114495)
* [set `max_atomic_width` for sparc-unknown-linux-gnu to 32](https://github.com/rust-lang/rust/pull/114496)
* [CFI: fix error compiling core with LLVM CFI enabled](https://github.com/rust-lang/rust/pull/113593)
* [std: Replace condv while loop with `cvar.wait_while`](https://github.com/rust-lang/rust/pull/114359)
* [`Expr::can_have_side_effects()` is incorrect for struct/enum/array/tuple literals](https://github.com/rust-lang/rust/pull/114678)
* [`rustc_data_structures`: `base_n` perf: remove unnecessary utf8 check](https://github.com/rust-lang/rust/pull/114339)
* [`offset_of`: guard against invalid use (with unsized fields)](https://github.com/rust-lang/rust/pull/114614)
* [add hotness data to LLVM remarks](https://github.com/rust-lang/rust/pull/114439)
* [add suggestion to quote inlined format argument as string literal](https://github.com/rust-lang/rust/pull/114507)
* [allow using external builds of the compiler-rt profile lib](https://github.com/rust-lang/rust/pull/114069)
* [allowing re-implementation of `mir_drops_elaborated` query](https://github.com/rust-lang/rust/pull/114628)
* [also consider `mem::transmute` with the `invalid_reference_casting` lint](https://github.com/rust-lang/rust/pull/114757)
* [avoid exporting `__rust_alloc_error_handler_should_panic` more than once](https://github.com/rust-lang/rust/pull/114376)
* [better error handling for `rust.codegen-backends` on deserialization](https://github.com/rust-lang/rust/pull/114278)
* [bubble up nested goals from equation in `predicates_for_object_candidate`](https://github.com/rust-lang/rust/pull/114196)
* [check for non-defining uses of RPIT](https://github.com/rust-lang/rust/pull/112842)
* [convert Const to Allocation in smir](https://github.com/rust-lang/rust/pull/114587)
* [core/any: remove Provider trait, rename Demand to Request](https://github.com/rust-lang/rust/pull/113464)
* [correctly lower `impl const` to bind to host effect param](https://github.com/rust-lang/rust/pull/114545)
* [cover ParamConst in smir](https://github.com/rust-lang/rust/pull/114703)
* [coverage: don't convert filename/symbol strings to `CString` for FFI](https://github.com/rust-lang/rust/pull/114005)
* [coverage: store BCB counter info externally, not directly in the BCB graph](https://github.com/rust-lang/rust/pull/114354)
* [detect method not found on arbitrary self type with different mutability](https://github.com/rust-lang/rust/pull/114469)
* [detect missing `;` that parses as function call](https://github.com/rust-lang/rust/pull/114474)
* [don't use `type_of` to determine if item has intrinsic shim](https://github.com/rust-lang/rust/pull/114670)
* [downgrade `internal_features` to warn](https://github.com/rust-lang/rust/pull/114692)
* [fixed `*const [type error] does not implement the Copy trait`](https://github.com/rust-lang/rust/pull/114752)
* [generate better function argument names in `global_allocator` expansion](https://github.com/rust-lang/rust/pull/114550)
* [interpret: remove incomplete protection against invalid where clauses](https://github.com/rust-lang/rust/pull/114615)
* [interpret: use ConstPropNonsense for more const-prop induced issues](https://github.com/rust-lang/rust/pull/114542)
* [issue numbers are enforced on active features; remove FIXME](https://github.com/rust-lang/rust/pull/114556)
* [llvm-wrapper: adapt for LLVM API changes](https://github.com/rust-lang/rust/pull/114510)
* [make Const more useful in smir](https://github.com/rust-lang/rust/pull/114745)
* [make `unconditional_recursion` warning detect recursive drops](https://github.com/rust-lang/rust/pull/113902)
* [make the provisional cache slightly less broken](https://github.com/rust-lang/rust/pull/114694)
* [map RPIT duplicated lifetimes back to fn captured lifetimes](https://github.com/rust-lang/rust/pull/114602)
* [migrate a trait selection error to use diagnostic translation](https://github.com/rust-lang/rust/pull/114548)
* [normalize in `trait_ref_is_knowable` in new solver](https://github.com/rust-lang/rust/pull/114457)
* [only check outlives goals on impl compared to trait](https://github.com/rust-lang/rust/pull/109356)
* [only resolve target type in `try_coerce` in new solver](https://github.com/rust-lang/rust/pull/114648)
* [open pidfd in child process and send to the parent via `SOCK_SEQPACKET+CMSG`](https://github.com/rust-lang/rust/pull/113939)
* [record binder for bare trait object in LifetimeCollectVisitor](https://github.com/rust-lang/rust/pull/114667)
* [remove constness from `ImplSource::Param`](https://github.com/rust-lang/rust/pull/114781)
* [remove redundant calls to `resolve_vars_with_obligations`](https://github.com/rust-lang/rust/pull/114684)
* [rename method in `opt-dist`](https://github.com/rust-lang/rust/pull/114596)
* [respect `#[expect]` the same way `#[allow]` is with the `dead_code` lint](https://github.com/rust-lang/rust/pull/114710)
* [restrict linker version script of proc-macro crates to just its two symbols](https://github.com/rust-lang/rust/pull/114470)
* [select obligations before processing wf obligation in `compare_method_predicate_entailment`](https://github.com/rust-lang/rust/pull/114787)
* [simplify handling of valtrees for unsized types](https://github.com/rust-lang/rust/pull/114520)
* [store the laziness of type aliases in their `DefKind`](https://github.com/rust-lang/rust/pull/114566)
* [structurally normalize weak and inherent in new solver](https://github.com/rust-lang/rust/pull/114594)
* [style fix and refactor on resolve diagnostics](https://github.com/rust-lang/rust/pull/114549)
* [suggest using `Arc` on `!Send`/`!Sync` types](https://github.com/rust-lang/rust/pull/88936)
* [TAITs do not constrain generic params](https://github.com/rust-lang/rust/pull/114742)
* [tests: uncomment now valid GAT code behind FIXME](https://github.com/rust-lang/rust/pull/114629)
* [unlock trailing where-clauses for lazy type aliases](https://github.com/rust-lang/rust/pull/114662)
* [use the correct `llvm-profdata` binary in `opt-dist`](https://github.com/rust-lang/rust/pull/114344)
* [warn when `#[macro_export]` is applied on decl macros](https://github.com/rust-lang/rust/pull/114413)
* [push DiscriminantKind implementation fact unconditionally](https://github.com/rust-lang/chalk/pull/800)
* [add trait decls to SMIR](https://github.com/rust-lang/rust/pull/114485)
* [add impl trait declarations to SMIR](https://github.com/rust-lang/rust/pull/114599)
* [stabilize `abi_thiscall`](https://github.com/rust-lang/rust/pull/114562)
* [miri: add checked float-to-int helper function](https://github.com/rust-lang/miri/pull/3025)
* [miri: add gamma function shims](https://github.com/rust-lang/miri/pull/3023)
* [miri: llvm.prefetch is not a math function](https://github.com/rust-lang/miri/pull/3024)
* [miri: replace AsAny hack by trait upcasting :)](https://github.com/rust-lang/miri/pull/3022)
* [tell LLVM that the negation in `<*const T>::sub` cannot overflow](https://github.com/rust-lang/rust/pull/114720)
* [implement `Option::take_if`](https://github.com/rust-lang/rust/pull/98935)
* [avoid using `ptr::Unique` in `LinkedList` code](https://github.com/rust-lang/rust/pull/114257)
* [rename copying `ascii::Char` methods from `as_` to `to_`](https://github.com/rust-lang/rust/pull/114641)
* [better `Debug` for `Vars` and `VarsOs`](https://github.com/rust-lang/rust/pull/114132)
* [make ExitStatus implement Default](https://github.com/rust-lang/rust/pull/106425)
* [partially stabilize `int_roundings`](https://github.com/rust-lang/rust/pull/94455)
* [add `Iterator::map_windows`](https://github.com/rust-lang/rust/pull/94667)
* [add a new `compare_bytes` intrinsic instead of calling `memcmp` directly](https://github.com/rust-lang/rust/pull/114382)
* [add gamma function to `f32` and `f64`](https://github.com/rust-lang/rust/pull/99747)
* [cargo-credential: reset stdin & stdout to the Console](https://github.com/rust-lang/cargo/pull/12469)
* [cargo: Make `--help` easier to browse](https://github.com/rust-lang/cargo/pull/11905)
* [cargo: enable ansi color only in terminal](https://github.com/rust-lang/cargo/pull/12488)
* [cargo: bail out an error when using cargo: in custom build script](https://github.com/rust-lang/cargo/pull/12332)
* [cargo: fix cargo remove incorrectly removing used patches](https://github.com/rust-lang/cargo/pull/12454)
* [cargo: fix panic when enabling http.debug for certain strings](https://github.com/rust-lang/cargo/pull/12468)
* [cargo: fix: preserve jobserver file descriptors on rustc invocation to get `TargetInfo`](https://github.com/rust-lang/cargo/pull/12447)
* [cargo: prompt the use of `--nocapture` flag if `cargo test` process is terminated via a signal](https://github.com/rust-lang/cargo/pull/12463)
* [rustfmt: don't flatten blocks that have labels](https://github.com/rust-lang/rustfmt/pull/5677)
* [rustfmt: enable rustfmt to compile when using the `generic-simd` feature](https://github.com/rust-lang/rustfmt/pull/5872)
* [rustfmt: improve formatting of empty `macro_rules!` definitions](https://github.com/rust-lang/rustfmt/pull/5883)
* [rustfmt: improve the `--file-lines` help](https://github.com/rust-lang/rustfmt/pull/5846)
* [rustfmt: refactor ABI formatting](https://github.com/rust-lang/rustfmt/pull/5845)
* [clippy: `iter_overeager_cloned`: detect `.cloned().filter()` and `.cloned().find()`](https://github.com/rust-lang/rust-clippy/pull/11289)
* [clippy: `filter_map_bool_then`: Don't ICE on late bound regions](https://github.com/rust-lang/rust-clippy/pull/11318)
* [clippy: `manual_retain`: add lint case for `binary_heap`](https://github.com/rust-lang/rust-clippy/pull/11329)
* [clippy: `redundant_guards`: don't lint on float literals](https://github.com/rust-lang/rust-clippy/pull/11305)
* [clippy: `redundant_locals`: fix FPs on mutated shadows](https://github.com/rust-lang/rust-clippy/pull/11320)
* [rust-analyzer: add check.ignore to list cargo check diagnostics to ignore `(dead_code, unused_imports,` ...)](https://github.com/rust-lang/rust-analyzer/pull/15262)
* [rust-analyzer: add mir lower support for tuple destructing assignment](https://github.com/rust-lang/rust-analyzer/pull/15419)
* [rust-analyzer: display fully qualified associated types correctly](https://github.com/rust-lang/rust-analyzer/pull/15416)
* [rust-analyzer: don't use control flow when extracted fn contains tail expr of original fn](https://github.com/rust-lang/rust-analyzer/pull/15250)
* [rust-analyzer: fix pinned version of lsp-types](https://github.com/rust-lang/rust-analyzer/pull/15428)
* [rust-analyzer: fix `only_types` config filtering out traits from world symbols](https://github.com/rust-lang/rust-analyzer/pull/15445)
* [rust-analyzer: fix float parser hack creating empty NameRef tokens](https://github.com/rust-lang/rust-analyzer/pull/15415)
* [rust-analyzer: fix parser being stuck in eager macro inputs](https://github.com/rust-lang/rust-analyzer/pull/15438)
* [rust-analyzer: handle `#[cfg]`s on generic parameters](https://github.com/rust-lang/rust-analyzer/pull/15350)
* [rust-analyzer: internal : Deunwrap `convert_named_struct_to_tuple_struct`](https://github.com/rust-lang/rust-analyzer/pull/15423)
* [rust-analyzer: support doc links that resolve to fields](https://github.com/rust-lang/rust-analyzer/pull/15405)

### Rust Compiler Performance Triage

A light week. Main thing to report is we got some improvements from telling LLVM
that the negation in `<*const T>::sub` cannot overflow.

Triage done by **@pnkfelix**.
Revision range: [443c3161..e8459109](https://perf.rust-lang.org/?start=443c3161dd04f4c1b656a626f9079921bee9c326&end=e8459109bbb440764c1c877032189a27b9e76c4e&absolute=false&stat=instructions%3Au)

0 Regressions, 1 Improvements, 4 Mixed; 1 of them in rollups
49 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-08-14.md) 

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Create a Testing sub-team](https://github.com/rust-lang/rfcs/pull/3455)
* [Allow cfg-attributes in where clauses](https://github.com/rust-lang/rfcs/pull/3399)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Cleaner assert_eq! & assert_ne! panic messages](https://github.com/rust-lang/rust/pull/111071)
* [disposition: merge] [Report monomorphization time errors in dead code, too](https://github.com/rust-lang/rust/pull/112879)
* [disposition: merge] [Allow explicit `#[repr(Rust)]`](https://github.com/rust-lang/rust/pull/114201)
* [disposition: merge] [style-guide: Add section on bugs, and resolving bugs](https://github.com/rust-lang/rust/pull/113383)
* [disposition: merge] [Lower `Or` pattern without allocating place](https://github.com/rust-lang/rust/pull/111752)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Unified String Literals](https://github.com/rust-lang/rfcs/pull/3475)
* [new] [RFC: `scheduled_removal` Parameter for `deprecated` Attribute](https://github.com/rust-lang/rfcs/pull/3471)
* [new] [crABI v1](https://github.com/rust-lang/rfcs/pull/3470)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-08-16 - 2023-09-13 🦀

### Virtual

* 2023-08-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/kmhpftyfclbvb/)
* 2023-08-17 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295051663/)
* 2023-08-22 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**Rust, Serverless and AWS**](https://www.meetup.com/Rust-Dublin/events/294587280/)
* 2023-08-23 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 32nd Edition**](https://www.meetup.com/rust-linz/events/294718621/)
* 2023-08-24 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/295250677/)
* 2023-09-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295207389/)
* 2023-09-05 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 4 - hybrid**](https://www.meetup.com/rust-munich/events/294186101/)
* 2023-09-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/294049877)
* 2023-09-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfcmbkb/)
* 2023-09-12 - 2023-09-15 | Virtual (Albuquerque, NM, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/gqdlgtyfcmbqb/)
* 2023-09-13 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**The unreasonable power of combinator APIs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294748626)
* 2023-09-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732655)

### Asia

* 2023-09-06 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**RustTLV @ Final - September Edition**](https://www.meetup.com/rust-tlv/events/295441355/)

### Europe

* 2023-08-16 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Native Graph Algorithms in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295191245/)
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

* 2023-08-16 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/294910746/)
* 2023-08-17 | Nashville, TN, US | [Seattle Rust User Group Meetup](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust goes where it pleases. Rust on the web and embedded**](https://www.meetup.com/music-city-rust-developers/events/294805470/)
* 2023-08-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295008514)
* 2023-08-24 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/295107743/)
* 2023-08-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #39 sponsored by Fermyon**](https://www.meetup.com/copenhagen-rust-community/events/294806394)
* 2023-09-06 | Bellevue, WA, US | [The Linux Foundation](https://www.linuxfoundation.org/)
    * [**Rust Global**](https://events.linuxfoundation.org/rust-global/)
* 2023-09-12 - 2023-09-15 | Albuquerque, NM, US  + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)

### Oceania

* 2023-08-24 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**August Meetup**](https://www.meetup.com/rust-brisbane/events/295415680/)

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

> It has been
>
> **0**
>
> days since someone tried and failed to use unsafe to circumvent the lifetime system.

– [H2CO3 on rust-users](https://users.rust-lang.org/t/announcing-shared-cell-an-additional-cell-api-with-zero-cost-concurrent-data-sharing-in-single-threaded-asynchronous-code/98342/15)

Thanks to [mdHMUpeyf8yluPfXI](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1459) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/15tajsl/this_week_in_rust_508/)</small>
