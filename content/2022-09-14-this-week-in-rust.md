Title: This Week in Rust 460
Number: 460
Date: 2022-09-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
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
* [Security advisories for Cargo (CVE-2022-36113, CVE-2022-36114)](https://blog.rust-lang.org/2022/09/14/cargo-cves.html)

### Foundation
* [Rust Foundation Establishes Security Team to Support and Advance Rust Programming Language](https://foundation.rust-lang.org/news/2022-09-13-rust-foundation-establishes-security-team/)

### Newsletters
* [Rust Nigeria Issue #9](https://rustnigeria.curated.co/issues/9#start)

### Project/Tooling Updates
* [rust-analyzer changelog #146](https://rust-analyzer.github.io/thisweek/2022/09/12/changelog-146.html)
* [IntelliJ Rust Changelog #178](https://intellij-rust.github.io/2022/09/12/changelog-178.html)
* [A byte string library for Rust](https://blog.burntsushi.net/bstr/)
* [Pomsky 0.7 released](https://pomsky-lang.org/blog/pomsky-0.7-released/)
* [Slint weekly updates (The GUI framework)](https://slint-ui.com/thisweek/2022-09-12.html)
* [Fang 0.9 - new version of the background processing framework for rust](https://fang.badykov.com/blog/fang-09-release/)
* [Fornjot (code-first CAD in Rust) - Weekly Release - 2022-W37](https://www.fornjot.app/blog/weekly-release/2022-w37/)
* [This week in Databend #59: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-09-14-databend-weekly/)
* [HexoSynth 2022 - Devlog #12 - Documentation for me and you](https://m8geil.de/posts/hexosynth-12/)

### Observations/Thoughts
* [You Can't Do That: Abstracting over Ownership in Rust with Higher-Rank Type Bounds. Or Can You?](https://lucumr.pocoo.org/2022/9/11/abstracting-over-ownership/)
* [Security and Correctness in Wasmtime](https://bytecodealliance.org/articles/security-and-correctness-in-wasmtime)
* [Attacking Firecracker: AWS' microVM Monitor Written in Rust](https://www.graplsecurity.com/post/attacking-firecracker)
* [&stress about &Strings](https://cooscoos.github.io/blog/stress-about-strings/)
* [A pair of Rust kernel modules](https://lwn.net/SubscriberLink/907685/75fc924f5ec91869/)
* [GNU `ld` Discards Section Containing Code – Section Flags are Important for ELF Files](https://phip1611.de/blog/gnu-ld-discards-section-containing-code)
* [Use Rust to Reduce the Size of Your SQLite Database](https://www.i-programmer.info/programming/database/15686-use-rust-to-reduce-the-size-of-your-sqlite-database-.html) 
* [video] [Coroutines: C++ vs Rust - Jonathan Müller - C++ on Sea 2022](https://www.youtube.com/watch?v=yt-gueRNCTU)
* [video] [Rust on Rails (write code that never crashes)](https://www.youtube.com/watch?v=sbVxq7nNtgo)
* [video] [Let's Code Asteroids in Rust with a First-Time Bevy User](https://www.youtube.com/watch?v=QCys49c44PU)
* [video] [Linux Plumbers Conference 2022 - Rust MC](https://www.youtube.com/watch?v=Xw9pKeJ-4Bw)
* [video] [series] [Rust Day on Google Open Source Live](https://www.youtube.com/playlist?list=PLxNYxgaZ8Rsf8JEZufwRvsZqrRfeqFiAt)

### Rust Walkthroughs
* [Kernighan software tools in rust](https://dannas.name/2022/09/08/Kernighan-tools-in-rust)
* [Speeding up incremental Rust compilation with dynamic libraries](https://robert.kra.hn/posts/2022-09-09-speeding-up-incremental-rust-compilation-with-dylibs/)
* [Learning Rust by implementing a SHA-1 hash cracker](https://kerkour.com/learning-rust-sha1-hash-cracker)
* [Chat Blast! A TCP chat server in Rust](https://www.superperfundo.tech/articles/chat-blast)
* [Concurrency in RustDb](https://rustdb.wordpress.com/2022/09/03/concurrency-in-rustdb/)
* [Beginners guide to Solana NFTs in Rust.](https://medium.com/@jacob_62353/introduction-365296716979)
* [STM32F4 Embedded Rust at the HAL: DMA Controllers](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-hal-dma-controllers)

### Miscellaneous
* [FR] [Rejoignez la communauté Rust (et devenez un "rustacé")](https://news.gandi.net/fr/2022/09/rejoignez-la-communaute-rust-devenez-rustace/)
* [DE] [Moderne Spieleprogrammierung mit dem Entity Component System und der Engine Bevy](https://www.heise.de/hintergrund/Moderne-Spieleprogrammierung-mit-dem-Entity-Component-System-und-der-Engine-Bevy-7257412.html)
* [DE] [Programmieren mit Rust für den FreeBSD-Kernel](https://www.heise.de/news/Rust-fuer-den-FreeBSD-Kernel-7255609.html)

## Crate of the Week
This week's crate is [bstr](https://lib.rs/crates/bstr), a fast and featureful byte-string library.

Thanks to [8573](https://users.rust-lang.org/t/crate-of-the-week/2704/1103) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

<!-- note for editor: links included through here: https://users.rust-lang.org/t/twir-call-for-participation/4821/462 -->
Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Support TCP keepalive for TCP clients](https://github.com/build-trust/ockam/issues/3450)
* [Ockam - Show ockam command help in $PAGER or less (clap based)](https://github.com/build-trust/ockam/issues/3434)
* [Ockam - Implement ockam reset clap command](https://github.com/build-trust/ockam/issues/3454)
* [lib3mf - Help compiling the upstream C++ library on Windows](https://github.com/Michael-F-Bryan/lib3mf/issues/1)
* [Artichoke Ruby - Help migrate more path helpers out of its monolith into a support crate.](https://github.com/artichoke/artichoke/issues/2179)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

324 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-09-05..2022-09-12

* [initial implementation of return-position `impl Trait` in traits](https://github.com/rust-lang/rust/pull/101224)
* [check that the types in return position `impl Trait` in traits are well-formed](https://github.com/rust-lang/rust/pull/101676)
* [deny return-position `impl Trait` in traits for object safety](https://github.com/rust-lang/rust/pull/101681)
* [only encode return-position `impl Trait` in trait when parent function has a default body](https://github.com/rust-lang/rust/pull/101682)
* [implement `std::marker::Tuple`, a marker trait for tuples](https://github.com/rust-lang/rust/pull/100251)
* [add `inline-llvm` option for disabling/enabling LLVM inlining](https://github.com/rust-lang/rust/pull/100293)
* [stabilize raw-dylib for non-x86](https://github.com/rust-lang/rust/pull/99916)
* [equate fn outputs when inferring RPITIT hidden types](https://github.com/rust-lang/rust/pull/101614)
* [allow generators to `impl Clone`/`Copy`](https://github.com/rust-lang/rust/pull/101673)
* [avoid infinite loop in function arguments checking](https://github.com/rust-lang/rust/pull/100502)
* [`const_generics`: correctly deal with bound variables](https://github.com/rust-lang/rust/pull/98900)
* [fix LLVM IR type mismatch](https://github.com/rust-lang/rust/pull/101647)
* [fix ICE in `opt_suggest_box_span`](https://github.com/rust-lang/rust/pull/101604)
* [fix ICE report flags display](https://github.com/rust-lang/rust/pull/101595)
* [fix ICE, generalize 'move generics to trait' suggestion for >0 non-rcvr arguments](https://github.com/rust-lang/rust/pull/101427)
* [fix RPIT ICE for implicit HRTB when missing dyn](https://github.com/rust-lang/rust/pull/101468)
* [fix code generation of `Rvalue::Repeat` with 128 bit values](https://github.com/rust-lang/rust/pull/101612)
* [fix compile errors for uwp-windows-msvc targets](https://github.com/rust-lang/rust/pull/101481)
* [normalize before erasing late-bound regions in `equal_up_to_regions`](https://github.com/rust-lang/rust/pull/101437)
* [recover from using `;` as separator between fields](https://github.com/rust-lang/rust/pull/101457)
* [fix the suggestion of format for `asm_sub_register`](https://github.com/rust-lang/rust/pull/101253)
* [adjust and slightly generalize operator error suggestion](https://github.com/rust-lang/rust/pull/101424)
* [add list of recognized repr attributes to the unrecognized repr error](https://github.com/rust-lang/rust/pull/101486)
* [shrink span for bindings with subpatterns](https://github.com/rust-lang/rust/pull/101399)
* [point at type parameter in plain path expr](https://github.com/rust-lang/rust/pull/101425)
* [point out when a callable is not actually callable because its return is not sized](https://github.com/rust-lang/rust/pull/101359)
* [allow `lower_lifetime_binder` receive a closure](https://github.com/rust-lang/rust/pull/101496)
* [do not suggest a semicolon for a macro without `!`](https://github.com/rust-lang/rust/pull/101502)
* [include enum path in variant suggestion](https://github.com/rust-lang/rust/pull/101357)
* [suggest adding array lengths to references to arrays if possible](https://github.com/rust-lang/rust/pull/101492)
* [suggest introducing an explicit lifetime if it does not exist](https://github.com/rust-lang/rust/pull/101445)
* [suggest pub instead of public for const type item](https://github.com/rust-lang/rust/pull/101668)
* [suggest removing unnecessary prefix let in patterns](https://github.com/rust-lang/rust/pull/101362)
* [migrate another part of `rustc_infer` to session diagnostic](https://github.com/rust-lang/rust/pull/101153)
* [migrate `rustc_middle` diagnostic](https://github.com/rust-lang/rust/pull/101021)
* [migrate `rustc_session` to use SessionDiagnostic - Pt. 2](https://github.com/rust-lang/rust/pull/101041)
* [miri: add a Machine hook for inline assembly](https://github.com/rust-lang/rust/pull/101402)
* [shrink `PredicateS`](https://github.com/rust-lang/rust/pull/101432)
* [shrink `hir::Ty` and `hir::Pat`](https://github.com/rust-lang/rust/pull/101467)
* [parameterize `ty::Visibility` over used ID](https://github.com/rust-lang/rust/pull/101498)
* [allow lint passes to be bound by `TyCtxt`](https://github.com/rust-lang/rust/pull/101501)
* [track PGO profiles in depinfo](https://github.com/rust-lang/rust/pull/100801)
* [use `RelocModel::Pic` for UEFI targets](https://github.com/rust-lang/rust/pull/101413)
* [use niche-filling optimization even when multiple variants have data](https://github.com/rust-lang/rust/pull/94075)
* [inline `<T as From<T>>::from`](https://github.com/rust-lang/rust/pull/100733)
* [lower the `assume` intrinsic to a MIR statement](https://github.com/rust-lang/rust/pull/98332)
* [compile `spin_loop_hint` as pause on x86 even without sse2 enabled](https://github.com/rust-lang/rust/pull/101495)
* [reimplement `carrying_add` and `borrowing_sub` for signed integers](https://github.com/rust-lang/rust/pull/93873)
* [optimize thread parking on NetBSD](https://github.com/rust-lang/rust/pull/101482)
* [remove `&[T]` from `vec_deque::Drain`](https://github.com/rust-lang/rust/pull/101299)
* [the `<*const T>::guaranteed_*` methods now return an option for the unknown case](https://github.com/rust-lang/rust/pull/101483)
* [use futex-based locks and thread parker on Hermit](https://github.com/rust-lang/rust/pull/101475)
* [hashbrown: add `HashSet::raw_table`](https://github.com/rust-lang/hashbrown/pull/358)
* [hashbrown: add `RawTable::is_full`](https://github.com/rust-lang/hashbrown/pull/354)
* [git2: implement `IntoIterator` for `Statuses`](https://github.com/rust-lang/git2-rs/pull/880)
* [codegen\_gcc: simd: impl `extract_element` for vector types](https://github.com/rust-lang/rustc_codegen_gcc/pull/215)
* [cargo: specify the max length for crate name](https://github.com/rust-lang/cargo/pull/11051)
* [rustdoc: avoid cleaning modules with duplicate names](https://github.com/rust-lang/rust/pull/101631)
* [rustdoc: correcty handle intra-doc-links to items without HTML page](https://github.com/rust-lang/rust/pull/101633)
* [rustdoc: more accurate struct type](https://github.com/rust-lang/rust/pull/101521)
* [rustdoc: store Variant Fields as their own item](https://github.com/rust-lang/rust/pull/101462)
* [clippy: do not expand macro in `nonminimal_bool` suggestions](https://github.com/rust-lang/rust-clippy/pull/9457)
* [clippy: don't lint `large_stack_array` inside static items](https://github.com/rust-lang/rust-clippy/pull/9466)
* [clippy: don't panic on invalid shift while constfolding](https://github.com/rust-lang/rust-clippy/pull/9464)
* [clippy: fix `FormatArgsExpn` parsing of `FormatSpec` positions](https://github.com/rust-lang/rust-clippy/pull/9469)
* [clippy: fix `range_{plus,minus}_one` bad suggestions](https://github.com/rust-lang/rust-clippy/pull/9446)
* [clippy: fix hang in `vec_init_then_push`](https://github.com/rust-lang/rust-clippy/pull/9441)
* [clippy: rename the `arithmetic` lint](https://github.com/rust-lang/rust-clippy/pull/9443)
* [clippy: suggest `unwrap_or_default` when closure returns `"".to_string`](https://github.com/rust-lang/rust-clippy/pull/9421)
* [clippy: use `visit_expr_field` for `ParamPosition`](https://github.com/rust-lang/rust-clippy/pull/9458)
* [clippy: use macro callsite when creating `Sugg` helper](https://github.com/rust-lang/rust-clippy/pull/9410)
* [clippy: make `Arithmetic` consider literals](https://github.com/rust-lang/rust-clippy/pull/9365)
* [clippy: `assertions_on_result_states`: fix suggestion when `assert!` is not in a statement](https://github.com/rust-lang/rust-clippy/pull/9453)
* [rust-analyzer: add config to unconditionally prefer core imports over std](https://github.com/rust-lang/rust-analyzer/pull/13212)
* [rust-analyzer: build release artifact against older glibc](https://github.com/rust-lang/rust-analyzer/pull/13214)
* [rust-analyzer: filter imports on find-all-references](https://github.com/rust-lang/rust-analyzer/pull/13186)
* [rust-analyzer: new assist: `move_format_string_arg`](https://github.com/rust-lang/rust-analyzer/pull/13216)
* [rust-analyzer: remove the `toggleInlayHints` command from VSCode](https://github.com/rust-lang/rust-analyzer/pull/13215)
* [rust-analyzer: use `proc-macro-srv` from sysroot in rust-project.json workspaces](https://github.com/rust-lang/rust-analyzer/pull/13200)
* [rust-analyzer: make clicking a closing brace inlay hint go to the opening brace](https://github.com/rust-lang/rust-analyzer/pull/13158)
* [rust-analyzer: add semicolon completion to mod](https://github.com/rust-lang/rust-analyzer/pull/13207)
* [rust-analyzer: handle lifetime variables in projection normalization](https://github.com/rust-lang/rust-analyzer/pull/13223)
* [rust-analyzer: handle trait methods as inherent methods for trait-related types](https://github.com/rust-lang/rust-analyzer/pull/13147)

### Rust Compiler Performance Triage

From the viewpoint of metrics gathering, this was an absolutely terrible week,
because the vast majority of this week's report is dominated by noise. Several
benchmarks (html5ever, cranelift-codegen, and keccak) have all been exhibiting
bimodal behavior where their compile-times would regress and improve randomly
from run to run. Looking past that, we had one small win from adding an inline
directive.

Triage done by **@pnkfelix**.
Revision range: [e7cdd4c0..17cbdfd0](https://perf.rust-lang.org/?start=e7cdd4c0909b62f2ee0368fd10e6e244f2af44b4&end=17cbdfd07178349d0a3cecb8e7dde8f915666ced&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u) | mean | range | count |
|:----------------:|:----:|:-----:|:-----:|
| Regressions ❌ <br /> (primary) | 1.1% | [0.2%, 6.2%] | 26    |
| Regressions ❌ <br /> (secondary) | 1.9% | [0.1%, 5.6%] | 34    |
| Improvements ✅ <br /> (primary) | -1.8% | [-29.4%, -0.2%] | 42    |
| Improvements ✅ <br /> (secondary) | -1.3% | [-5.3%, -0.2%] | 50    |
| All ❌✅ (primary) | -0.7% | [-29.4%, 6.2%] | 68    |


11 Regressions, 11 Improvements, 13 Mixed; 11 of them in rollups
71 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-09-13.md)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [De-RFC: Remove type ascription](https://github.com/rust-lang/rfcs/pull/3307)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Rust Style Team](https://github.com/rust-lang/rfcs/pull/3309)
* [RFC: Statics in patterns](https://github.com/rust-lang/rfcs/pull/3305)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: close] [Loosen shadowing check inside macro contexts (attempt 2).](https://github.com/rust-lang/rust/pull/100453)
* [disposition: merge] [Tracking issue for `std::hint::black_box`](https://github.com/rust-lang/rust/issues/64102)
* [disposition: merge] [Commit to safety rules for dyn trait upcasting](https://github.com/rust-lang/rust/issues/101336)
* [disposition: merge] [Tracking Issue for constifying some `{BTreeMap,Set}` functions](https://github.com/rust-lang/rust/issues/71835)
* [disposition: merge] [Tracking Issue for `Option::unzip()`](https://github.com/rust-lang/rust/issues/87800)
* [disposition: merge] [Tracking issue for map_first_last: first/last methods on BTreeSet and BTreeMap](https://github.com/rust-lang/rust/issues/62924)
* [disposition: merge] [Make `Sized` coinductive, again](https://github.com/rust-lang/rust/pull/100386)
* [disposition: merge] [Neither require nor imply lifetime bounds on opaque type for well formedness](https://github.com/rust-lang/rust/pull/95474)
* [disposition: merge] [Make typeck aware of uninhabited types](https://github.com/rust-lang/rust/pull/100288)
* [disposition: merge] [Stabilize `let else`](https://github.com/rust-lang/rust/pull/93628)
* [disposition: merge] [Fix `#[derive(Default)]` on a generic `#[default]` enum adding unnecessary `Default` bounds](https://github.com/rust-lang/rust/pull/101040)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2022-09-14 - 2022-10-12 🦀

### Virtual

* 2022-09-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcmbsb/)
* 2022-09-14 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Introduction to Async in Rust**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/288154493/)
* 2022-09-14 | Virtual (Malaysia)| [Golang Malaysia](https://docs.google.com/forms/d/e/1FAIpQLScKGolWclIOR1OBCzTOitVU5Am5lSYxSlVhK71DGsc-fa-Yhg/viewform)
    * [**Rust Meetup September 2022**](https://discord.gg/9Xj8H2EXTD)
* 2022-09-15 | Virtual (Columbus, OH, US) | [GDG Columbus](https://www.meetup.com/gdg-columbus/)
    * [**Past, Present, and Future of Internet Money! (Custom tokenomics, RUST and CosmWASM library...)**](https://www.meetup.com/gdg-columbus/events/287972746/)
* 2022-09-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcmbtb/)
* 2022-09-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful: Bencher—Catch Performance Regressions in CI—Everett Pompeii**](https://www.meetup.com/rustdc/events/287004599/)
* 2022-09-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out (Call for Participation)**](https://www.meetup.com/vancouver-rust/events/285933975/)
* 2022-09-22 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Rust based Bluetooth tools (BlueR) you can use today**](https://www.meetup.com/charlottesville-rust-meetup/events/288123436/)
* 2022-09-22 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Rust Iran Meetup #9 - Let's Write An Async Executor**](https://rust-meetup.ir/2022/09/22/9th-meetup.html)
* 2022-09-23 | Virtual (Tokyo, JP) | [Rust Tokyo](https://rust.tokyo)
    * [**Rust Tokyo 2022**](https://rust.tokyo/2022)
* 2022-09-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcmbkc/)
* 2022-09-28 | Virtual (London, UK) | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-10-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcnbgb/)
* 2022-10-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcnbhb/)
* 2022-10-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcnbhb/)
* 2022-10-06 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #18**](https://www.meetup.com/rust-noris/events/hlvbvsydcnbrb/)
* 2022-10-08 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-10-12 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcnbqb/)
* 2022-10-12 | Virtual (San Francisco, CA, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Getting Started with Rust: Building Rust Projects**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475796/)

### Europe

* 2022-09-15 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #52**](https://www.meetup.com/rust-paris/events/288136736/)
* 2022-09-27 | Nijmegen, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Rust at RU**](https://www.meetup.com/rust-nederland/events/288376119/)
    * [**Student track: Rust at RU**](https://www.meetup.com/rust-nederland/events/288440591/)
* 2022-09-28 | London, UK + Virtual | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-09-29 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/288266277/)
* 2022-09-29 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #29**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179100/)
* 2022-09-29 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Going full stack on Rust**](https://www.meetup.com/dutch-rust-meetup/events/286727064/)
* 2022-10-02 | Florence, IT + Virtual | [RustLab](https://rustlab.it/)
    * [**RustLab Conference 2022 (Oct 2-4)**](https://rustlab.it/schedule/)
* 2022-10-03 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Microsoft Reactor**](https://www.meetup.com/Stockholm-Rust/events/288453360/)
* 2022-10-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - EuroRust B-Sides**](https://www.meetup.com/rust-berlin/events/288175448/)

### North America

* 2022-09-14 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/288287206/)
* 2022-09-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcmbbc/)
* 2022-09-22 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Game Prototyping with Rusty Engine with Nathan Stocks and Food!**](https://www.meetup.com/utah-rust/events/rvpgxsydcmbmc/)
* 2022-09-29 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Nuestra RustMX comunidad tiene página**](https://www.meetup.com/rust-mx/events/288388973/)

### Oceania

* 2022-09-14 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust-Sydney Lightning Talks**](https://www.meetup.com/rust-sydney/events/287979855/)
* 2022-09-20 | Phillip, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/288450836/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/wm0k6q/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> In Rust We Trust

– [Alexander Sidorov on Medium](https://medium.com/@siberianguy/an-almost-religious-case-for-rust-e4c4764acd8d)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1287) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/xenssy/this_week_in_rust_460/)</small>
