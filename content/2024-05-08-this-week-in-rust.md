Title: This Week in Rust 546
Number: 546
Date: 2024-05-08
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
* [Announcing Rust 1.78.0](https://blog.rust-lang.org/2024/05/02/Rust-1.78.0.html)
* [Announcing Rustup 1.27.1](https://blog.rust-lang.org/2024/05/06/Rustup-1.27.1.html)
* [Automatic checking of cfgs at compile-time](https://blog.rust-lang.org/2024/05/06/check-cfg.html)
* [Announcing Google Summer of Code 2024 selected projects](https://blog.rust-lang.org/2024/05/01/gsoc-2024-selected-projects.html)
* [Rust participates in OSPP 2024](https://blog.rust-lang.org/2024/05/07/OSPP-2024.html)
* [This Development-cycle in Cargo: 1.79](https://blog.rust-lang.org/inside-rust/2024/05/07/this-development-cycle-in-cargo-1.79.html)
* [Rust Project Goals Submission Period](https://blog.rust-lang.org/inside-rust/2024/05/07/announcing-project-goals.html)

### Foundation
* [$1M Microsoft Donation to Fund Key Rust Foundation & Project Priorities](https://foundation.rust-lang.org/news/1m-microsoft-donation-to-fund-key-rust-foundation-project-priorities/)

### Newsletters
* [This Month in Rust OSDev: April 2024](https://rust-osdev.com/this-month/2024-04/)
* [This Month in Rust GameDev #50 - April 2024](https://gamedev.rs/news/050/)
* [RustFest.ch - All talks announced](https://rustfest.ch/posts/2024-05-07/all-talks-announced/)

### Project/Tooling Updates
* [image v0.25: performance improvements, production-ready WebP](https://www.reddit.com/r/rust/comments/1cj94va/image_v025_performance_improvements/)
* [rust-analyzer changelog #232](https://rust-analyzer.github.io/thisweek/2024/05/06/changelog-232.html)
* [rustc_codegen_gcc: Progress Report #32](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-32)
* [Introducing graphql-lint and gqlint](https://grafbase.com/changelog/graphql-lint)
* [Meilisearch releases v1.8](https://blog.meilisearch.com/meilisearch-1-8/)
* [r3bl_terminal_async v0.5.2 released](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v052-2020-05-06)
* [Zed Decoded: Linux when?](https://zed.dev/blog/zed-decoded-linux-when)

### Observations/Thoughts
* [Pair Your Compilers At The ABI Café - Faultlore](https://faultlore.com/blah/abi-puns/)
* [Unwind considered harmful?](https://smallcultfollowing.com/babysteps/blog/2024/05/02/unwind-considered-harmful/)
* [Async Rust Complexity](https://v5.chriskrycho.com/journal/async-rust-complexity/)
* [Download Accelerator - Async Rust Edition](https://ochagavia.nl/blog/download-accelerator-async-rust-edition/)
* [video] [David Lattimore - A Linker in the Wild](https://www.youtube.com/watch?v=WSHt3-gwVxc)
* [audio] [curl - Daniel Stenberg, Open Source Maintainer and Public Speaker](https://corrode.dev/podcast/s02e01-curl/)

### Rust Walkthroughs
* [How to rewrite a C++ codebase successfully](https://gaultier.github.io/blog/how_to_rewrite_a_cpp_codebase_successfully.html)
* [Building a Redis / Kafka Data Sink in Rust](https://www.sea-ql.org/blog/2024-05-05-redis-kafka-data-sink/)
* [ZH | EN] [Writing a GPT Plugin in Rust, and Lost Gems](https://ideas.reify.ing/en/blog/gpt-plugin-rust-and-lost-gems/)
* [What is in a Rust Allocator?](https://blog.sulami.xyz/posts/what-is-in-a-rust-allocator/)
* [How hard can generating 1024-bit primes really be?](https://glitchcomet.com/articles/1024-bit-primes/)
* [STM32F4 Embedded Rust at the PAC: System Clock Configuration](https://blog.theembeddedrustacean.com/stm32f4-embedded-rust-at-the-pac-system-clock-configuration)
* [video] [Make a port scanner in #rustlang with Tokio and learn async Rust](https://www.youtube.com/watch?v=J3C6sNK2wnk)

### Research
* [Rust Digger: 47,764 (32.92%) of the crates include a Cargo.lock file](https://rust-digger.code-maven.com/news/cargo-lock-and-main-rs)
* [VERT: Verified Equivalent Rust Transpilation with Few-Shot Learning](https://arxiv.org/abs/2404.18852)

### Miscellaneous
* [Elapsed time logger](https://rust.code-maven.com/elapsed-time-logger)

## Crate of the Week

This week's crate is [derive\_more](https://docs.rs/derive_more), a crate for deriving a whole lot of traits

Thanks to [teor](https://users.rust-lang.org/t/crate-of-the-week/2704/1306) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No calls for testing were issued this week.*

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* [RFC: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](https://github.com/rust-lang/rfcs/pull/3537)
  * [Testing steps](https://github.com/rust-lang/cargo/issues/13873)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* [CfT: Test out Rustup's `reqwest` backend with `rustls`](https://github.com/rust-lang/rustup/issues/3806)
  * [Testing steps](https://github.com/rust-lang/rustup/issues/3806#issue-2278962476)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [greptimedb - Add TLS support for GreptimeDB's gRPC service](https://github.com/GreptimeTeam/greptimedb/issues/3336)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [EuroRust 2024](https://www.papercall.io/eurorust-2024)| Closes 2024-06-03 | Vienna, Austria & online | Event date: 2024-10-10
* [Scientific Computing in Rust 2024](https://scientificcomputing.rs/)| Closes 2024-06-14 | online | Event date: 2024-07-17 - 2024-07-19
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Closes 2024-07-22 | online | Event date: 2024-08-22

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

426 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-04-30..2024-05-07

* [AST pretty: Use `builtin_syntax` for type ascription](https://github.com/rust-lang/rust/pull/124637)
* [`const_eval_select`: add tracking issue](https://github.com/rust-lang/rust/pull/124626)
* [`default_alloc_error_hook`: explain difference to default `__rdl_oom` in alloc](https://github.com/rust-lang/rust/pull/124059)
* [account for immutably borrowed locals in MIR copy-prop and GVN](https://github.com/rust-lang/rust/pull/123602)
* [add `normalize()` in run-make `Diff` type](https://github.com/rust-lang/rust/pull/124561)
* [add `rustfmt` cfg to well known cfgs list](https://github.com/rust-lang/rust/pull/124742)
* [add a lint against never type fallback affecting unsafe code](https://github.com/rust-lang/rust/pull/123939)
* [add support for inputing via stdin with run-make-support](https://github.com/rust-lang/rust/pull/124612)
* [adjust `#[macro_export]`/doctest help suggestion for `non_local_defs` lint](https://github.com/rust-lang/rust/pull/124568)
* [always print nice 'std not found' error when std is not found](https://github.com/rust-lang/rust/pull/124582)
* [borrowck: prepopulate opaque storage more eagerly](https://github.com/rust-lang/rust/pull/124809)
* [consider inner modules to be local in the `non_local_definitions` lint](https://github.com/rust-lang/rust/pull/124539)
* [deref patterns: impl `DerefPure` for more std types](https://github.com/rust-lang/rust/pull/123480)
* [don't consider candidates with no failing where clauses when refining obligation causes in new solver](https://github.com/rust-lang/rust/pull/124771)
* [enable `--check-cfg` by default in UI tests](https://github.com/rust-lang/rust/pull/124345)
* [enable reusing CI Docker cache when running CI images locally](https://github.com/rust-lang/rust/pull/124663)
* [fix `NormalizesTo` proof tree issue](https://github.com/rust-lang/rust/pull/124566)
* [fix unwinding on 32-bit watchOS ARM](https://github.com/rust-lang/rust/pull/124494)
* [fix unwinding on 32-bit watchOS ARM (v2)](https://github.com/rust-lang/rust/pull/124748)
* [generalize `adjust_from_tcx` for `Allocation`](https://github.com/rust-lang/rust/pull/124492)
* [implement `do_not_recommend` in the new solver](https://github.com/rust-lang/rust/pull/124717)
* [implement `ptr_as_ref_unchecked`](https://github.com/rust-lang/rust/pull/122492)
* [improve check-cfg CLI errors with more structured diagnostics](https://github.com/rust-lang/rust/pull/124679)
* [interpret, miri: uniform treatments of intrinsics/functions with and without return block](https://github.com/rust-lang/rust/pull/124715)
* [interpret: drop: always evaluate place](https://github.com/rust-lang/rust/pull/124720)
* [interpret: hide some reexports in rustdoc](https://github.com/rust-lang/rust/pull/124627)
* [lazily normalize inside trait ref during orphan check & consider ty params in rigid alias types to be uncovered](https://github.com/rust-lang/rust/pull/117164)
* [let miri and const eval execute intrinsics' fallback bodies](https://github.com/rust-lang/rust/pull/124293)
* [only consider ambiguous goals when finding best obligation for ambiguities](https://github.com/rust-lang/rust/pull/124690)
* [prefer lower vtable candidates in select in new solver](https://github.com/rust-lang/rust/pull/124724)
* [record impl args in the proof tree](https://github.com/rust-lang/rust/pull/124718)
* [record impl args in the proof tree in new solver](https://github.com/rust-lang/rust/pull/124759)
* [remove redundant union check in `KnownPanicsLint` const prop](https://github.com/rust-lang/rust/pull/124550)
* [rewrite select (in the new solver) to use a `ProofTreeVisitor`](https://github.com/rust-lang/rust/pull/124529)
* [shallow resolve in orphan check](https://github.com/rust-lang/rust/pull/124623)
* [split mcdc code to a sub module of coverageinfo](https://github.com/rust-lang/rust/pull/124399)
* [stop `llvm.expect`ing assert terminators](https://github.com/rust-lang/rust/pull/124606)
* [support `Result<T, E>` across FFI when niche optimization can be used](https://github.com/rust-lang/rust/pull/122253)
* [tweak `consts_may_unify`](https://github.com/rust-lang/rust/pull/124610)
* [use `ObligationCtxt` in favor of `TraitEngine` in many more places](https://github.com/rust-lang/rust/pull/124588)
* [use `super_fold` in `RegionsToStatic` visitor](https://github.com/rust-lang/rust/pull/124808)
* [use `tcx.types.unit` instead of `Ty::new_unit(tcx)`](https://github.com/rust-lang/rust/pull/124624)
* [use a proof tree visitor to refine the `Obligation` for error reporting in new solver](https://github.com/rust-lang/rust/pull/124418)
* [miri: /miri run: support -v flag to print what it is doing](https://github.com/rust-lang/miri/pull/3545)
* [miri: don’t print `Preparing a sysroot` when `-q`/`--quiet` is passed](https://github.com/rust-lang/miri/pull/3531)
* [miri: macos: use getentropy from libc](https://github.com/rust-lang/miri/pull/3551)
* [miri: make file descriptors into refcount references](https://github.com/rust-lang/miri/pull/3533)
* [miri: make many-seeds a mode of ./miri run rather than a separate command](https://github.com/rust-lang/miri/pull/3548)
* [miri: only show the 'basic API common for this target' message when this is a missing foreign function](https://github.com/rust-lang/miri/pull/3562)
* [miri: pthread shims: reorganize field offset handling, and add sanity checks](https://github.com/rust-lang/miri/pull/3564)
* [miri: solaris: make pre-main code work](https://github.com/rust-lang/miri/pull/3570)
* [miri: sync: better error in invalid synchronization primitive ID](https://github.com/rust-lang/miri/pull/3560)
* [miri: tls dtors: treat all unixes uniformly](https://github.com/rust-lang/miri/pull/3550)
* [miri: tree Borrows: first apply transition, then check protector with new 'initialized'](https://github.com/rust-lang/miri/pull/3532)
* [miri: unix/thread: properly use `pthread_t` for thread IDs](https://github.com/rust-lang/miri/pull/3568)
* [some hir cleanups](https://github.com/rust-lang/rust/pull/124401)
* [stabilize `exclusive_range_pattern`](https://github.com/rust-lang/rust/pull/124459)
* [stabilize `exclusive_range_pattern` (v2)](https://github.com/rust-lang/rust/pull/124749)
* [stabilize `split_at_checked`](https://github.com/rust-lang/rust/pull/124678)
* [improve several `Read` implementations](https://github.com/rust-lang/rust/pull/122441)
* [add constants for f16 and f128](https://github.com/rust-lang/rust/pull/123850)
* [compiler builtins for `f16`/`f128` float conversions](https://github.com/rust-lang/compiler-builtins/pull/593)
* [cargo lint: Warn not Error on unsupported lint tool](https://github.com/rust-lang/cargo/pull/13833)
* [cargo lints: Prevent inheritance from bring exposed for published packages](https://github.com/rust-lang/cargo/pull/13852)
* [cargo lints: Remove ability to specify `-` in lint name](https://github.com/rust-lang/cargo/pull/13837)
* [cargo resolver: Treat unset MSRV as compatible](https://github.com/rust-lang/cargo/pull/13791)
* [cargo toml: Don't lose 'public' when inheriting a dep](https://github.com/rust-lang/cargo/pull/13836)
* [cargo toml: On 2024 Edition, disallow ignored `default-features` when inheriting](https://github.com/rust-lang/cargo/pull/13839)
* [cargo toml: Remove unstable rejrected frontmatter syntax for cargo script](https://github.com/rust-lang/cargo/pull/13861)
* [cargo toml: Validate `crates_types/proc-macro` for bin like others](https://github.com/rust-lang/cargo/pull/13841)
* [cargo toml: Avoid inferring when targets are known](https://github.com/rust-lang/cargo/pull/13849)
* [cargo clean package perf improvements](https://github.com/rust-lang/cargo/pull/13818)
* [cargo: error when unstable lints are specified but not enabled](https://github.com/rust-lang/cargo/pull/13805)
* [cargo: populate git information when building Cargo from Rust's source tarball](https://github.com/rust-lang/cargo/pull/13832)
* [cargo: stabilize `-Zcheck-cfg` as always enabled](https://github.com/rust-lang/cargo/pull/13571)
* [cargo: workaround copying file returning EAGAIN on ZFS on mac OS](https://github.com/rust-lang/cargo/pull/13845)
* [rustdoc-search: search for references](https://github.com/rust-lang/rust/pull/124148)
* [clippy: allow more attributes in `clippy::useless_attribute`](https://github.com/rust-lang/rust-clippy/pull/12755)
* [clippy: don't lint `assigning_clones` on nested late init locals](https://github.com/rust-lang/rust-clippy/pull/12742)
* [clippy: don't suggest `Box::default()` in functions with differing generics](https://github.com/rust-lang/rust-clippy/pull/12687)
* [clippy: fix `FormatArgs` storage when `-Zthreads` \> 1](https://github.com/rust-lang/rust-clippy/pull/12567)
* [clippy: fix `for x in y unsafe { }`](https://github.com/rust-lang/rust-clippy/pull/12515)
* [clippy: fix suggestion error for `manual_is_ascii_check` with missing type](https://github.com/rust-lang/rust-clippy/pull/11988)
* [clippy: suggest collapsing nested or patterns if the MSRV allows it](https://github.com/rust-lang/rust-clippy/pull/12745)
* [clippy: type safe CLI implementation for clippy-dev](https://github.com/rust-lang/rust-clippy/pull/12747)
* [rust-analyzer: make generate function assist generate a function as a constructor if the generated function has the name "new" and is an asscociated function](https://github.com/rust-lang/rust-analyzer/pull/17138)
* [rust-analyzer: fix Run lens showing when lenses are disabled](https://github.com/rust-lang/rust-analyzer/pull/17177)
* [rust-analyzer: fix impl trait params not being counted properly](https://github.com/rust-lang/rust-analyzer/pull/17176)
* [rust-analyzer: correctly handle `no_core`/`no_std` for preludes](https://github.com/rust-lang/rust-analyzer/pull/17172)
* [rust-analyzer: discard path when the path is invalid](https://github.com/rust-lang/rust-analyzer/pull/17161)
* [rust-analyzer: fix implicit ty args being lowered where they shouldn't](https://github.com/rust-lang/rust-analyzer/pull/17175)
* [rust-analyzer: implement creating generics for impl traits in associated types](https://github.com/rust-lang/rust-analyzer/pull/17160)
* [rust-analyzer: lifetime's Bound Var Debrujin Index in Dyn Traits](https://github.com/rust-lang/rust-analyzer/pull/17190)

### Rust Compiler Performance Triage

Largely uneventful week; the most notable shifts were considered false-alarms
that arose from changes related to cfg-checking (either cargo enabling it, or
adding cfg's like `rustfmt` to the "well-known cfgs list").

Triage done by **@pnkfelix**.
Revision range: [c65b2dc9..69f53f5e](https://perf.rust-lang.org/?start=c65b2dc935c27c0c8c3997c6e8d8894718a2cb1a&end=69f53f5e5583381267298ac182eb02c7f1b5c1cd&absolute=false&stat=instructions%3Au)

3 Regressions, 2 Improvements, 3 Mixed; 5 of them in rollups
54 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/6774c877ace0a2d9138b2b06ef0aabf6c2317a43/triage/2024-05-06.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Merge RFC 3325: Unsafe attributes](https://github.com/rust-lang/rfcs/pull/3325)
* [Merge RFC 3593: Reserve unprefixed guarded strings](https://github.com/rust-lang/rfcs/pull/3593)
* [Merge RFC 3606: Shorter temp lifetimes in tail exprs](https://github.com/rust-lang/rfcs/pull/3606)
* [Merge RFC 3519: Arbitrary self types v2](https://github.com/rust-lang/rfcs/pull/3519)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [RFC: New range types for Edition 2024](https://github.com/rust-lang/rfcs/pull/3550)
* [disposition: merge] [RFC: cargo-script](https://github.com/rust-lang/rfcs/pull/3502)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [[ptr] Document maximum allocation size](https://github.com/rust-lang/rust/pull/116675)
* [disposition: merge] [Stabilize `min_exhaustive_patterns`](https://github.com/rust-lang/rust/pull/122792)
* [disposition: merge] [Fix #124275: Implemented Default for Arc<\str>](https://github.com/rust-lang/rust/pull/124367)
* [disposition: merge] [elaborate obligations in coherence](https://github.com/rust-lang/rust/pull/124532)
* [disposition: merge] [Tracking Issue for `AtomicBool::fetch_not`](https://github.com/rust-lang/rust/issues/98485)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Function body blocks](https://github.com/rust-lang/rfcs/pull/3629)
* [new] [`async T` and `gen T` types](https://github.com/rust-lang/rfcs/pull/3628)
* [new] [Match ergonomics 2024](https://github.com/rust-lang/rfcs/pull/3627)
* [new] [Extend format_args implicit arguments to allow field access](https://github.com/rust-lang/rfcs/pull/3626)
* [new] [Supertrait item shadowing v2](https://github.com/rust-lang/rfcs/pull/3624)
* [new] [RFC: #[derive(SmartPointer)]](https://github.com/rust-lang/rfcs/pull/3621)

## Upcoming Events

Rusty Events between 2024-05-08 - 2024-06-05 🦀

### Virtual

* 2024-05-09 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477697/)
* 2024-05-09 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Rust at Microsoft, Tel Aviv - Are we embedded yet?**](https://www.meetup.com/code-mavens/events/300144781/)
* 2024-05-09 | Virtual (Nuremberg/Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945257/)
* 2024-05-10 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Rust's Safety Net: Crafting Memory-Safe Applications**](https://www.eventbrite.com/e/rusts-safety-net-crafting-memory-safe-applications-tickets-881820838867)
* 2024-05-10 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events/)
    * [**#RustSemineri - 4**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-4-4c6c7276)
* 2024-05-12 | Virtual (Bangalore, IN) | [CoderRange](https://www.meetup.com/coderrange-endless-programming-languages/)
    * [**Rust Lang : Live Experts Discussion & Case Studies**](https://www.meetup.com/coderrange-endless-programming-languages/events/300692075/)
* 2024-05-13 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events/)
    * [**#RustSemineri - 5**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-5-119bbf68)
* 2024-05-14 | Virtual | [Rust for Lunch](https://lecture.senfcall.de/hay-gmh-wox-mru)
    * [**Rust for Lunch (May)**](https://lecture.senfcall.de/hay-gmh-wox-mru)
* 2024-05-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341699/)
* 2024-05-14 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/300437775/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-15 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events/)
    * [**#RustSemineri - 7**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-7-0a97e784)
* 2024-05-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 6 - Testing**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300819214/)
* 2024-05-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**NativeLink**](https://www.meetup.com/vancouver-rust/events/298542331/)
* 2024-05-16 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events)
    * [**#RustSemineri - 8**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-8-ddfe6b15)
* 2024-05-16 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298312423/)
* 2024-05-17 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Rust at Full Speed: Harnessing Concurrency with Confidence**](https://www.eventbrite.com/e/rust-at-full-speed-harnessing-concurrency-with-confidence-tickets-884842296127)
* 2024-05-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—forensic parsing via Artemis**](https://www.meetup.com/rustdc/events/299346490/)
* 2024-05-23 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477699/)
* 2024-05-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/300533392/)
* 2024-05-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298542326/)
* 2024-06-04 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191681/)

### Africa

* 2024-06-01 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia

* 2024-05-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2024-rustacean-meetup/)
* 2024-05-14 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Wrapping C++ in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/300879432/)

### Europe

* 2024-05-08 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/300573716/)
* 2024-05-09 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #2**](https://www.meetup.com/rust-gdansk/events/299766774/)
* 2024-05-14 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust Hack Night #5: Beginner-friendly**](https://www.meetup.com/copenhagen-rust-community/events/300898453/)
* 2024-05-14 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #7: Creative Coding & Data Visualization !**](https://www.meetup.com/meetup-group-zgphbyet/events/300776874/)
* 2024-05-14 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Hack & Learn May 2024**](https://www.meetup.com/rust-london-user-group/events/300715979/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-14 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (May 2024)**](https://www.meetup.com/rust-prague/events/300566374/)
* 2024-05-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/299694474/)
* 2024-05-16 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #7**](https://www.meetup.com/rust-meetup-augsburg/events/300174327/)
* 2024-05-16 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #68**](https://mobilizon.fr/events/14b51ccc-211f-400f-9615-707d9d871e78)
* 2024-05-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/300307155/)
* 2024-05-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Save the date - Mai Meetup**](https://www.meetup.com/rust-zurich/events/300513957/)
* 2024-05-22 | Leiden, NL | [Future-proof Software Development by FreshMinds](https://www.meetup.com/freshminds-future-proof-software-development/)
    * [**Coding Dojo Session**](https://www.meetup.com/freshminds-future-proof-software-development/events/300566391/)
* 2024-05-23 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #2**](https://www.meetup.com/rust-bern/events/300286917/)
* 2024-05-23 | Łodz, PL | [Mobica](https://www.linkedin.com/posts/mobica_rust-programming-embeddedsoftware-activity-7193232853717946369-CK68/)
    * [**Zapisz się na warsztat Rust / Embedded w Łodzi! / What's all the fuss about Rust?**](https://www.interankiety.pl/f/b4D7G7xO)
* 2024-05-24 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #3: Discussions**](https://www.meetup.com/bordeaux-rust/events/300723854/)
* 2024-05-28 - 2024-05-30 | Berlin, DE | [Oxidize](https://oxidizeconf.com/)
    * [**Oxidize Conf 2024**](https://oxidizeconf.com/)
* 2024-05-30 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**Rust for the web**](https://www.meetup.com/bcnrust/events/300765894/)
* 2024-05-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288963/)
* 2024-05-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #47 sponsored by Microsoft!**](https://www.meetup.com/copenhagen-rust-community/events/300458222/)
* 2024-05-30 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/300453310/)

### North America

* 2024-05-08 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Social - Ann Arbor**](https://www.meetup.com/detroitrust/events/300763859/)
* 2024-05-09 - 2024-05-10 | Estes Park, CO, US | [Hackathons Denver](https://www.meetup.com/hackathons-denver/)
    * [**Lambda Conf 2024: Rust Workshop by John A. De Goes**](https://www.meetup.com/hackathons-denver/events/300370255/)
* 2024-05-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Writing and deploying a simple multi-threaded web backend**](https://www.meetup.com/utah-rust/events/300782766/)
* 2024-05-09 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300020003/)
* 2024-05-12 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, May 12**](https://www.meetup.com/bostonrust/events/300116747/)
* 2024-05-14 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/300744140/)
* 2024-05-14 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Meetup**](https://www.meetup.com/rust-nyc/events/300818688/)
* 2024-05-16 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775539/)
* 2024-05-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509369/)
* 2024-05-20 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, May 20**](https://www.meetup.com/bostonrust/events/300116765/)
* 2024-05-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186931/)
* 2024-05-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygchbdc/)
* 2024-05-25 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Talk Double Feature**](https://www.meetup.com/deep-dish-rust/events/300665520/)
* 2024-05-30 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775547/)
* 2024-05-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch, May 31**](https://www.meetup.com/bostonrust/events/300116786/)
* 2024-06-05 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn June 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)

### Oceania

* 2024-05-28 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**a demo 🤯 & a lightning ⚡show ✨**](https://www.meetup.com/rust-sydney/events/300854266/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1cixuzr/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Rust and its borrow checker are like proper form when lifting boxes. While you might have been lifting boxes "the natural way" for decades without a problem, and its an initial embuggerance to think and perform proper lifting form, it is learnable, efficient, and prevents some important problems.
>
> Or more succinctly:  
> C/C++: It'll screw your back(end).

And the reply:

> 1. there’s a largish group of men who would feel their masculinity attacked if you implied they should learn it
> 2. while it's learnable finding usefully targeted educational resources are hard to come by
> 3. proper form while lifting boxes are a really terrible way to model graphs

– [Brett Witty and Leon on Mastodon](https://mstdn.social/@brettwitty/111734369720814683)

Thanks to [Brett Witty](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1566) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1cnn8dh/this_week_in_rust_546/)</small>
