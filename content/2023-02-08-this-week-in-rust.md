Title: This Week in Rust 481
Number: 481
Date: 2023-02-08
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
* [Announcing Rustup 1.25.2](https://blog.rust-lang.org/2023/02/01/Rustup-1.25.2.html)

### Foundation
* [Member Spotlight: AdaCore](https://foundation.rust-lang.org/news/member-spotlight-adacore/)

### FOSDEM 2023
* [video] [Building an actor library for Quickwit's indexing pipeline](https://fosdem.org/2023/schedule/event/building_an_actor_library_for_quickwits_indexing_pipeline/)
* [video] [Building a distributed search engine with tantivy](https://fosdem.org/2023/schedule/event/rust_building_a_distributed_search_engine_with_tantivy/)
* [video] [Aurae: Distributed Runtime](https://fosdem.org/2023/schedule/event/rust_aurae_a_new_pid_1_for_distributed_systems/)
* [video] [BastionLab, a Rust open-source privacy framework for confidential data science collaboration](https://fosdem.org/2023/schedule/event/rust_bastionlab/)
* [video] [Neovim and rust-analyzer are best friends](https://fosdem.org/2023/schedule/event/rust_neovim_and_rust_analyzer_are_best_friends/)
* [video] [A Rusty CHERI - The path to hardware capabilities in Rust](https://fosdem.org/2023/schedule/event/rust_a_rusty_cheri_the_path_to_hardware_capabilities_in_rust/)
* [video] [Slint: Are we GUI yet?](https://fosdem.org/2023/schedule/event/rust_slint_are_we_gui_yet/)
* [video] [Rust API Design Learnings](https://fosdem.org/2023/schedule/event/rust_rust_api_design_learnings/)
* [video] [A deep dive inside the Rust frontend for GCC](https://fosdem.org/2023/schedule/event/rust_a_deep_dive_inside_the_rust_frontend_for_gcc/)
* [video] [Merging process of the rust compiler](https://fosdem.org/2023/schedule/event/rust_merging_process_of_the_rust_compiler/)
* [video] [Let's write Snake game!](https://fosdem.org/2023/schedule/event/rust_lets_write_snake_game/)
* [video] [Glidesort](https://fosdem.org/2023/schedule/event/rust_glidesort/)
* [video] [How Pydantic V2 leverages Rust's Superpowers](https://fosdem.org/2023/schedule/event/rust_how_pydantic_v2_leverages_rusts_superpowers/)
* [video] [Scalable graph algorithms in Rust (and Python)](https://fosdem.org/2023/schedule/event/rust_scalable_graph_algorithms_in_rust_and_python/)
* [video] [Using Rust for your network management tools!](https://fosdem.org/2023/schedule/event/rust_using_rust_for_your_network_management_tools/)
* [video] [Backward and forward compatibility for security features](https://fosdem.org/2023/schedule/event/rust_backward_and_forward_compatibility_for_security_features/)
* [video] [atuin: magical shell history with Rust](https://fosdem.org/2023/schedule/event/rust_atuin_magical_shell_history_with_rust/)
* [video] [A Rust-Based, modular Unikernel for MicroVMs](https://fosdem.org/2023/schedule/event/rustunikernel/)

### Newsletters
* [This Month in Rust OSDev: January 2023](https://rust-osdev.com/this-month/2023-01/)
* [The first issue of Rust Magazine has been published 🎉🎉](https://rustmagazine.org/issue-1/)

### Project/Tooling Updates
* [Release Engineering Is Exhausting So Here's cargo-dist](https://blog.axo.dev/2023/02/cargo-dist)
* [rust-analyzer changelog #167](https://rust-analyzer.github.io/thisweek/2023/02/06/changelog-167.html)
* [Glidesort, my stable adaptive quicksort/mergesort hybrid sorting algorithm](https://github.com/orlp/glidesort#readme)
* [Fornjot (code-first CAD in Rust) - Weekly Release - Ostensibly Quiet](https://www.fornjot.app/blog/weekly-release/2023-w06/)
* [derive-adhoc: "derive by example", an ergonomic replacement for (some) proc macros](https://diziet.dreamwidth.org/14345.html)
* [Lemmy release 0.17.0 - A link aggregator for the Fediverse](https://join-lemmy.org/news/2023-01-31_-_Lemmy_Release_v0.17.0)
* [Arti 1.1.1 is released: Groundwork for onion services](https://blog.torproject.org/arti_111_released/)
* [SeaORM 0.11.0 - 🐚 An async & dynamic ORM for Rust](https://www.sea-ql.org/blog/2023-02-08-whats-new-in-seaorm-0.11.0/)

### Observations/Thoughts
* [Async trait send bounds, part 1: intro](https://smallcultfollowing.com/babysteps/blog/2023/02/01/async-trait-send-bounds-part-1-intro/)
* [Speeding up Rust semver-checking by over 2000x](https://predr.ag/blog/speeding-up-rust-semver-checking-by-over-2000x/)
* [Announcing Masonry 0.1, and my vision for Rust UI](https://poignardazur.github.io/2023/02/02/masonry-01-and-my-vision-for-rust-ui/)
* [Rust Is Beyond Object-Oriented, Part 2: Polymorphism](https://www.thecodedmessage.com/posts/oop-2-polymorphism/)
* [video] [Rust Malaysia - The journey to rust from a student](https://www.youtube.com/watch?v=rh1ZJXOuco8&list=PL85XCvVPmGQh3V0Pz-_xFm6VAUTR4aLUw&index=18)
* [Improving incremental test times in Rust](https://blog.waleedkhan.name/rust-incremental-test-times/)
* [Using HTML as a compile target](https://blog.yoshuawuyts.com/compiled-html/)
* [Tauri vs Iced vs egui: Rust GUI framework performance comparison](https://lukaskalbertodt.github.io/2023/02/03/tauri-iced-egui-performance-comparison.html)
* [Improving Rust compile times to enable adoption of memory safety](https://www.memorysafety.org/blog/remy-rakic-compile-times/)
* [audio] [Cloudflare with Adam Chalmers](https://rustacean-station.org/episode/adam-chalmers/)

### Rust Walkthroughs
* [Building a Rust parser using Pest and PEG](https://blog.logrocket.com/building-rust-parser-pest-peg/)
* [Rustler - Using Rust crates in Elixir](https://mainmatter.com/blog/2023/02/01/using-rust-crates-in-elixir/)
* [LifetimeKata: Exercises to learn how to use lifetimes](https://tfpk.github.io/lifetimekata/)
* [Exploring Rust for Vulkan drivers, part 1](https://www.collabora.com/news-and-blog/blog/2023/02/02/exploring-rust-for-vulkan-drivers-part-1/)
* [Rustproofing Linux (Part 1/4 Leaking Addresses)](https://research.nccgroup.com/2023/02/06/rustproofing-linux-part-1-4-leaking-addresses/)
* [STM32F4 Embedded Rust at the PAC: svd2rust](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-pac-svd2rust)

### Miscellaneous
* [My Reaction to Dr. Stroustrup's Recent Memory Safety Comments](https://www.thecodedmessage.com/posts/stroustrup-response/)
* [This Is the Kind of Rust the Automotive Industry Needs](https://blogs.blackberry.com/en/2023/02/this-is-the-kind-of-rust-the-automotive-industry-needs)

## Crate of the Week

This week's crate is [envious](https://github.com/TheNeikos/envious), a serde-based deserializer from environment variables.

Thanks to [musicmatze](https://users.rust-lang.org/t/crate-of-the-week/2704/1156) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [this-week-in-rust - Make dark mode more accessible](https://github.com/rust-lang/this-week-in-rust/issues/4031?)
* [Ockam - 'ockam tcp-connection create' should return the tcp-connection ID](https://github.com/build-trust/ockam/issues/4177)
* [Ockam - 'ockam node stop' should output a message when exiting successfully](https://github.com/build-trust/ockam/issues/4175)
* [comprehensive-rust - We now support translations!](https://github.com/google/comprehensive-rust/issues/282)
* [comprehensive-rust - Extract text more carefully in mdbook-xgettext](https://github.com/google/comprehensive-rust/issues/318)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

350 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-01-30..2023-02-06

* [retry opening proc-macro DLLs a few times on Windows](https://github.com/rust-lang/rust/pull/107595)
* [autotrait bounds on dyn-safe trait methods](https://github.com/rust-lang/rust/pull/107082)
* [do not depend on Generator trait when deducing closure signature](https://github.com/rust-lang/rust/pull/107499)
* [parse and recover from type ascription in patterns](https://github.com/rust-lang/rust/pull/107602)
* [recover `_` as `..` in field pattern](https://github.com/rust-lang/rust/pull/106919)
* [recover form missing expression in `for` loop](https://github.com/rust-lang/rust/pull/107526)
* [recover from lifetimes with default lifetimes in generic args](https://github.com/rust-lang/rust/pull/107580)
* [don't generate unecessary `&&self.field` in deriving Debug](https://github.com/rust-lang/rust/pull/107599)
* [emit warnings on unused parens in index expressions](https://github.com/rust-lang/rust/pull/107539)
* [erase regions before doing uninhabited check in borrowck](https://github.com/rust-lang/rust/pull/107532)
* [extend `-Z print-type-sizes` to distinguish generator upvars+locals from "normal" fields](https://github.com/rust-lang/rust/pull/107533)
* [fix invalid float literal suggestions when recovering an integer](https://github.com/rust-lang/rust/pull/105650)
* [fix suggestion for coercing `Option<&String>` to `Option<&str>`](https://github.com/rust-lang/rust/pull/107633)
* [fix syntax in `-Zunpretty-expanded` output for derived `PartialEq`](https://github.com/rust-lang/rust/pull/107488)
* [fixing confusion between mod and remainder](https://github.com/rust-lang/rust/pull/107389)
* [implement unsizing in the new trait solver](https://github.com/rust-lang/rust/pull/107245)
* [don't point at nonexisting code beyond EOF when warning about delims](https://github.com/rust-lang/rust/pull/107663)
* [improve diagnostic for missing space in range pattern](https://github.com/rust-lang/rust/pull/107493)
* [improve panic message for slice windows and chunks](https://github.com/rust-lang/rust/pull/107442)
* [improve pretty-printing of `HirIdValidator` errors](https://github.com/rust-lang/rust/pull/107515)
* [make `unaligned_reference` a hard error](https://github.com/rust-lang/rust/pull/102513)
* [make const/fn return params more suggestable](https://github.com/rust-lang/rust/pull/106887)
* [make the "extra if in let...else block" hint a suggestion](https://github.com/rust-lang/rust/pull/107487)
* [provide structured suggestion for binding needing type on E0594](https://github.com/rust-lang/rust/pull/107646)
* [refine error spans for "The trait bound `T: Trait` is not satisfied" when passing literal structs/tuples](https://github.com/rust-lang/rust/pull/106477)
* [remove confusing 'while checking' note from opaque future type mismatches](https://github.com/rust-lang/rust/pull/107201)
* [sort Generator `print-type-sizes` according to their yield points](https://github.com/rust-lang/rust/pull/107692)
* [suggest `move` in nested closure when appropriate](https://github.com/rust-lang/rust/pull/106575)
* [suggest `std::ptr::null` if literal 0 is given to a raw pointer function argument](https://github.com/rust-lang/rust/pull/107553)
* [suggest `{var:?}` when finding `{?:var}` in inline format strings](https://github.com/rust-lang/rust/pull/106805)
* [suggest adding a return type for async functions](https://github.com/rust-lang/rust/pull/107685)
* [intern external constraints in new solver](https://github.com/rust-lang/rust/pull/107621)
* [optimize `fold_ty`](https://github.com/rust-lang/rust/pull/107627)
* [do not deaggregate MIR](https://github.com/rust-lang/rust/pull/107267)
* [adapt SROA MIR opt for aggregated MIR](https://github.com/rust-lang/rust/pull/107687)
* [test `drop_tracking_mir` before querying generator](https://github.com/rust-lang/rust/pull/107443)
* [track bound types like bound regions](https://github.com/rust-lang/rust/pull/107486)
* [futures: poll `Select` futures without moving them](https://github.com/rust-lang/futures-rs/pull/2704)
* [cargo: add partial support for SSH known hosts markers](https://github.com/rust-lang/cargo/pull/11635)
* [cargo: config: deny `CARGO_HOME` in env table](https://github.com/rust-lang/cargo/pull/11644)
* [cargo: do not error for `auth-required: true` without `-Z sparse-registry`](https://github.com/rust-lang/cargo/pull/11661)
* [cargo: handle .cargo-ok being truncated](https://github.com/rust-lang/cargo/pull/11665)
* [cargo: make cargo install report needed features](https://github.com/rust-lang/cargo/pull/11647)
* [cargo: verify source before recompile](https://github.com/rust-lang/cargo/pull/11672)
* [rustfmt: prevent shorthand init for tuple struct](https://github.com/rust-lang/rustfmt/pull/5520)
* [rustfmt: lists doc comments](https://github.com/rust-lang/rustfmt/pull/5560)
* [clippy: `needless_lifetimes`: lint local macros](https://github.com/rust-lang/rust-clippy/pull/10257)
* [clippy: `unused_io_amount`: lint with `is_ok` and `is_err`](https://github.com/rust-lang/rust-clippy/pull/10225)
* [clippy: `wildcard_enum_match_arm` lint takes the enum origin into account](https://github.com/rust-lang/rust-clippy/pull/10250)
* [clippy: add `extra_unused_type_parameters` lint](https://github.com/rust-lang/rust-clippy/pull/10028)
* [clippy: add machine applicable suggestion for `needless_lifetimes`](https://github.com/rust-lang/rust-clippy/pull/10222)
* [clippy: don't depend on FormatArgsExpn in ManualAssert](https://github.com/rust-lang/rust-clippy/pull/10276)
* [clippy: fix version declared for `semicolon_inside_block` and `semicolon_outside…`](https://github.com/rust-lang/rust-clippy/pull/10256)
* [clippy: mark `uninlined_format_args` as pedantic](https://github.com/rust-lang/rust-clippy/pull/10265)
* [rust-analyzer: add more basic issue templates with auto category labeling](https://github.com/rust-lang/rust-analyzer/pull/14083)
* [rust-analyzer: expand docs section on Visual Studio to mention all three available extensions](https://github.com/rust-lang/rust-analyzer/pull/14072)
* [rust-analyzer: remove support for 1.58 proc-macro abi](https://github.com/rust-lang/rust-analyzer/pull/14063)
* [rust-analyzer: fix negative trait bound in outline view (#14044)](https://github.com/rust-lang/rust-analyzer/pull/14058)
* [rust-analyzer: consider relative offset to fake ident token in expansion for completion](https://github.com/rust-lang/rust-analyzer/pull/14043)
* [rust-analyzer: don't panic on broken syntax trees in adjustment inlay hints](https://github.com/rust-lang/rust-analyzer/pull/14092)
* [rust-analyzer: don't render fieldless discriminant inlay hints for datacarrying enums](https://github.com/rust-lang/rust-analyzer/pull/14071)
* [rust-analyzer: support non-ascii characters in case conversion](https://github.com/rust-lang/rust-analyzer/pull/14082)
* [rust-analyzer: unsize cast array only on pointer type](https://github.com/rust-lang/rust-analyzer/pull/14068)
* [rust-analyzer: implement proc-macro-api versioning](https://github.com/rust-lang/rust-analyzer/pull/14070)
* [rust-analyzer: record method resolution for remaining operator expressions](https://github.com/rust-lang/rust-analyzer/pull/14036)
* [rust-analyzer: reuse fetching target data layout from rustc function](https://github.com/rust-lang/rust-analyzer/pull/14094)
* [rust-analyzer: support computing layout of RPIT](https://github.com/rust-lang/rust-analyzer/pull/14087)
* [rust-analyzer: support generic function in `generate_function` assist](https://github.com/rust-lang/rust-analyzer/pull/14065)
* [rust-analyzer: support sysroot library source being defined inside the workspace](https://github.com/rust-lang/rust-analyzer/pull/14091)

### Rust Compiler Performance Triage

Much noise in benchmarks this week, which makes it hard to tell what the real
improvements were and what they were due to. A query cache change (PR #107667)
is part of the story. In addition, much improvement was reaped from the change
to *not* deaggregate MIR (PR #107267). Finally, microoptimizing `fold_ty`
(PR #107627) yielded a small improvement to a broad set of benchmarks.

Triage done by **@pnkfelix**.
Revision range: [a64ef7d0..e4dd9edb](https://perf.rust-lang.org/?start=a64ef7d07d0411315be85a646586cb85eeb9c136&end=e4dd9edb76a34ecbca539967f9662b8c0cc9c7fb&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 2.4%  | [0.3%, 18.8%]   | 18    |
| Regressions ❌ <br /> (secondary)  | 1.8%  | [0.2%, 4.1%]    | 21    |
| Improvements ✅ <br /> (primary)   | -1.0% | [-3.2%, -0.3%]  | 88    |
| Improvements ✅ <br /> (secondary) | -4.0% | [-13.1%, -0.1%] | 47    |
| All ❌✅ (primary)                 | -0.4% | [-3.2%, 18.8%]  | 106   |


3 Regressions, 3 Improvements, 8 Mixed; 3 of them in rollups
41 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-02-07.md)

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

* [disposition: merge] [rework min_choice algorithm of member constraints](https://github.com/rust-lang/rust/pull/105300)
* [disposition: merge] [fix: Unexpected trait bound not satisfied in HRTB and Associated Type](https://github.com/rust-lang/rust/pull/103695)
* [disposition: merge] [Stabilize feature `cstr_from_bytes_until_nul`](https://github.com/rust-lang/rust/pull/107429)
* [disposition: merge] [rustdoc: remove inconsistently-present sidebar tooltips](https://github.com/rust-lang/rust/pull/107490)
* [disposition: merge] [Relax ordering rules for `asm!` operands](https://github.com/rust-lang/rust/pull/105798)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [`recommended-bin-crates` field in `Cargo.toml`](https://github.com/rust-lang/rfcs/pull/3383)
* [new] [Ref-wrapping](https://github.com/rust-lang/rfcs/pull/3382)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-02-08 - 2023-03-08 🦀

### Virtual

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
* 2023-02-15 | Virtual | [MongoDB](https://www.mongodb.com/)
    * [**Write a Microservice With Rust and MongoDB**](https://www.mongodb.com/webinar/write-a-microservice-with-rust-and-mongodb)
* 2023-02-15 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US; São Paulo, BR) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224621/) | [**São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224623/)
* 2023-02-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Show & Tell: Rust state machines in a file processor**](https://www.meetup.com/vancouver-rust/events/tqvhxsyfcdbtb/)
* 2023-02-16 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsyfcdbvb/)
* 2023-02-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/vdhxgsyfcdbcc/)
* 2023-02-23 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Tock, a Rust based Embedded Operating System**](https://www.meetup.com/charlottesville-rust-meetup/events/291248593/)
* 2023-02-23 | Virtual (Kassel, DE) | [Java User Group Hessen](https://www.meetup.com/java-user-group-hessen-jugh/)
    * [**Eine Einführung in Rust (Stefan Baumgartner)**](https://www.meetup.com/java-user-group-hessen-jugh/events/290346591/)
* 2023-02-23 | Virtual (México City, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Rust: ¿por qué es una opción adecuada para implantar Blockchain?**](https://www.meetup.com/rust-mx/events/291456677/)
* 2023-02-28 | Virtual (Berlin, DE) | [Open Tech School Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/290852327/)
* 2023-02-28 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust Nation - What we learnt**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291338734/)
* 2023-02-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcdblc/)
* 2023-02-28 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/291437669/)
* 2023-03-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfcfbcb/)
* 2023-03-02 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 30th Edition**](https://www.meetup.com/rust-linz/events/291483339/)
* 2023-03-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcfbkb/)
* 2023-03-08 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/) 
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcfblb/)

### Asia

* 2023-02-14 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Crafting Command Line Tools With Rust**](https://www.meetup.com/tokyo-rust-meetup/events/291349232/)
* 2023-02-20 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**February Edition - Redis and BioCatch talking Rust!**](https://www.meetup.com/rust-tlv/events/291182881/)

### Europe

* 2023-02-09 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet)
    * [**Rust Lille #2**](https://www.meetup.com/meetup-group-zgphbyet/events/291046592/)
* 2023-02-15 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Nation Pre-Conference Reception with The Rust Foundation**](https://www.meetup.com/rust-london-user-group/events/290903823/)
* 2023-02-15 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim)
    * [**Rust New Year's Resolution Bug Hunt**](https://www.meetup.com/rust-trondheim/events/290889889/)
* 2023-02-16 | Bordeaux, FR | [DedoTalk](https://www.meetup.com/dedotalk/)
    * [**#1 DedoTalk 🎙️ : Rust pour un développeur Python**](https://www.meetup.com/dedotalk/events/291199962/)
* 2023-02-16, 2023-02-17 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation '23**](https://www.rustnationuk.com/)
* 2023-02-18 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Post-Conference Rust in Enterprise Brunch Hosted at Red Badger**](https://www.meetup.com/rust-london-user-group/events/291297886/)
* 2023-02-21 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #56**](https://www.meetup.com/rust-paris/events/291334081/)
* 2023-02-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Practical Cryptography - February Meetup (Registration opens 7 Feb 2023)**](https://www.meetup.com/de-DE/rust-zurich/events/290915075/)
* 2023-02-23 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust metup #33**](https://www.meetup.com/copenhagen-rust-community/events/291288154/)
* 2023-02-23 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Meetup Revived with an Exciting Exploration of Ownership!**](https://www.meetup.com/rust-vienna/events/291465732/)
* 2023-02-28 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/291437669/)

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

* 2023-02-23 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**February Meetup**](https://www.meetup.com/rust-brisbane/events/291377036/)
* 2023-02-28 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/291278417/)
* 2023-03-01 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 Lightning Talks - We are back!**](https://www.meetup.com/rust-sydney/events/291265163/)

### South America

* 2023-02-22 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/)
    * [**Hands on: Lifetimes**](https://www.meetup.com/rust-uruguay/events/291386143/)

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

> It's been 7.5 years since [#27060 ](https://github.com/rust-lang/rust/issues/27060) was reported, but the problem is finally fixed for good. :‍)

– [Ralf Jung on github](https://github.com/rust-lang/rust/issues/82523#issuecomment-1416850743)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1371) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/10xiw1a/this_week_in_rust_481/)</small>
