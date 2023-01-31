Title: This Week in Rust 477
Number: 477
Date: 2023-01-11
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
* [Announcing Rust 1.66.1](https://blog.rust-lang.org/2023/01/10/Rust-1.66.1.html)
* [Security advisory for Cargo (CVE-2022-46176)](https://blog.rust-lang.org/2023/01/10/cve-2022-46176.html)
* [Updating the Android NDK in Rust 1.68](https://blog.rust-lang.org/2023/01/09/android-ndk-update-r25.html)

### Foundation
* [Introducing Our Newest Project Grantees](https://foundation.rust-lang.org/news/community-grants-program-awards-announcement-introducing-our-latest-project-grantees/)

### Newsletters
* [This Month in Rust OSDev: December 2022](https://rust-osdev.com/this-month/2022-12/)
* [Rust Nigeria newsletter Issue 13](https://rustnigeria.curated.co/issues/13)

### Project/Tooling Updates
* [Announcing turmoil](https://tokio.rs/blog/2023-01-03-announcing-turmoil)
* [rust-analyzer changelog #163](https://rust-analyzer.github.io/thisweek/2023/01/09/changelog-163.html)
* [rustc_codegen_gcc: Progress Report #19](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-19)
* [Fyrox 0.29 Feature Highlights](https://fyrox.rs/blog/post/feature-highlights-0-29/)
* [Release of `how-u-doin`; a progress reporting abstraction](https://www.kurtlawrence.info/blog/how-u-doin-a-progress-reporting-abstraction)
* [Introducing OkayWAL: A write-ahead log for Rust](https://bonsaidb.io/blog/introducing-okaywal/)

### Observations/Thoughts
* [Rust Atomics and Locks by Mara Bos](https://marabos.nl/atomics/)
* [Oh the Crates You'll Go! A 2022 Retrospective](https://jack.wrenn.fyi/blog/2022-retrospective/)
* [Rust for Java developers, an introduction](https://wcgw.dev/posts/2023/rusty-java-intro/)
* [On Random Numbers](https://matklad.github.io/2023/01/04/on-random-numbers.html)
* [Potential DoS Vulnerability in Rust Hyper](https://jfrog.com/blog/watch-out-for-dos-when-using-rusts-popular-hyper-package/)
* [Is coding in Rust as bad as in C++?](https://quick-lint-js.com/blog/cpp-vs-rust-build-times/)
* [Rust vs C++ Formatting](https://brevzin.github.io/c++/2023/01/02/rust-cpp-format/)
* [My impressions of Rust after a year of working with it](https://reltech.substack.com/p/my-impressions-of-rust-after-a-year)
* [audio] [Fermyon with Matt Butcher](https://rustacean-station.org/episode/matt-butcher/)

### Rust Walkthroughs
* [Rust and Default Parameters](https://www.thecodedmessage.com/posts/default-params/)
* [Testing SIMD instructions on ARM with Rust on Android](https://gendignoux.com/blog/2023/01/05/rust-arm-simd-android.html)
* [Running Zola on WebAssembly](https://dstaley.com/posts/running-zola-on-wasm/)
* [Who needs Haskell? Straight to Rust Hell](https://www.jernesto.com/articles/who_needs_haskell.html)
* [Sharing Data Among Tasks in Rust Embassy: Synchronization Primitives](https://apollolabsblog.hashnode.dev/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives)
* [Cross compiling Rust with Drone CI and Gitea](https://jacobkiers.net/post/rust-cross-compilation-on-drone/)

## Crate of the Week

This week's crate is [schnellru](https://crates.io/crates/schnellru), which contains a fast and flexible LRU map.

Thanks to [Squirrel](https://users.rust-lang.org/t/crate-of-the-week/2704/1146) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust Playground - Call for contributors](https://users.rust-lang.org/t/call-for-contributors-to-the-rust-playground-for-upcoming-features/87110)
* [meilisearch - When the `lat` and `lng` are strings the wrong error message is returned](https://github.com/meilisearch/meilisearch/issues/3007)
* [meilisearch - Bad latitude / Bad longitude should share a common message between the invalid sort and invalid filte](https://github.com/meilisearch/meilisearch/issues/3006)
* [meilisearch - When both `lat` and `lng` are missing it doesn't return the right error](https://github.com/meilisearch/meilisearch/issues/3005)
* [meilisearch - We must return an error for when _geo is not an object](https://github.com/meilisearch/meilisearch/issues/3003)
* [diesel - A pure rust postgres diesel connection](https://github.com/sfackler/rust-postgres/issues/890)
* [diesel - A pure rust mysql diesel connection implementation](https://github.com/blackbeam/rust-mysql-simple/discussions/320)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

443 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-01-02..2023-01-09

* [playStation Vita support](https://github.com/rust-lang/rust/pull/105712)
* [add JSON output to -Zdump-mono-stats](https://github.com/rust-lang/rust/pull/106274)
* [add default and latest stable edition to --edition in rustc (attempt 2)](https://github.com/rust-lang/rust/pull/106542)
* [split `-Zchalk` flag into `-Ztrait-solver=(classic|chalk|next)` flag](https://github.com/rust-lang/rust/pull/106385)
* [only specify `--target` by default for `-Zgcc-ld=lld` on wasm](https://github.com/rust-lang/rust/pull/101792)
* [do not add `noalias` in return position](https://github.com/rust-lang/rust/pull/106371)
* [`has_overflow` only if value is *not* within limit](https://github.com/rust-lang/rust/pull/106392)
* [account for return-position `impl Trait` in trait in `opt_suggest_box_span`](https://github.com/rust-lang/rust/pull/105846)
* [add help diag. for `const = Enum` missing braces around `Enum`](https://github.com/rust-lang/rust/pull/106283)
* [add retry mechanism for rustdoc GUI tests to reduce flakyness](https://github.com/rust-lang/rust/pull/106449)
* [add type flags support for `Ty` and `Const` late-bound variables](https://github.com/rust-lang/rust/pull/105733)
* [always permit `ConstProp` to exploit arithmetic identities](https://github.com/rust-lang/rust/pull/106340)
* [correct detection of elided lifetimes in impl-trait](https://github.com/rust-lang/rust/pull/106501)
* [default OOM handler: use non-unwinding panic, to match std handler](https://github.com/rust-lang/rust/pull/106045)
* [detect bindings assigned blocks without tail expressions](https://github.com/rust-lang/rust/pull/106519)
* [detect closures assigned to binding in block](https://github.com/rust-lang/rust/pull/106509)
* [do not emit structured suggestion for turbofish with wrong span](https://github.com/rust-lang/rust/pull/106606)
* [do not emit wrong E0308 suggestion for closure mismatch](https://github.com/rust-lang/rust/pull/106581)
* [don't deduce a signature that makes a closure cyclic](https://github.com/rust-lang/rust/pull/105409)
* [point at expressions where inference refines an unexpected type](https://github.com/rust-lang/rust/pull/106400)
* [point out span where we could introduce higher-ranked lifetime](https://github.com/rust-lang/rust/pull/105859)
* [reenable limited top-down MIR inlining](https://github.com/rust-lang/rust/pull/106364)
* [remove invalid case for mutable borrow suggestion](https://github.com/rust-lang/rust/pull/105655)
* [structured suggestion for `&mut dyn Iterator` when possible](https://github.com/rust-lang/rust/pull/106363)
* [suggest `impl Fn*` and `impl Future` in `-> _` return suggestions](https://github.com/rust-lang/rust/pull/106200)
* [suggest `mut self: &mut Self` for `?Sized` impls](https://github.com/rust-lang/rust/pull/106410)
* [suggest adding named lifetime when the return contains value borrowed from more than one lifetimes of function inputs](https://github.com/rust-lang/rust/pull/105805)
* [suggest using clone when we have &T and T implemented Clone](https://github.com/rust-lang/rust/pull/106497)
* [suppress type errors that come from private fields](https://github.com/rust-lang/rust/pull/106600)
* [inference: change a `commit_if_ok` call to probe](https://github.com/rust-lang/rust/pull/105292)
* [miri: ignore symbol shim clash when symbol is provided by `compiler_builtins`](https://github.com/rust-lang/miri/pull/2748)
* [miri: make `env::current_exe` work on Windows](https://github.com/rust-lang/miri/pull/2752)
* [only include metadata for non-dynamic libraries in rustc-dev](https://github.com/rust-lang/rust/pull/105609)
* [merge borrowck permission checks](https://github.com/rust-lang/rust/pull/106451)
* [don't normalize in AstConv](https://github.com/rust-lang/rust/pull/101947)
* [perform `SimplifyLocals` before `ConstProp`](https://github.com/rust-lang/rust/pull/105323)
* [use `FxIndexSet` when updating obligation causes in `adjust_fulfillment_errors_for_expr_obligation`](https://github.com/rust-lang/rust/pull/106468)
* [shrink `ParseResult` in the hot path](https://github.com/rust-lang/rust/pull/106416)
* [stabilize `main_separator_str`](https://github.com/rust-lang/rust/pull/103104)
* [`Split*::as_str` refactor](https://github.com/rust-lang/rust/pull/95644)
* [loosen the bound on the `Debug` implementation of `Weak`](https://github.com/rust-lang/rust/pull/90291)
* [customize `Debug` impl for `OnceWith` & `RepeatWith`](https://github.com/rust-lang/rust/pull/104163)
* [futures: add `PhantomData` marker to Context to make Context !Send and !Sync](https://github.com/rust-lang/rust/pull/95985)
* [futures: set to `None` only if necessary](https://github.com/rust-lang/futures-rs/pull/2683)
* [arch: stabilize cmpxchg16b instrinsic](https://github.com/rust-lang/stdarch/pull/1358)
* [cargo: cargo by default saves credentials to `.cargo/credentials.toml`](https://github.com/rust-lang/cargo/pull/11533)
* [cargo: fix panic on target dependency errors](https://github.com/rust-lang/cargo/pull/11541)
* [rustdoc: fix rustdoc ICE on bad typedef with mismatching types](https://github.com/rust-lang/rust/pull/106366)
* [clippy: `drop_ref`: don't lint idiomatic in match arm](https://github.com/rust-lang/rust-clippy/pull/10142)
* [clippy: `arithmetic_side_effects`: Consider negative numbers and add more tests](https://github.com/rust-lang/rust-clippy/pull/10112)
* [clippy: don't lint `field_reassign` when field in closure](https://github.com/rust-lang/rust-clippy/pull/10143)
* [clippy: expand derivable-impls to cover enums with a default unit variant](https://github.com/rust-lang/rust-clippy/pull/10161)
* [clippy: fix false positive in `single_element_loop`](https://github.com/rust-lang/rust-clippy/pull/10162)
* [clippy: fix `empty_structs_with_brackets` suggestion errors](https://github.com/rust-lang/rust-clippy/pull/10141)
* [clippy: make the `iter_kv_map` lint handle `ref`/`mut` annotations](https://github.com/rust-lang/rust-clippy/pull/10159)
* [clippy: suggest using `Path` for comparing extensions](https://github.com/rust-lang/rust-clippy/pull/10107)
* [clippy: trim paths in `box_default`](https://github.com/rust-lang/rust-clippy/pull/10153) and [`default_trait_access`/`clone_on_copy` suggestions](https://github.com/rust-lang/rust-clippy/pull/10160)
* [rust-analyzer: add wrapping/checked/saturating assist](https://github.com/rust-lang/rust-analyzer/pull/13458)
* [rust-analyzer: apply fallback before final obligation resolution](https://github.com/rust-lang/rust-analyzer/pull/13894)
* [rust-analyzer: complete record enum variants without parens when snippets are disabled](https://github.com/rust-lang/rust-analyzer/pull/13893)
* [rust-analyzer: `extract_expressions_from_format_string`](https://github.com/rust-lang/rust-analyzer/pull/13684)
* [rust-analyzer: add `unqualify_method_call` assist](https://github.com/rust-lang/rust-analyzer/pull/13825)
* [rust-analyzer: add action to expand a declarative macro once, inline. Fixes #13598](https://github.com/rust-lang/rust-analyzer/pull/13810)
* [rust-analyzer: add the ability to limit the number of threads launched by `main_loop`](https://github.com/rust-lang/rust-analyzer/pull/13744)
* [rust-analyzer: colorize `cargo check` diagnostics in VSCode via text decorations](https://github.com/rust-lang/rust-analyzer/pull/13848)
* [rust-analyzer: add generic `TypeBoundList` in generated derivable impl](https://github.com/rust-lang/rust-analyzer/pull/13763)
* [rust-analyzer: generate async delegate methods](https://github.com/rust-lang/rust-analyzer/pull/13843)
* [rust-analyzer: keep whitespace in extract function handler](https://github.com/rust-lang/rust-analyzer/pull/13891)
* [rust-analyzer: only set machine-applicable rustc diagnostics as preferred](https://github.com/rust-lang/rust-analyzer/pull/13887)
* [rust-analyzer: unescape inline module names in module resolution](https://github.com/rust-lang/rust-analyzer/pull/13890)
* [rust-analyzer: postfix adjustment hints](https://github.com/rust-lang/rust-analyzer/pull/13816)
* [rust-analyzer: skip lifetime elision on fn pointers and fn trait types](https://github.com/rust-lang/rust-analyzer/pull/13885)
* [rust-analyzer: use ZWNJ to prevent VSCode from forming ligatures between hints and code](https://github.com/rust-lang/rust-analyzer/pull/13886)
* [rust-analyzer: use diagnostic code as link to full message](https://github.com/rust-lang/rust-analyzer/pull/13853)

### Rust Compiler Performance Triage

A very quiet week, with few changes in either direction, and none of significant magnitude.

Triage done by **@simulacrum**.
Revision range: [b435960..0442fba](https://perf.rust-lang.org/?start=b435960c4cfd3975651c7051be56d7f5d6c201ab&end=0442fbabe24ec43636a80ad1f40a0ad92a2e38df&absolute=false&stat=instructions%3Au)

1 Regressions, 1 Improvements, 3 Mixed; 1 of them in rollups
48 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-01-10.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Add `core::mem::offset_of!` RFC](https://github.com/rust-lang/rfcs/pull/3308)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Create an Operational Semantics Team](https://github.com/rust-lang/rfcs/pull/3346)
* [disposition: merge] [RFC: Move `std::net::IpAddr` types into `core::net`.](https://github.com/rust-lang/rfcs/pull/2832)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Add `SEMICOLON_IN_EXPRESSIONS_FROM_MACROS` to future-incompat report](https://github.com/rust-lang/rust/pull/103418)
* [disposition: merge] [Tracking issue for RFC 2515, "Permit impl Trait in type aliases"](https://github.com/rust-lang/rust/issues/63063)
* [disposition: merge] [Check ADT fields for copy implementations considering regions](https://github.com/rust-lang/rust/pull/105102)
* [disposition: merge] [Stabilise inline_const](https://github.com/rust-lang/rust/pull/104087)
* [disposition: merge] [Partial stabilization of `once_cell`](https://github.com/rust-lang/rust/pull/105587)
* [disposition: merge] [Loosen `From<&[T]> for Box<[T]>` bound to `T: Clone`](https://github.com/rust-lang/rust/pull/103406)
* [disposition: merge] [Leak amplification for peek_mut() to ensure BinaryHeap's invariant is always met](https://github.com/rust-lang/rust/pull/105851)
* [disposition: merge] [rustdoc: simplify JS search routine by not messing with lev distance](https://github.com/rust-lang/rust/pull/105796)
* [disposition: merge] [Only include stable lints in `rustdoc::all` group](https://github.com/rust-lang/rust/pull/106316)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [The `#[diagnostic]` attribute namespace](https://github.com/rust-lang/rfcs/pull/3368)
* [new] [RFC: Multi-Type Return Position Impl Trait (MTRPIT)](https://github.com/rust-lang/rfcs/pull/3367)
* [new] [RFC: re-export stdlib macros from submodules](https://github.com/rust-lang/rfcs/pull/3365)
* [new] [Command improvements for ergonomics and error handling](https://github.com/rust-lang/rfcs/pull/3362)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-01-11 - 2023-02-08 🦀

### Virtual

* 2023-01-11 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/) 
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/290277662/)
* 2023-01-12 | Virtual (San Francisco, CA, US; Stockholm, SE; New York, NY US) | [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor Stockholm](https://www.meetup.com/microsoft-reactor-stockholm/)
    * [**Crack code interview problems in Rust - Ep. 1**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290071417/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/290071415/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290071420/)
* 2023-01-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsyfccbqb/)
* 2023-01-14 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2023-01-16 | Virtual (San Francisco, CA, US; São Paulo, BR; New York, NY, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/)
    * [**Primeros pasos con Rust - Qué es y Configuración el entorno de desarrollo**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224512/) | [**São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224516/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224515/)
* 2023-01-17 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/289581080/)
* 2023-01-17 | Virtual (San Francisco, CA, US; São Paulo, BR; New York, NY, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
    * [**Primeros pasos con Rust - Creación del primer programa de Rust**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224517/) | [***São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224521/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
* 2023-01-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/289015967/)
* 2023-01-18 | Virtual (San Francisco, CA, US; São Paulo, BR; New York, NY US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224523/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224525/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224523/) | [**Sao Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224522/)
* 2023-01-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsyfccbxb/)
* 2023-01-19 | Virtual (Redmond, WA, US; San Francisco, CA, US; New York, NY, US; Stockholm, SE) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor Stockholm](https://www.meetup.com/microsoft-reactor-stockholm/)
    * [**Crack code interview problems in Rust - Ep. 2**](https://www.meetup.com/microsoft-reactor-redmond/events/290085767/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290085766/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290085770/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/290085769/)
* 2023-01-19 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsyfccbzb/)
* 2023-01-23 | Virtual (Durham, NC, US) | [Triangle Rust](https://www.meetup.com/triangle-rust/)
    * [**Online Code and Chat**](https://www.meetup.com/triangle-rust/events/290712105/)
* 2023-01-23 | Virtual (New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Condiciones con expresiones if/else en Rust**](https://www.meetup.com/microsoft-reactor-new-york/events/290224532/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224533/)
* 2023-01-24 | Virtual (Redmond, WA, US; New York, NY, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Uso de bucles para iterar por datos en Rust**](https://www.meetup.com/microsoft-reactor-redmond/events/290224536/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224538/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224535/)
* 2023-01-25 | Virtual (Redmond, WA, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224544/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224540/)
* 2023-01-26 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust Lightning Talks!**](https://www.meetup.com/charlottesville-rust-meetup/events/290122935/)
* 2023-01-26 | Virtual (Redmond, WA, US; San Francisco, CA, US; New York, NY, US; Stockholm, SE) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor Stockholm](https://www.meetup.com/microsoft-reactor-stockholm/)
    * [**Crack code interview problems in Rust - Ep. 3**](https://www.meetup.com/microsoft-reactor-redmond/events/290086420/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290086421/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290086422/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/290086418/)
* 2023-01-30 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Control de errores en Rust**](https://www.meetup.com/microsoft-reactor-redmond/events/290224559/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224558/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224561/)
* 2023-01-31 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/290697014/)
* 2023-01-31 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfccbpc/)
* 2023-01-31 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Compresión de cómo Rust administra la memoria**](https://www.meetup.com/microsoft-reactor-redmond/events/290224861/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224860/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224859/)
* 2023-02-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfcdbcb/)
* 2023-02-01 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224570/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224566/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224570/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224568/)
* 2023-02-01 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcdbcb/)
* 2023-02-06| Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224572/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Implementación de tipos y rasgos genéricos**](https://www.meetup.com/microsoft-reactor-redmond/events/290224576/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224572/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224573/)
* 2023-02-07 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/290609896/)
* 2023-02-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcdbkb/)
* 2023-02-07 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Módulos, paquetes y contenedores de terceros**](https://www.meetup.com/microsoft-reactor-redmond/events/290224578/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224577/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224579/)
* 2023-02-08 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224584/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224583/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224585/)

### Asia

* 2023-01-15 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Property-Based Testing in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/290667325/)

### Europe

* 2023-01-12 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Rust Meetup - Subject TBA**](https://www.meetup.com/dutch-rust-meetup/events/289021643/)
* 2023-01-20 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsyfccbbc/)
* 2023-01-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #55**](https://www.meetup.com/rust-paris/events/290100223/)
* 2023-01-26 | Copenhagen, DK | [Copenhagen Rust Meetup Group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #32**](https://www.meetup.com/copenhagen-rust-meetup-group/events/290037532/)
* 2023-02-02 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn February 2023**](https://www.meetup.com/rust-meetup-hamburg/events/290824576/)
* 2023-02-02 | Lyon, FR | [Rust Lyon](https://mobilizon.fr/events/b8577678-d072-4d9a-9562-974715f1dfbb)
    * [**Rust Lyon meetup #01**](https://mobilizon.fr/events/b8577678-d072-4d9a-9562-974715f1dfbb)


### North America

* 2023-01-11 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/290597764/)
* 2023-01-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/rwvwzsyfccbwb/)
* 2023-01-26 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #32**](https://www.meetup.com/copenhagen-rust-meetup-group/events/290037532/)
* 2023-01-26 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Building a Rust Playground with WASM and Lane and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsyfccbjc/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/zpd1qo/official_rrust_whos_hiring_thread_for_jobseekers))

# Quote of the Week

> Now macros are fine, I mean we use them for implementing internals and you know if you have something that \[...\] needs to be implemented for lots and lots of different concrete types, then macros are a fine choice for that, but exposing that to users is something to be very careful about.

– [Raph Levien](https://youtu.be/Phk0C-kLlho?t=542)

llogiq is a tad sad there were no suggestions, but still likes the quote he ended up with!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/109mcyv/this_week_in_rust_477)</small>
