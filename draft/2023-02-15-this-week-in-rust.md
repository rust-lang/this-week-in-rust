Title: This Week in Rust 482
Number: 482
Date: 2023-02-15
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

This week's crate is [bkmr](https://github.com/sysid/bkmr), a fast CLI bookmark manager and launcher.

Thanks to [sysid](https://users.rust-lang.org/t/crate-of-the-week/2704/1159) for the self-suggestion!

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

387 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-02-06..2023-02-13

* [add `-Z instrument-xray` flag](https://github.com/rust-lang/rust/pull/102963)
* [add missing normalization for union fields types](https://github.com/rust-lang/rust/pull/106938)
* [add only modified subcommand for compiletest](https://github.com/rust-lang/rust/pull/107657)
* [add parentheses properly for borrowing suggestion](https://github.com/rust-lang/rust/pull/105019)
* [allow multiple candidates with same response in new solver](https://github.com/rust-lang/rust/pull/107863)
* [allow wasi-libc to initialize its environment variables lazily](https://github.com/rust-lang/rust/pull/107866)
* [avoid exposing type parameters and implementation details sourced from macro expansions](https://github.com/rust-lang/rust/pull/107789)
* [disqualify `auto trait` built-in impl in new solver if explicit `impl` exists](https://github.com/rust-lang/rust/pull/107815)
* [do not bring trait alias supertraits into scope](https://github.com/rust-lang/rust/pull/107803)
* [do not eagerly recover for bad `impl Trait` types in macros](https://github.com/rust-lang/rust/pull/107813)
* [enable new rlib in non stable cases](https://github.com/rust-lang/rust/pull/105601)
* [fix implied outlives bounds logic for projections](https://github.com/rust-lang/rust/pull/101680)
* [fix suggestions rendering when the diff span is multiline](https://github.com/rust-lang/rust/pull/107671)
* [implement `deferred_projection_equality` for erica solver](https://github.com/rust-lang/rust/pull/107507)
* [introduce `-Zterminal-urls` to use OSC8 for error codes](https://github.com/rust-lang/rust/pull/107838)
* [make &mut !Unpin not dereferenceable, and `Box<!Unpin>` not noalias](https://github.com/rust-lang/rust/pull/106180)
* [make `derive_const` derive properly const-if-const impls](https://github.com/rust-lang/rust/pull/107777)
* [mark `'atomic_mut_ptr'` methods const](https://github.com/rust-lang/rust/pull/107706)
* [mir-Opt for copying enums with large discrepancies](https://github.com/rust-lang/rust/pull/85158)
* [optimize `TyKind::eq`](https://github.com/rust-lang/rust/pull/107717)
* [optimize `query_cache_hit` to reduce code size of the query hot path](https://github.com/rust-lang/rust/pull/107529)
* [reduce interning](https://github.com/rust-lang/rust/pull/107869)
* [simplify layout calculations in rawvec](https://github.com/rust-lang/rust/pull/107167)
* [suggest function call on pattern type mismatch](https://github.com/rust-lang/rust/pull/107098)
* [support `true` and `false` as boolean flag params](https://github.com/rust-lang/rust/pull/107043)
* [turn projections into copies in `CopyProp`](https://github.com/rust-lang/rust/pull/107662)
* [unused-lifetimes: don't warn about lifetimes originating from expanded code](https://github.com/rust-lang/rust/pull/107648)
* [implement `AsFd` and `AsRawFd` for `Rc`](https://github.com/rust-lang/rust/pull/107317)
* [implement cursors for BTreeMap](https://github.com/rust-lang/rust/pull/105641)
* [improve the `array::map` codegen](https://github.com/rust-lang/rust/pull/107634)
* [reverse Timsort scan direction](https://github.com/rust-lang/rust/pull/107191)
* [speedup heapsort by 1.5x by making it branchless](https://github.com/rust-lang/rust/pull/107894)
* [stabilize feature `cstr_from_bytes_until_nul`](https://github.com/rust-lang/rust/pull/107429)
* [stop at the first `NULL` argument when iterating `argv`](https://github.com/rust-lang/rust/pull/106001)
* [cargo: `-Zrustdoc-scrape-example` must fail with bad build script](https://github.com/rust-lang/cargo/pull/11694)
* [cargo: add '-C' flag for changing current dir before build](https://github.com/rust-lang/cargo/pull/10952)
* [cargo: re-export `cargo_new::NewProjectKind` as public](https://github.com/rust-lang/cargo/pull/11700)
* [clippy: make `arithmetic_side_effects` mind constant items](https://github.com/rust-lang/rust-clippy/pull/10310)
* [clippy: `cast_possible_truncation`: issue proper help message](https://github.com/rust-lang/rust-clippy/pull/10330)
* [clippy: `suspicious_to_owned`: use `span_suggestions` to suggest both intents](https://github.com/rust-lang/rust-clippy/pull/10295)
* [clippy: add `suspicious_command_arg_space` lint](https://github.com/rust-lang/rust-clippy/pull/10317)
* [clippy: `almost_swapped`: detect almost-swaps using `let` statements](https://github.com/rust-lang/rust-clippy/pull/10177)
* [clippy: negate suggestions when needed in `bool_assert_comparison`](https://github.com/rust-lang/rust-clippy/pull/10293)
* [rust-analyzer: add braces assist](https://github.com/rust-lang/rust-analyzer/pull/13991)
* [rust-analyzer: add postfix completion for `unsafe`](https://github.com/rust-lang/rust-analyzer/pull/14095)
* [rust-analyzer: add setting for limiting number of completions](https://github.com/rust-lang/rust-analyzer/pull/13986)
* [rust-analyzer: build `i686-pc-windows-msvc` binaries](https://github.com/rust-lang/rust-analyzer/pull/14127)
* [rust-analyzer: don't include `r#` prefix in filesystem changes](https://github.com/rust-lang/rust-analyzer/pull/14138)
* [rust-analyzer: don't insert a semicolon when typing = if parse errors are encountered](https://github.com/rust-lang/rust-analyzer/pull/14103)
* [rust-analyzer: fix bind pat inlay hints rendering for constant patterns](https://github.com/rust-lang/rust-analyzer/pull/14125)
* [rust-analyzer: fix completions after functions with no bodies](https://github.com/rust-lang/rust-analyzer/pull/14110)
* [rust-analyzer: fix parsing of nested tuple field accesses in a cursed way](https://github.com/rust-lang/rust-analyzer/pull/14084)
* [rust-analyzer: fix proc-macro-server incorrectly stripping delimiters](https://github.com/rust-lang/rust-analyzer/pull/14140)
* [rust-analyzer: insert spaces when inlining macros](https://github.com/rust-lang/rust-analyzer/pull/14114)
* [rust-analyzer: properly use location links for type hints of impl Future and its assoc type](https://github.com/rust-lang/rust-analyzer/pull/14099)
* [rust-analyzer: suppress extra indent after the end of field and function chains](https://github.com/rust-lang/rust-analyzer/pull/13975)
* [rust-analyzer: support `DidChangeWorkspaceFolders` notifications](https://github.com/rust-lang/rust-analyzer/pull/14098)

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

Rusty Events between 2023-02-15 - 2023-03-15 🦀

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
* 2023-03-09 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Rust Meetup #7**](https://www.meetup.com/rust-basel/events/291228934/)

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

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> All the pro C/C++ arguments seem to come down to "Good drivers don’t need seat belts because they don’t get in accidents"

– [otwkme on /r/rust](https://www.reddit.com/r/rust/comments/10rnymj/comment/j6x90x5)

Thanks to [azzamsa](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1372) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
