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

* [Building an actor library for Quickwit's indexing pipeline](https://fosdem.org/2023/schedule/event/building_an_actor_library_for_quickwits_indexing_pipeline/)
* [Building a distributed search engine with tantivy](https://fosdem.org/2023/schedule/event/rust_building_a_distributed_search_engine_with_tantivy/)
* [Aurae: Distributed Runtime](https://fosdem.org/2023/schedule/event/rust_aurae_a_new_pid_1_for_distributed_systems/)
* [BastionLab, a Rust open-source privacy framework for confidential data science collaboration](https://fosdem.org/2023/schedule/event/rust_bastionlab/)
* [Neovim and rust-analyzer are best friends](https://fosdem.org/2023/schedule/event/rust_neovim_and_rust_analyzer_are_best_friends/)
* [A Rusty CHERI - The path to hardware capabilities in Rust](https://fosdem.org/2023/schedule/event/rust_a_rusty_cheri_the_path_to_hardware_capabilities_in_rust/)
* [Slint: Are we GUI yet?](https://fosdem.org/2023/schedule/event/rust_slint_are_we_gui_yet/)
* [Rust API Design Learnings](https://fosdem.org/2023/schedule/event/rust_rust_api_design_learnings/)
* [A deep dive inside the Rust frontend for GCC](https://fosdem.org/2023/schedule/event/rust_a_deep_dive_inside_the_rust_frontend_for_gcc/)
* [Merging process of the rust compiler](https://fosdem.org/2023/schedule/event/rust_merging_process_of_the_rust_compiler/)
* [Let's write Snake game!](https://fosdem.org/2023/schedule/event/rust_lets_write_snake_game/)
* [Glidesort](https://fosdem.org/2023/schedule/event/rust_glidesort/)
* [How Pydantic V2 leverages Rust's Superpowers](https://fosdem.org/2023/schedule/event/rust_how_pydantic_v2_leverages_rusts_superpowers/)
* [Scalable graph algorithms in Rust (and Python)](https://fosdem.org/2023/schedule/event/rust_scalable_graph_algorithms_in_rust_and_python/)
* [Using Rust for your network management tools!](https://fosdem.org/2023/schedule/event/rust_using_rust_for_your_network_management_tools/)
* [Backward and forward compatibility for security features](https://fosdem.org/2023/schedule/event/rust_backward_and_forward_compatibility_for_security_features/)
* [atuin: magical shell history with Rust](https://fosdem.org/2023/schedule/event/rust_atuin_magical_shell_history_with_rust/)
* [RustyHermit @ FOSDEM 2023: A Rust-Based, Modular Unikernel For MicroVMs](https://fosdem.org/2023/schedule/event/rustunikernel/)

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

### Research

### Miscellaneous

* [My Reaction to Dr. Stroustrup's Recent Memory Safety Comments](https://www.thecodedmessage.com/posts/stroustrup-response/)
* [This Is the Kind of Rust the Automotive Industry Needs](https://blogs.blackberry.com/en/2023/02/this-is-the-kind-of-rust-the-automotive-industry-needs)

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

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

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
