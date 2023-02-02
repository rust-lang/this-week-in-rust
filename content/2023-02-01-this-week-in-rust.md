Title: This Week in Rust 480
Number: 480
Date: 2023-02-01
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
* [Announcing Rust 1.67.0](https://blog.rust-lang.org/2023/01/26/Rust-1.67.0.html)
* [Help test Cargo's new index protocol](https://blog.rust-lang.org/inside-rust/2023/01/30/cargo-sparse-protocol.html)

### Foundation
* [Rust Foundation Annual Report 2022](https://foundation.rust-lang.org/news/rust-foundation-annual-report-2022/)

### Newsletters
* [This Month in Rust GameDev #41 - December 2022](https://gamedev.rs/news/041/)

### Project/Tooling Updates
* [Scaphandre 0.5.0, to measure energy consumption of IT servers and computers, is released : windows compatibility (experimental), multiple sensors support, and much more...](https://github.com/hubblo-org/scaphandre/discussions/258)
* [IntelliJ Rust Changelog #187](https://intellij-rust.github.io/2023/01/30/changelog-187.html)
* [rust-analyzer changelog #166](https://rust-analyzer.github.io/thisweek/2023/01/30/changelog-166.html)
* [argmin 0.8.0 and argmin-math 0.3.0 released](https://argmin-rs.org/blog/version-v0-8-0/)
* [Fornjot (code-first CAD in Rust) - Weekly Release - The Usual Rabbit Hole](https://www.fornjot.app/blog/weekly-release/2023-w05/)
* [One step forward, an easier interoperability between Rust and Haskell](https://engineering.iog.io/2023-01-26-hs-bindgen-introduction/)
* [Managing complex communication over raw I/O streams using `async-io-typed` and `async-io-converse`](https://xaeroxe.github.io/async-io/)
* [Autometrics - a macro that tracks metrics for any function & inserts links to Prometheus charts right into each function's doc comments](https://github.com/fiberplane/autometrics-rs)

### Observations/Thoughts
* [Ordering Numbers, How Hard Can It Be?](https://orlp.net/blog/ordering-numbers/)
* [Next Rust Compiler](https://matklad.github.io/2023/01/25/next-rust-compiler.html)
* [Forking Chrome to render in a terminal](https://fathy.fr/carbonyl)
* [40x Faster! We rewrote our project with Rust!](https://medium.com/@xpf6677/40x-faster-we-rewrote-our-project-with-rust-120b006c6abe)
* [Moving and re-exporting a Rust type can be a major breaking change](https://predr.ag/blog/moving-and-reexporting-rust-type-can-be-major-breaking-change/)
* [What the HAL? The Quest for Finding a Suitable Embedded Rust HAL](https://apollolabsblog.hashnode.dev/what-the-hal-the-quest-for-finding-a-suitable-embedded-rust-hal)
* [Some Rust breaking changes don't require a major version](https://predr.ag/blog/some-rust-breaking-changes-do-not-require-major-version/)
* [Crash! And now what?](https://tweedegolf.nl/en/blog/82/crash-and-now-what)

### Rust Walkthroughs
* [Handling Integer Overflow in Rust](https://bmoxb.io/2023/01/28/integer-overflow-in-rust.html)
* [Rust error handling with anyhow](https://antoinerr.github.io/blog-website/2023/01/28/rust-anyhow.html)
* [Synchronizing state with Websockets and JSON Patch](https://cetra3.github.io/blog/synchronising-with-websocket/)
* [Plugins for Rust](https://reorchestrate.com/posts/plugins-for-rust/)
* [series] [Protohackers in Rust, Part 00: Dipping the toe in async and Tokio](https://d2718.net/blog/posts/protohax_00.html)
* [Building gRPC APIs with Rust using Tonic](https://konghq.com/blog/building-grpc-apis-with-rust)

### Miscellaneous
* [Rust's Ugly Syntax](https://matklad.github.io/2023/01/26/rusts-ugly-syntax.html)
* [video] [Rust's Witchcraft](https://www.youtube.com/watch?v=MWRPYBoCEaY)
* [DE] [CodeSandbox: Nun auch für die Rust-Entwicklung](https://www.heise.de/news/CodeSandbox-Nun-auch-fuer-die-Rust-Entwicklung-7470518.html)

## Crate of the Week

This week's crate is [symphonia](https://lib.rs/crates/symphonia), a collection of pure-Rust audio decoders for many common formats.

Thanks to [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1151) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [diesel - Generate matching SQL types for Mysql enums](https://github.com/diesel-rs/diesel/issues/3386)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821
<!-- https://users.rust-lang.org/t/twir-call-for-participation/4821/489 -->

## Updates from the Rust Project

377 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-01-23..2023-01-30

* [move `format_args!()` into AST (and expand it during AST lowering)](https://github.com/rust-lang/rust/pull/106745)
* [implement Hash for `proc_macro::LineColumn`](https://github.com/rust-lang/rust/pull/106946)
* [add help message about function pointers](https://github.com/rust-lang/rust/pull/105552)
* [add hint for missing lifetime bound on trait object when type alias is used](https://github.com/rust-lang/rust/pull/105345)
* [add suggestion to remove if in `let`..`else` block](https://github.com/rust-lang/rust/pull/107213)
* [assume MIR types are fully normalized in `ascribe_user_type`](https://github.com/rust-lang/rust/pull/107197)
* [check for missing space between fat arrow and range pattern](https://github.com/rust-lang/rust/pull/107425)
* [compute generator saved locals on MIR](https://github.com/rust-lang/rust/pull/101692)
* [core: support variety of atomic widths in width-agnostic functions](https://github.com/rust-lang/rust/pull/106856)
* [correct suggestions for closure arguments that need a borrow](https://github.com/rust-lang/rust/pull/107306)
* [detect references to non-existant messages in Fluent resources](https://github.com/rust-lang/rust/pull/107096)
* [disable `ConstGoto` opt in cleanup blocks](https://github.com/rust-lang/rust/pull/107323)
* [don't merge vtables when full debuginfo is enabled](https://github.com/rust-lang/rust/pull/107373)
* [fix def-use dominance check](https://github.com/rust-lang/rust/pull/107097)
* [fix thin archive reading](https://github.com/rust-lang/rust/pull/107360)
* [impl `DispatchFromDyn` for `Cell` and `UnsafeCell`](https://github.com/rust-lang/rust/pull/97373)
* [implement simple `CopyPropagation` based on SSA analysis](https://github.com/rust-lang/rust/pull/106908)
* [improve proc macro attribute diagnostics](https://github.com/rust-lang/rust/pull/106407)
* [insert whitespace to avoid ident concatenation in suggestion](https://github.com/rust-lang/rust/pull/106540)
* [only compute `mir_generator_witnesses` query in `drop_tracking_mir` mode](https://github.com/rust-lang/rust/pull/107406)
* [preserve split DWARF files when building archives](https://github.com/rust-lang/rust/pull/106904)
* [recover from more const arguments that are not wrapped in curly braces](https://github.com/rust-lang/rust/pull/107190)
* [reimplement `NormalizeArrayLen` based on `SsaLocals`](https://github.com/rust-lang/rust/pull/107172)
* [remove overlapping parts of multipart suggestions](https://github.com/rust-lang/rust/pull/106916)
* [special-case deriving `PartialOrd` for enums with dataless variants](https://github.com/rust-lang/rust/pull/103659)
* [suggest coercion of `Result` using `?`](https://github.com/rust-lang/rust/pull/106583)
* [suggest qualifying bare associated constants](https://github.com/rust-lang/rust/pull/107204)
* [suggest using a lock for `*Cell: Sync` bounds](https://github.com/rust-lang/rust/pull/106944)
* [teach parser to understand fake anonymous enum syntax](https://github.com/rust-lang/rust/pull/106960)
* [use `can_eq` to compare types for default assoc type error](https://github.com/rust-lang/rust/pull/107304)
* [use proper `InferCtxt` when probing for associated types in astconv](https://github.com/rust-lang/rust/pull/107100)
* [use stable metric for const eval limit instead of current terminator-based logic](https://github.com/rust-lang/rust/pull/106227)
* [remove optimistic spinning from `mpsc::SyncSender`](https://github.com/rust-lang/rust/pull/106836)
* [stabilize the `const_socketaddr` feature](https://github.com/rust-lang/rust/pull/104252)
* [codegen\_gcc: fix/signed integer overflow](https://github.com/rust-lang/rustc_codegen_gcc/pull/249)
* [cargo: `cargo add` check `[dependencies]` order without considering the dotted item](https://github.com/rust-lang/cargo/pull/11612)
* [cargo: avoid saving the same `future_incompat` warning multiple times](https://github.com/rust-lang/cargo/pull/11648)
* [cargo: fix split-debuginfo support detection](https://github.com/rust-lang/cargo/pull/11347)
* [cargo: make cargo aware of dwp files](https://github.com/rust-lang/cargo/pull/11572)
* [cargo: mention current default value in `publish.timeout` docs](https://github.com/rust-lang/cargo/pull/11652)
* [rustdoc: collect rustdoc-reachable items during early doc link resolution](https://github.com/rust-lang/rust/pull/107054)
* [rustdoc: prohibit scroll bar on source viewer in Safari](https://github.com/rust-lang/rust/pull/107266)
* [rustdoc: use smarter encoding for playground URL](https://github.com/rust-lang/rust/pull/107284)
* [rustdoc: add option to include private items in library docs](https://github.com/rust-lang/rust/pull/107264)
* [fix infinite loop in rustdoc `get_all_import_attributes` function](https://github.com/rust-lang/rust/pull/107357)
* [rustfmt: don't wrap comments that are part of a table](https://github.com/rust-lang/rustfmt/pull/5475)
* [rustfmt: fix for handling empty code block in doc comment](https://github.com/rust-lang/rustfmt/pull/5601)
* [clippy: `invalid_regex`: show full error when string value doesn't match source](https://github.com/rust-lang/rust-clippy/pull/10231)
* [clippy: `multiple_unsafe_ops_per_block`: don't lint in external macros](https://github.com/rust-lang/rust-clippy/pull/10260)
* [clippy: improve span for `module_name_repetitions`](https://github.com/rust-lang/rust-clippy/pull/10226)
* [clippy: missing config](https://github.com/rust-lang/rust-clippy/pull/10248)
* [clippy: prevents `len_without_is_empty` from triggering when `len` takes arguments besides `&self`](https://github.com/rust-lang/rust-clippy/pull/10255)
* [rust-analyzer: adding section for Visual Studio IDE Rust development support](https://github.com/rust-lang/rust-analyzer/pull/14012)
* [rust-analyzer: don't fail workspace loading if sysroot can't be found](https://github.com/rust-lang/rust-analyzer/pull/14038)
* [rust-analyzer: improve "match to let else" assist](https://github.com/rust-lang/rust-analyzer/pull/14057)
* [rust-analyzer: show signature help when typing record literal](https://github.com/rust-lang/rust-analyzer/pull/14041)
* [rust-analyzer: ide-assists: unwrap block when it parent is let stmt](https://github.com/rust-lang/rust-analyzer/pull/14011)
* [rust-analyzer: fix config substitution failing extension activation](https://github.com/rust-lang/rust-analyzer/pull/14023)
* [rust-analyzer: don't include lifetime or label apostrophe when renaming](https://github.com/rust-lang/rust-analyzer/pull/14015)
* [rust-analyzer: fix "add missing impl members" assist for impls inside blocks](https://github.com/rust-lang/rust-analyzer/pull/14039)
* [rust-analyzer: fix assoc item search finding unrelated definitions](https://github.com/rust-lang/rust-analyzer/pull/14020)
* [rust-analyzer: fix process-changes not deduplicating changes correctly](https://github.com/rust-lang/rust-analyzer/pull/14025)
* [rust-analyzer: handle boolean scrutinees in match `<->` if let replacement assists better](https://github.com/rust-lang/rust-analyzer/pull/14037)
* [rust-analyzer: substitute VSCode variables more generally](https://github.com/rust-lang/rust-analyzer/pull/14019)

### Rust Compiler Performance Triage

Overall a positive week, with relatively few regressions overall and a number of improvements.

Triage done by **@simulacrum**.
Revision range: [c8e6a9e..a64ef7d](https://perf.rust-lang.org/?start=c8e6a9e8b6251bbc8276cb78cabe1998deecbed7&end=a64ef7d07d0411315be85a646586cb85eeb9c136&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.6%, 0.6%]   | 1     |
| Regressions ❌ <br /> (secondary)  | 0.3%  | [0.3%, 0.3%]   | 1     |
| Improvements ✅ <br /> (primary)   | -0.8% | [-2.0%, -0.2%] | 27    |
| Improvements ✅ <br /> (secondary) | -0.9% | [-1.9%, -0.5%] | 11    |
| All ❌✅ (primary)                 | -0.8% | [-2.0%, 0.6%]  | 28    |


2 Regressions, 4 Improvements, 6 Mixed; 2 of them in rollups
44 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-01-31.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Create an Operational Semantics Team](https://github.com/rust-lang/rfcs/pull/3346)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Stabilize feature `cstr_from_bytes_until_nul`](https://github.com/rust-lang/rust/pull/107429)
* [disposition: merge] [Support `true` and `false` as boolean flag params](https://github.com/rust-lang/rust/pull/107043)
* [disposition: merge] [Implement `AsFd` and `AsRawFd` for `Rc`](https://github.com/rust-lang/rust/pull/107317)
* [disposition: merge] [rustdoc: compute maximum Levenshtein distance based on the query](https://github.com/rust-lang/rust/pull/107141)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Permissions](https://github.com/rust-lang/rfcs/pull/3380)
* [new] [Add text for the CFG OS Version RFC](https://github.com/rust-lang/rfcs/pull/3379)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [Feature: Help test Cargo's new index protocol](https://blog.rust-lang.org/inside-rust/2023/01/30/cargo-sparse-protocol.html)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-02-01 - 2023-03-01 🦀

### Virtual

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
* 2023-02-08 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/) 
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcdblb/)
* 2023-02-08 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224584/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224583/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224585/)
* 2023-02-09 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsyfcdbmb/)
* 2023-02-11 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-02-13 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Escritura de pruebas automatizadas**](https://www.meetup.com/microsoft-reactor-redmond/events/290224610/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224608/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224609/)
* 2023-02-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfcdbsb/)
* 2023-02-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfcdbsb/)
* 2023-02-14 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Creamos un programa de ToDos en la línea de comandos**](https://www.meetup.com/microsoft-reactor-redmond/events/290224616/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224613/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224617/)
* 2023-02-14 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 26u16**](https://www.meetup.com/rust-saar/events/290040138/)
* 2023-02-15 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US; São Paulo, BR) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224621/) | [**São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224623/)
* 2023-02-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsyfcdbtb/)
* 2023-02-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/vdhxgsyfcdbcc/)
* 2023-02-23 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Tock, a Rust based Embedded Operating System**](https://www.meetup.com/charlottesville-rust-meetup/events/291248593/)
* 2023-02-23 | Virtual (Kassel, DE) | [Java User Group Hessen](https://www.meetup.com/java-user-group-hessen-jugh/)
    * [**Eine Einführung in Rust (Stefan Baumgartner)**](https://www.meetup.com/java-user-group-hessen-jugh/events/290346591/)
* 2023-02-28 | Virtual (Berlin, DE) | [Open Tech School Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/290852327/)
* 2023-02-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcdblc/)
* 2023-03-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfcfbcb/)

### Asia

* 2023-02-01 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust)
    * [**Rust talk: How to implement Iterator on tuples... kind of**](https://www.meetup.com/kansai-rust/events/291020672)
* 2023-02-20 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**February Edition - Redis and BioCatch talking Rust!**](https://www.meetup.com/rust-tlv/events/291182881/)

### Europe

* 2023-02-02 | Berlin, DE | [Prenzlauer Berg Software Engineers](https://www.meetup.com/prenzlauer-berg-software-engineers/)
    * [**PBerg engineers - inaugural meetup; Lukas: Serverless APIs using Rust and Azure functions (Fee)**](https://www.meetup.com/prenzlauer-berg-software-engineers/events/290513923/)
* 2023-02-02 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn February 2023**](https://www.meetup.com/rust-meetup-hamburg/events/290824576/)
* 2023-02-02 | Lyon, FR | [Rust Lyon](https://mobilizon.fr/events/b8577678-d072-4d9a-9562-974715f1dfbb)
    * [**Rust Lyon meetup #01**](https://mobilizon.fr/events/b8577678-d072-4d9a-9562-974715f1dfbb)
* 2023-02-04 | Brussels, BE | [FOSDEM](https://fosdem.org/)
    * [**FOSDEM 2023 Conference: Rust devroom**](https://fosdem.org/2023/schedule/track/rust/)
* 2023-02-09 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet)
    * [**Rust Lille #2**](https://www.meetup.com/meetup-group-zgphbyet/events/291046592/)
* 2023-02-15 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Nation Pre-Conference Reception with The Rust Foundation**](https://www.meetup.com/rust-london-user-group/events/290903823/)
* 2023-02-15 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim)
    * [**Rust New Year's Resolution Bug Hunt**](https://www.meetup.com/rust-trondheim/events/290889889/)
* 2023-02-16, 2023-02-17 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation '23**](https://www.rustnationuk.com/)
* 2023-02-18 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Post-Conference Rust in Enterprise Brunch Hosted at Red Badger**](https://www.meetup.com/rust-london-user-group/events/291297886/)
* 2023-02-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Practical Cryptography - February Meetup (Registration opens 7 Feb 2023)**](https://www.meetup.com/de-DE/rust-zurich/events/290915075/)
* 2023-02-23 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust metup #33**](https://www.meetup.com/copenhagen-rust-community/events/291288154/)


### North America

* 2023-02-09 | Mountain View, CA, US | [Mountain View Rust Study Group](https://www.meetup.com/rust-study-group/)
    * [**Rust Study Group at Hacker Dojo**](https://www.meetup.com/rust-study-group/events/291190532/)
* 2023-02-09 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**A Night of Interop: Rust in React Native & Rust in Golang (two talks)**](https://www.meetup.com/rust-nyc/events/291239545/)
* 2023-02-13 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Happy Hour and Beginner Embedded Rust Hacking Session (#3!)**](https://www.meetup.com/minneapolis-rust-meetup/events/291299604/)
* 2023-02-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/rwvwzsyfcdbcc/)
* 2023-02-23 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Upcoming Event**](https://www.meetup.com/utah-rust/events/dsbpxsyfcdbfc/)

### Oceania

* 2023-02-28 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/291278417/)
* 2023-03-01 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 Lightning Talks - We are back!**](https://www.meetup.com/rust-sydney/events/291265163/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/10nmtew/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Compilers are an error reporting tool with a code generation side-gig.

– [Esteban Küber on Hacker News](https://news.ycombinator.com/item?id=34544449)

Thanks to [Stefan Majewsky](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1369) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/10rbkhz/this_week_in_rust_480/)</small>
