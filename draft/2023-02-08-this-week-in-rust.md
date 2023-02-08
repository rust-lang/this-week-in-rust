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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [envious](https://github.com/TheNeikos/envious), a serde-based deserializer from environment variables.

Thanks to [musicmatze](https://users.rust-lang.org/t/crate-of-the-week/2704/1156) for the suggestion!

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

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

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

> It's been 7.5 years since [#27060 ](https://github.com/rust-lang/rust/issues/27060) was reported, but the problem is finally fixed for good. :‍)

– [Ralf Jung on github](https://github.com/rust-lang/rust/issues/82523#issuecomment-1416850743)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1371) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
