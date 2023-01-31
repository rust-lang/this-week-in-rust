Title: This Week in Rust 479
Number: 479
Date: 2023-01-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
* [Officially announcing the types team](https://blog.rust-lang.org/2023/01/20/types-announcement.html)
* [Diversifying our Content Delivery Networks](https://blog.rust-lang.org/inside-rust/2023/01/24/content-delivery-networks.html)

### Foundation
* [Lars Bergstrom Elected as Rust Foundation Board of Directors Chair](https://foundation.rust-lang.org/news/lars-bergstrom-elected-as-rust-foundation-board-of-directors-chair/)
* [Join the Rust Foundation at Rust Nation UK 2023](https://foundation.rust-lang.org/news/join-the-rust-foundation-at-rust-nation-uk-2023/)

### Newsletters

### Project/Tooling Updates
* [rust-analyzer changelog #165](https://rust-analyzer.github.io/thisweek/2023/01/23/changelog-165.html)
* [hyper-ish 2022 in review](https://seanmonstar.com/post/706802392260362240/hyper-ish-2022-in-review)
* [Mobc 0.8.1 release with improved stability](https://www.garrensmith.com/new-mobc-release-0-8-1/)
* [Zenoh 0.7.0, a pure Rust Pub/Sub/Query protocol for cloud-to-thing continuum, was released and it is packed with new features.](https://zenoh.io/blog/2023-01-10-zenoh-charmander/)
* [Fornjot (code-first CAD in Rust) - Weekly Release](https://www.fornjot.app/blog/weekly-release/2023-w04/)
* [Slint 0.3.4 release](https://slint-ui.com/blog/slint-0.3.4-released.html)
* [Astra: A Blocking HTTP Server Built on Top of Hyper](https://ibraheem.ca/posts/announcing-astra/)
* [First steps with NGenate - A dataflow and visual programming platform built with rust](https://codetrance.io/blog/1/rust-visual-programming-dataflow-ngenate-first-steps/)
* [`toml` vs `toml_edit`](https://epage.github.io/blog/2023/01/toml-vs-toml-edit/)
* [This Week in Fyrox #11](https://fyrox.rs/blog/post/twif11/)
* [The year 2022 in Dimforge and our objectives for 2023](https://dimforge.com/blog/2023/01/22/the-year-2022-in-dimforge/)

### Observations/Thoughts
* [Rust in 2023: Growing up](https://smallcultfollowing.com/babysteps/blog/2023/01/20/rust-in-2023-growing-up/)
* [The State of Developer Ecosystem 2022 in Rust: Discover recent trends](https://blog.jetbrains.com/rust/2023/01/18/rust-deveco-2022-discover-recent-trends/)
* [The size of Rust Futures](https://swatinem.de/blog/future-size/)
* [Capability-Safety I: Prelude](https://blog.yoshuawuyts.com/capabilities-1/)
* [Surprises in the Rust JSON Ecosystem](https://ecton.dev/rust-json-ecosystem/)
* [The Git source code audit, viewed as a Rust programmer](https://litchipi.github.io/infosec/2023/01/24/git-code-audit-viewed-as-rust-programmer.html)
* [Turning a Rust struct into an enum is not always a major breaking change](https://predr.ag/blog/turning-rust-struct-to-enum-is-not-always-breaking/)
* [14 Rust Tools for Linux Terminal Dwellers](https://itsfoss.com/rust-cli-tools/)
* [audio] [Rust Magazine with Shuang Zhu](https://rustacean-station.org/episode/shuang-zhu/)
* [audio] [Rust Nation with Ernest Kissiedu](https://rustacean-station.org/episode/ernest-kissiedu/)

### Rust Walkthroughs
* [Temporary Values, Borrowing, and Lifetimes](https://blog.vashishtha.in/temporary-lifetimes/)
* [Due to limitations in the borrow checker, this implies a 'static lifetime](https://kazlauskas.me/entries/due-to-borrowck-limitations)
* [Rust concepts I wish I learned earlier](https://rauljordan.com/rust-concepts-i-wish-i-learned-earlier/)
* [Comparative fuzzing in Rust](https://medium.com/@adetaylor/comparative-fuzzing-parallel-rust-tools-fac5ce9c9c2d)
* [domain-specific error macros](https://blog.yoshuawuyts.com/ensure-macros/)
* [Building a Simple DB in Rust - Part 2 - Basic Execution](https://johns.codes/blog/build-a-db/part02)
* [Rust FFI and cbindgen: Integrating Embedded Rust Code in C](https://apollolabsblog.hashnode.dev/rust-ffi-and-cbindgen-integrating-embedded-rust-code-in-c)

### Research

### Miscellaneous
* [The crates.io registry is now a GitHub secret scanning integrator](https://github.blog/changelog/2023-01-19-the-crate-io-registry-is-now-a-github-secret-scanning-integrator/)
* [Six fun things to do with Rust operator overloading](https://wishawa.github.io/posts/fun-rust-operators/)
* [Packaging Rust Applications for the NPM Registry](https://blog.orhun.dev/packaging-rust-for-npm/)
* [Announcing Rust Support in CodeSandbox](https://codesandbox.io/blog/announcing-rust-support-in-codesandbox)
* [video] [10 Reasons Not To Use Rust (The Whole Truth)](https://www.youtube.com/watch?v=ul9vyWuT8SU)
* [video] [Sneaking By The Rust Borrow Checker - Interior Mutability](https://www.youtube.com/watch?v=HwupNf9iCJk)

## Crate of the Week

This week's crate is [Darkbird](https://github.com/Rustixir/darkbird), a mnesia-inspired high concurrency, real time, in-memory storage library.

Thanks to [DanyalMh](https://users.rust-lang.org/t/crate-of-the-week/2704/1149) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

* [Ockam - Implement 'ockam node logs' CLI command](https://github.com/build-trust/ockam/issues/4129)
* [Ockam - Implement 'ockam worker list' CLI command](https://github.com/build-trust/ockam/issues/4130)
* [Ockam - Add a CI check to avoid conflicts in 'TypeTag' ids](https://github.com/build-trust/ockam/issues/4108)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

378 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-01-16..2023-01-23

* [llvm-wrapper: adapt for LLVM API change](https://github.com/rust-lang/rust/pull/106113)
* [enable sanitizers for s390x-linux](https://github.com/rust-lang/rust/pull/107127)
* [put `noundef` on all scalars that don't allow uninit](https://github.com/rust-lang/rust/pull/106294)
* [add 'static lifetime suggestion when GAT implied 'static requirement from HRTB](https://github.com/rust-lang/rust/pull/106747)
* [add raw identifier for keyword in suggestion](https://github.com/rust-lang/rust/pull/106928)
* [check ADT fields for copy implementations considering regions](https://github.com/rust-lang/rust/pull/105102)
* [constify `TypeId` ordering impls](https://github.com/rust-lang/rust/pull/101698)
* [diagnostics: suggest changing `s@self::{macro}@::macro` for exported](https://github.com/rust-lang/rust/pull/104347)
* [dont randomly use `_` to print out const generic arguments](https://github.com/rust-lang/rust/pull/106873)
* [drop tracking Visit break expressions](https://github.com/rust-lang/rust/pull/106699)
* [encode const mir for closures if they're const](https://github.com/rust-lang/rust/pull/106917)
* [fix check macro expansion](https://github.com/rust-lang/rust/pull/107124)
* [label closure captures/generator locals that make opaque types recursive](https://github.com/rust-lang/rust/pull/106578)
* [lazy dominator tree construction in borrowck](https://github.com/rust-lang/rust/pull/106976)
* [make `CastError::NeedsDeref` create a `MachineApplicable` suggestion](https://github.com/rust-lang/rust/pull/106927)
* [make error emitted on `impl &Trait` nicer](https://github.com/rust-lang/rust/pull/106712)
* [refactor basic blocks control flow caches](https://github.com/rust-lang/rust/pull/106975)
* [simplify `derive(Debug)` output for fieldless enums](https://github.com/rust-lang/rust/pull/106884)
* [suggest remove deref for type mismatch](https://github.com/rust-lang/rust/pull/107203)
* [suggestion for attempted integer identifier in patterns](https://github.com/rust-lang/rust/pull/106591)
* [tweak "borrow closure argument" suggestion](https://github.com/rust-lang/rust/pull/106891)
* [unify stable and unstable sort implementations in same core module](https://github.com/rust-lang/rust/pull/104672)
* [use UnordMap and UnordSet for id collections (DefIdMap, LocalDefIdMap, etc)](https://github.com/rust-lang/rust/pull/106977)
* [various cleanups around pre-TyCtxt queries and functions](https://github.com/rust-lang/rust/pull/106810)
* [add heapsort fallback in `select_nth_unstable`](https://github.com/rust-lang/rust/pull/106997)
* [implement `alloc::vec::IsZero` for `Option<$NUM>` types](https://github.com/rust-lang/rust/pull/106989)
* [lift `T: Sized` bounds from some `strict_provenance` pointer methods](https://github.com/rust-lang/rust/pull/103702)
* [add `Arc::into_inner` for safely discarding `Arc`s without calling the destructor on the inner type](https://github.com/rust-lang/rust/pull/106854)
* [hashbrown: provide default hasher types to `Vacant` and `Occupied` entries](https://github.com/rust-lang/hashbrown/pull/389)
* [futures: add `Either::as_pin_mut` and `Either::as_pin_ref`](https://github.com/rust-lang/futures-rs/pull/2691)
* [futures: implement `FusedStream` for all streams in `ReadyChunks`](https://github.com/rust-lang/futures-rs/pull/2693)
* [(cherry-pick) WebAssembly multivalue stackify fix](https://github.com/rust-lang/llvm-project/pull/144)
* [cargo: stabilize sparse-registry](https://github.com/rust-lang/cargo/pull/11224)
* [cargo: wrapper type for data that should never be logged](https://github.com/rust-lang/cargo/pull/11545)
* [rustfmt: correct span for structs with const generics](https://github.com/rust-lang/rustfmt/pull/5669)
* [clippy: add `multiple_unsafe_ops_per_block` lint](https://github.com/rust-lang/rust-clippy/pull/10206)
* [clippy: add machine applicable suggestion for `bool_assert_comparison`](https://github.com/rust-lang/rust-clippy/pull/10218)
* [clippy: fix false positive in `unnecessary_safety_comment`](https://github.com/rust-lang/rust-clippy/pull/10106)
* [clippy: fix suggestion in `transmutes_expressible_as_ptr_casts` when the source type is a borrow](https://github.com/rust-lang/rust-clippy/pull/10193)
* [rust-analyzer: don't escape non-snippets in assist](https://github.com/rust-lang/rust-analyzer/pull/14004)
* [rust-analyzer: don't respond with a ContentModified while loading the workspace](https://github.com/rust-lang/rust-analyzer/pull/13985)
* [rust-analyzer: fix checkOnSave to check config patching not always working](https://github.com/rust-lang/rust-analyzer/pull/13980)
* [rust-analyzer: fix markdown removal in hover handling whitespace weirdly](https://github.com/rust-lang/rust-analyzer/pull/13988)
* [rust-analyzer: handle slice patterns in "Fill match arms"](https://github.com/rust-lang/rust-analyzer/pull/13978)
* [rust-analyzer: more precise binop inference](https://github.com/rust-lang/rust-analyzer/pull/13971)
* [rust-analyzer: substitute vscode variables in `config.serverPath`](https://github.com/rust-lang/rust-analyzer/pull/13993)
* [rust-analyzer: parse `const_closures` syntax](https://github.com/rust-lang/rust-analyzer/pull/13983)
* [rust-analyzer: replace SmolStr usage with lang item enum for lang items](https://github.com/rust-lang/rust-analyzer/pull/14001)
* [rust-analyzer: use workspace.dependencies to declare local dependencies](https://github.com/rust-lang/rust-analyzer/pull/13969)

### Rust Compiler Performance Triage

Largely a win for compiler performance with 100 test cases in real-world crates showing some sort of change in performance with an average 1% improvement. These wins were a combination of many different changes including how `doc(hidden)` gets more efficiently encoded in metadata, some optimizations in the borrow checker, and simplification of the output from `derive(Debug)` for fieldless enums.

Triage done by **@rylev**.
Revision range: [1f72129f..c8e6a9e8](https://perf.rust-lang.org/?start=1f72129ffe5e8c495113f9a2d4e1730f7fad3209&end=c8e6a9e8b6251bbc8276cb78cabe1998deecbed7&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.2%, 0.7%]    | 19    |
| Regressions ❌ <br /> (secondary)  | 0.9%  | [0.2%, 1.5%]    | 34    |
| Improvements ✅ <br /> (primary)   | -1.3% | [-17.2%, -0.2%] | 81    |
| Improvements ✅ <br /> (secondary) | -2.1% | [-7.1%, -0.2%]  | 64    |
| All ❌✅ (primary)                 | -1.0% | [-17.2%, 0.7%]  | 100   |


2 Regressions, 5 Improvements, 3 Mixed; 1 of them in rollups
34 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-01-24.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Autotrait bounds on dyn-safe trait methods](https://github.com/rust-lang/rust/pull/107082)
* [disposition: close] [Stabilize `ControlFlow::{BREAK, CONTINUE}`](https://github.com/rust-lang/rust/pull/102697)
* [disposition: merge] [Add missing normalization for union fields types](https://github.com/rust-lang/rust/pull/106938)
* [disposition: merge] [rustdoc: change trait bound formatting](https://github.com/rust-lang/rust/pull/102842)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Cargo target features](https://github.com/rust-lang/rfcs/pull/3374)
* [new] [Avoid non-local definitions in functions](https://github.com/rust-lang/rfcs/pull/3373)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-01-25 - 2023-02-22 🦀

### Virtual

* 2023-01-25 | Virtual (Redmond, WA, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224544/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224540/)
* 2023-01-26 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust Lightning Talks!**](https://www.meetup.com/charlottesville-rust-meetup/events/290122935/)
* 2023-01-26 | Virtual (Karlsruhe, DE) | [The Karlsruhe Functional Programmers Meetup Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA)**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/290406675/)
* 2023-01-26 | Virtual (Redmond, WA, US; San Francisco, CA, US; New York, NY, US; Stockholm, SE) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor Stockholm](https://www.meetup.com/microsoft-reactor-stockholm/)
    * [**Crack code interview problems in Rust - Ep. 3**](https://www.meetup.com/microsoft-reactor-redmond/events/290086420/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290086421/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290086422/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/290086418/)
* 2023-01-27 | Virtual (Tunis, TN) | [Rust Meetup Tunisia](https://www.meetup.com/rust-tunisia/)
    * [**Rust Meetup Tunisia - Volume I, Number I**](https://www.meetup.com/rust-tunisia/events/291103663/)
* 2023-01-30 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Control de errores en Rust**](https://www.meetup.com/microsoft-reactor-redmond/events/290224559/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224558/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224561/)
* 2023-01-31 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/290697014/)
* 2023-01-31 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfccbpc/)
* 2023-01-31 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Compresión de cómo Rust administra la memoria**](https://www.meetup.com/microsoft-reactor-redmond/events/290224861/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224860/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224859/)
* 2023-02-01 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**New Year Virtual Social + Share**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291184911/)
* 2023-02-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfcdbcb/)
* 2023-02-01 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224570/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224566/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224570/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224568/)
* 2023-02-01 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcdbcb/)
* 2023-02-06 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224572/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Implementación de tipos y rasgos genéricos**](https://www.meetup.com/microsoft-reactor-redmond/events/290224576/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224572/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224573/)
* 2023-02-07 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/290609896/)
* 2023-02-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcdbkb/)
* 2023-02-07 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Módulos, paquetes y contenedores de terceros**](https://www.meetup.com/microsoft-reactor-redmond/events/290224578/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224577/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224579/)
* 2023-02-08 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224584/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224583/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224585/)
* 2023-02-11 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-02-13 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Escritura de pruebas automatizadas**](https://www.meetup.com/microsoft-reactor-redmond/events/290224610/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224608/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224609/)
* 2023-02-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfcdbsb/)
* 2023-02-14 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Creamos un programa de ToDos en la línea de comandos**](https://www.meetup.com/microsoft-reactor-redmond/events/290224616/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224613/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224617/)
* 2023-02-14 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 26u16**](https://www.meetup.com/rust-saar/events/290040138/)
* 2023-02-15 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US; São Paulo, BR) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224621/) | [**São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224623/)
* 2023-02-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsyfcdbtb/)

### Asia

* 2023-02-01 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust)
    * [**Rust talk: How to implement Iterator on tuples... kind of**](https://www.meetup.com/kansai-rust/events/291020672)

### Europe

* 2023-01-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #55**](https://www.meetup.com/rust-paris/events/290100223/)
* 2023-01-26 | Copenhagen, Dk | [Copenhagen Rust Meetup Group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #32**](https://www.meetup.com/copenhagen-rust-meetup-group/events/290037532/)
* 2023-02-02 | Berlin, DE | [Prenzlauer Berg Software Engineers](https://www.meetup.com/prenzlauer-berg-software-engineers/)
    * [**PBerg engineers - inaugural meetup; Lukas: Serverless APIs using Rust and Azure functions (Fee)**](https://www.meetup.com/prenzlauer-berg-software-engineers/events/290513923/)
* 2023-02-02 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn February 2023**](https://www.meetup.com/rust-meetup-hamburg/events/290824576/)
* 2023-02-02 | Lyon, FR | [Rust Lyon](https://mobilizon.fr/events/b8577678-d072-4d9a-9562-974715f1dfbb)
    * [**Rust Lyon meetup #01**](https://mobilizon.fr/events/b8577678-d072-4d9a-9562-974715f1dfbb)
* 2023-02-04 | Brussels, BE | [FOSDEM](https://fosdem.org/)
    * [**FOSDEM 2023 Conference: Rust devroom**](https://fosdem.org/2023/schedule/track/rust/)
* 2023-02-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Practical Cryptography - February Meetup (Registration opens 7 Feb 2023)**](https://www.meetup.com/de-DE/rust-zurich/events/290915075/)


### North America

* 2023-01-26 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Building a Rust Playground with WASM and Lane and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsyfccbjc/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/zpd1qo/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Rust has demonstrated that you using a type system as a vehicle for separation logic works, even in imperative languages, and it's nothing as arcane as those immutable functional predecessors would suggest. It did this by making sure the language defines a type system that helps you, by making sure core properties of soundness *can* be expressed in it.
>
> * soundness requirement for memory access: lifetimes
> * soundness requirements for references *with* value semantics: > `&`/`&mut _`
> * soundness requirements for resources: `Copy` and `Drop`
> * making sure your logic is monotic: traits instead of inheritance, lack of specialization (yes, that's a *feature*).
> * (notably missing: no dependent types; apparently not 'necessary' but I'm sure it could be useful; however, research is heavily ongoing; caution is good)
>
> This allows the standard library to encode all of its relevant requirements as types. And doing this everywhere is its soundness property: safe functions have no requirements beyond the sum of its parameter type, `unsafe functions` can. Nothing new or special there, nothing that makes Rust's notion of soundness special.
>
> Basing your mathematical reasoning on separation logic makes soundness reviews *local* instead of requiring whole program analysis. This is what makes it practical. It did this pretty successfully and principled, but did no single truly revolutionary thing. It's a sum of good bits from the last decade of type system research. That's probably why people refer to it as 'the soundness definition', it's just a very poignant way to say: "we learned that a practical type systems works as a proof checker".

– [HeroicKatora on /r/cpp](https://www.reddit.com/r/cpp/comments/10d4qny/a_call_to_action_think_seriously_about_safety/j4ks225/)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1365) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/10ledzv/this_week_in_rust_479/)</small>
