Title: This Week in Rust 538
Number: 538
Date: 2024-03-13
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
* [Announcing Rustup 1.27.0](https://blog.rust-lang.org/2024/03/11/Rustup-1.27.0.html)
* [crates.io: Download changes](https://blog.rust-lang.org/2024/03/11/crates-io-download-changes.html)

### Newsletters
* [This Month in Rust OSDev: February 2024](https://rust-osdev.com/this-month/2024-02/)

### Project/Tooling Updates
* [Bevy Foundation](https://bevyengine.org/news/bevy-foundation/)
* [Rust Flashcards - 557 cards to learn Rust from first principles](https://github.com/ad-si/Rust-Flashcards)
* [Lib.rs website improvements](https://users.rust-lang.org/t/lib-rs-website-improvements/108218)
* [Sudo-rs dependencies: when less is better](https://www.memorysafety.org/blog/reducing-dependencies-in-sudo/)
* [rust-analyzer changelog #224](https://rust-analyzer.github.io/thisweek/2024/03/11/changelog-224.html)
* [biscotti, a new crate for HTTP cookies](https://www.lpalmieri.com/posts/biscotti-http-cookies-in-rust/)
* [Boa release v0.18](https://boajs.dev/blog/2024/03/07/boa-release-18)
* [Announcing Relm4 0.7 and 0.8](https://relm4.org/blog/posts/announcing_relm4_v0.7/)
* [Meilisearch 1.7 — New OpenAI models & GPU support for Hugging Face embeddings](https://blog.meilisearch.com/meilisearch-1-7/)
* [Cargo wizard: automate Cargo project configuration](https://kobzol.github.io/rust/cargo/2024/03/10/rust-cargo-wizard.html)
* [GreptimeDB v0.7 is ready for cloud-native monitoring](https://greptime.com/blogs/2024-03-07-greptimedb-v0.7)
* [video] [Project Syn - Simon Gerber - Rust Zürisee March 2024](https://www.youtube.com/watch?list=PL85XCvVPmGQipj690WrVgsnU4K4x7qFGy&v=N7GMHcX-WdA)

### Observations/Thoughts
* [Fast Development In Rust, Part One](https://blog.sdf.com/p/fast-development-in-rust-part-one)
* [Mental model for unsafe (complete rewrite)](https://ia0.github.io/unsafe-mental-model/)
* [10 years in Open Source](https://ochagavia.nl/blog/10-years-in-open-source/)
* [audio] [Season Finale - Rust in Production Podcast](https://corrode.dev/podcast/s01e07-season-finale/)
* [video] [SemVer in Rust: Breakage, Tooling, and Edge Cases](https://www.youtube.com/watch?v=VArNQtYBC6Y)

### Rust Walkthroughs
* [How to Use Rust Procedural Macros to Replace Panic with syn’s Fold](https://www.infoq.com/articles/rust-procedural-macros-replace-panic/)
* [Rust Iterators Beyond the Basics - part 1](https://blog.jetbrains.com/rust/2024/03/12/rust-iterators-beyond-the-basics-part-i-building-blocks/)
* [Rust Multi-crate project in a monorepo](https://rust.code-maven.com/multi-crate-project)
* [Using Stripe Payments with Rust](https://www.shuttle.rs/blog/2024/03/07/stripe-payments-rust)
* [Problems around modelling an asynchronous API for database transaction in Rust](https://blog.weiznich.de/blog/async-transaction-problem/)
* [Embedded Rust Bluetooth on ESP: BLE Scanner](https://apollolabsblog.hashnode.dev/embedded-rust-bluetooth-on-esp-ble-scanner)
* [video] [Implementing (parts of) git from scratch in Rust](https://www.youtube.com/watch?v=u0VotuGzD_w)
* [video] [The Billion Rows Challenge in Rust - an intro to Rust for data engineering](https://www.youtube.com/watch?v=-1VGwmFKKf8)

### Miscellaneous
* [Rust impact on engineering management](https://bastienvigneron.medium.com/rust-impact-on-engineering-management-59647e5d0265)
* [From medical doctor to rust developer](https://surrealdb.com/blog/from-medical-doctor-to-rust-developer--interview-with-our-new-senior-clinical-research-fellow)
* [February 2024 Rust Jobs Report](https://filtra.io/rust-feb-24)

## Crate of the Week

This week's crate is [biscotti](https://github.com/LukeMathWalker/biscotti), an opinionated library to deal with HTTP cookies on the server side.

We saw a lamentable lack of suggestions this week. Nevertheless, llogiq is pleased with his selection.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [Tracking Issue for `min_exhaustive_patterns`](https://github.com/rust-lang/rust/issues/119612)
    * [Testing Steps](https://github.com/rust-lang/rust/issues/119612#issuecomment-1967092452)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Hyperswitch - [REFACTOR]: Remove Default Case Handling - Bambora](https://github.com/juspay/hyperswitch/issues/4054)
* [Hyperswitch - [REFACTOR]: Remove Default Case Handling - Mollie](https://github.com/juspay/hyperswitch/issues/4055)
* [Hyperswitch - [REFACTOR]: Remove Default Case Handling - Braintree](https://github.com/juspay/hyperswitch/issues/4058)
* [GreptimeDB - Painless integrating with Grafana via Prometheus plugin](https://github.com/GreptimeTeam/greptimedb/issues/3492)
* [GreptimeDB - Add strict mode to validate protocol strings](https://github.com/GreptimeTeam/greptimedb/issues/3435)
* [Fluvio - fvm switch fails on some systems with running local cluster](https://github.com/infinyon/fluvio/issues/3765)
* [Fluvio - Add new command fluvio cluster resume](https://github.com/infinyon/fluvio/issues/3810)
* [quinn - Add CI for mobile platforms (iOS & Android)](https://github.com/quinn-rs/quinn/issues/1778)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker. They are ordered below by when the CFP closes.

* [RustFest Zürich 2024](https://rustfest.ch/cfp/) | Closes 2024-03-31 | Zürich, Switzerland | Event date: 2024-06-19 - 2024-06-24
* [Oxidize 2024](https://pretalx.com/oxidize-berlin-2024/cfp) | Closes 2024-03-24 | Berlin, Germany | Event date: 2024-05-28 - 2024-05-30
* [RustConf 2024](https://foundation.rust-lang.org/news/the-rustconf-2024-call-for-talk-proposals-is-open/) | Closes 2024-04-25 | Montreal, Canada | Event date: 2024-09-10
* [EuroRust 2024](https://www.papercall.io/eurorust-2024)| Closes 2024-06-03 | Vienna, Austria & online | Event on 2024-10-10
* [Scientific Computing in Rust 2024](https://scientificcomputing.rs/)| Closes 2024-06-14 | online | Event date: 2024-07-17 - 2024-07-19

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

506 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-03-05..2024-03-12

* [add new Tier-3 target: `loongarch64-unknown-linux-musl`](https://github.com/rust-lang/rust/pull/121832)
* [add arm64ec-pc-windows-msvc target](https://github.com/rust-lang/rust/pull/119199)
* [LLVM Bitcode Linker: A self contained linker for nvptx and other targets](https://github.com/rust-lang/rust/pull/117458)
* [AST validation: Improve handling of inherent impls nested within functions and anon consts](https://github.com/rust-lang/rust/pull/122004)
* [`const_eval_select`: make it safe but be careful with what we expose on stable for now](https://github.com/rust-lang/rust/pull/121894)
* [`hir_analysis`: enums return `None` in `find_field`](https://github.com/rust-lang/rust/pull/121975)
* [add a `description` field to target definitions](https://github.com/rust-lang/rust/pull/121905)
* [add an option to have an external download/bootstrap cache](https://github.com/rust-lang/rust/pull/121976)
* [add asm goto support to `asm!`](https://github.com/rust-lang/rust/pull/119365)
* [add metadata to targets](https://github.com/rust-lang/rust/pull/122305)
* [add proper cfgs for `struct` HirIdValidator used only with debug-assert](https://github.com/rust-lang/rust/pull/122066)
* [add the new description field to `Target::to_json,` and add descriptions for some MSVC targets](https://github.com/rust-lang/rust/pull/122157)
* [allow codegen backends to opt-out of parallel codegen](https://github.com/rust-lang/rust/pull/116791)
* [allow multiple `impl Into<{D,Subd}iagMessage>` parameters in a function](https://github.com/rust-lang/rust/pull/122315)
* [allow targets to override default codegen backend](https://github.com/rust-lang/rust/pull/116793)
* [apply `EarlyBinder` only to `TraitRef` in `ImplTraitHeader`](https://github.com/rust-lang/rust/pull/122043)
* [avoid invoking the `intrinsic` query for DefKinds other than `Fn` or `AssocFn`](https://github.com/rust-lang/rust/pull/122010)
* [avoid overlapping privacy suggestion for single nested imports](https://github.com/rust-lang/rust/pull/121190)
* [change some attributes to `only_local`](https://github.com/rust-lang/rust/pull/122014)
* [check that return type is WF in typeck](https://github.com/rust-lang/rust/pull/122078)
* [detect unused `struct` impls pub trait](https://github.com/rust-lang/rust/pull/121752)
* [diagnostics: do not suggest using `#[unix_sigpipe]` without a value](https://github.com/rust-lang/rust/pull/122293)
* [distinguish between library and lang UB in `assert_unsafe_precondition`](https://github.com/rust-lang/rust/pull/121662)
* [do not try to format removed files](https://github.com/rust-lang/rust/pull/122026)
* [don't ICE if we collect no RPITITs unless there are no unification errors](https://github.com/rust-lang/rust/pull/122172)
* [don't ICE in CTFE if raw/fn-ptr types differ](https://github.com/rust-lang/rust/pull/122173)
* [don't pass a break scope to `Builder::break_for_else`](https://github.com/rust-lang/rust/pull/122137)
* [don't require specifying unrelated assoc types when trait alias is in `dyn` type](https://github.com/rust-lang/rust/pull/122123)
* [dynamically size sigaltstk in std](https://github.com/rust-lang/rust/pull/113525)
* [errors: share `SilentEmitter` between rustc and rustfmt](https://github.com/rust-lang/rust/pull/121301)
* [fix crash in internal late lint checking](https://github.com/rust-lang/rust/pull/122181)
* [fix legacy numeric constant diag items](https://github.com/rust-lang/rust/pull/122271)
* [fix linting paths with qself in `unused_qualifications`](https://github.com/rust-lang/rust/pull/122038)
* [fix metadata for dyn-star in new solver](https://github.com/rust-lang/rust/pull/122304)
* [fix misaligned loads when loading UEFI arg pointers](https://github.com/rust-lang/rust/pull/122164)
* [fix redundant import errors for preload extern crate](https://github.com/rust-lang/rust/pull/121958)
* [fix type resolution of associated const equality bounds (take 2)](https://github.com/rust-lang/rust/pull/119385)
* [fix: localWaker memory leak and some stability attributes](https://github.com/rust-lang/rust/pull/122244)
* [implement async closure signature deduction](https://github.com/rust-lang/rust/pull/121857)
* [improve diagnostics for parenthesized type arguments](https://github.com/rust-lang/rust/pull/122152)
* [interpret: do not call machine read hooks during validation](https://github.com/rust-lang/rust/pull/122249)
* [limit the number of names and values in check-cfg diagnostics](https://github.com/rust-lang/rust/pull/121202)
* [lint singleton gaps after exclusive ranges](https://github.com/rust-lang/rust/pull/118879)
* [loongarch: add frecipe and relax target feature](https://github.com/rust-lang/rust/pull/122022)
* [lower transmutes from int to pointer type as gep on null](https://github.com/rust-lang/rust/pull/121282)
* [make TAITs and ATPITs capture late-bound lifetimes in scope](https://github.com/rust-lang/rust/pull/122103)
* [make `DefiningAnchor::Bind` only store the opaque types that may be constrained, instead of the current infcx root item](https://github.com/rust-lang/rust/pull/121796)
* [make `std::os::unix::ucred` module private](https://github.com/rust-lang/rust/pull/122147)
* [make not finding core a fatal error](https://github.com/rust-lang/rust/pull/122114)
* [make the lowering of `thir::ExprKind::If` easier to follow](https://github.com/rust-lang/rust/pull/122063)
* [match lowering: define a convenient `struct`](https://github.com/rust-lang/rust/pull/122221)
* [only compare ambiguity item that have hard error](https://github.com/rust-lang/rust/pull/121846)
* [only set noalias on Box with the global allocator](https://github.com/rust-lang/rust/pull/122018)
* [record mtime in bootstrap's LLVM linker script](https://github.com/rust-lang/rust/pull/122138)
* [refactor pre-getopts command line argument handling](https://github.com/rust-lang/rust/pull/121194)
* [refer to "slice" instead of "vector" in Ord and PartialOrd trait impl of slices](https://github.com/rust-lang/rust/pull/122072)
* [remove `feed_local_def_id`](https://github.com/rust-lang/rust/pull/121089)
* [removing absolute path in proc-macro](https://github.com/rust-lang/rust/pull/121959)
* [rework `untranslatable_diagnostic` lint](https://github.com/rust-lang/rust/pull/121382)
* [run a single huge `par_body_owners` instead of many small ones after each other](https://github.com/rust-lang/rust/pull/122140)
* [silence mismatched types errors for implied projections](https://github.com/rust-lang/rust/pull/121863)
* [stabilize the `#[diagnostic]` namespace and `#[diagnostic::on_unimplemented]` attribute](https://github.com/rust-lang/rust/pull/119888)
* [std support for wasm32 panic=unwind](https://github.com/rust-lang/rust/pull/121438)
* [store backtrace for `must_produce_diag`](https://github.com/rust-lang/rust/pull/122299)
* [temporarily make allow-by-default the `non_local_definitions` lint](https://github.com/rust-lang/rust/pull/122107)
* [tweak the way we protect in-place function arguments in interpreters](https://github.com/rust-lang/rust/pull/122076)
* [uplift some feeding out of `associated_type_for_impl_trait_in_impl` and into queries](https://github.com/rust-lang/rust/pull/122027)
* [use GEP inbounds for ZST and DST field offsets](https://github.com/rust-lang/rust/pull/122048)
* [use ptradd for vtable indexing](https://github.com/rust-lang/rust/pull/122320)
* [MIR printing: print the path of uneval'd const](https://github.com/rust-lang/rust/pull/122290)
* [miri: do not apply aliasing restrictions to `Box` with custom allocator](https://github.com/rust-lang/rust/pull/122233)
* [miri: remove the ability to disable ABI checking](https://github.com/rust-lang/miri/pull/3340)
* perf: build `rustc` with 1CGU on [`x86_64-apple-darwin`](https://github.com/rust-lang/rust/pull/112268) and [`x86_64-pc-windows-msvc`](https://github.com/rust-lang/rust/pull/112267)
* [replace the default branch with an unreachable branch If it is the last variant](https://github.com/rust-lang/rust/pull/120268)
* [optimize write with `as_const_str` for shorter code](https://github.com/rust-lang/rust/pull/122059)
* [fix quadratic behavior of repeated vectored writes](https://github.com/rust-lang/rust/pull/121938)
* [net: don't use checked arithmetic when parsing numbers with known max digits](https://github.com/rust-lang/rust/pull/121428)
* [`align_offset, align_to`: no longer allow implementations to spuriously fail to align](https://github.com/rust-lang/rust/pull/121201)
* [impl `From<TryReserveError>` for `io::Error`](https://github.com/rust-lang/rust/pull/121403)
* [make `impl<Fd: AsFd>` impl take `?Sized`](https://github.com/rust-lang/rust/pull/114655)
* [add `Read` impl for `&Stdin`](https://github.com/rust-lang/rust/pull/99153)
* [`std::rand`: enable getrandom for dragonflybsd too](https://github.com/rust-lang/rust/pull/121942)
* [`std::threads`: revisit stack address calculation on netbsd](https://github.com/rust-lang/rust/pull/122002)
* [win10: use `GetSystemTimePreciseAsFileTime` directly](https://github.com/rust-lang/rust/pull/121633)
* [windows: implement condvar, mutex and rwlock using futex](https://github.com/rust-lang/rust/pull/121956)
* [add `slice::try_range`](https://github.com/rust-lang/rust/pull/121148)
* [implement `MaybeUninit::fill{,_with,_from}`](https://github.com/rust-lang/rust/pull/121280)
* [`RawVec::into_box`: avoid unnecessary intermediate reference](https://github.com/rust-lang/rust/pull/122298)
* [`Vec::try_with_capacity`](https://github.com/rust-lang/rust/pull/120504)
* [add `#[inline]` to `BTreeMap::new` constructor](https://github.com/rust-lang/rust/pull/122099)
* [add `std::ffi::c_str` module](https://github.com/rust-lang/rust/pull/112136)
* [futures: add a helper for always ready futures](https://github.com/rust-lang/futures-rs/pull/2825)
* [cargo: cli: allow logging to chrome traces](https://github.com/rust-lang/cargo/pull/13399)
* [cargo: doc: collapse down generated statuses without --verbose](https://github.com/rust-lang/cargo/pull/13557)
* [cargo: log: trace parameters to align with profile](https://github.com/rust-lang/cargo/pull/13538)
* [cargo: lockfile: make diffing/printing more reusable](https://github.com/rust-lang/cargo/pull/13564)
* [cargo: consistently compare MSRVs](https://github.com/rust-lang/cargo/pull/13537)
* [rustfmt: fix failure with `=>` in comment after match `=>`](https://github.com/rust-lang/rustfmt/pull/6092)
* clippy: new lints: [`duplicated_attributes`](https://github.com/rust-lang/rust-clippy/pull/12378), [`manual_unwrap_or_default`](https://github.com/rust-lang/rust-clippy/pull/12440), [`nonminimal_bool`](https://github.com/rust-lang/rust-clippy/pull/12401), [`zero_repeat_side_effects`](https://github.com/rust-lang/rust-clippy/pull/12449) and [`const_is_empty`](https://github.com/rust-lang/rust-clippy/pull/12310)
* clippy: fix duplicate diagnostics in [`manual_retain`](https://github.com/rust-lang/rust-clippy/pull/12452), [`mut_mut`](https://github.com/rust-lang/rust-clippy/pull/12442), [`no_effect_replace`](https://github.com/rust-lang/rust-clippy/pull/12443) and [`single_match`](https://github.com/rust-lang/rust-clippy/pull/12448)
* [clippy: don't lint `redundant_field_names` across macro boundaries](https://github.com/rust-lang/rust-clippy/pull/12429)
* [clippy: fix `std_instead_of_core` false positive](https://github.com/rust-lang/rust-clippy/pull/12447)
* [clippy: fix `missing_docs_in_private_items` on some proc macros](https://github.com/rust-lang/rust-clippy/pull/12433)
* [clippy: have more lints respect `#[allow]` on exprs](https://github.com/rust-lang/rust-clippy/pull/12446)
* [rust-analyzer: add QuickFix for unresolved field](https://github.com/rust-lang/rust-analyzer/pull/16762)
* [rust-analyzer: add fix for `unused_variables`](https://github.com/rust-lang/rust-analyzer/pull/16810)
* [rust-analyzer: add proc macro semantic token type](https://github.com/rust-lang/rust-analyzer/pull/16808)
* [rust-analyzer: add config and capability for test explorer](https://github.com/rust-lang/rust-analyzer/pull/16773)
* [rust-analyzer: don't escape `\` and `$` in "Extract format expressions" assist](https://github.com/rust-lang/rust-analyzer/pull/16781)
* [rust-analyzer: don't force draw a dependency edge to the `real_span_map` query](https://github.com/rust-lang/rust-analyzer/pull/16776)
* [rust-analyzer: don't invalid body query results when generating desugared names](https://github.com/rust-lang/rust-analyzer/pull/16777)
* [rust-analyzer: fix method resolution snapshotting `receiver_ty` too early](https://github.com/rust-lang/rust-analyzer/pull/16811)
* [rust-analyzer: function argument type inference with associated type impl trait](https://github.com/rust-lang/rust-analyzer/pull/16769)
* [rust-analyzer: keep attributes in assist `'generate_delegate_trait'`](https://github.com/rust-lang/rust-analyzer/pull/16766)
* [rust-analyzer: panic when using float numbers without dots in chain calls](https://github.com/rust-lang/rust-analyzer/pull/16770)
* [rust-analyzer: preserve $ and \ in postfix format completions](https://github.com/rust-lang/rust-analyzer/pull/16782)
* [rust-analyzer: remove accidental dependency between `parse_macro_expansion` and `parse`](https://github.com/rust-lang/rust-analyzer/pull/16775)
* [rust-analyzer: skip match diagnostics for partially unknown types](https://github.com/rust-lang/rust-analyzer/pull/16779)
* [rust-analyzer: for toolchain binaries use the full path found in $PATH](https://github.com/rust-lang/rust-analyzer/pull/16755)
* [rust-analyzer: stop eagerly resolving inlay hint text edits for VSCode](https://github.com/rust-lang/rust-analyzer/pull/16473)

### Rust Compiler Performance Triage

A mixed week, with a vast number of improvements (in large part due to PR
#122010, which undoes a prior regression; PR #120985, a host LLVM update).
But also three admittedly small-ish regressions which seemed unanticipated and
were still large enough that I did not feel comfortable rubber-stamping them
with a perf-regression-triaged marking.

Triage done by **@pnkfelix**.
Revision range: [41d97c8a..e919669d](https://perf.rust-lang.org/?start=41d97c8a5dea2731b0e56fe97cd7cb79e21cff79&end=e919669d42dfb8950866d4cb268c5359eb3f7c54&absolute=false&stat=instructions%3Au)

2 Regressions, 5 Improvements, 9 Mixed; 5 of them in rollups
54 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/69fb50d19f3d9bb28a903ddcee7326eef6a11518/triage/2024-03-11.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Packages as (optional) namespaces](https://github.com/rust-lang/rfcs/pull/3243)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Normalize trait ref before orphan check & consider ty params in alias types to be uncovered](https://github.com/rust-lang/rust/pull/117164)
* [disposition: merge] [Split refining_impl_trait lint into _reachable, _internal variants](https://github.com/rust-lang/rust/pull/121720)
* [disposition: merge] [debuginfo: Stabilize `-Z debug-macros`, `-Z collapse-macro-debuginfo` and `#[collapse_debuginfo]`](https://github.com/rust-lang/rust/pull/120845)
* [disposition: merge] [Stabilize associated type bounds (RFC 2289)](https://github.com/rust-lang/rust/pull/122055)
* [disposition: merge] [`c_unwind` full stabilization request: change in `extern "C"` behavior](https://github.com/rust-lang/rust/issues/115285)
* [disposition: postpone] [Consider linting against 00B7 aka interpunct aka middle dot](https://github.com/rust-lang/rust/issues/120797)
* [disposition: merge] [Prevent opaque types being instantiated twice with different regions within the same function](https://github.com/rust-lang/rust/pull/116935)
* [disposition: merge] [instantiate higher ranked goals outside of candidate selection](https://github.com/rust-lang/rust/pull/119820)
* [disposition: merge] [Add `wasm_c_abi` `future-incompat` lint](https://github.com/rust-lang/rust/pull/117918)
* [disposition: merge] [stabilize ptr.is_aligned, move ptr.is_aligned_to to a new feature gate ](https://github.com/rust-lang/rust/pull/121948)
* [disposition: merge] [feat: `implement {Div,Rem}Assign<NonZero<X>>` on `X`](https://github.com/rust-lang/rust/pull/121952)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [cargo: prevent dashes in lib.name](https://github.com/rust-lang/cargo/pull/12783)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Make `cargo install` respect lockfiles by default](https://github.com/rust-lang/rfcs/pull/3585)

## Upcoming Events

Rusty Events between 2024-03-13 - 2024-04-10 🦀

### Virtual

* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Web Frontend Co-Learning (online)**](https://www.meetup.com/opentechschool-berlin/events/298406445/)
* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457903/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/03/14/rust-hack-and-learn.html)
* 2024-03-14 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945252/)
* 2024-03-14 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust/)
    * [**San Diego Rust March 2024 Tele-Meetup**](https://www.meetup.com/san-diego-rust/events/299743034/)
* 2024-03-19 | Virtual | [Formal Land](https://github.com/formal-land/)
    * [**Rust for Lunch: Formal verification for Rust with coq-of-rust. Speaker: Guillaume Claret**](https://lecture.senfcall.de/hay-gmh-wox-mru) | [Docs](https://github.com/formal-land/coq-of-rust)
* 2024-03-19 | Virtual (Washinigton, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299335006/)
* 2024-03-20 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 3 - Designing Interfaces**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/299505703/)
* 2024-03-20 | Virtual (Vancouver, BC, CA)| [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763494/)
* 2024-03-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368793/)
* 2024-03-26 | Virtual + In Person (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/) - [Stream](https://www.youtube.com/@bcnrust)
* 2024-03-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/299644917/)
* 2024-03-28 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457904/)
* 2024-04-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/mrnrktygcgbdb/)
* 2024-04-03 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 4 - Error Handling**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/299507234/)
* 2024-04-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047892/)
* 2024-04-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341660/)

### Africa

* 2024-04-05 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia

* 2024-03-30 | New Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Rust Delhi Meetup #6**](https://www.meetup.com/rustdelhi/events/299771772/)

### Europe

* 2024-03-13 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.com/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-830340830777)
* 2024-03-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/298533419/)
* 2024-03-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/299028814/)
* 2024-03-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Rust Interactive Session**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/299309224/)
* 2024-03-19 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/299515169/)
    * [**Rust Meetup @ Charles University**](https://www.meetup.com/rust-prague/events/299515169/)
* 2024-03-20 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Introduction to programming Microcontrollers with Rust**](https://www.meetup.com/rust-girona/events/299172343/)
* 2024-03-20 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #9**](https://www.meetup.com/fr-FR/rust-lyon/events/299527560/)
* 2024-03-21 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #6**](https://www.meetup.com/de-DE/rust-meetup-augsburg/events/299354449/)
* 2024-03-21 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #6: Du RSS et de L'ECS !**](https://www.meetup.com/meetup-group-zgphbyet/events/299295547/)
* 2024-03-21 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - March - Unsafe Rust**](https://www.meetup.com/rust-vienna/events/299682390/)
* 2024-03-23 | Stockholm, SE | [Ferris' Fika Forum](https://www.google.com/calendar/event?eid=NWMzaDNqZDZjcG5oZXNwYzJycHRzMXI5djUgYXBkOXZtYmMyMmVnZW5tdHU1bDZjNWpiZmNAZw&ctz=America/Los_Angeles)
    * [**Ferris' Fika Forum**](https://www.google.com/calendar/event?eid=NWMzaDNqZDZjcG5oZXNwYzJycHRzMXI5djUgYXBkOXZtYmMyMmVnZW5tdHU1bDZjNWpiZmNAZw&ctz=America/Los_Angeles) | [Map](https://maps.google.com/maps?hl=en&q=Starbucks%2C%20Hamngatan%2033%2C%20111%2047%20Stockholm%2C%20Sweden)
* 2024-03-26 | Barcelona, ES + Virtual | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/)
* 2024-03-26 - 2024-03-28 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation 2024 - Conference**](https://www.rustnationuk.com/)
* 2024-03-28 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell**](https://www.meetup.com/rust-berlin/events/299288961/)
* 2024-04-10 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Rust Meetup Reboot 3**](https://www.meetup.com/cambridge-rust-meetup/events/299730322/)
* 2024-04-10 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/299488225/)

### North America

* 2024-03-13 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Northeastern Rust Lunch**](https://www.meetup.com/bostonrust/events/299262009/)
* 2024-03-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186823/)
* 2024-03-21 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631832/)
* 2024-03-22 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Somerville Union Square Rust Lunch, Mar 22**](https://www.meetup.com/bostonrust/events/299262036/)
* 2024-03-26 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Getting started with Rust!**](https://www.meetup.com/minneapolis-rust-meetup/events/299489274/)
* 2024-03-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299220136/)
* 2024-03-27 | Hawthorne, CA, US | [Freeform](https://freeform.co/)
    * [**Rust in the Physical World 🦀 Tech Talk Event at Freeform**](https://freeformxrust.rsvpify.com/)
* 2024-03-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Beacon Hill Rust Lunch, Mar 31**](https://www.meetup.com/bostonrust/events/299262047/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1arr8xi/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> In 10 years we went from “Rust will never replace C and C++” to “New C/C++ should not be written anymore, and you should use Rust”. Good job.

– [dpc_pw on lobste.rs](https://lobste.rs/s/dbwn98/secure_by_design_google_s_perspective_on#c_f6j9ok)

Thanks to [Dennis Luxen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1547) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1bec9sy/this_week_in_rust_538/)</small>
